#![no_std]
#![no_main]

extern crate alloc;

use stem::{info, warn};
use stem::thing::ThingId;
use stem::thing::sys::{find, prop_get, prop_set};
use stem::syscall::{root_watch_open, root_watch_next};
use abi::schema::{kinds, keys};
use abi::types::WatchSpec;
use abi::root::{RootWatchFilter, OP_SET_PROP, REF_ABSOLUTE, BATCH_MAGIC, BATCH_VERSION};
use abi::ids::HandleId;
use alloc::vec::Vec;
use core::time::Duration;

struct ActiveBinding {
    source: ThingId,
    target: ThingId,
    watch_id: usize,
    /// Cached last value (for initial sync)
    last_value: Option<u64>,
    key_filter: Option<u32>,
}

#[derive(Clone, Copy)]
enum BatchParseError {
    BadHeader { magic: u32, version: u16 },
    Truncated,
    UnknownOp(u8),
    InvalidRefKind(u8),
}

fn log_unknown_shape_once(err: BatchParseError, len: usize) {
    use core::sync::atomic::{AtomicU64, Ordering};

    const ISSUE_BAD_HEADER: u64 = 1 << 0;
    const ISSUE_TRUNCATED: u64 = 1 << 1;

    static ISSUE_FLAGS: AtomicU64 = AtomicU64::new(0);
    static UNKNOWN_TAGS: AtomicU64 = AtomicU64::new(0);
    static INVALID_REF_KINDS: AtomicU64 = AtomicU64::new(0);

    match err {
        BatchParseError::BadHeader { magic, version } => {
            let prev = ISSUE_FLAGS.fetch_or(ISSUE_BAD_HEADER, Ordering::Relaxed);
            if prev & ISSUE_BAD_HEADER == 0 {
                warn!(
                    "cambium: unknown event shape: bad batch header magic=0x{:08x} version={} len={}",
                    magic, version, len
                );
            }
        }
        BatchParseError::Truncated => {
            let prev = ISSUE_FLAGS.fetch_or(ISSUE_TRUNCATED, Ordering::Relaxed);
            if prev & ISSUE_TRUNCATED == 0 {
                warn!(
                    "cambium: unknown event shape: truncated batch payload len={}",
                    len
                );
            }
        }
        BatchParseError::UnknownOp(tag) => {
            if tag < 64 {
                let bit = 1u64 << tag;
                let prev = UNKNOWN_TAGS.fetch_or(bit, Ordering::Relaxed);
                if prev & bit == 0 {
                    warn!(
                        "cambium: unknown event shape: unknown op tag=0x{:02x}",
                        tag
                    );
                }
            } else {
                warn!(
                    "cambium: unknown event shape: unknown op tag=0x{:02x}",
                    tag
                );
            }
        }
        BatchParseError::InvalidRefKind(kind) => {
            if kind < 64 {
                let bit = 1u64 << kind;
                let prev = INVALID_REF_KINDS.fetch_or(bit, Ordering::Relaxed);
                if prev & bit == 0 {
                    warn!(
                        "cambium: unknown event shape: invalid thing ref kind=0x{:02x}",
                        kind
                    );
                }
            } else {
                warn!(
                    "cambium: unknown event shape: invalid thing ref kind=0x{:02x}",
                    kind
                );
            }
        }
    }
}

fn parse_ref(batch: &[u8], cursor: &mut usize) -> Result<u64, BatchParseError> {
    if *cursor >= batch.len() {
        return Err(BatchParseError::Truncated);
    }
    let kind = batch[*cursor];
    *cursor += 1;
    match kind {
        REF_ABSOLUTE => {
            if *cursor + 16 > batch.len() {
                return Err(BatchParseError::Truncated);
            }
            let id = u64::from_le_bytes(batch[*cursor..*cursor + 8].try_into().unwrap());
            *cursor += 16;
            Ok(id)
        }
        abi::root::REF_LOCAL => {
            if *cursor + 2 > batch.len() {
                return Err(BatchParseError::Truncated);
            }
            *cursor += 2;
            Ok(0)
        }
        _ => Err(BatchParseError::InvalidRefKind(kind)),
    }
}

