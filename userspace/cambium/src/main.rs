#![no_std]
#![no_main]

extern crate alloc;

use stem::{info, warn};
use stem::thing::ThingId;
use stem::thing::sys::{bytespace_create, bytespace_write, find, prop_get, prop_set};
use stem::syscall::{root_watch_open, root_watch_next};
use abi::schema::{kinds, keys};
use abi::types::WatchSpec;
use abi::root::RootWatchFilter;
use abi::watch::{self, DecodeError, ValueEncoding, WatchOp};
use abi::ids::HandleId;
use alloc::vec::Vec;
use alloc::string::String;
use core::time::Duration;

struct ActiveBinding {
    source: ThingId,
    target: ThingId,
    watch_id: usize,
    /// Cached last value (for initial sync)
    last_value: Option<u64>,
    key_filter: Option<u32>,
}

fn hex_prefix(bytes: &[u8], max: usize) -> String {
    let mut out = String::new();
    let take = core::cmp::min(bytes.len(), max);
    for &b in &bytes[..take] {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0xf) as usize] as char);
    }
    out
}

fn log_unknown_shape_once(err: DecodeError, payload: &[u8], seq: u64) {
    use core::sync::atomic::{AtomicU64, Ordering};

    const ISSUE_BAD_VERSION: u64 = 1 << 0;
    const ISSUE_UNKNOWN_OP: u64 = 1 << 1;
    const ISSUE_UNKNOWN_ENCODING: u64 = 1 << 2;
    const ISSUE_BAD_LENGTH: u64 = 1 << 3;
    const ISSUE_INVALID_UTF8: u64 = 1 << 4;
    const ISSUE_NONZERO_FLAGS: u64 = 1 << 5;

    static ISSUE_FLAGS: AtomicU64 = AtomicU64::new(0);

    static WATCH_DECODE_ERRORS_TOTAL: AtomicU64 = AtomicU64::new(0);
    static WATCH_DECODE_BAD_VERSION_TOTAL: AtomicU64 = AtomicU64::new(0);
    static WATCH_DECODE_UNKNOWN_OP_TOTAL: AtomicU64 = AtomicU64::new(0);
    static WATCH_DECODE_UNKNOWN_ENCODING_TOTAL: AtomicU64 = AtomicU64::new(0);
    static WATCH_DECODE_BAD_LENGTH_TOTAL: AtomicU64 = AtomicU64::new(0);
    static WATCH_DECODE_INVALID_UTF8_TOTAL: AtomicU64 = AtomicU64::new(0);
    static WATCH_DECODE_NONZERO_FLAGS_TOTAL: AtomicU64 = AtomicU64::new(0);

    WATCH_DECODE_ERRORS_TOTAL.fetch_add(1, Ordering::Relaxed);

    let prefix = hex_prefix(payload, 16);
    match err {
        DecodeError::BadVersion(version) => {
            WATCH_DECODE_BAD_VERSION_TOTAL.fetch_add(1, Ordering::Relaxed);
            let prev = ISSUE_FLAGS.fetch_or(ISSUE_BAD_VERSION, Ordering::Relaxed);
            if prev & ISSUE_BAD_VERSION == 0 {
                warn!(
                    "cambium: unknown event shape: bad version={} len={} seq={} hex_prefix={}",
                    version,
                    payload.len(),
                    seq,
                    prefix
                );
            }
        }
        DecodeError::UnknownOp(op) => {
            WATCH_DECODE_UNKNOWN_OP_TOTAL.fetch_add(1, Ordering::Relaxed);
            let prev = ISSUE_FLAGS.fetch_or(ISSUE_UNKNOWN_OP, Ordering::Relaxed);
            if prev & ISSUE_UNKNOWN_OP == 0 {
                warn!(
                    "cambium: unknown event shape: unknown op=0x{:02x} len={} seq={} hex_prefix={}",
                    op,
                    payload.len(),
                    seq,
                    prefix
                );
            }
        }
        DecodeError::UnknownEncoding(encoding) => {
            WATCH_DECODE_UNKNOWN_ENCODING_TOTAL.fetch_add(1, Ordering::Relaxed);
            let prev = ISSUE_FLAGS.fetch_or(ISSUE_UNKNOWN_ENCODING, Ordering::Relaxed);
            if prev & ISSUE_UNKNOWN_ENCODING == 0 {
                warn!(
                    "cambium: unknown event shape: unknown encoding=0x{:02x} len={} seq={} hex_prefix={}",
                    encoding,
                    payload.len(),
                    seq,
                    prefix
                );
            }
        }
        DecodeError::BadLength { expected, got } => {
            WATCH_DECODE_BAD_LENGTH_TOTAL.fetch_add(1, Ordering::Relaxed);
            let prev = ISSUE_FLAGS.fetch_or(ISSUE_BAD_LENGTH, Ordering::Relaxed);
            if prev & ISSUE_BAD_LENGTH == 0 {
                warn!(
                    "cambium: unknown event shape: bad length expected={} got={} seq={} hex_prefix={}",
                    expected,
                    got,
                    seq,
                    prefix
                );
            }
        }
        DecodeError::InvalidUtf8 => {
            WATCH_DECODE_INVALID_UTF8_TOTAL.fetch_add(1, Ordering::Relaxed);
            let prev = ISSUE_FLAGS.fetch_or(ISSUE_INVALID_UTF8, Ordering::Relaxed);
            if prev & ISSUE_INVALID_UTF8 == 0 {
                warn!(
                    "cambium: unknown event shape: invalid utf8 len={} seq={} hex_prefix={}",
                    payload.len(),
                    seq,
                    prefix
                );
            }
        }
        DecodeError::NonZeroFlags(flags) => {
            WATCH_DECODE_NONZERO_FLAGS_TOTAL.fetch_add(1, Ordering::Relaxed);
            let prev = ISSUE_FLAGS.fetch_or(ISSUE_NONZERO_FLAGS, Ordering::Relaxed);
            if prev & ISSUE_NONZERO_FLAGS == 0 {
                warn!(
                    "cambium: unknown event shape: nonzero flags=0x{:04x} len={} seq={} hex_prefix={}",
                    flags,
                    payload.len(),
                    seq,
                    prefix
                );
            }
        }
    }
}

