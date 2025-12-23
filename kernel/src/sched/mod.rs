extern crate alloc;

pub use self::types::ThreadState;
use crate::{graph, graph_kinds};
use abi::{ProcessId, ThingId, ThreadId, USER_HEAP_END};
use alloc::string::String;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use heapless::Vec;
use spin::Mutex;
use thing_models::PropValue;
pub use crate::sched::types::{Thread, SleepEntry, Process, FpuContext, ScheduledThread};

pub mod types;
pub mod tick;
pub mod legacy_graph;
pub mod graph_sync;

#[cfg(test)]
mod tests;

pub static TICKS: AtomicU64 = AtomicU64::new(0);
pub static PREEMPT_COUNT: AtomicU32 = AtomicU32::new(0);
pub static NEED_RESCHED: AtomicBool = AtomicBool::new(false);

pub fn preempt_disable() {
    PREEMPT_COUNT.fetch_add(1, Ordering::Relaxed);
    core::sync::atomic::compiler_fence(Ordering::SeqCst);
}

pub fn preempt_enable() {
    core::sync::atomic::compiler_fence(Ordering::SeqCst);
    PREEMPT_COUNT.fetch_sub(1, Ordering::Relaxed);
}

pub fn without_preemption<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    preempt_disable();
    let res = f();
    preempt_enable();
    res
}

pub fn with_scheduler<F, R>(f: F) -> R
where
    F: FnOnce(&mut Scheduler) -> R,
{
    without_preemption(|| {
        let mut sched = SCHEDULER.lock();
        f(&mut *sched)
    })
}

pub const MAX_THREADS: usize = 32;
pub const MAX_PROCESSES: usize = 16;
const FAKE_SLICE_NS: u64 = 5_000_000;

pub struct SchedCache {
    pub run_queue: Vec<ThreadId, MAX_THREADS>,
    pub sleep_queue: Vec<SleepEntry, MAX_THREADS>,
    pub last_graph_revision: u64,
}

pub struct Scheduler {
    pub cache: SchedCache,
    pub threads: [Option<Thread>; MAX_THREADS],
    pub processes: [Option<Process>; MAX_PROCESSES],
    pub current: Option<ThreadId>,
    pub graph_enabled: bool,
    pub fake_time_ns: u64,
    pub cpu_thing_id: Option<ThingId>,

    // Metrics
    pub max_tick_time_ns: u64,
    pub avg_tick_time_ns: u64,
    pub total_ticks: u64,
    pub rebuild_count: u64,
    pub total_switches: u64,
    pub last_switch_ns: u64,
    pub last_heartbeat_ns: u64,
    pub last_runtime_total: u64,
    pub last_runtime_idle: u64,
    pub last_runtime_actualizer: u64,
}

fn thread_index(tid: ThreadId) -> usize {
    tid.0
        .checked_sub(1)
        .expect("ThreadId 0 is invalid; IDs start at 1") as usize
}

