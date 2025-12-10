#[test]
fn test_memory_and_scheduler_summary_via_userland_std() {
    // Initialize kernel core (simulated)
    kernel_core::init();
    kernel_core::create_builtin_things();

    kernel_core::init_boot_graph();

    let mem = userland_std::memory_summary().expect("memory summary");
    assert!(mem.total_frames >= mem.used_frames);
    // init_boot_graph creates 3 PhysFrames
    assert!(mem.total_frames >= 3);

    let sched = userland_std::scheduler_summary().expect("scheduler summary");
    // init_boot_graph creates 1 thread
    assert!(sched.thread_count >= 1);
    assert!(sched.runnable_threads >= 1);
}
