extern crate alloc;

pub use crate::sched_types::ThreadState;
use crate::{graph, graph_kinds};
use abi::{ProcessId, ThingId, ThreadId, USER_HEAP_END};
use alloc::string::String;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use heapless::Vec;
use spin::Mutex;
use thing_models::PropValue;

pub const MAX_THREADS: usize = 32;
pub const MAX_PROCESSES: usize = 16;
pub static NEED_RESCHED: AtomicBool = AtomicBool::new(false);

// Until you have a real timer/preemption story, we still need forward progress
// so that sleepers wake and userspace "ticks" without huge delays.
const FAKE_SLICE_NS: u64 = 5_000_000; // 5ms

#[derive(Copy, Clone, Debug)]
pub struct SleepEntry {
    pub thread_id: ThreadId,
    pub wake_at_ns: u64,
}

pub static TICKS: AtomicU64 = AtomicU64::new(0);
pub static PREEMPT_COUNT: AtomicU32 = AtomicU32::new(0);

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



#[repr(align(16))]
#[derive(Copy, Clone, Debug)]
pub struct FpuContext {
    pub data: [u8; 512],
}

impl Default for FpuContext {
    fn default() -> Self {
        let mut data = [0u8; 512];
        // FCW = 0x037F
        data[0] = 0x7F;
        data[1] = 0x03;
        // MXCSR = 0x00001F80
        data[24] = 0x80;
        data[25] = 0x1F;
        Self { data }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Thread {
    pub id: ThreadId,
    pub process_id: ProcessId,
    pub state: ThreadState,
    pub name: &'static str,
    pub priority: u64,
    pub entry_point: u64,
    pub user_arg: u64,
    pub user_stack_top: u64,
    pub context: [u64; 20],
    pub fpu_context: FpuContext,
    pub started: bool,
    pub thing_id: Option<ThingId>,
    pub sleep_event_id: Option<ThingId>,
    pub sleep_until_ns: u64,
    pub last_run_start_ns: u64,
    pub total_run_ns: u64,
    pub address_space_token: Option<u64>,
    pub pending_wake: bool,
    pub is_idle: bool,
}

pub struct ScheduledThread {
    pub tid: ThreadId,
    pub name: &'static str,
    pub started: bool,
    pub entry_point: u64,
    pub user_stack_top: u64,
    pub user_arg: u64,
    pub context: [u64; 20],
    pub fpu_context: FpuContext,
    pub address_space_token: Option<u64>,
    pub is_idle: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct Process {
    pub id: ProcessId,
    pub name: &'static str,
    pub thing_id: Option<ThingId>,
    pub address_space_token: Option<u64>,
    pub heap_base: usize,
    pub heap_limit: usize,
    pub next_map_base: u64,
    pub next_resident_map_base: u64,
}

pub struct Scheduler {
    // HOT PATH: keep queues in-memory; graph is a mirror, not the scheduler's brain.
    run_queue: Vec<ThreadId, MAX_THREADS>,
    sleep_queue: Vec<SleepEntry, MAX_THREADS>,

    threads: [Option<Thread>; MAX_THREADS],
    processes: [Option<Process>; MAX_PROCESSES],
    current: Option<ThreadId>,

    graph_enabled: bool,
    fake_time_ns: u64,
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
            run_queue: Vec::new(),
            sleep_queue: Vec::new(),
            threads: [const { None }; MAX_THREADS],
            processes: [const { None }; MAX_PROCESSES],
            current: None,
            graph_enabled: false,
            fake_time_ns: 0,
        }
    }

    /// Enable graph mirroring for scheduler state. Once enabled, process and thread
    /// mutations are reflected into the kernel graph so hosted environments can
    /// inspect scheduling state the same way as on real hardware.
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
                self.graph_update_thread_state(index);
                // self.graph_restore_sleep_link(index);
            }
        }
    }

    pub fn set_current(&mut self, tid: ThreadId) {
        let idx = thread_index(tid);
        if let Some(thr) = self.threads[idx].as_mut() {
            thr.state = ThreadState::Running;
            thr.last_run_start_ns = self.fake_time_ns;
            self.graph_update_thread_state(idx);
        }
        self.current = Some(tid);
    }

    fn advance_time_slice(&mut self) {
        // If you later wire a real monotonic clock, swap this out.
        self.fake_time_ns = self.fake_time_ns.saturating_add(FAKE_SLICE_NS);
    }

    pub fn sleep_current_until(&mut self, wake_at_ns: u64) {
        let tid = match self.current {
            Some(t) => t,
            None => return,
        };
        let idx = thread_index(tid);
        if let Some(thr) = self.threads[idx].as_mut() {
            thr.state = ThreadState::Sleeping;
            thr.sleep_until_ns = wake_at_ns;
            self.graph_update_thread_state(idx);
            self.graph_record_sleep_event(idx, wake_at_ns);
        }

        self.sleep_queue
            .push(SleepEntry { thread_id: tid, wake_at_ns })
            .expect("sleep_queue full");
        self.current = None;
    }

    pub fn wake_sleepers(&mut self, now_ns: u64) {
        let mut i = 0;
        while i < self.sleep_queue.len() {
            let entry = self.sleep_queue[i];
            if entry.wake_at_ns <= now_ns {
                let idx = thread_index(entry.thread_id);
                self.graph_clear_sleep_event(idx);
                if let Some(thr) = self.threads[idx].as_mut() {
                    thr.state = ThreadState::Runnable;
                    thr.sleep_until_ns = 0;
                }
                self.graph_update_thread_state(idx);
                self.run_queue
                    .push(entry.thread_id)
                    .expect("run_queue full while waking sleeper");
                self.sleep_queue.swap_remove(i);
                // swapped element must be checked; do not i += 1
            } else {
                i += 1;
            }
        }
    }

    pub fn add_process(&mut self, name: &'static str) -> ProcessId {
        for (i, slot) in self.processes.iter_mut().enumerate() {
            if slot.is_none() {
                let pid = ProcessId(i as u64 + 1);

                // Create Process Thing
                let props = alloc::vec![(crate::symbols::intern("pid"), PropValue::U64(pid.0))];
                let thing_id = Some(graph::create_thing(
                    crate::symbols::intern(graph_kinds::KIND_PROCESS),
                    props,
                ));

                *slot = Some(Process {
                    id: pid,
                    name,
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
                    last_run_start_ns: 0,
                    total_run_ns: 0,
                    address_space_token,
                    pending_wake: false,
                    is_idle: false,
                    sleep_until_ns: 0,
                });
                if self.graph_enabled {
                    self.ensure_thread_thing(i);
                    self.graph_update_thread_state(i);
                }
                // Add to run queue
                if self.run_queue.push(tid).is_err() {
                    panic!("Run queue full");
                }
                return tid;
            }
        }
        panic!("Max threads reached");
    }

    pub fn add_idle_thread(&mut self, process_id: ProcessId) -> ThreadId {
        for (i, slot) in self.threads.iter_mut().enumerate() {
            if slot.is_none() {
                let tid = ThreadId(i as u64 + 1);
                // Idle thread shares address space of the process (likely kernel/init)
                let address_space_token = self
                    .processes
                    .get(process_index(process_id))
                    .and_then(|p| p.as_ref())
                    .and_then(|p| p.address_space_token);

                *slot = Some(Thread {
                    id: tid,
                    process_id,
                    state: ThreadState::Runnable, // Always runnable
                    name: "idle",
                    priority: 0, // Lowest priority
                    entry_point: 0,
                    user_arg: 0,
                    user_stack_top: 0,
                    context: [0; 20],
                    fpu_context: FpuContext::default(),
                    started: false,
                    thing_id: None,
                    sleep_event_id: None,
                    last_run_start_ns: 0,
                    total_run_ns: 0,
                    address_space_token,
                    pending_wake: false,
                    is_idle: true,
                    sleep_until_ns: 0,
                });
                if self.graph_enabled {
                    self.ensure_thread_thing(i);
                    self.graph_update_thread_state(i);
                }
                // Add to run queue
                if self.run_queue.push(tid).is_err() {
                    panic!("Run queue full creating idle thread");
                }
                return tid;
            }
        }
        panic!("Max threads reached creating idle thread");
    }

    pub fn yield_current(&mut self) {
        if let Some(tid) = self.current.take() {
            let idx = thread_index(tid);
            if let Some(thr) = self.threads[idx].as_mut() {
                if thr.state == ThreadState::Running {
                    thr.state = ThreadState::Runnable;
                    self.graph_update_thread_state(idx);
                    self.run_queue.push(tid).expect("run_queue full on yield");
                }
            }
        }
    }

    pub fn mark_terminated(&mut self, tid: ThreadId, reason: &'static str, code: u64) {
        let index = thread_index(tid);
        if self.threads.get(index).and_then(|t| t.as_ref()).is_some() {
            self.finish_running_thread(index);
            self.graph_clear_sleep_event(index);

            let mut pid = ProcessId(0);
            if let Some(thread) = self.threads.get_mut(index).and_then(|t| t.as_mut()) {
                thread.state = ThreadState::Exited;
                pid = thread.process_id;
            }
            self.graph_update_thread_state(index);

            if self.is_process_dead(pid) {
                self.emit_process_exit_event(pid, reason, code);
            }
        }
    }

    fn is_process_dead(&self, pid: ProcessId) -> bool {
        for thread in self.threads.iter().flatten() {
            if thread.process_id == pid && thread.state != ThreadState::Exited {
                return false;
            }
        }
        true
    }

    fn emit_process_exit_event(&self, pid: ProcessId, reason: &'static str, code: u64) {
        if !self.graph_enabled {
            return;
        }
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

            // Check for respawn policy
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
                    graph_kinds::RESPAWN_ON_CRASH => reason != "Exited", // Assuming "Exited" is normal exit?
                    _ => false,
                };

                if should_respawn {
                    // Defer spawning to avoid deadlock with scheduler lock
                    // Pass the old process thing ID to enable linking RESPAWNED_FROM
                    crate::work_queue::push_normal(crate::work_queue::WorkItem::SpawnProgram(
                        bp_id,
                        Some(proc_thing),
                    ));
                    crate::log("Respawn scheduled via WorkQueue");
                }
            }
        }
    }

    fn choose_next_runnable(&mut self) -> Option<ThreadId> {
        if self.run_queue.is_empty() { return None; }

        // Strict: highest priority wins; ties remain FIFO.
        let mut best_index = 0usize;
        let mut best_prio = 0u64;
        for (i, &tid) in self.run_queue.iter().enumerate() {
            let idx = thread_index(tid);
            if let Some(thr) = self.threads[idx].as_ref() {
                if thr.priority > best_prio {
                    best_prio = thr.priority;
                    best_index = i;
                }
            }
        }
        Some(self.run_queue.remove(best_index))
    }

    pub fn tick(&mut self) {
        // 1) move time forward (until you have a real timer)
        self.advance_time_slice();

        // Advance kernel timekeeping by 1 tick (assuming FAKE_SLICE_NS ~ 1ms)
        // This drives TimeSource updates and Alarms.
        crate::time::advance_ticks(1);

        // 2) wake sleepers due at "now"
        // Only wake sleepers once per second (or roughly so) to avoid excessive calls
        // to wake_sleepers which iterates over all threads.
        if self.fake_time_ns % 1_000_000_000 < FAKE_SLICE_NS {
            let now = self.fake_time_ns;
            self.wake_sleepers(now);
        }

        // 3) pick next runnable and context switch if needed
        if self.current.is_none() {
            if let Some(next) = self.choose_next_runnable() {
                self.set_current(next);
            }
        }
    }

    // Compatibility methods for arch crate
    pub fn sleep_current_thread(&mut self, wake_at_ns: u64) {
        self.sleep_current_until(wake_at_ns);
    }

    pub fn mark_yield(&mut self, _tid: ThreadId) {
        // arch calls this after saving context.
        // We assume it means the current thread wants to yield.
        self.yield_current();
    }

    pub fn choose_next_thread(&mut self, _now_ns: u64) -> Option<ScheduledThread> {
        // Use fake_time_ns because that's what tick() advances
        self.wake_sleepers(self.fake_time_ns);
        
        // If we don't have a current thread (e.g. yielded or slept), pick one
        if self.current.is_none() {
             if let Some(next) = self.choose_next_runnable() {
                 self.set_current(next);
             }
        }

        let tid = self.current?;
        
        let idx = thread_index(tid);
        let thread = self.threads[idx].as_ref()?;
        
        Some(ScheduledThread {
            tid: thread.id,
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

    pub fn current_id(&self) -> Option<ThreadId> {
        self.current
    }

    pub fn thread_mut(&mut self, tid: ThreadId) -> Option<&mut Thread> {
        self.threads[thread_index(tid)].as_mut()
    }

    pub fn process_thing_id(&self, pid: ProcessId) -> Option<ThingId> {
        let index = process_index(pid);
        self.processes
            .get(index)
            .and_then(|slot| slot.as_ref())
            .and_then(|process| process.thing_id)
    }

    pub fn thread_thing_id(&self, tid: ThreadId) -> Option<ThingId> {
        let index = thread_index(tid);
        self.threads
            .get(index)
            .and_then(|slot| slot.as_ref())
            .and_then(|thread| thread.thing_id)
    }

    pub fn thread_by_thing(&self, thing: ThingId) -> Option<&Thread> {
        self.threads
            .iter()
            .flatten()
            .find(|thread| thread.thing_id == Some(thing))
    }

    #[allow(dead_code)]
    pub fn thread_mut_by_thing(&mut self, thing: ThingId) -> Option<&mut Thread> {
        self.threads
            .iter_mut()
            .flatten()
            .find(|thread| thread.thing_id == Some(thing))
    }

    pub fn thread_id_for_thing(&self, thing: ThingId) -> Option<ThreadId> {
        self.thread_by_thing(thing).map(|thread| thread.id)
    }

    pub fn all_done(&self) -> bool {
        for thread in self.threads.iter().flatten() {
            if thread.state != ThreadState::Exited {
                return false;
            }
        }
        true
    }

    fn finish_running_thread(&mut self, index: usize) {
        if let Some(thread) = self.threads.get_mut(index).and_then(|t| t.as_mut()) {
            if thread.state == ThreadState::Running {
                thread.total_run_ns = thread.total_run_ns.saturating_add(FAKE_SLICE_NS);
                self.fake_time_ns = self.fake_time_ns.saturating_add(FAKE_SLICE_NS);
            }
        }
    }

    fn ensure_process_thing(&mut self, index: usize) {
        if !self.graph_enabled {
            return;
        }
        if let Some(process) = self.processes.get_mut(index).and_then(|p| p.as_mut()) {
            if process.thing_id.is_some() {
                return;
            }
            let props = alloc::vec![
                (crate::symbols::intern("pid"), PropValue::U64(process.id.0)),
                (
                    crate::symbols::intern("name"),
                    PropValue::Str(String::from(process.name))
                ),
            ];
            process.thing_id = Some(graph::create_thing(
                crate::symbols::intern(graph_kinds::KIND_PROCESS),
                props,
            ));
        }
    }

    fn ensure_thread_thing(&mut self, index: usize) {
        if !self.graph_enabled {
            return;
        }
        if let Some(thread) = self.threads.get_mut(index).and_then(|t| t.as_mut()) {
            if thread.thing_id.is_some() {
                return;
            }
            let props = alloc::vec![
                (crate::symbols::intern("tid"), PropValue::U64(thread.id.0)),
                (
                    crate::symbols::intern("name"),
                    PropValue::Str(String::from(thread.name))
                ),
                (
                    crate::symbols::intern("state"),
                    PropValue::Str(String::from(thread.state.as_str()))
                ),
                (
                    crate::symbols::intern("priority"),
                    PropValue::U64(thread.priority)
                ),
                (
                    crate::symbols::intern("runtime_ns"),
                    PropValue::U64(thread.total_run_ns)
                ),
                (
                    crate::symbols::intern("last_started_ns"),
                    PropValue::U64(thread.last_run_start_ns)
                ),
                (crate::symbols::intern("sleep_until_ns"), PropValue::U64(0)),
            ];
            thread.thing_id = Some(graph::create_thing(
                crate::symbols::intern(graph_kinds::KIND_THREAD),
                props,
            ));
            self.graph_link_process_thread(index);
        }
    }

    fn graph_link_process_thread(&mut self, thread_index: usize) {
        if !self.graph_enabled {
            return;
        }
        let Some(thread) = self.threads.get(thread_index).and_then(|t| t.as_ref()) else {
            return;
        };
        let Some(thread_thing) = thread.thing_id else {
            return;
        };
        let proc_index = process_index(thread.process_id);
        if let Some(process) = self
            .processes
            .get(proc_index)
            .and_then(|slot| slot.as_ref())
        {
            if let Some(process_thing) = process.thing_id {
                let _ = graph::add_link(process_thing, graph_kinds::LINK_OWNS_THREAD, thread_thing);
            }
        }
    }

    fn graph_update_thread_state(&mut self, index: usize) {
        if !self.graph_enabled {
            return;
        }
        if let Some(thread) = self.threads.get(index).and_then(|t| t.as_ref()) {
            if let Some(thread_thing) = thread.thing_id {
                let props = alloc::vec![
                    (
                        crate::symbols::intern("state"),
                        PropValue::Str(String::from(thread.state.as_str()))
                    ),
                    (
                        crate::symbols::intern("runtime_ns"),
                        PropValue::U64(thread.total_run_ns)
                    ),
                    (
                        crate::symbols::intern("last_started_ns"),
                        PropValue::U64(thread.last_run_start_ns)
                    ),
                ];
                let _ = graph::update_thing(thread_thing, props);
            }
        }
    }

    fn graph_record_sleep_event(&mut self, index: usize, wake_at_ns: u64) {
        if !self.graph_enabled {
            return;
        }
        let thread_thing = self
            .threads
            .get(index)
            .and_then(|t| t.as_ref())
            .and_then(|thread| thread.thing_id);
        let Some(thread_id) = thread_thing else {
            return;
        };

        // Update the thread with the sleep deadline
        let props = alloc::vec![(
            crate::symbols::intern("sleep_until_ns"),
            PropValue::U64(wake_at_ns)
        ),];
        let _ = graph::update_thing(thread_id, props);
    }

    fn graph_clear_sleep_event(&mut self, index: usize) {
        if !self.graph_enabled {
            return;
        }
        if let Some(thread) = self.threads.get(index).and_then(|t| t.as_ref()) {
            if let Some(thread_thing) = thread.thing_id {
                let props =
                    alloc::vec![(crate::symbols::intern("sleep_until_ns"), PropValue::U64(0))];
                let _ = graph::update_thing(thread_thing, props);
            }
        }
    }

    fn graph_restore_sleep_link(&mut self, _index: usize) {
        // No-op: sleep state is now on the thread itself, and we don't persist
        // sleep events across graph re-initialization in this simplified model.
    }

    pub fn mark_blocked(&mut self, tid: ThreadId) -> bool {
        let index = thread_index(tid);
        // Check conditions first
        let should_block =
            if let Some(thread) = self.threads.get_mut(index).and_then(|t| t.as_mut()) {
                if thread.pending_wake {
                    // Was woken while running, so don't block
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
            self.graph_update_thread_state(index);
            return true;
        }
        false
    }

    pub fn wake_thread(&mut self, tid: ThreadId) {
        let index = thread_index(tid);
        let state = self
            .threads
            .get(index)
            .and_then(|t| t.as_ref())
            .map(|t| t.state);

        if let Some(state) = state {
            if state == ThreadState::Blocked || state == ThreadState::Sleeping {
                if state == ThreadState::Sleeping {
                    self.graph_clear_sleep_event(index);
                    // Remove from sleep queue if present
                    if let Some(pos) = self.sleep_queue.iter().position(|&e| e.thread_id == tid) {
                        self.sleep_queue.swap_remove(pos);
                    }
                }

                if let Some(thread) = self.threads.get_mut(index).and_then(|t| t.as_mut()) {
                    thread.state = ThreadState::Runnable;
                    thread.pending_wake = false;
                }
                self.graph_update_thread_state(index);
                if self.run_queue.push(tid).is_err() {
                    // Queue full, but we must run eventualy. Panic for now.
                    panic!("Run queue full in wake_thread");
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

pub static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());

pub fn yield_current_thread() {
    with_scheduler(|sched| {
        sched.yield_current();
    });
}

pub fn exit_current_thread(reason: &'static str, code: u64) {
    with_scheduler(|sched| {
        if let Some(tid) = sched.current {
            sched.mark_terminated(tid, reason, code);
            sched.current = None;
        }
    });
}
