#![no_std]
#![no_main]

extern crate alloc;
extern crate stem; // Forcing linkage of runtime features

use stem::{info, warn};
use stem::thing::{ThingId, HandleId};
use stem::thing::sys::{bytespace_create, bytespace_write, find, prop_get, prop_set};
use stem::syscall::{root_watch_open, root_watch_next, root_watch_close};
use stem::thing::symbol::IntoSymbolRef;
use abi::schema::{kinds, keys};
use abi::types::{WatchSpec, WATCH_START_LATEST};
use abi::root::RootWatchFilter;
use abi::watch::{self, DecodeError, ValueEncoding, WatchOp, MAX_WATCH_PAYLOAD_BYTES};
use alloc::vec::Vec;
use alloc::string::String;
use core::time::Duration;

// ============================================================================
// Constants & Configuration
// ============================================================================

const MAX_EVENTS_PER_TICK_TOTAL: usize = 4096;
const RESYNC_COOLDOWN_MS: u64 = 250;

// ============================================================================
// Active Binding State
// ============================================================================

struct ActiveBinding {
    source: ThingId,
    target: ThingId,
    watch_id: usize,
    
    last_value_u64: Option<u64>,
    last_value_bytes: Option<Vec<u8>>,
    
    key_filter: Option<u32>,
    to_key: u32,
    
    has_pending: bool,
    pending_u64: Option<u64>,
    pending_bytes: Option<Vec<u8>>,
    
    needs_resync: bool,
    last_resync_time_ms: u64,

    overflow_count: u64,
    overflows_suppressed: u64,
    resync_count: u64,
    drained_events_total: u64,
    events_buffered_this_tick: u64,
}

impl ActiveBinding {
    fn refresh_now(&mut self) {
        let src_key = if let Some(k) = self.key_filter {
            k
        } else {
            return;
        };

        let target_key = if self.to_key != 0 {
            self.to_key
        } else {
            stem::thing::sys::intern(keys::UI_TEXT).unwrap_or(0)
        };
        
        if let Ok(val) = prop_get(self.source, src_key) {
            if self.last_value_u64 == Some(val) {
                return;
            }
            if prop_set(self.target, target_key, val).is_ok() {
                self.last_value_u64 = Some(val);
                self.last_value_bytes = None;
                info!("[cambium] refresh: binding_src={} target={} val={}", self.source.to_u64_lossy(), self.target.to_u64_lossy(), val);
            }
        }
    }

    fn mark_overflow(&mut self) {
        let now_ms = stem::monotonic_ns() / 1_000_000;
        if now_ms.saturating_sub(self.last_resync_time_ms) < RESYNC_COOLDOWN_MS {
            self.overflows_suppressed += 1;
        } else {
            self.overflow_count += 1;
            self.needs_resync = true;
        }
    }

    fn stage_u64(&mut self, val: u64) {
        self.pending_u64 = Some(val);
        self.pending_bytes = None;
        self.has_pending = true;
    }

    fn stage_bytes(&mut self, val: &[u8]) {
        self.pending_bytes = Some(val.to_vec());
        self.pending_u64 = None;
        self.has_pending = true;
    }

    fn apply_pending(&mut self, _seq: u64) {
        if !self.has_pending { return; }

        let target_key = if self.to_key != 0 {
            self.to_key
        } else {
            stem::thing::sys::intern(keys::UI_TEXT).unwrap_or(0)
        };

        if let Some(val) = self.pending_u64 {
            if self.last_value_u64 == Some(val) {
                self.has_pending = false;
                return;
            }
            if prop_set(self.target, target_key, val).is_ok() {
                self.last_value_u64 = Some(val);
                self.last_value_bytes = None;
            }
        } else if let Some(ref bytes) = self.pending_bytes {
            if self.last_value_bytes.as_ref() == Some(bytes) {
                self.has_pending = false;
                return;
            }
            if let Ok(text) = core::str::from_utf8(bytes) {
               set_string_prop(self.target, target_key, text);
               self.last_value_bytes = Some(bytes.clone());
               self.last_value_u64 = None;
            }
        }
        self.has_pending = false;
    }
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

fn log_unknown_shape_once(err: DecodeError, payload: &[u8]) {
    warn!("cambium: decode error {:?} prefix={}", err, hex_prefix(payload, 8));
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
            warn!("cambium: bytespace_create failed: {:?}", e);
        }
    }
}

