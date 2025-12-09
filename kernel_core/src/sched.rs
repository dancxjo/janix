use abi::{ProcessId, ThreadId, ThingId, PropValue};
use heapless::Vec;
use spin::Mutex;
use crate::sched_graph::SchedulerGraphMirror;
use crate::graph;

pub const MAX_THREADS: usize = 32;
pub const MAX_PROCESSES: usize = 16;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ThreadState {
    New,
    Runnable,
    Running,
    Waiting,
    Sleeping,
    Terminated,
}

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
    pub user_entry: Option<extern "C" fn(u64) -> !>,
    pub user_arg: u64,
    pub user_stack_top: u64,
    pub context: [u64; 34],
    pub started: bool,
    pub thing_id: Option<ThingId>,
    pub sleep_event_id: Option<ThingId>,
    pub last_run_start_ns: u64,
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
    graph_mirror: Option<SchedulerGraphMirror>,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            run_queue: Vec::new(),
            threads: [const { None }; MAX_THREADS],
            processes: [const { None }; MAX_PROCESSES],
            current: None,
            sleep_queue: Vec::new(),
            graph_mirror: None,
        }
    }

    pub fn init_graph_mirror(&mut self) {
        self.graph_mirror = Some(SchedulerGraphMirror::new());
    }

    pub fn sleep_current_thread(&mut self, wake_at_ns: u64) {
        let tid = self
            .current
            .expect("no current thread in sleep_current_thread");

        let thr = self.threads[tid.0 as usize]
            .as_mut()
            .expect("sleep_current_thread: missing thread");

        thr.state = ThreadState::Sleeping;

        if let Some(mirror) = &mut self.graph_mirror {
            if let Some(thing_id) = thr.thing_id {
                mirror.update_thread_state(thing_id, ThreadState::Sleeping, 0);
                let sleep_id = mirror.create_sleep_event(thing_id, wake_at_ns, 0);
                thr.sleep_event_id = Some(sleep_id);
                // We should also record run slice here, but we need current time.
                // Assuming 0 for now or we skip it.
            }
        }

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
                if let Some(thr) = self.threads[entry.thread_id.0 as usize].as_mut() {
                    thr.state = ThreadState::Runnable;

                    if let Some(mirror) = &mut self.graph_mirror {
                        if let Some(thing_id) = thr.thing_id {
                            mirror.update_thread_state(thing_id, ThreadState::Runnable, now_ns);
                            if let Some(sleep_id) = thr.sleep_event_id {
                                mirror.clear_sleep_event(sleep_id);
                                thr.sleep_event_id = None;
                            }
                        }
                    }
                }

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
                let props = [
                    ("pid", PropValue::U64(pid.0)),
                ];
                let thing_id = graph::create_thing("Process", &props);

                *slot = Some(Process { id: pid, name, thing_id });
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
                
                let mut thing_id = None;
                if let Some(mirror) = &mut self.graph_mirror {
                    let proc_thing_id = self.processes[process_id.0 as usize]
                        .as_ref()
                        .and_then(|p| p.thing_id);
                    
                    if let Some(pid_thing) = proc_thing_id {
                        thing_id = Some(mirror.register_thread(pid_thing, name));
                    }
                }

                *slot = Some(Thread {
                    id: tid,
                    process_id,
                    state: ThreadState::New,
                    name,
                    user_entry: Some(entry),
                    user_arg: arg,
                    user_stack_top: stack_top,
                    context: [0; 34],
                    started: false,
                    thing_id,
                    sleep_event_id: None,
                    last_run_start_ns: 0,
                });
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
        if let Some(thread) = &mut self.threads[tid.0 as usize] {
            if thread.state == ThreadState::Running {
                thread.state = ThreadState::Runnable;
                
                if let Some(mirror) = &mut self.graph_mirror {
                    if let Some(thing_id) = thread.thing_id {
                        mirror.update_thread_state(thing_id, ThreadState::Runnable, 0);
                        mirror.record_run_slice(thing_id, thread.last_run_start_ns, 0);
                    }
                }

                if self.run_queue.push(tid).is_err() {
                    // Should not happen if we manage queue correctly
                    panic!("Run queue full on yield");
                }
            }
        }
    }

    pub fn mark_terminated(&mut self, tid: ThreadId) {
        if let Some(thread) = &mut self.threads[tid.0 as usize] {
            thread.state = ThreadState::Terminated;

            if let Some(mirror) = &mut self.graph_mirror {
                if let Some(thing_id) = thread.thing_id {
                    mirror.update_thread_state(thing_id, ThreadState::Terminated, 0);
                    mirror.record_run_slice(thing_id, thread.last_run_start_ns, 0);
                }
            }
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
        if let Some(thread) = &mut self.threads[tid.0 as usize] {
            thread.state = ThreadState::Running;
            thread.last_run_start_ns = 0; // TODO: use real time

            if let Some(mirror) = &mut self.graph_mirror {
                if let Some(thing_id) = thread.thing_id {
                    mirror.update_thread_state(thing_id, ThreadState::Running, 0);
                }
            }
        }
    }

    pub fn current_id(&self) -> Option<ThreadId> {
        self.current
    }

    pub fn thread_mut(&mut self, tid: ThreadId) -> Option<&mut Thread> {
        self.threads[tid.0 as usize].as_mut()
    }

    pub fn all_done(&self) -> bool {
        for thread in self.threads.iter().flatten() {
            if thread.state != ThreadState::Terminated {
                return false;
            }
        }
        true
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
