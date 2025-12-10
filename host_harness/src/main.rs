use kernel_core::console::{ConsoleSink, register_sink};
use kernel_core::model::dashboard_snapshot;
use kernel_core::sched::Scheduler;

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::mpsc::{Sender, channel};
use std::thread;

mod frame_pool;
use abi::{FrameId, FrameInfo, KernelRequest, KernelResponse, MemorySummary, ThreadId};
use frame_pool::{allocate_frame, frame_stats, free_frame, init_host_frame_pool};
use heartbeat;
use hello;
use userland_rt::Sys;

struct HostConsole;
unsafe impl Sync for HostConsole {}
unsafe impl Send for HostConsole {}

impl ConsoleSink for HostConsole {
    fn write_str(&self, s: &str) {
        print!("{}", s);
    }
}

static HOST_CONSOLE: HostConsole = HostConsole;

thread_local! {
    static CURRENT_THREAD_ID: RefCell<Option<ThreadId>> = RefCell::new(None);
}

static SCHED_TX: Mutex<Option<Sender<SchedEvent>>> = Mutex::new(None);

enum SchedEvent {
    Yield(ThreadId),
    Exit(ThreadId),
}

struct HarnessSys;

impl Sys for HarnessSys {
    fn syscall(&self, request: KernelRequest) -> KernelResponse {
        match request {
            KernelRequest::AllocFrame { .. } => {
                if let Some(frame) = allocate_frame() {
                    let frame_info = FrameInfo {
                        id: FrameId(frame.id),
                        base: frame.id,
                        size: 4096,
                    };
                    KernelResponse::FrameAllocated { frame: frame_info }
                } else {
                    KernelResponse::Error {
                        message: "Host out of frames",
                    }
                }
            }
            KernelRequest::FreeFrame { frame_id } => {
                free_frame(frame_id.0);
                KernelResponse::FrameFreed { frame_id }
            }
            KernelRequest::GetMemorySummary => {
                let (total, used, free) = frame_stats();
                let summary = MemorySummary {
                    total_frames: total,
                    used_frames: used,
                    free_frames: free,
                };
                KernelResponse::MemorySummary { summary }
            }
            _ => kernel_core::handle_request(request),
        }
    }

    fn time_now_ns(&mut self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }

    fn time_monotonic_ns(&mut self) -> u64 {
        self.time_now_ns()
    }

    fn time_system_ns(&mut self) -> u64 {
        0
    }

    fn sleep_for_ns(&mut self, delta_ns: u64) {
        std::thread::sleep(std::time::Duration::from_nanos(delta_ns));
    }

    fn sleep_until_ns(&mut self, _deadline_ns: u64) {}

    fn yield_now(&mut self) {
        let tid: Option<ThreadId> = CURRENT_THREAD_ID.with(|id| *id.borrow());
        if let Some(tid) = tid {
            {
                if let Some(tx) = SCHED_TX.lock().unwrap().as_ref() {
                    // println!("Thread {:?} yielding", tid);
                    tx.send(SchedEvent::Yield(tid)).unwrap();
                }
            }
            thread::park();
            // println!("Thread {:?} resumed", tid);
        }
    }

    fn exit_thread(&mut self) -> ! {
        let tid: Option<ThreadId> = CURRENT_THREAD_ID.with(|id| *id.borrow());
        if let Some(tid) = tid {
            {
                if let Some(tx) = SCHED_TX.lock().unwrap().as_ref() {
                    tx.send(SchedEvent::Exit(tid)).unwrap();
                }
            }
            thread::park();
        }
        loop {}
    }
}

extern "C" fn dummy_entry(_: u64) -> ! {
    loop {}
}

