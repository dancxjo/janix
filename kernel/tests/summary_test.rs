extern crate alloc;
use abi::{KernelRequest, KernelResponse, PropValue};
use kernel::model;
use alloc::string::String;
use alloc::vec::Vec;

#[test]
#[ignore]
fn test_memory_summary() {
    let _guard = kernel::test_lock();
    kernel::init();

    // Create some PhysFrames
    let _ = model::create_phys_frame(0x1000, 4096);
    let _ = model::create_phys_frame(0x2000, 4096);

    // Create a FramePool
    let _ = model::create_frame_pool(0x1000, 0x3000, 4096);

    let response = kernel::handle_request(KernelRequest::GetMemorySummary);

    if let KernelResponse::MemorySummary { summary } = response {
        assert!(
            summary.total_frames >= 2,
            "Expected at least 2 total frames, got {}",
            summary.total_frames
        );
        // By default allocated is false
        assert!(
            summary.free_frames >= 2,
            "Expected at least 2 free frames, got {}",
            summary.free_frames
        );
        assert_eq!(
            summary.used_frames, 0,
            "Expected 0 used frames, got {}",
            summary.used_frames
        );
    } else {
        panic!("Expected MemorySummary response, got {:?}", response);
    }
}

#[test]
#[ignore]
fn test_scheduler_summary() {
    let _guard = kernel::test_lock();
    kernel::init();

    // Create a Process
    let kind_process = kernel::symbols::intern("Process");
    let props = [(kernel::symbols::intern("pid"), PropValue::U64(1))];
    kernel::graph::create_thing(kind_process, props.to_vec());

    // Create a Thread
    let kind_thread = kernel::symbols::intern("Thread");
    let props = [
        (kernel::symbols::intern("tid"), PropValue::U64(1)),
        (kernel::symbols::intern("state"), PropValue::Str(String::from("Running"))),
        (kernel::symbols::intern("priority"), PropValue::U64(1)),
        (kernel::symbols::intern("runtime_ns"), PropValue::U64(0)),
        (kernel::symbols::intern("last_started_ns"), PropValue::U64(0)),
    ];
    kernel::graph::create_thing(kind_thread, props.to_vec());

    let response = kernel::handle_request(KernelRequest::GetSchedulerSummary);

    if let KernelResponse::SchedulerSummary { summary } = response {
        assert!(
            summary.process_count >= 1,
            "Expected at least 1 process, got {}",
            summary.process_count
        );
        assert!(
            summary.thread_count >= 1,
            "Expected at least 1 thread, got {}",
            summary.thread_count
        );
        assert!(
            summary.runnable_threads >= 1,
            "Expected at least 1 runnable thread, got {}",
            summary.runnable_threads
        );
    } else {
        panic!("Expected SchedulerSummary response, got {:?}", response);
    }
}