/// Parse a batch and find SET_PROP operations on the given subject.
/// Returns the value of the last SET_PROP found (for simplicity).
fn find_set_prop_value(
    batch: &[u8],
    subject: u64,
    key_filter: Option<u32>,
) -> Result<Option<u64>, BatchParseError> {
    // Minimal batch parsing: header (8 bytes) then ops
    if batch.len() < 8 {
        return Err(BatchParseError::Truncated);
    }
    let magic = u32::from_le_bytes(batch[0..4].try_into().unwrap());
    let version = u16::from_le_bytes(batch[4..6].try_into().unwrap());
    let op_count = u16::from_le_bytes(batch[6..8].try_into().unwrap()) as usize;

    if magic != BATCH_MAGIC || version != BATCH_VERSION {
        return Err(BatchParseError::BadHeader { magic, version });
    }

    let mut cursor = 8usize;
    let mut result = None;

    for _ in 0..op_count {
        if cursor >= batch.len() {
            return Err(BatchParseError::Truncated);
        }
        let tag = batch[cursor];
        cursor += 1;

        match tag {
            OP_SET_PROP => {
                // SET_PROP format: ThingRef + Key(16) + Value(8)
                let subj = parse_ref(batch, &mut cursor)?;
                if cursor + 16 + 8 > batch.len() {
                    return Err(BatchParseError::Truncated);
                }
                let key_id = u32::from_le_bytes(batch[cursor..cursor + 4].try_into().unwrap());
                cursor += 16; // key
                let value = u64::from_le_bytes(batch[cursor..cursor + 8].try_into().unwrap());
                cursor += 8;

                if subj == subject && subj != 0 && key_filter.map_or(true, |k| k == key_id) {
                    result = Some(value);
                }
            }
            0x01 => {
                // CREATE_NODE: kind(16) + out_ref(2) = 18 bytes
                if cursor + 18 > batch.len() {
                    return Err(BatchParseError::Truncated);
                }
                cursor += 18;
            }
            0x02 => {
                // PUT_EDGE: subject ThingRef + predicate(16) + object ThingRef
                let _src = parse_ref(batch, &mut cursor)?;
                if cursor + 16 > batch.len() {
                    return Err(BatchParseError::Truncated);
                }
                cursor += 16; // predicate
                let _dst = parse_ref(batch, &mut cursor)?;
            }
            _ => {
                return Err(BatchParseError::UnknownOp(tag));
            }
        }
    }

    Ok(result)
}

// `drain_watch` removed, replaced by `stem::root_watch::watch_drain`


