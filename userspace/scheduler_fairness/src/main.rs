//! Scheduler Fairness Test
//!
//! Verifies that the scheduler properly preempts CPU-bound threads so that
//! all runnable threads make progress. Fails loudly if starvation is detected.

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use stem::info;
use stem::syscall::{monotonic_ns, sleep_ms, spawn_thread};
use stem::stack::{Stack, StackSpec};

/// Shared counters for all workers
static BUSY_COUNT: AtomicU64 = AtomicU64::new(0);
static WORKER_A_COUNT: AtomicU64 = AtomicU64::new(0);
static WORKER_B_COUNT: AtomicU64 = AtomicU64::new(0);

/// Signal for threads to stop
static STOP: AtomicBool = AtomicBool::new(false);

/// Busy Bee thread - tight loop with no yields, no sleeps, no I/O
extern "C" fn busy_bee() -> ! {
    loop {
        if STOP.load(Ordering::Relaxed) {
            stem::syscall::yield_now();
        }
        BUSY_COUNT.fetch_add(1, Ordering::Relaxed);
        core::hint::black_box(());
    }
}

/// Worker A thread - tight loop incrementing counter
extern "C" fn worker_a() -> ! {
    loop {
        if STOP.load(Ordering::Relaxed) {
            stem::syscall::yield_now();
        }
        WORKER_A_COUNT.fetch_add(1, Ordering::Relaxed);
        core::hint::black_box(());
    }
}

/// Worker B thread - tight loop incrementing counter  
extern "C" fn worker_b() -> ! {
    loop {
        if STOP.load(Ordering::Relaxed) {
            stem::syscall::yield_now();
        }
        WORKER_B_COUNT.fetch_add(1, Ordering::Relaxed);
        core::hint::black_box(());
    }
}

#[stem::main]
fn main() -> ! {
    info!("=== SCHEDULER FAIRNESS TEST ===");
    info!("Spawning 3 CPU-bound threads...");
    
    // Allocate stacks for worker threads (smaller than default for test)
    let spec = StackSpec {
        reserve_bytes: 64 * 1024,
        initial_commit_bytes: 16 * 1024,
        guard_pages: 1,
        grow_chunk_bytes: 16 * 1024,
    };
    
    let stack_busy = Stack::alloc_growing_stack(spec).expect("busy stack");
    let stack_a = Stack::alloc_growing_stack(spec).expect("worker_a stack");
    let stack_b = Stack::alloc_growing_stack(spec).expect("worker_b stack");
    
    // Spawn worker threads
    let _tid_busy = spawn_thread(busy_bee, &stack_busy).expect("spawn busy");
    let _tid_a = spawn_thread(worker_a, &stack_a).expect("spawn worker_a");
    let _tid_b = spawn_thread(worker_b, &stack_b).expect("spawn worker_b");
    
    info!("Threads spawned. Starting 10-second fairness check...");
    
    let mut prev_busy = 0u64;
    let mut prev_a = 0u64;
    let mut prev_b = 0u64;
    let mut consecutive_zero_busy = 0;
    let mut consecutive_zero_a = 0;
    let mut consecutive_zero_b = 0;
    let mut passed = true;
    
    for t in 1..=10 {
        sleep_ms(1000);
        
        let busy = BUSY_COUNT.load(Ordering::Relaxed);
        let a = WORKER_A_COUNT.load(Ordering::Relaxed);
        let b = WORKER_B_COUNT.load(Ordering::Relaxed);
        
        let d_busy = busy.saturating_sub(prev_busy);
        let d_a = a.saturating_sub(prev_a);
        let d_b = b.saturating_sub(prev_b);
        
        info!(
            "t={:02} busy={:10} a={:10} b={:10} (d_busy={:8} d_a={:8} d_b={:8})",
            t, busy, a, b, d_busy, d_a, d_b
        );
        
        // Check for starvation
        if d_busy == 0 {
            consecutive_zero_busy += 1;
        } else {
            consecutive_zero_busy = 0;
        }
        
        if d_a == 0 {
            consecutive_zero_a += 1;
        } else {
            consecutive_zero_a = 0;
        }
        
        if d_b == 0 {
            consecutive_zero_b += 1;
        } else {
            consecutive_zero_b = 0;
        }
        
        // Fail if any counter stalled for 2+ consecutive seconds
        if consecutive_zero_busy >= 2 {
            info!("FAIL: busy_bee starved for 2+ seconds!");
            passed = false;
            break;
        }
        if consecutive_zero_a >= 2 {
            info!("FAIL: worker_a starved for 2+ seconds!");
            passed = false;
            break;
        }
        if consecutive_zero_b >= 2 {
            info!("FAIL: worker_b starved for 2+ seconds!");
            passed = false;
            break;
        }
        
        prev_busy = busy;
        prev_a = a;
        prev_b = b;
    }
    
    // Stop all threads
    STOP.store(true, Ordering::SeqCst);
    
    if passed {
        info!("=== SCHEDULER FAIRNESS: PASS ===");
    } else {
        info!("=== SCHEDULER FAIRNESS: FAIL ===");
    }
    
    // Exit
    stem::syscall::exit(if passed { 0 } else { 1 });
}