fn process_index(pid: ProcessId) -> usize {
    pid.0
        .checked_sub(1)
        .expect("ProcessId 0 is invalid; IDs start at 1") as usize
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            cache: SchedCache {
                run_queue: Vec::new(),
                sleep_queue: Vec::new(),
                last_graph_revision: 0,
            },
            threads: [const { None }; MAX_THREADS],
            processes: [const { None }; MAX_PROCESSES],
            current: None,
            graph_enabled: false,
            fake_time_ns: 0,
            cpu_thing_id: None,
            max_tick_time_ns: 0,
            avg_tick_time_ns: 0,
            total_ticks: 0,
            rebuild_count: 0,
            total_switches: 0,
            last_switch_ns: 0,
            last_heartbeat_ns: 0,
            last_runtime_total: 0,
            last_runtime_idle: 0,
            last_runtime_actualizer: 0,
        }
    }

    pub fn init_graph_mirror(&mut self) {
        self.graph_enabled = true;

        for index in 0..self.processes.len() {
            if self.processes[index].is_some() {
                self.ensure_process_thing(index);
            }
        }

        for index in 0..self.threads.len() {
            if self.threads[index].is_some() {
                self.ensure_thread_thing(index);
                // We don't manually call update here, assume initial state is synced
                // or will be synced on next switch/rebuild.
                // Actually we should make sure graph matches local state initially.
                self.graph_update_thread_state_initial(index);
            }
        }

        // Find CPU Thing (assuming single core 0)
        let kind = crate::symbols::intern(graph_kinds::KIND_CPU_CORE);
        let mut curr = None;
        // Use loop to find CPU with index 0
        while let Some(id) = graph::next_thing_of_kind(kind, curr.unwrap_or(ThingId(0))) {
            if let Some(PropValue::U64(idx)) = graph::get_prop(id, "index") {
                if idx == 0 {
                    self.cpu_thing_id = Some(id);
                    break;
                }
            }
            curr = Some(id);
        }

        // Force rebuild on next tick
        self.cache.last_graph_revision = 0;
    }

    fn ensure_cache_valid(&mut self) {
        if !self.graph_enabled {
            return;
        }
        let rev = graph::get_revision();
        if rev != self.cache.last_graph_revision {
            let rebuilt = graph_sync::rebuild_cache_from_graph(&mut self.threads);
            self.cache.run_queue = rebuilt.run_queue;
            self.cache.sleep_queue = rebuilt.sleep_queue;
            // We trust local current over rebuilt current for now,
            // but in a pure graph system, we might assert they match.
            self.cache.last_graph_revision = rev;
            self.rebuild_count += 1;
        }
    }

    pub fn sleep_current_thread(&mut self, wake_at_ns: u64) {
        let tid = self
            .current
            .expect("no current thread in sleep_current_thread");

        let index = thread_index(tid);
        if self.threads[index].is_none() {
            panic!("sleep_current_thread: missing thread");
        }

        self.finish_running_thread(index);
        if let Some(thr) = self.threads[index].as_mut() {
            thr.state = ThreadState::Sleeping;
            thr.sleep_until_ns = wake_at_ns;
        }

        // We update local cache immediately to prevent running again before tick/rebuild
        if let Some(pos) = self.cache.run_queue.iter().position(|&id| id == tid) {
            self.cache.run_queue.swap_remove(pos);
        }
        let _ = self.cache.sleep_queue.push(SleepEntry {
            thread_id: tid,
            wake_at_ns,
        });

        self.current = None;
    }

    pub fn wake_sleepers(&mut self, now_ns: u64) {
        let mut i = 0;
        while i < self.cache.sleep_queue.len() {
            let entry = self.cache.sleep_queue[i];
            if entry.wake_at_ns <= now_ns {
                // Wake this thread.
                let index = thread_index(entry.thread_id);
                if let Some(thr) = self.threads[index].as_mut() {
                    thr.state = ThreadState::Runnable;
                    thr.sleep_until_ns = 0;

                    // Commit wake to graph immediately because this is an async event
                    // (not necessarily coupled with a switch of THIS thread).
                    graph_sync::commit_thread_wake(thr);
                }

                // Put back on run queue.
                let _ = self.cache.run_queue.push(entry.thread_id);

                // Remove this entry from sleep_queue by swap_remove.
                self.cache.sleep_queue.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn add_process(&mut self, name: &'static str, package_name: &str) -> ProcessId {
        for (i, slot) in self.processes.iter_mut().enumerate() {
            if slot.is_none() {
                let pid = ProcessId(i as u64 + 1);

                // Create Process Thing
                let package_id = crate::symbols::intern(package_name);
                let props = alloc::vec![
                    (crate::symbols::intern("pid"), PropValue::U64(pid.0)),
                    (crate::symbols::intern("package_id"), PropValue::Symbol(package_id)),
                ];
                let thing_id = Some(graph::create_thing(
                    crate::symbols::intern(graph_kinds::KIND_PROCESS),
                    props,
                ));

                *slot = Some(Process {
                    id: pid,
                    name,
                    package_id,
                    thing_id,
                    address_space_token: None,
                    heap_base: 0,
                    heap_limit: 0,
                    next_map_base: USER_HEAP_END as u64,
                    next_resident_map_base: abi::USER_RESIDENT_BASE as u64,
                });
                if self.graph_enabled {
                    self.ensure_process_thing(i);
                }
                return pid;
            }
        }
        panic!("Max processes reached");
    }

    // ... setters ...
    pub fn set_process_address_space(&mut self, pid: ProcessId, token: u64) {
        let idx = process_index(pid);
        if let Some(proc_slot) = self.processes.get_mut(idx).and_then(|p| p.as_mut()) {
            proc_slot.address_space_token = Some(token);
        }
    }

    pub fn set_process_heap(&mut self, pid: ProcessId, base: usize, limit: usize) {
        let idx = process_index(pid);
        if let Some(proc_slot) = self.processes.get_mut(idx).and_then(|p| p.as_mut()) {
            proc_slot.heap_base = base;
            proc_slot.heap_limit = limit;
        }
    }

    pub fn current_process_id(&self) -> Option<ProcessId> {
        let tid = self.current?;
        let idx = thread_index(tid);
        self.threads
            .get(idx)
            .and_then(|t| t.as_ref())
            .map(|t| t.process_id)
    }

    pub fn reserve_user_region(
        &mut self,
        pid: ProcessId,
        size: usize,
        align: usize,
    ) -> Option<u64> {
        let idx = process_index(pid);
        let proc_slot = self.processes.get_mut(idx)?.as_mut()?;
        let alignment = if align == 0 { 1 } else { align } as u64;
        let start = proc_slot.next_map_base.max(0x5000_0000 as u64);
        let aligned = if start % alignment == 0 {
            start
        } else {
            start + (alignment - (start % alignment))
        };
        let end = aligned.checked_add(size as u64)?;
        proc_slot.next_map_base = end;
        Some(aligned)
    }

    pub fn reserve_resident_region(
        &mut self,
        pid: ProcessId,
        size: usize,
        align: usize,
    ) -> Option<u64> {
        let idx = process_index(pid);
        let proc_slot = self.processes.get_mut(idx)?.as_mut()?;
        let alignment = if align == 0 { 1 } else { align } as u64;

        let start = proc_slot.next_resident_map_base;
        let aligned = if start % alignment == 0 {
            start
        } else {
            start + (alignment - (start % alignment))
        };

        let end = aligned.checked_add(size as u64)?;
        if end > abi::USER_RESIDENT_LIMIT as u64 {
            return None;
        }

        proc_slot.next_resident_map_base = end;
        Some(aligned)
    }

    pub fn add_thread(
        &mut self,
        process_id: ProcessId,
        name: &'static str,
        entry: extern "C" fn(u64) -> !,
        arg: u64,
        stack_top: u64,
        priority: u64,
    ) -> ThreadId {
        let entry_point = entry as u64;
        self.add_thread_with_entry_point(process_id, name, entry_point, arg, stack_top, priority)
    }

    pub fn add_thread_with_entry_point(
        &mut self,
        process_id: ProcessId,
        name: &'static str,
        entry_point: u64,
        arg: u64,
        stack_top: u64,
        priority: u64,
    ) -> ThreadId {
        for (i, slot) in self.threads.iter_mut().enumerate() {
            if slot.is_none() {
                let tid = ThreadId(i as u64 + 1);
                let address_space_token = self
                    .processes
                    .get(process_index(process_id))
                    .and_then(|p| p.as_ref())
                    .and_then(|p| p.address_space_token);

                *slot = Some(Thread {
                    id: tid,
                    process_id,
                    state: ThreadState::New,
                    name,
                    priority,
                    entry_point,
                    user_arg: arg,
                    user_stack_top: stack_top,
                    context: [0; 20],
                    fpu_context: FpuContext::default(),
                    started: false,
                    thing_id: None,
                    sleep_event_id: None,
                    sleep_until_ns: 0,
                    last_run_start_ns: 0,
                    total_run_ns: 0,
                    address_space_token,
                    pending_wake: false,
                    is_idle: false,
                });

                // Create Thing immediately (updates revision)
                // If graph_enabled is false, we don't.
                // But we must add to cache locally.

                if self.graph_enabled {
                    self.ensure_thread_thing(i);
                    self.graph_update_thread_state_initial(i);
                    // This bumped revision, so ensure_cache_valid will rebuild run_queue
                } else {
                     // Manually add to run_queue since we won't rebuild
                     let _ = self.cache.run_queue.push(tid);
                }

                return tid;
            }
        }
        panic!("Max threads reached");
    }

    pub fn add_idle_thread(&mut self, process_id: ProcessId) -> ThreadId {
         // Same logic as add_thread but state=Runnable, priority=0
        for (i, slot) in self.threads.iter_mut().enumerate() {
            if slot.is_none() {
                let tid = ThreadId(i as u64 + 1);
                let address_space_token = self
                    .processes
                    .get(process_index(process_id))
                    .and_then(|p| p.as_ref())
                    .and_then(|p| p.address_space_token);

                *slot = Some(Thread {
                    id: tid,
                    process_id,
                    state: ThreadState::Runnable,
                    name: "idle",
                    priority: 0,
                    entry_point: 0,
                    user_arg: 0,
                    user_stack_top: 0,
                    context: [0; 20],
                    fpu_context: FpuContext::default(),
                    started: false,
                    thing_id: None,
                    sleep_event_id: None,
                    sleep_until_ns: 0,
                    last_run_start_ns: 0,
                    total_run_ns: 0,
                    address_space_token,
                    pending_wake: false,
                    is_idle: true,
                });

                if self.graph_enabled {
                    self.ensure_thread_thing(i);
                    self.graph_update_thread_state_initial(i);
                } else {
                     let _ = self.cache.run_queue.push(tid);
                }

                return tid;
            }
        }
        panic!("Max threads reached creating idle thread");
    }

    pub fn mark_yield(&mut self, tid: ThreadId) {
        let index = thread_index(tid);
        let is_running = self
            .threads
            .get(index)
            .and_then(|t| t.as_ref())
            .map(|t| t.state == ThreadState::Running)
            .unwrap_or(false);

        if is_running {
            self.finish_running_thread(index);
            if let Some(thread) = self.threads.get_mut(index).and_then(|t| t.as_mut()) {
                thread.state = ThreadState::Runnable;
            }
            // Update cache locally (it will be consistent with graph after commit_switch)
            if self.cache.run_queue.push(tid).is_err() {
                 crate::log("Run queue full on yield");
            }
        }
    }

    pub fn mark_terminated(&mut self, tid: ThreadId, reason: &'static str, code: u64) {
        let index = thread_index(tid);
        if self.threads.get(index).and_then(|t| t.as_ref()).is_some() {
            self.finish_running_thread(index);
            // clear sleep event?
            // self.graph_clear_sleep_event(index); // graph_sync handles this?

            let mut pid = ProcessId(0);
            if let Some(thread) = self.threads.get_mut(index).and_then(|t| t.as_mut()) {
                thread.state = ThreadState::Exited;
                pid = thread.process_id;
            }

            // Graph update happens at commit_switch (Running -> Exited).
            // But if we need to emit Exit Event, we might need to do it here or in commit_switch.
            // For now, let's keep emit_process_exit_event logic but decouple it from graph_update_thread_state.

            if self.is_process_dead(pid) {
                self.emit_process_exit_event(pid, reason, code);
            }
        }
    }

    // ... is_process_dead, emit_process_exit_event ...
    fn is_process_dead(&self, pid: ProcessId) -> bool {
        for thread in self.threads.iter().flatten() {
            if thread.process_id == pid && thread.state != ThreadState::Exited {
                return false;
            }
        }
        true
    }

    fn emit_process_exit_event(&self, pid: ProcessId, reason: &'static str, code: u64) {
         // Logic identical to old code
         if !self.graph_enabled { return; }
         // ... (copy paste old logic)
         // Since this uses graph::create_thing, it's allowed.
         // It doesn't modify thread state.

        let Some(proc_thing) = self.process_thing_id(pid) else {
            return;
        };

        let props = alloc::vec![
            (
                crate::symbols::intern(graph_kinds::PROP_EXIT_REASON),
                PropValue::Str(String::from(reason))
            ),
            (
                crate::symbols::intern(graph_kinds::PROP_EXIT_CODE),
                PropValue::U64(code)
            ),
            (
                crate::symbols::intern(graph_kinds::PROP_TIMESTAMP),
                PropValue::U64(self.fake_time_ns)
            ),
        ];

        let event_id = graph::create_thing(
            crate::symbols::intern(graph_kinds::KIND_PROCESS_EXIT_EVENT),
            props,
        );
        if event_id.0 != 0 {
            let _ = graph::add_link(event_id, graph_kinds::LINK_ABOUT, proc_thing);
            // Respawn logic ...
             let mut boot_program_id = None;
            let mut buf = [None; 1];
            graph::neighbors(proc_thing, graph_kinds::LINK_RUNNING, &mut buf);
            if let Some(id) = buf[0] {
                boot_program_id = Some(id);
            }

            if let Some(bp_id) = boot_program_id {
                let mut policy = String::from(graph_kinds::RESPAWN_NEVER);
                if let Some(val) = graph::get_prop(bp_id, graph_kinds::PROP_RESPAWN_POLICY) {
                    if let PropValue::Str(s) = val {
                        policy = s.clone();
                    }
                }

                let should_respawn = match policy.as_str() {
                    graph_kinds::RESPAWN_ALWAYS => true,
                    graph_kinds::RESPAWN_ON_CRASH => reason != "Exited",
                    _ => false,
                };

                if should_respawn {
                    crate::work_queue::push_normal(crate::work_queue::WorkItem::SpawnProgram(
                        bp_id,
                        Some(proc_thing),
                    ));
                    crate::log("Respawn scheduled via WorkQueue");
                }
            }
        }
    }

    pub fn next_runnable(&mut self) -> Option<ThreadId> {
        if self.cache.run_queue.is_empty() {
            return None;
        }

        let mut best_index = 0;
        let mut best_prio = 0;

        for (i, &tid) in self.cache.run_queue.iter().enumerate() {
            let index = thread_index(tid);
            if let Some(thr) = self.threads[index].as_ref() {
                if thr.priority > best_prio {
                    best_prio = thr.priority;
                    best_index = i;
                }
                // Ties? FIFO.
            }
        }

        Some(self.cache.run_queue.remove(best_index))
    }

    pub fn choose_next_thread(&mut self, now_ns: u64) -> Option<ScheduledThread> {
        let start = crate::time::monotonic_now_ns();
        self.wake_sleepers(now_ns);
        self.ensure_cache_valid();

        let tid = self.next_runnable()?;
        self.current = Some(tid);

        let fake_time = self.fake_time_ns;
        if let Some(thread) = self.thread_mut(tid) {
            thread.state = ThreadState::Running;
            thread.last_run_start_ns = fake_time;
        }

        let end = crate::time::monotonic_now_ns();
        let dur = end.saturating_sub(start);
        if dur > self.max_tick_time_ns {
            self.max_tick_time_ns = dur;
        }
        self.total_ticks += 1;
        self.avg_tick_time_ns = (self.avg_tick_time_ns * (self.total_ticks - 1) + dur) / self.total_ticks;

        if self.total_ticks % 10 == 0 {
             let run_queue_len = self.cache.run_queue.len();
             let mut actualizer_state = "Not Found";
             let mut actualizer_prio = 0;
             for t in self.threads.iter().flatten() {
                 if t.name == "actualizer" {
                     actualizer_state = t.state.as_str();
                     actualizer_prio = t.priority;
                     break;
                 }
             }
             crate::log(&alloc::format!(
                 "Sched Heartbeat: run_q={} act_state={} act_prio={} max={}ns avg={}ns",
                 run_queue_len, actualizer_state, actualizer_prio,
                 self.max_tick_time_ns, self.avg_tick_time_ns
             ));
        }

        let thread = self.thread_mut(tid).expect("Scheduled thread missing backing state");

        Some(ScheduledThread {
            tid,
            process_id: thread.process_id,
            name: thread.name,
            started: thread.started,
            entry_point: thread.entry_point,
            user_stack_top: thread.user_stack_top,
            user_arg: thread.user_arg,
            context: thread.context,
            fpu_context: thread.fpu_context,
            address_space_token: thread.address_space_token,
            is_idle: thread.is_idle,
        })
    }

    pub fn commit_switch(&mut self, old_tid: Option<ThreadId>, new_tid: Option<ThreadId>, now_ns: u64) {
         let old_state = if let Some(tid) = old_tid {
             if let Some(t) = self.threads[thread_index(tid)].as_ref() {
                 t.state
             } else {
                 ThreadState::Runnable
             }
         } else { ThreadState::Runnable };

         let new_state = ThreadState::Running;
         let runtime = FAKE_SLICE_NS; // Approximation

         graph_sync::commit_decision_to_graph(old_tid, new_tid, old_state, new_state, &self.threads, runtime, now_ns, self.cpu_thing_id);

         self.cache.last_graph_revision = graph::get_revision();
    }

    // ... set_current, current_id, thread_mut ...
    pub fn set_current(&mut self, tid: ThreadId) {
        self.current = Some(tid);
        // We don't update state/graph here. commit_switch does it.
    }

    pub fn current_id(&self) -> Option<ThreadId> {
        self.current
    }

    pub fn thread_mut(&mut self, tid: ThreadId) -> Option<&mut Thread> {
        self.threads[thread_index(tid)].as_mut()
    }

    // ... helpers ...
    pub fn process_thing_id(&self, pid: ProcessId) -> Option<ThingId> {
        let index = process_index(pid);
        self.processes.get(index).and_then(|slot| slot.as_ref()).and_then(|p| p.thing_id)
    }

    pub fn thread_thing_id(&self, tid: ThreadId) -> Option<ThingId> {
        let index = thread_index(tid);
        self.threads.get(index).and_then(|slot| slot.as_ref()).and_then(|t| t.thing_id)
    }

    pub fn thread_by_thing(&self, thing: ThingId) -> Option<&Thread> {
        self.threads.iter().flatten().find(|thread| thread.thing_id == Some(thing))
    }

    pub fn thread_id_for_thing(&self, thing: ThingId) -> Option<ThreadId> {
        self.thread_by_thing(thing).map(|thread| thread.id)
    }

    fn finish_running_thread(&mut self, index: usize) {
        if let Some(thread) = self.threads.get_mut(index).and_then(|t| t.as_mut()) {
            if thread.state == ThreadState::Running {
                thread.total_run_ns = thread.total_run_ns.saturating_add(FAKE_SLICE_NS);
                self.fake_time_ns = self.fake_time_ns.saturating_add(FAKE_SLICE_NS);
            }
        }
    }

    fn runtime_snapshot(&self) -> (u64, u64, u64) {
        let mut total: u64 = 0;
        let mut idle: u64 = 0;
        let mut actualizer: u64 = 0;
        for thread in self.threads.iter().flatten() {
            total = total.saturating_add(thread.total_run_ns);
            if thread.is_idle {
                idle = idle.saturating_add(thread.total_run_ns);
            }
            if thread.name == "actualizer" {
                actualizer = thread.total_run_ns;
            }
        }
        (total, idle, actualizer)
    }

    fn maybe_log_heartbeat(&mut self, now_ns: u64) {
        if self.last_switch_ns == 0 {
            self.last_switch_ns = now_ns;
        }

        if self.last_heartbeat_ns == 0 {
            let (total, idle, actualizer) = self.runtime_snapshot();
            self.last_runtime_total = total;
            self.last_runtime_idle = idle;
            self.last_runtime_actualizer = actualizer;
            self.last_heartbeat_ns = now_ns;
            return;
        }

        let elapsed = now_ns.saturating_sub(self.last_heartbeat_ns);
        if elapsed < 1_000_000_000 {
            return;
        }

        let (total, idle, actualizer) = self.runtime_snapshot();
        let delta_total = total.saturating_sub(self.last_runtime_total);
        let delta_idle = idle.saturating_sub(self.last_runtime_idle);
        let delta_actualizer = actualizer.saturating_sub(self.last_runtime_actualizer);

        let idle_share = percent(delta_idle, delta_total);
        let actualizer_share = percent(delta_actualizer, delta_total);
        let run_q = self.cache.run_queue.len();
        let since_switch_ms = now_ns.saturating_sub(self.last_switch_ns) / 1_000_000;

        let current_str = if let Some(tid) = self.current {
            if let Some(thread) = self.threads[thread_index(tid)].as_ref() {
                alloc::format!("{}:{}", tid.0, thread.name)
            } else {
                String::from("unknown")
            }
        } else {
            String::from("none")
        };

        crate::log(&alloc::format!(
            "Sched Heartbeat: current={} run_q={} idle_share={} act_share={} switches={} max_tick={}ns avg_tick={}ns",
            current_str,
            run_q,
            idle_share,
            actualizer_share,
            self.total_switches,
            self.max_tick_time_ns,
            self.avg_tick_time_ns
        ));

        if since_switch_ms > 50 {
            crate::log(&alloc::format!(
                "Sched Watchdog: no context switch for {}ms (current={} run_q={})",
                since_switch_ms,
                current_str,
                run_q
            ));
        }

        self.last_runtime_total = total;
        self.last_runtime_idle = idle;
        self.last_runtime_actualizer = actualizer;
        self.last_heartbeat_ns = now_ns;
    }

    fn ensure_process_thing(&mut self, index: usize) {
        if !self.graph_enabled { return; }
        if let Some(process) = self.processes.get_mut(index).and_then(|p| p.as_mut()) {
            if process.thing_id.is_some() { return; }
            let props = alloc::vec![
                (crate::symbols::intern("pid"), PropValue::U64(process.id.0)),
                (crate::symbols::intern("name"), PropValue::Str(String::from(process.name))),
            ];
            process.thing_id = Some(graph::create_thing(crate::symbols::intern(graph_kinds::KIND_PROCESS), props));
        }
    }

    fn ensure_thread_thing(&mut self, index: usize) {
        if !self.graph_enabled { return; }
        if let Some(thread) = self.threads.get_mut(index).and_then(|t| t.as_mut()) {
            if thread.thing_id.is_some() { return; }
            let props = alloc::vec![
                (crate::symbols::intern("tid"), PropValue::U64(thread.id.0)),
                (crate::symbols::intern("name"), PropValue::Str(String::from(thread.name))),
                (crate::symbols::intern("state"), PropValue::Str(String::from(thread.state.as_str()))),
                (crate::symbols::intern("priority"), PropValue::U64(thread.priority)),
                (crate::symbols::intern("runtime_ns"), PropValue::U64(thread.total_run_ns)),
                (crate::symbols::intern("last_started_ns"), PropValue::U64(thread.last_run_start_ns)),
                (crate::symbols::intern("sleep_until_ns"), PropValue::U64(0)),
            ];
            thread.thing_id = Some(graph::create_thing(crate::symbols::intern(graph_kinds::KIND_THREAD), props));
            self.graph_link_process_thread(index);
        }
    }

    fn graph_link_process_thread(&mut self, thread_index: usize) {
        if !self.graph_enabled { return; }
        let Some(thread) = self.threads.get(thread_index).and_then(|t| t.as_ref()) else { return; };
        let Some(thread_thing) = thread.thing_id else { return; };
        let proc_index = process_index(thread.process_id);
        if let Some(process) = self.processes.get(proc_index).and_then(|slot| slot.as_ref()) {
            if let Some(process_thing) = process.thing_id {
                let _ = graph::add_link(process_thing, graph_kinds::LINK_OWNS_THREAD, thread_thing);
            }
        }
    }

    // Only used for initial setup or explicit manual sync
    fn graph_update_thread_state_initial(&mut self, index: usize) {
        // Implementation similar to old graph_update_thread_state but we rely on commit_switch usually.
        // We keep this for ensuring initial state is committed.
        if !self.graph_enabled { return; }
        if let Some(thread) = self.threads.get(index).and_then(|t| t.as_ref()) {
            if let Some(thread_thing) = thread.thing_id {
                let props = alloc::vec![
                    (crate::symbols::intern("state"), PropValue::Str(String::from(thread.state.as_str()))),
                    (crate::symbols::intern("runtime_ns"), PropValue::U64(thread.total_run_ns)),
                    (crate::symbols::intern("last_started_ns"), PropValue::U64(thread.last_run_start_ns)),
                ];
                let _ = graph::update_thing(thread_thing, props);
            }
        }
    }

    pub fn mark_blocked(&mut self, tid: ThreadId) -> bool {
        let index = thread_index(tid);
        // Check conditions first
        let should_block =
            if let Some(thread) = self.threads.get_mut(index).and_then(|t| t.as_mut()) {
                if thread.pending_wake {
                    thread.pending_wake = false;
                    return false;
                }
                thread.state == ThreadState::Running
            } else {
                false
            };

        if should_block {
            self.finish_running_thread(index);
            if let Some(thread) = self.threads.get_mut(index).and_then(|t| t.as_mut()) {
                thread.state = ThreadState::Blocked;
            }
            // Blocked threads are removed from run_queue (they are running, so not in queue).
            // commit_switch will see Blocked state and update graph.
            return true;
        }
        false
    }

    pub fn wake_thread(&mut self, tid: ThreadId) {
        let index = thread_index(tid);
        let state = self.threads.get(index).and_then(|t| t.as_ref()).map(|t| t.state);

        if let Some(state) = state {
            if state == ThreadState::Blocked || state == ThreadState::Sleeping {
                if state == ThreadState::Sleeping {
                    // Remove from sleep queue if present
                     if let Some(pos) = self.cache.sleep_queue.iter().position(|&e| e.thread_id == tid) {
                        self.cache.sleep_queue.swap_remove(pos);
                    }
                }

                if let Some(thread) = self.threads.get_mut(index).and_then(|t| t.as_mut()) {
                    thread.state = ThreadState::Runnable;
                    thread.pending_wake = false;

                     // Update graph immediately as we are changing state outside of a switch
                    graph_sync::commit_thread_wake(thread);
                }

                if self.cache.run_queue.push(tid).is_err() {
                     crate::log("Run queue full in wake_thread");
                }
            } else {
                // Already Running or Runnable, just mark pending
                if let Some(thread) = self.threads.get_mut(index).and_then(|t| t.as_mut()) {
                    thread.pending_wake = true;
                }
            }
        }
    }
}

fn percent(part: u64, total: u64) -> u64 {
    if total == 0 {
        return 0;
    }
    part.saturating_mul(100) / total
}
pub static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());

pub fn yield_current_thread() {
    without_preemption(|| {
        let mut sched = SCHEDULER.lock();
        if let Some(tid) = sched.current {
            sched.mark_yield(tid);
            sched.current = None; // This logic remains to signal "not running"
        }
    })
}

pub fn exit_current_thread(reason: &'static str, code: u64) {
    without_preemption(|| {
        let mut sched = SCHEDULER.lock();
        if let Some(tid) = sched.current {
            sched.mark_terminated(tid, reason, code);
            sched.current = None;
        }
    })
}
pub fn block_current_thread() {
    without_preemption(|| {
        let mut sched = SCHEDULER.lock();
        if let Some(tid) = sched.current {
            sched.mark_blocked(tid);
            sched.current = None;
        }
    });
    // Yield CPU
    #[cfg(target_arch = "x86_64")]
    x86_64::instructions::hlt();
}

pub fn wait_for_interrupt() {
    #[cfg(target_arch = "x86_64")]
    x86_64::instructions::hlt();
}
