use kernel_core::model;

#[test]
fn test_memory_allocator() {
    kernel_core::init();
    kernel_core::create_builtin_things();
    kernel_core::init_boot_graph();

    // init_boot_graph creates 3 frames.
    // Let's try to allocate them.
    let f1 = model::alloc_frame().expect("frame 1");
    let f2 = model::alloc_frame().expect("frame 2");
    let f3 = model::alloc_frame().expect("frame 3");

    // Should be out of frames now
    assert!(model::alloc_frame().is_none());

    // Free one
    assert!(model::free_frame(f2.id));

    // Allocate again - should get one back
    let f4 = model::alloc_frame().expect("frame 4");
    assert_eq!(f4.id, f2.id);
}

#[test]
fn test_scheduler_basic() {
    kernel_core::init();
    kernel_core::create_builtin_things();
    kernel_core::init_boot_graph(); 
    // init_boot_graph creates Process(1) and Thread(1) (Running)

    // Create another process/thread
    // Note: create_process_abi returns Option<u64> (pid), not bool
    assert!(model::create_process_abi(2).is_some());
    // create_thread_abi returns Option<u64> (tid)
    assert!(model::create_thread_abi(2, 2, 10).is_some()); 

    // Tick 1: 
    // init_boot_graph creates Thread 1 (Running).
    // create_thread_abi creates Thread 2 (Ready) and sets CURRENT_THREAD = Thread 2 (because CURRENT_THREAD was None).
    // So Thread 2 is current.
    // Tick 1: Thread 2 -> runtime=1, state=READY.
    // Pick best: Thread 1 (runtime=0), Thread 2 (runtime=1). Thread 1 wins.
    
    let t = model::scheduler_tick().expect("tick 1");
    assert_eq!(t.tid, 1);
    assert_eq!(t.state, 1); // RUNNING

    // Tick 2: Thread 1 running.
    // 1. Thread 1 -> runtime=1, state=READY
    // 2. Pick best: Thread 1 (runtime=1), Thread 2 (runtime=1). 
    // Thread 1 (ID 4) vs Thread 2 (ID 13). Thread 1 wins.
    let t2 = model::scheduler_tick().expect("tick 2");
    assert_eq!(t2.tid, 1);
}
