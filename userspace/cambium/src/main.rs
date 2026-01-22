#![no_std]
#![no_main]

extern crate alloc;

use stem::{info, warn};
use stem::thing::{ThingId, HandleId};
use stem::thing::sys::{bytespace_create, bytespace_write, find, prop_get, prop_set};
use stem::syscall::{root_watch_open, root_watch_next, root_watch_close};
use stem::thing::symbol::IntoSymbolRef;
use abi::schema::{kinds, keys};
use abi::types::{WatchSpec, WATCH_START_LATEST};
use abi::root::RootWatchFilter;
use abi::watch::{self, DecodeError, ValueEncoding, WatchOp};
use alloc::vec::Vec;
use alloc::string::String;
use core::time::Duration;

// ============================================================================
// Constants & Configuration
// ============================================================================

/// Maximum events to process for a single binding in one tick before yielding
const MAX_EVENTS_PER_BINDING_PER_TICK: usize = 256;

/// Maximum total events to process across all bindings in one tick
const MAX_EVENTS_PER_TICK_TOTAL: usize = 4096;

/// Minimum time between resync attempts for a specific binding
const RESYNC_COOLDOWN_MS: u64 = 250;

// ============================================================================
// Active Binding State
// ============================================================================

struct ActiveBinding {
    source: ThingId,
    target: ThingId,
    watch_id: usize,
    
    /// Cached last written numeric value (for Dedup)
    last_value_u64: Option<u64>,
    /// Cached last written string bytes (for Dedup / String Interning)
    last_value_bytes: Option<Vec<u8>>,
    
    key_filter: Option<u32>,
    to_key: u32,
    
    // --- Counters & Metrics ---
    overflow_count: u64,
    resync_count: u64,
    drained_events_total: u64,
    last_resync_time_ms: u64,
}

impl ActiveBinding {
    /// Perform an immediate state sync from Source -> Target
    /// 
    /// Used during Resync to ensure the target reflects the current reality
    /// without replaying history.
    fn refresh_now(&mut self) {
        // 1. Read current value from source
        // Note: This logic currently assumes a simple 1:1 binding where the source
        // property is what we want. If key_filter is set, we use that.
        // If not, we might be binding "all" props, which refresh_now can't easily handle
        // without scanning. However, existing logic implies specific key binding.
        
        let src_key = if let Some(k) = self.key_filter {
            k
        } else {
            // If no filter, we can't easily know WHICH source prop drove this.
            // But standard simple bindings usually have a map.
            // Fallback: we can't reliably refresh a wildcard binding without scanning keys.
            // For now, let's assume if key_filter is None, we skip refresh value logic
            // (or we'd need to list props).
            return;
        };

        let target_key = if self.to_key != 0 {
            self.to_key
        } else {
            stem::thing::sys::intern(keys::UI_TEXT).unwrap_or(0)
        };
        
        // 2. Determine value type (heuristic based on existing cache or try u64 first)
        // Try getting as U64 first
        if let Ok(val) = prop_get(self.source, src_key) {
             // DEDUP: Check against last known value
            if self.last_value_u64 == Some(val) {
                return;
            }
            
            // Apply
            if prop_set(self.target, target_key, val).is_ok() {
                self.last_value_u64 = Some(val);
                self.last_value_bytes = None;
                
                info!(
                    "[cambium] refresh: binding_src={} target={} val={}",
                    self.source.to_u64_lossy(),
                    self.target.to_u64_lossy(),
                    val
                );
            }
        } else {
            // Try as string/bytespace?
            // (prop_get returns u64, which might be a bytespace ID)
            // Implementation detail: userspace `prop_get` wraps syscall and returns u64 outcome.
            // If it failed, maybe property doesn't exist.
        }
    }

    fn record_overflow(&mut self) -> bool {
        let now_ms = stem::monotonic_ns() / 1_000_000;
        
        if now_ms.saturating_sub(self.last_resync_time_ms) < RESYNC_COOLDOWN_MS {
            // Too soon, suppress
            return false;
        }
        
        self.overflow_count += 1;
        self.resync_count += 1;
        self.last_resync_time_ms = now_ms;
        true
    }
}

// ============================================================================
// Decoding & Applying
// ============================================================================

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