#[stem::main]
fn main() -> ! {
    info!("cambium starting (v3: catch-up then stream)...");

    let mut bindings: Vec<ActiveBinding> = Vec::new();

    // Initial scan for bindings
    let mut binding_ids = [ThingId::default(); 16];

    for _ in 0..120 {
        if let Ok(count) = find(kinds::BINDING, &mut binding_ids) {
            if count > 0 {
                info!("Found {} bindings", count);
                for i in 0..count {
                    let b_id = binding_ids[i];

                    let src_id = prop_get(b_id, keys::BINDING_SOURCE)
                        .map(ThingId::from_u64)
                        .unwrap_or(ThingId::default());
                    let dst_id = prop_get(b_id, keys::BINDING_TARGET)
                        .map(ThingId::from_u64)
                        .unwrap_or(ThingId::default());
                    let key_filter = prop_get(b_id, keys::BINDING_MAP)
                        .ok()
                        .and_then(|v| if v == 0 { None } else { Some(v as u32) });

                    if src_id.to_u64_lossy() == 0 || dst_id.to_u64_lossy() == 0 {
                        continue;
                    }

                    // Create a Root watch with subject filter
                    // Use start_seq=0 to catch up from oldest available
                    let filter = RootWatchFilter::subject(src_id.to_u64_lossy());
                    let spec = WatchSpec {
                        mode: 1, // StreamOnly
                        start_seq: 0, // Catch-up from oldest available (not WATCH_START_LATEST)
                        filter_ptr: &filter as *const _ as u64,
                        filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
                        ..Default::default()
                    };

                    match root_watch_open(&spec) {
                        Ok(watch_id) => {
                            info!(
                                "Opened watch {} for source {} (binding {}, start_seq=0)",
                                watch_id,
                                src_id.to_u64_lossy(),
                                b_id.to_u64_lossy()
                            );
                            bindings.push(ActiveBinding {
                                source: src_id,
                                target: dst_id,
                                watch_id,
                                last_value: None,
                                key_filter,
                            });
                        }
                        Err(e) => {
                            info!("Failed to open watch for source {}: {:?}", src_id.to_u64_lossy(), e);
                        }
                    }
                }
                break;
            }
        }
        stem::sleep(Duration::from_secs(1));
    }

    if bindings.is_empty() {
        info!("No bindings found after retries. Exiting.");
        loop {
            stem::sleep(Duration::from_secs(10));
        }
    }

    // ============================================================
    // PHASE 1: Drain all watches to catch up on historical events
    // ============================================================
    info!("CATCH-UP: Draining {} watches for historical events...", bindings.len());
    let mut batch_buf = [0u8; 4096];
    let mut total_drained = 0usize;
    let mut total_overflows = 0usize;
    let mut last_seq_max: Option<u64> = None;
    
    for binding in &mut bindings {
        let res = stem::root_watch::watch_drain(binding.watch_id, &mut batch_buf, |seq, batch| {
            match find_set_prop_value(
                batch,
                binding.source.to_u64_lossy(),
                binding.key_filter,
            ) {
                Ok(Some(value)) => {
                    binding.last_value = Some(value);
                    if prop_set(binding.target, keys::UI_TEXT, value).is_ok() {
                        info!(
                            "DRAIN: Updated target {} with value {} (seq={})",
                            binding.target.to_u64_lossy(),
                            value,
                            seq
                        );
                    }
                }
                Ok(None) => {}
                Err(e) => {
                    log_unknown_shape_once(e, batch.len());
                }
            }
        });
        
        match res {
            Ok(stats) => {
                total_drained += stats.batches;
                total_overflows += stats.overflows;
                if let Some(seq) = stats.last_seq {
                    last_seq_max = Some(match last_seq_max {
                        Some(current) => core::cmp::max(current, seq),
                        None => seq,
                    });
                }
            }
            Err(e) => {
                info!("cambium: drain error on watch {}: {:?}", binding.watch_id, e);
            }
        }
    }
    
    match last_seq_max {
        Some(seq) => {
            info!(
                "cambium: drain complete batches={} overflows={} last_seq={}",
                total_drained, total_overflows, seq
            );
        }
        None => {
            info!(
                "cambium: drain complete batches={} overflows={} last_seq=none",
                total_drained, total_overflows
            );
        }
    }

    // ============================================================
    // PHASE 2: Enter steady-state event loop
    // ============================================================
    info!("Entering event loop with {} bindings (v3)", bindings.len());

    loop {
        let mut did_work = false;

        for binding in &mut bindings {
            let mut seq: u64 = 0;

            match root_watch_next(binding.watch_id, &mut seq, &mut batch_buf) {
                Ok(len) if len > 0 => {
                    did_work = true;

                    // Parse batch to find SET_PROP value for our subject
                    match find_set_prop_value(
                        &batch_buf[..len],
                        binding.source.to_u64_lossy(),
                        binding.key_filter,
                    ) {
                        Ok(Some(value)) => {
                            binding.last_value = Some(value);
                            // Update target's UI_TEXT property
                            if prop_set(binding.target, keys::UI_TEXT, value).is_ok() {
                                info!(
                                    "Updated target {} with value {} (seq={})",
                                    binding.target.to_u64_lossy(),
                                    value,
                                    seq
                                );
                            }
                        }
                        Ok(None) => {}
                        Err(e) => {
                            log_unknown_shape_once(e, len);
                        }
                    }
                }
                Ok(_) => {
                    // No events or zero-length batch
                }
                Err(abi::errors::Errno::EAGAIN) => {
                    // No pending events
                }
                Err(abi::errors::Errno::EOVERFLOW) => {
                    info!("Watch {} overflow, resyncing", binding.watch_id);
                }
                Err(e) => {
                    info!("watch_next error: {:?}", e);
                }
            }
        }

        if !did_work {
            stem::sleep(Duration::from_millis(50));
        }
    }
}
