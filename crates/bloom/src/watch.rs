use abi::ids::{ThingId, WatchId};
use abi::types::{
    WaitFlags, WakeReasonCode, WatchEvent, WatchEventKind, WatchKind, MAX_WATCH_EVENTS,
};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use thing_std::watch::{watch_create, watch_poll, watch_wait};
use thing_std::log_info;
use alloc::format;

const EMPTY_EVENT: WatchEvent = WatchEvent {
    kind: WatchEventKind::ThingUpdated,
    flags: 0,
    subject: ThingId(0),
    arg0: ThingId(0),
};

pub struct WatchSet {
    pub graph_id: ThingId,
    pub graph_watch: WatchId,
    window_watches: BTreeMap<ThingId, WatchId>,
}

impl WatchSet {
    pub fn new(graph_id: ThingId) -> Result<Self, i32> {
        let graph_watch = watch_create(WatchKind::GraphMembership, graph_id)?;
        Ok(Self {
            graph_id,
            graph_watch,
            window_watches: BTreeMap::new(),
        })
    }

    pub fn ensure_window_watch(&mut self, window: ThingId) -> Result<Option<WatchId>, i32> {
        if self.window_watches.contains_key(&window) {
            return Ok(None);
        }
        let watch_id = watch_create(WatchKind::Thing, window)?;
        self.window_watches.insert(window, watch_id);
        Ok(Some(watch_id))
    }

    pub fn remove_window_watch(&mut self, window: ThingId) -> Option<WatchId> {
        self.window_watches.remove(&window)
    }

    pub fn watch_ids(&self) -> Vec<WatchId> {
        let mut ids = Vec::with_capacity(1 + self.window_watches.len());
        ids.push(self.graph_watch);
        ids.extend(self.window_watches.values().copied());
        ids
    }

    pub fn wait(&self, timeout_ticks: u64) -> Result<WakeReasonCode, i32> {
        let ids = self.watch_ids();
        if ids.is_empty() {
            return Ok(WakeReasonCode::Timeout);
        }
        let reason = watch_wait(&ids, WaitFlags::WAIT_ANY, timeout_ticks)?;
        Ok(reason.reason)
    }

    pub fn drain(&self, out: &mut Vec<WatchEvent>) -> Result<usize, i32> {
        // DIAG: entering drain
        static mut DRAIN_CALL: u64 = 0;
        let call_num = unsafe { DRAIN_CALL += 1; DRAIN_CALL };
        if call_num <= 3 {
            log_info(&format!("DRAIN[{}]: enter", call_num));
        }
        
        let mut total = 0usize;
        let mut scratch = [EMPTY_EVENT; MAX_WATCH_EVENTS];
        
        if call_num <= 3 {
            log_info(&format!("DRAIN[{}]: scratch created", call_num));
        }
        
        let ids = self.watch_ids();
        if call_num <= 3 {
            log_info(&format!("DRAIN[{}]: watch_ids len={}", call_num, ids.len()));
        }
        
        for watch_id in ids {
            if call_num <= 3 {
                log_info(&format!("DRAIN[{}]: polling watch {}", call_num, watch_id.0));
            }
            loop {
                let n = watch_poll(watch_id, &mut scratch)?;
                if call_num <= 3 && n > 0 {
                    log_info(&format!("DRAIN[{}]: watch_poll returned n={}", call_num, n));
                }
                if n == 0 {
                    break;
                }
                if call_num <= 3 {
                    log_info(&format!("DRAIN[{}]: about to extend_from_slice n={}", call_num, n));
                }
                total += n;
                out.extend_from_slice(&scratch[..n]);
                if call_num <= 3 {
                    log_info(&format!("DRAIN[{}]: extended, total={}", call_num, total));
                }
            }
        }
        
        if call_num <= 3 {
            log_info(&format!("DRAIN[{}]: done total={}", call_num, total));
        }
        Ok(total)
    }
}