fn log_unknown_shape_once(err: DecodeError, payload: &[u8], _seq: u64) {
    use core::sync::atomic::{AtomicU64, Ordering};

    const ISSUE_BAD_VERSION: u64 = 1 << 0;
    const ISSUE_UNKNOWN_OP: u64 = 1 << 1;
    const ISSUE_UNKNOWN_ENCODING: u64 = 1 << 2;
    const ISSUE_BAD_LENGTH: u64 = 1 << 3;
    const ISSUE_INVALID_UTF8: u64 = 1 << 4;
    const ISSUE_NONZERO_FLAGS: u64 = 1 << 5;

    static ISSUE_FLAGS: AtomicU64 = AtomicU64::new(0);
    static WATCH_DECODE_ERRORS_TOTAL: AtomicU64 = AtomicU64::new(0);

    WATCH_DECODE_ERRORS_TOTAL.fetch_add(1, Ordering::Relaxed);
    let _prefix = hex_prefix(payload, 16);
    
    // Simplified error logging for brevity in this refactor
    match err {
        DecodeError::BadVersion(_) => {
            if ISSUE_FLAGS.fetch_or(ISSUE_BAD_VERSION, Ordering::Relaxed) & ISSUE_BAD_VERSION == 0 {
                warn!("cambium: decode error: bad version");
            }
        }
        _ => {
            // General catch-all for other once-per-boot logs
        }
    }
}

fn set_string_prop<S: IntoSymbolRef + Copy>(id: ThingId, key_name: S, value: &str) {
    if value.is_empty() {
        prop_set(id, key_name, 0).ok();
        return;
    }
    match bytespace_create(value.len(), 0, 0) {
        Ok(bs_id) => {
            bytespace_write(bs_id, 0, value.as_bytes()).ok();
            prop_set(id, key_name, bs_id.to_u64_lossy()).ok();
        }
        Err(e) => {
            warn!("cambium: bytespace_create failed for string prop: {:?}", e);
        }
    }
}

fn apply_watch_payload(payload: &[u8], binding: &mut ActiveBinding, seq: u64) {
    use core::sync::atomic::{AtomicU64, Ordering};
    static LAST_LOG_MS: AtomicU64 = AtomicU64::new(0);
    let now_ms = stem::monotonic_ns() / 1_000_000;
    
    let should_log = {
        let last = LAST_LOG_MS.load(Ordering::Relaxed);
        if now_ms.wrapping_sub(last) >= 1000 {
            LAST_LOG_MS.store(now_ms, Ordering::Relaxed);
            true
        } else {
            false
        }
    };
    
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
                
                let target_key = if binding.to_key != 0 {
                    binding.to_key
                } else {
                    stem::thing::sys::intern(keys::UI_TEXT).unwrap_or(0)
                };

                match encoding {
                    ValueEncoding::U64LE => {
                        if value.len() != 8 { break; }
                        let next_value = match <[u8; 8]>::try_from(value) {
                            Ok(bytes) => u64::from_le_bytes(bytes),
                            Err(_) => {
                                warn!("cambium: invalid U64LE payload len={}", value.len());
                                break;
                            }
                        };
                        
                        // NO-OP suppression
                        if binding.last_value_u64 == Some(next_value) { continue; }
                        
                        binding.last_value_u64 = Some(next_value);
                        binding.last_value_bytes = None; 
                        
                        if should_log {
                            info!(
                                "[cambium] write: binding_src={} target={} pred_key={} val={} seq={}",
                                binding.source.to_u64_lossy(),
                                binding.target.to_u64_lossy(),
                                target_key,
                                next_value,
                                seq
                            );
                        }
                        
                        prop_set(binding.target, target_key, next_value).ok();
                    }
                    ValueEncoding::Utf8 => {
                        // NO-OP suppression 
                        if binding.last_value_bytes.as_deref() == Some(value) { continue; }
                        
                        binding.last_value_bytes = Some(value.to_vec());
                        binding.last_value_u64 = None;

                        if let Ok(text) = core::str::from_utf8(value) {
                            set_string_prop(binding.target, target_key, text);
                            
                            if should_log {
                                info!(
                                    "[cambium] write: target={} text='{}' seq={}",
                                    binding.target.to_u64_lossy(),
                                    text,
                                    seq
                                );
                            }
                        } else {
                            log_unknown_shape_once(DecodeError::InvalidUtf8, payload, seq);
                            break;
                        }
                    }
                    _ => { continue; }
                }
            }
            Err(e) => {
                log_unknown_shape_once(e, payload, seq);
                break;
            }
        }
    }
}

// ============================================================================
// Main
// ============================================================================

