extern crate alloc;

pub use crate::sched_types::ThreadState;
use crate::{graph, graph_kinds};
use abi::{ProcessId, PropValue, ThingId, ThreadId};
use alloc::string::String;
use heapless::Vec;
use spin::Mutex;

pub const MAX_THREADS: usize = 32;
pub const MAX_PROCESSES: usize = 16;
const FAKE_SLICE_NS: u64 = 5_000_000;

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
    pub user_entry: Option<extern "C" fn(u64) -> !>,
    pub user_arg: u64,
    pub user_stack_top: u64,
    pub context: [u64; 34],
    pub started: bool,
    pub thing_id: Option<ThingId>,
    pub sleep_event_id: Option<ThingId>,
    pub last_run_start_ns: u64,
    pub total_run_ns: u64,
}

#[derive(Copy, Clone, Debug)]
pub struct Process {
    pub id: ProcessId,
    pub name: &'static str,
    pub thing_id: Option<ThingId>,
}

pub struct Scheduler {
    // Simple round-robin run queue
    run_queue: Vec<ThreadId, MAX_THREADS>,
    threads: [Option<Thread>; MAX_THREADS],
    processes: [Option<Process>; MAX_PROCESSES],
    current: Option<ThreadId>,
    sleep_queue: Vec<SleepEntry, MAX_THREADS>,
    graph_enabled: bool,
    fake_time_ns: u64,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            run_queue: Vec::new(),
            threads: [const { None }; MAX_THREADS],
            processes: [const { None }; MAX_PROCESSES],
            current: None,
            sleep_queue: Vec::new(),
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
                self.graph_restore_sleep_edge(index);
            }
        }
    }

    pub fn sleep_current_thread(&mut self, wake_at_ns: u64) {
        let tid = self
            .current
            .expect("no current thread in sleep_current_thread");

        let index = tid.0 as usize;
        if self.threads[index].is_none() {
            panic!("sleep_current_thread: missing thread");
        }

        self.finish_running_thread(index);
        if let Some(thr) = self.threads[index].as_mut() {
            thr.state = ThreadState::Sleeping;
        }
        self.graph_update_thread_state(index);
        self.graph_record_sleep_event(index, wake_at_ns);

        // Remove from run_queue if it’s there.
        if let Some(pos) = self.run_queue.iter().position(|&id| id == tid) {
            self.run_queue.swap_remove(pos);
        }

        // Register in sleep queue.
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
                // Wake this thread.
                let index = entry.thread_id.0 as usize;
                self.graph_clear_sleep_event(index);
                if let Some(thr) = self.threads[index].as_mut() {
                    thr.state = ThreadState::Runnable;
                }
                self.graph_update_thread_state(index);

                // Put back on run queue.
                self.run_queue
                    .push(entry.thread_id)
                    .expect("run_queue full while waking sleeper");

                // Remove this entry from sleep_queue by swap_remove.
                self.sleep_queue.swap_remove(i);
                // Do NOT increment i; swapped element needs to be checked.
            } else {
                i += 1;
            }
        }
    }

    pub fn add_process(&mut self, name: &'static str) -> ProcessId {
        for (i, slot) in self.processes.iter_mut().enumerate() {
            if slot.is_none() {
                let pid = ProcessId(i as u64);

                // Create Process Thing
                let props = [("pid", PropValue::U64(pid.0))];
                let thing_id = graph::create_thing("Process", &props);

                *slot = Some(Process {
                    id: pid,
                    name,
                    thing_id,
                });
                if self.graph_enabled {
                    self.ensure_process_thing(i);
                }
                return pid;
            }
        }
        panic!("Max processes reached");
    }

    pub fn add_thread(
        &mut self,
        process_id: ProcessId,
        name: &'static str,
        entry: extern "C" fn(u64) -> !,
        arg: u64,
        stack_top: u64,
    ) -> ThreadId {
        for (i, slot) in self.threads.iter_mut().enumerate() {
            if slot.is_none() {
                let tid = ThreadId(i as u64);

                *slot = Some(Thread {
                    id: tid,
                    process_id,
                    state: ThreadState::New,
                    name,
                    priority: 0,
                    user_entry: Some(entry),
                    user_arg: arg,
                    user_stack_top: stack_top,
                    context: [0; 34],
                    started: false,
                    thing_id: None,
                    sleep_event_id: None,
                    last_run_start_ns: 0,
                    total_run_ns: 0,
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

    pub fn mark_yield(&mut self, tid: ThreadId) {
        let index = tid.0 as usize;
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
            self.graph_update_thread_state(index);

            if self.run_queue.push(tid).is_err() {
                // Should not happen if we manage queue correctly
                panic!("Run queue full on yield");
            }
        }
    }

    pub fn mark_terminated(&mut self, tid: ThreadId) {
        let index = tid.0 as usize;
        if self.threads.get(index).and_then(|t| t.as_ref()).is_some() {
            self.finish_running_thread(index);
            self.graph_clear_sleep_event(index);
            if let Some(thread) = self.threads.get_mut(index).and_then(|t| t.as_mut()) {
                thread.state = ThreadState::Exited;
            }
            self.graph_update_thread_state(index);
        }
    }

    pub fn next_runnable(&mut self) -> Option<ThreadId> {
        // round-robin: pop front
        if !self.run_queue.is_empty() {
            return Some(self.run_queue.remove(0));
        }
        None
    }

    pub fn set_current(&mut self, tid: ThreadId) {
        self.current = Some(tid);
        let index = tid.0 as usize;
        if let Some(thread) = self.threads.get_mut(index).and_then(|t| t.as_mut()) {
            thread.state = ThreadState::Running;
            thread.last_run_start_ns = self.fake_time_ns;
            thread.started = true;
        }
        self.graph_update_thread_state(index);
    }

    pub fn current_id(&self) -> Option<ThreadId> {
        self.current
    }

    pub fn thread_mut(&mut self, tid: ThreadId) -> Option<&mut Thread> {
        self.threads[tid.0 as usize].as_mut()
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
            let props = [
                ("pid", PropValue::U64(process.id.0)),
                ("name", PropValue::Str(String::from(process.name))),
            ];
            process.thing_id = graph::create_thing(graph_kinds::KIND_PROCESS, &props);
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
            let props = [
                ("tid", PropValue::U64(thread.id.0)),
                ("name", PropValue::Str(String::from(thread.name))),
                ("state", PropValue::Str(String::from(thread.state.as_str()))),
                ("priority", PropValue::U64(thread.priority)),
                ("runtime_ns", PropValue::U64(thread.total_run_ns)),
                ("last_started_ns", PropValue::U64(thread.last_run_start_ns)),
            ];
            thread.thing_id = graph::create_thing(graph_kinds::KIND_THREAD, &props);
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
        let proc_index = thread.process_id.0 as usize;
        if let Some(process) = self
            .processes
            .get(proc_index)
            .and_then(|slot| slot.as_ref())
        {
            if let Some(process_thing) = process.thing_id {
                let _ = graph::add_edge(process_thing, graph_kinds::EDGE_OWNS_THREAD, thread_thing);
            }
        }
    }

    fn graph_update_thread_state(&mut self, index: usize) {
        if !self.graph_enabled {
            return;
        }
        if let Some(thread) = self.threads.get(index).and_then(|t| t.as_ref()) {
            if let Some(thread_thing) = thread.thing_id {
                let props = [
                    ("state", PropValue::Str(String::from(thread.state.as_str()))),
                    ("runtime_ns", PropValue::U64(thread.total_run_ns)),
                    ("last_started_ns", PropValue::U64(thread.last_run_start_ns)),
                ];
                let _ = graph::update_thing(thread_thing, &props);
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

        self.graph_clear_sleep_event(index);

        let props = [
            ("wake_at_ns", PropValue::U64(wake_at_ns)),
            ("created_at_ns", PropValue::U64(self.fake_time_ns)),
        ];
        if let Some(event_id) = graph::create_thing(graph_kinds::KIND_SLEEP_EVENT, &props) {
            if let Some(thread) = self.threads.get_mut(index).and_then(|t| t.as_mut()) {
                thread.sleep_event_id = Some(event_id);
            }
            let _ = graph::add_edge(thread_id, graph_kinds::EDGE_SLEEPS_UNTIL, event_id);
        }
    }

    fn graph_clear_sleep_event(&mut self, index: usize) {
        if let Some(thread) = self.threads.get_mut(index).and_then(|t| t.as_mut()) {
            if let Some(event) = thread.sleep_event_id.take() {
                if self.graph_enabled {
                    if let Some(thread_thing) = thread.thing_id {
                        let _ =
                            graph::remove_edge(thread_thing, graph_kinds::EDGE_SLEEPS_UNTIL, event);
                    }
                    let _ = graph::delete_thing(event);
                }
            }
        }
    }

    fn graph_restore_sleep_edge(&mut self, index: usize) {
        if !self.graph_enabled {
            return;
        }
        if let Some(thread) = self.threads.get(index).and_then(|t| t.as_ref()) {
            if let (Some(thread_thing), Some(event)) = (thread.thing_id, thread.sleep_event_id) {
                let _ = graph::add_edge(thread_thing, graph_kinds::EDGE_SLEEPS_UNTIL, event);
            }
        }
    }
}

pub static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());

pub fn yield_current_thread() {
    let mut sched = SCHEDULER.lock();
    if let Some(tid) = sched.current {
        sched.mark_yield(tid);
        sched.current = None;
    }
}

pub fn exit_current_thread() {
    let mut sched = SCHEDULER.lock();
    if let Some(tid) = sched.current {
        sched.mark_terminated(tid);
        sched.current = None;
    }
}
