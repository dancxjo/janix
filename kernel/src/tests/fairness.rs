use crate::BootRuntime;
use crate::task;

extern "C" fn fair_thread_a(arg: usize) -> ! {
    for i in 0..5 {
        crate::kprintln!("Fairness A: {}", i);
        unsafe { task::scheduler::yield_now_current(); }
    }
    loop { unsafe { task::scheduler::yield_now_current(); } }
}

extern "C" fn fair_thread_b(arg: usize) -> ! {
    for i in 0..5 {
        crate::kprintln!("Fairness B: {}", i);
        unsafe { task::scheduler::yield_now_current(); }
    }
    loop { unsafe { task::scheduler::yield_now_current(); } }
}

pub fn run<R: BootRuntime>() {
    crate::kprintln!("Running fairness smoke test...");
    task::spawn::<R>(fair_thread_a, 0);
    task::spawn::<R>(fair_thread_b, 0);
    
    // Yield a few times to let them run
    for _ in 0..15 {
         task::yield_now::<R>();
    }
    crate::kprintln!("Fairness test complete (check logs for interleaving)");
}