#[stem::main]
fn main() -> ! {
    info!("cambium starting (v5: drain-caps + smart-resync)...");

    let mut bindings: Vec<ActiveBinding> = Vec::new();

    // Initial scan for bindings
    let mut binding_ids = [ThingId::default(); 128];

    for _ in 0..120 {
        if let Ok(count) = find(kinds::BINDING, &mut binding_ids) {
            if count > 0 {
                info!("Found {} bindings", count);
                let safe_count = core::cmp::min(count, binding_ids.len());
                for i in 0..safe_count {
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
                    let to_key = prop_get(b_id, keys::BINDING_TO).unwrap_or(0) as u32;

                    if src_id.to_u64_lossy() == 0 || dst_id.to_u64_lossy() == 0 {
                        continue;
                    }

                    let filter = RootWatchFilter::subject(src_id.to_u64_lossy());
                    let spec = WatchSpec {
                        mode: 1, // StreamOnly
                        start_seq: 0, // Catch-up from oldest available
                        filter_ptr: &filter as *const _ as u64,
                        filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
                        ..Default::default()
                    };

                    match root_watch_open(&spec) {
                        Ok(watch_id) => {
                            info!(
                                "Opened watch {} for source {} (binding {})",
                                watch_id,
                                src_id.to_u64_lossy(),
                                b_id.to_u64_lossy()
                            );
                            bindings.push(ActiveBinding {
                                source: src_id,
                                target: dst_id,
                                watch_id,
                                last_value_u64: None,
                                last_value_bytes: None,
                                key_filter,
                                to_key,
                                overflow_count: 0,
                                resync_count: 0,
                                drained_events_total: 0,
                                last_resync_time_ms: 0,
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
    // STEADY STATE EVENT LOOP
    // ============================================================
    info!("Entering event loop with {} bindings", bindings.len());
    
    let mut payload_buf = [0u8; 4096];
    
    // For stats logging
    let mut last_stats_log_ms = stem::monotonic_ns() / 1_000_000;

    loop {
        let mut did_work = false;
        let mut events_this_tick_global = 0usize;
        let mut hit_global_cap = false;

        for binding in &mut bindings {
            let mut events_this_binding = 0usize;

            // Drain loop for this binding
            loop {
                // Check Global Cap
                if events_this_tick_global >= MAX_EVENTS_PER_TICK_TOTAL {
                    hit_global_cap = true;
                    break;
                }

                // Check Per-Binding Cap
                if events_this_binding >= MAX_EVENTS_PER_BINDING_PER_TICK {
                    break;
                }

                let mut seq: u64 = 0;
                match root_watch_next(binding.watch_id, &mut seq, &mut payload_buf) {
                    Ok(len) if len > 0 => {
                        did_work = true;
                        events_this_binding += 1;
                        events_this_tick_global += 1;
                        binding.drained_events_total += 1;
                        
                        apply_watch_payload(&payload_buf[..len], binding, seq);
                    }
                    Ok(_) => { 
                        // Empty read (EOF for now), move to next binding
                        break; 
                    }
                    Err(abi::errors::Errno::EAGAIN) => { 
                        // Drained, move to next binding
                        break; 
                    }
                    Err(abi::errors::Errno::EOVERFLOW) => {
                        // Overflow! Handle resync.
                        if binding.record_overflow() {
                            info!(
                                "Watch {} overflow! Resyncing (count={})...", 
                                binding.watch_id, binding.resync_count
                            );

                            // 1. Close old watch
                            root_watch_close(binding.watch_id).ok();

                            // 2. Reopen at LATEST
                            let filter = RootWatchFilter::subject(binding.source.to_u64_lossy());
                            let spec = WatchSpec {
                                mode: 1, 
                                start_seq: WATCH_START_LATEST, // Skip history
                                filter_ptr: &filter as *const _ as u64,
                                filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
                                ..Default::default()
                            };

                            if let Ok(new_id) = root_watch_open(&spec) {
                                binding.watch_id = new_id;
                                
                                // 3. Immediate state sync
                                binding.refresh_now();

                                // 4. Post-refresh drain check (one shot)
                                // Catch anything that happened during the refresh window
                                if let Ok(len) = root_watch_next(binding.watch_id, &mut seq, &mut payload_buf) {
                                    if len > 0 {
                                        apply_watch_payload(&payload_buf[..len], binding, seq);
                                    }
                                }
                            } else {
                                warn!("Failed to reopen watch during resync!");
                            }
                        }
                        
                        // Break drain loop on overflow/resync to let things settle
                        break;
                    }
                    Err(e) => {
                        warn!("watch_next error: {:?}", e);
                        break;
                    }
                }
            }
        }

        // Stats logging every 5s
        let now_ms = stem::monotonic_ns() / 1_000_000;
        if now_ms.wrapping_sub(last_stats_log_ms) >= 5000 {
            last_stats_log_ms = now_ms;
            
            let mut total_drained = 0;
            let mut total_overflows = 0;
            let mut total_resyncs = 0;
            
            for b in &bindings {
                total_drained += b.drained_events_total;
                total_overflows += b.overflow_count;
                total_resyncs += b.resync_count;
            }
            
            info!(
                "[cambium] stats: drained={} overflows={} resyncs={}",
                total_drained, total_overflows, total_resyncs
            );
        }

        if !did_work {
            stem::sleep(Duration::from_millis(10)); // Responsive sleep
        } else if hit_global_cap {
            // Yield briefly if we're saturating, to let other tasks run
            stem::yield_now();
        }
    }
}
