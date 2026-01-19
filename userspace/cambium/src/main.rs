#![no_std]
#![no_main]

extern crate alloc;

use stem::info;
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
}

/// Parse a batch and find SET_PROP operations on the given subject.
/// Returns the value of the last SET_PROP found (for simplicity).
fn find_set_prop_value(batch: &[u8], subject: u64) -> Option<u64> {
    // Minimal batch parsing: header (8 bytes) then ops
    if batch.len() < 8 {
        return None;
    }
    let magic = u32::from_le_bytes(batch[0..4].try_into().ok()?);
    let version = u16::from_le_bytes(batch[4..6].try_into().ok()?);
    let op_count = u16::from_le_bytes(batch[6..8].try_into().ok()?) as usize;

    if magic != BATCH_MAGIC || version != BATCH_VERSION {
        return None;
    }

    let mut cursor = 8usize;
    let mut result = None;

    for _ in 0..op_count {
        if cursor >= batch.len() {
            break;
        }
        let tag = batch[cursor];
        cursor += 1;

        match tag {
            OP_SET_PROP => {
                // SET_PROP format: ThingRef + Key(16) + Value(8)
                if cursor >= batch.len() {
                    break;
                }
                let ref_kind = batch[cursor];
                cursor += 1;

                let (subj, ref_size) = if ref_kind == REF_ABSOLUTE {
                    if cursor + 16 > batch.len() {
                        break;
                    }
                    let id = u64::from_le_bytes(batch[cursor..cursor+8].try_into().ok()?);
                    (id, 16)
                } else {
                    // Local ref - skip (we only care about absolute refs to our subject)
                    (0, 2)
                };
                cursor += ref_size;

                // Key: 16 bytes
                if cursor + 16 > batch.len() {
                    break;
                }
                cursor += 16;

                // Value: 8 bytes
                if cursor + 8 > batch.len() {
                    break;
                }
                let value = u64::from_le_bytes(batch[cursor..cursor+8].try_into().ok()?);
                cursor += 8;

                if subj == subject {
                    result = Some(value);
                }
            }
            0x01 => {
                // CREATE_NODE: kind(16) + out_ref(2) = 18 bytes
                cursor += 18;
            }
            0x02 => {
                // PUT_EDGE: subject ThingRef + predicate(16) + object ThingRef + flags(4)
                // Variable size due to ThingRefs
                if cursor >= batch.len() {
                    break;
                }
                let src_kind = batch[cursor];
                cursor += 1;
                cursor += if src_kind == REF_ABSOLUTE { 16 } else { 2 };
                cursor += 16; // predicate
                if cursor >= batch.len() {
                    break;
                }
                let dst_kind = batch[cursor];
                cursor += 1;
                cursor += if dst_kind == REF_ABSOLUTE { 16 } else { 2 };
                cursor += 4; // flags
            }
            _ => {
                // Unknown op, stop parsing
                break;
            }
        }
    }

    result
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
    
    for binding in &mut bindings {
        let res = stem::root_watch::watch_drain(binding.watch_id, &mut batch_buf, |seq, batch| {
             if let Some(value) = find_set_prop_value(batch, binding.source.to_u64_lossy()) {
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
        });
        
        match res {
            Ok(stats) => {
                if stats.batches > 0 || stats.overflows > 0 {
                     info!("CATCH-UP: Watch {} drained {} batches (overflows={})", binding.watch_id, stats.batches, stats.overflows);
                }
                total_drained += stats.batches;
                total_overflows += stats.overflows;
            }
            Err(e) => {
                info!("CATCH-UP: Watch {} drain error: {:?}", binding.watch_id, e);
            }
        }
    }
    
    info!("CATCH-UP complete: {} total batches processed, {} overflows", total_drained, total_overflows);

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
                    if let Some(value) = find_set_prop_value(&batch_buf[..len], binding.source.to_u64_lossy()) {
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
                    } else {
                        // Log unrecognized event for debugging
                        let hex_preview: alloc::string::String = batch_buf[..core::cmp::min(len, 32)]
                            .iter()
                            .map(|b| alloc::format!("{:02x}", b))
                            .collect();
                        info!(
                            "Unrecognized event: seq={} len={} preview={}",
                            seq, len, hex_preview
                        );
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
