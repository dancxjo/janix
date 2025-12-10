use kernel_core::model;
use std::sync::Mutex;

static TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn test_memory_allocator() {
    let _guard = TEST_LOCK.lock().unwrap();
    kernel_core::init();
    kernel_core::create_builtin_things();
    kernel_core::init_boot_graph();

    // init_boot_graph creates 3 frames.
    // Let's try to allocate them.
    let _f1 = model::alloc_frame().expect("frame 1");
    let f2 = model::alloc_frame().expect("frame 2");
    let _f3 = model::alloc_frame().expect("frame 3");

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
    let _guard = TEST_LOCK.lock().unwrap();
    kernel_core::init();
    kernel_core::create_builtin_things();
    kernel_core::init_boot_graph();
    // init_boot_graph creates Process(1) and Thread(1) (Running and bound to CpuCore 0)

    // Create another process/thread
    // Note: create_process_abi returns Option<u64> (pid), not bool
    assert!(model::create_process_abi(2).is_some());
    // create_thread_abi returns Option<u64> (tid)
    assert!(model::create_thread_abi(2, 2, 10).is_some());

    // Tick 1 keeps the existing running thread on CPU 0.
    let t = model::scheduler_tick().expect("tick 1");
    assert_eq!(t.tid, 1);
    assert_eq!(t.state, 1); // RUNNING encoded

    // Tick 2 still favors thread 1 because the time slice has not expired.
    let t2 = model::scheduler_tick().expect("tick 2");
    assert_eq!(t2.tid, 1);
}