fn main() {
    register_sink(&HOST_CONSOLE);
    kernel_core::graph::init();
    init_host_frame_pool(128);

    let (tx, rx) = channel();
    *SCHED_TX.lock().unwrap() = Some(tx);

    let mut sched = Scheduler::new();
    sched.init_graph_mirror();

    let p1 = sched.add_process("hello");
    let t1 = sched.add_thread(p1, "hello", dummy_entry, 1, 0, 0);

    let p2 = sched.add_process("heartbeat");
    let t2 = sched.add_thread(p2, "heartbeat", dummy_entry, 2, 0, 0);

    let mut threads = HashMap::new();

    // Spawn t1
    let t1_handle = thread::spawn(move || {
        CURRENT_THREAD_ID.with(|id: &RefCell<Option<ThreadId>>| *id.borrow_mut() = Some(t1));
        thread::park(); // Wait for scheduler
        let mut sys = HarnessSys;
        hello::run(&mut sys);
    });
    threads.insert(t1, t1_handle.thread().clone());

    // Spawn t2
    let t2_handle = thread::spawn(move || {
        CURRENT_THREAD_ID.with(|id: &RefCell<Option<ThreadId>>| *id.borrow_mut() = Some(t2));
        thread::park(); // Wait for scheduler
        let mut sys = HarnessSys;
        heartbeat::run(&mut sys);
    });
    threads.insert(t2, t2_handle.thread().clone());

    println!("Starting scheduler loop...");
    while !sched.all_done() {
        if let Some(tid) = sched.next_runnable() {
            sched.set_current(tid);

            if let Some(handle) = threads.get(&tid) {
                handle.unpark();
            }

            let event = rx.recv().unwrap();
            match event {
                SchedEvent::Yield(t) => {
                    assert_eq!(t, tid);
                    sched.mark_yield(tid);
                }
                SchedEvent::Exit(t) => {
                    assert_eq!(t, tid);
                    sched.mark_terminated(tid);
                }
            }
        } else {
            break;
        }
    }
    println!("Finished scheduler loop.");

    // Print dashboard snapshot
    let snap = dashboard_snapshot();
    let (mem_total, mem_used, mem_free) = frame_stats();

    println!("Host Dashboard Snapshot:");
    println!(
        "  Memory: total={} used={} free={}",
        mem_total, mem_used, mem_free,
    );
    println!(
        "  Scheduler: processes={} threads={} runnable={}",
        snap.scheduler.process_count, snap.scheduler.thread_count, snap.scheduler.runnable_threads,
    );
    println!("  Things:");
    println!("    total       = {}", snap.counts.total_things);
    println!("    processes   = {}", snap.counts.processes);
    println!("    threads     = {}", snap.counts.threads);
    println!("    PhysFrame   = {}", snap.counts.phys_frames);
    println!("    VirtRegion  = {}", snap.counts.virt_regions);
    println!("    FramePool   = {}", snap.counts.frame_pools);
    println!("    AddressSpace= {}", snap.counts.address_spaces);
    println!("    CpuCore     = {}", snap.counts.cpu_cores);

    println!("\nScheduler Graph Snapshot:");
    println!("  Threads:");
    kernel_core::graph::iter_things(|thing| {
        if thing.kind == "Thread" {
            let mut name = "unknown";
            let mut state = "unknown";
            let mut runtime_ns = 0_u64;

            for prop in thing.props.iter().flatten() {
                match prop.0 {
                    "name" => {
                        if let abi::PropValue::Str(s) = &prop.1 {
                            name = s
                        }
                    }
                    "state" => {
                        if let abi::PropValue::Str(s) = &prop.1 {
                            state = s
                        }
                    }
                    "runtime_ns" => {
                        if let abi::PropValue::U64(v) = prop.1 {
                            runtime_ns = v
                        }
                    }
                    _ => {}
                }
            }
            println!(
                "    - {} (id={}): state={} runtime_ns={}",
                name, thing.id.0, state, runtime_ns
            );
        }
    });

    println!("  SleepEvents:");
    let mut found_sleep = false;
    kernel_core::graph::iter_things(|thing| {
        if thing.kind == "SleepEvent" {
            found_sleep = true;
            let mut wake_at = 0_u64;
            let mut created_at = 0_u64;
            for prop in thing.props.iter().flatten() {
                match prop.0 {
                    "wake_at_ns" => {
                        if let abi::PropValue::U64(v) = prop.1 {
                            wake_at = v
                        }
                    }
                    "created_at_ns" => {
                        if let abi::PropValue::U64(v) = prop.1 {
                            created_at = v
                        }
                    }
                    _ => {}
                }
            }
            println!("    - wake_at={} created_at={}", wake_at, created_at);
        }
    });
    if !found_sleep {
        println!("    (none)");
    }
}
