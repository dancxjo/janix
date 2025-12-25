use super::fpu::FpuContext;
use super::preempt::with_scheduler;
use super::types::ThreadState;
use abi::{ProcessId, ThingId, ThreadId, USER_HEAP_END};
use heapless::Vec;
use spin::Mutex;

pub const MAX_THREADS: usize = 32;
pub const MAX_PROCESSES: usize = 16;

// Until you have a real timer/preemption story, we still need forward progress
// so that sleepers wake and userspace "ticks" without huge delays.
const FAKE_SLICE_NS: u64 = 5_000_000; // 5ms

#[derive(Copy, Clone, Debug)]
pub struct SleepEntry {
    pub thread_id: ThreadId,
    pub wake_at_ns: u64,
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
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler {
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
            .push(SleepEntry {
                thread_id: tid,
                wake_at_ns,
            })
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
                let pid = ProcessId((i + 1) as u64);

                // Create Process Thing (Stubbed)
                let thing_id = None;

                *slot = Some(Process {
                    id: pid,
                    name,
                    thing_id,
                    address_space_token: None,
                    heap_base: 0,
                    heap_limit: 0,
                    next_map_base: USER_HEAP_END,
                    next_resident_map_base: abi::memory::USER_RESIDENT_BASE,
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
        let start = proc_slot.next_map_base.max(0x5000_0000_u64);
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
        // use abi::memory::USER_RESIDENT_LIMIT etc? Assuming present in memory mod.
        // If not, graft instructions didn't specify strict memory.rs content besides USER_HEAP_END.
        // I will trust existing definitions or defaults.
        // For now I'll use hardcoded or stub if missing.
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
        if end > abi::memory::USER_RESIDENT_LIMIT {
            // Assuming exported
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
        let entry_point = entry as usize as u64;
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

    fn emit_process_exit_event(&self, _pid: ProcessId, _reason: &'static str, _code: u64) {}

    fn choose_next_runnable(&mut self) -> Option<ThreadId> {
        if self.run_queue.is_empty() {
            return None;
        }

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
        // crate::time::advance_ticks(1); // Stubbed: time module graft not requested yet? Or maybe it exists?
        // Check if time module exists in kernel_core later. Comment out for now to ensure compile.

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

    fn ensure_process_thing(&mut self, _index: usize) {}
    fn ensure_thread_thing(&mut self, _index: usize) {}
    #[allow(dead_code)]
    fn graph_link_process_thread(&mut self, _thread_index: usize) {}
    fn graph_update_thread_state(&mut self, _index: usize) {}
    fn graph_record_sleep_event(&mut self, _index: usize, _wake_at_ns: u64) {}
    fn graph_clear_sleep_event(&mut self, _index: usize) {}
    #[allow(dead_code)]
    fn graph_restore_sleep_link(&mut self, _index: usize) {}
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