fn drain_watch_payload(payload: &[u8], binding: &mut ActiveBinding) {
    let mut cursor = 0usize;
    while cursor < payload.len() {
        match watch::decode_event(&payload[cursor..]) {
            Ok((header, value)) => {
                let event_len = watch::WATCH_EVENT_HEADER_LEN + value.len();
                cursor += event_len;

                let op = WatchOp::from_u8(header.op).unwrap_or(WatchOp::Upsert);
                if op != WatchOp::Upsert { continue; }

                let subject = header.subject.to_u64_lossy();
                if subject == 0 || subject != binding.source.to_u64_lossy() { continue; }

                let predicate = header.predicate.to_u32_lossy();
                if let Some(filter) = binding.key_filter {
                    if predicate != filter { continue; }
                }

                let encoding = ValueEncoding::from_u8(header.value_encoding).unwrap_or(ValueEncoding::Bytes);
                binding.events_buffered_this_tick += 1;

                match encoding {
                    ValueEncoding::U64LE => {
                        if value.len() == 8 {
                            if let Ok(bytes) = <[u8; 8]>::try_from(value) {
                                binding.stage_u64(u64::from_le_bytes(bytes));
                            }
                        }
                    }
                    ValueEncoding::Utf8 => { binding.stage_bytes(value); }
                    _ => { continue; }
                }
            }
            Err(e) => {
                log_unknown_shape_once(e, payload);
                break;
            }
        }
    }
}

