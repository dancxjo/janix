use abi::{KernelRequest, KernelResponse, PropValue};
use kernel_core::model;

#[test]
fn test_memory_summary() {
    kernel_core::init();
    
    // Create some PhysFrames
    let _ = model::create_phys_frame(0x1000, 4096);
    let _ = model::create_phys_frame(0x2000, 4096);
    
    // Create a FramePool
    let _ = model::create_frame_pool(0x1000, 0x3000, 4096);
    
    let response = kernel_core::handle_request(KernelRequest::GetMemorySummary);
    
    if let KernelResponse::MemorySummary { summary } = response {
        assert!(summary.total_frames >= 2, "Expected at least 2 total frames, got {}", summary.total_frames);
        // By default allocated is false
        assert!(summary.free_frames >= 2, "Expected at least 2 free frames, got {}", summary.free_frames);
        assert_eq!(summary.used_frames, 0, "Expected 0 used frames, got {}", summary.used_frames);
    } else {
        panic!("Expected MemorySummary response, got {:?}", response);
    }
}

#[test]
fn test_scheduler_summary() {
    kernel_core::init();
    
    // Create a Process
    let props = &[
        ("pid", PropValue::U64(1)),
        ("state", PropValue::U64(model::STATE_READY)),
    ];
    kernel_core::graph::create_thing("Process", props);
    
    // Create a Thread
    let props = &[
        ("tid", PropValue::U64(1)),
        ("state", PropValue::U64(model::STATE_RUNNING)),
        ("priority", PropValue::U64(1)),
        ("runtime_ns", PropValue::U64(0)),
    ];
    kernel_core::graph::create_thing("Thread", props);
    
    let response = kernel_core::handle_request(KernelRequest::GetSchedulerSummary);
    
    if let KernelResponse::SchedulerSummary { summary } = response {
        assert!(summary.process_count >= 1, "Expected at least 1 process, got {}", summary.process_count);
        assert!(summary.thread_count >= 1, "Expected at least 1 thread, got {}", summary.thread_count);
        assert!(summary.runnable_threads >= 1, "Expected at least 1 runnable thread, got {}", summary.runnable_threads);
    } else {
        panic!("Expected SchedulerSummary response, got {:?}", response);
    }
}