fn set_string_prop(id: ThingId, key_name: &str, value: &str) {
    if value.is_empty() {
        prop_set(id, key_name, 0).ok();
        return;
    }
    let bs_id = bytespace_create(value.len(), 0, 0).expect("create bytespace");
    bytespace_write(bs_id, 0, value.as_bytes()).ok();
    prop_set(id, key_name, bs_id.to_u64_lossy()).ok();
}

fn apply_watch_payload(payload: &[u8], binding: &mut ActiveBinding, seq: u64) {
    let mut cursor = 0usize;

    while cursor < payload.len() {
        match watch::decode_event(&payload[cursor..]) {
            Ok((header, value)) => {
                let event_len = watch::WATCH_EVENT_HEADER_LEN + value.len();
                cursor += event_len;

                let op = WatchOp::from_u8(header.op).unwrap_or(WatchOp::Upsert);
                if op != WatchOp::Upsert {
                    continue;
                }

                let subject = header.subject.to_u64_lossy();
                if subject == 0 || subject != binding.source.to_u64_lossy() {
                    continue;
                }

                let predicate = header.predicate.to_u32_lossy();
                if let Some(filter) = binding.key_filter {
                    if predicate != filter {
                        continue;
                    }
                }

                let encoding = ValueEncoding::from_u8(header.value_encoding)
                    .unwrap_or(ValueEncoding::Bytes);
                match encoding {
                    ValueEncoding::U64LE => {
                        if value.len() != 8 {
                            log_unknown_shape_once(
                                DecodeError::BadLength { expected: 8, got: value.len() },
                                payload,
                                seq,
                            );
                            break;
                        }

                        let next_value = u64::from_le_bytes(value.try_into().unwrap());
                        binding.last_value = Some(next_value);
                        if prop_set(binding.target, keys::UI_TEXT, next_value).is_ok() {
                            info!(
                                "Updated target {} with value {} (seq={})",
                                binding.target.to_u64_lossy(),
                                next_value,
                                seq
                            );
                        }
                    }
                    ValueEncoding::Utf8 => {
                        if let Ok(text) = core::str::from_utf8(value) {
                            set_string_prop(binding.target, keys::UI_TEXT, text);
                            binding.last_value = None;
                            info!(
                                "Updated target {} with text '{}' (seq={})",
                                binding.target.to_u64_lossy(),
                                text,
                                seq
                            );
                        } else {
                            log_unknown_shape_once(DecodeError::InvalidUtf8, payload, seq);
                            break;
                        }
                    }
                    _ => {
                        continue;
                    }
                }
            }
            Err(e) => {
                log_unknown_shape_once(e, payload, seq);
                break;
            }
        }
    }
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
    let mut payload_buf = [0u8; 4096];
    let mut total_drained = 0usize;
    let mut total_overflows = 0usize;
    let mut last_seq_max: Option<u64> = None;
    
    for binding in &mut bindings {
        let res = stem::root_watch::watch_drain(binding.watch_id, &mut payload_buf, |seq, payload| {
            apply_watch_payload(payload, binding, seq);
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
                "cambium: drain complete payloads={} overflows={} last_seq={}",
                total_drained, total_overflows, seq
            );
        }
        None => {
            info!(
                "cambium: drain complete payloads={} overflows={} last_seq=none",
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

            match root_watch_next(binding.watch_id, &mut seq, &mut payload_buf) {
                Ok(len) if len > 0 => {
                    did_work = true;

                    apply_watch_payload(&payload_buf[..len], binding, seq);
                }
                Ok(_) => {
                    // No events or zero-length payload
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