#[stem::main]
fn main() -> ! {
    info!("cambium starting (v5.1: drain-caps + smart-resync)...");

    let mut bindings: Vec<ActiveBinding> = Vec::new();
    let mut binding_ids = [ThingId::default(); 128];

    for _ in 0..120 {
        if let Ok(count) = find(kinds::BINDING, &mut binding_ids) {
            if count > 0 {
                info!("Found {} bindings", count);
                let safe_count = core::cmp::min(count, binding_ids.len());
                for i in 0..safe_count {
                    let b_id = binding_ids[i];
                    let src_id = prop_get(b_id, keys::BINDING_SOURCE).map(ThingId::from_u64).unwrap_or(ThingId::default());
                    let dst_id = prop_get(b_id, keys::BINDING_TARGET).map(ThingId::from_u64).unwrap_or(ThingId::default());
                    let key_filter = prop_get(b_id, keys::BINDING_MAP).ok().and_then(|v| if v == 0 { None } else { Some(v as u32) });
                    let to_key = prop_get(b_id, keys::BINDING_TO).unwrap_or(0) as u32;

                    if src_id.to_u64_lossy() == 0 || dst_id.to_u64_lossy() == 0 { continue; }

                    let filter = RootWatchFilter::subject(src_id.to_u64_lossy());
                    let spec = WatchSpec {
                        mode: 1, 
                        start_seq: 0, 
                        filter_ptr: &filter as *const _ as u64,
                        filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
                        ..Default::default()
                    };

                    match root_watch_open(&spec) {
                        Ok(watch_id) => {
                            info!("Opened watch {} for source {} (binding {})", watch_id, src_id.to_u64_lossy(), b_id.to_u64_lossy());
                            bindings.push(ActiveBinding {
                                source: src_id, target: dst_id, watch_id,
                                last_value_u64: None, last_value_bytes: None,
                                key_filter, to_key,
                                has_pending: false, pending_u64: None, pending_bytes: None,
                                needs_resync: false, last_resync_time_ms: 0,
                                overflow_count: 0, overflows_suppressed: 0, resync_count: 0, drained_events_total: 0, events_buffered_this_tick: 0,
                            });
                        }
                        Err(e) => { info!("Failed to watch source {}: {:?}", src_id.to_u64_lossy(), e); }
                    }
                }
                break;
            }
        }
        stem::sleep(Duration::from_secs(1));
    }

    if bindings.is_empty() {
        info!("No bindings found. Exiting.");
        loop { stem::sleep(Duration::from_secs(10)); }
    }

    info!("Entering event loop with {} bindings (buf size: {})", bindings.len(), MAX_WATCH_PAYLOAD_BYTES);
    
    let mut payload_buf = Vec::with_capacity(MAX_WATCH_PAYLOAD_BYTES);
    unsafe { payload_buf.set_len(MAX_WATCH_PAYLOAD_BYTES); }
    
    let mut last_stats_log_ms = stem::monotonic_ns() / 1_000_000;

    loop {
        let mut did_work = false;
        let mut events_this_tick_global = 0usize;
        let mut hit_global_cap = false;

        for binding in &mut bindings {
            binding.events_buffered_this_tick = 0;
            let mut seq: u64 = 0;

            loop {
                if events_this_tick_global >= MAX_EVENTS_PER_TICK_TOTAL {
                    hit_global_cap = true;
                    break;
                }

                match root_watch_next(binding.watch_id, &mut seq, &mut payload_buf) {
                    Ok(len) if len > 0 => {
                        did_work = true;
                        drain_watch_payload(&payload_buf[..len], binding);
                        events_this_tick_global += len.min(10); 
                        binding.drained_events_total += 1; 
                    }
                    Ok(_) => { break; }
                    Err(abi::errors::Errno::EAGAIN) => { break; }
                    Err(abi::errors::Errno::EOVERFLOW) => {
                        binding.mark_overflow();
                        break;
                    }
                    Err(e) => {
                        warn!("watch_next error: {:?}", e);
                        break;
                    }
                }
            }
            
            if binding.needs_resync {
                 let now = stem::monotonic_ns() / 1_000_000;
                 if now.saturating_sub(binding.last_resync_time_ms) >= RESYNC_COOLDOWN_MS {
                    binding.resync_count += 1;
                    binding.needs_resync = false;
                    binding.last_resync_time_ms = now;
                    binding.has_pending = false;

                    root_watch_close(binding.watch_id).ok();

                    let filter = RootWatchFilter::subject(binding.source.to_u64_lossy());
                    let spec = WatchSpec {
                        mode: 1, 
                        start_seq: WATCH_START_LATEST,
                        filter_ptr: &filter as *const _ as u64,
                        filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
                        ..Default::default()
                    };
                    
                    let old_id = binding.watch_id;

                    match root_watch_open(&spec) {
                        Ok(new_id) => {
                            binding.watch_id = new_id;
                            info!("Resync watch {} -> {}", old_id, new_id);
                            binding.refresh_now(); 
                        }
                        Err(e) => { warn!("Failed resync: {:?}", e); }
                    }
                 }
            } else {
                binding.apply_pending(seq);
            }
        }

        let now_ms = stem::monotonic_ns() / 1_000_000;
        if now_ms.wrapping_sub(last_stats_log_ms) >= 5000 {
            last_stats_log_ms = now_ms;
            let mut total_drained = 0;
            let mut total_overflows = 0;
            let mut total_suppressed = 0;
            let mut total_resyncs = 0;
            for b in &bindings {
                total_drained += b.drained_events_total;
                total_overflows += b.overflow_count;
                total_suppressed += b.overflows_suppressed;
                total_resyncs += b.resync_count;
            }
            info!("[cambium] stats: drained_calls={} overflows={} suppressed={} resyncs={}",
                total_drained, total_overflows, total_suppressed, total_resyncs);
        }

        if !did_work {
            stem::sleep(Duration::from_millis(10));
        } else if hit_global_cap {
            stem::yield_now();
        }
    }
}
