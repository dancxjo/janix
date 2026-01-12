//! Stack torture tests

use stem::info;

const FRAME_BYTES: usize = 4096;

/// Test deep recursion with sentinel verification on unwind
pub fn stack_recursion(target_depth: usize) {
    info!("[torture] Running stack_recursion target_depth={}", target_depth);
    recurse(0, target_depth);
    info!("PASS: stack_recursion");
}

fn recurse(depth: usize, max: usize) {
    // Consume stack space with a large local buffer
    let mut buf = [0u8; FRAME_BYTES];
    
    // Write sentinel value
    unsafe {
        core::ptr::write_volatile(buf.as_mut_ptr(), depth as u8);
        core::ptr::write_volatile(buf.as_mut_ptr().add(FRAME_BYTES - 1), (depth ^ 0xFF) as u8);
    }
    
    if depth % 64 == 0 {
        stem::info!("[torture] recursion depth={}", depth);
    }
    
    if depth < max {
        recurse(depth + 1, max);
    }
    
    // Verify sentinels on unwind - detects stack corruption during recursion
    let sentinel_low = unsafe { core::ptr::read_volatile(buf.as_ptr()) };
    let sentinel_high = unsafe { core::ptr::read_volatile(buf.as_ptr().add(FRAME_BYTES - 1)) };
    
    if sentinel_low != depth as u8 {
        stem::error!("FAIL: stack_recursion sentinel_low mismatch at depth {} (expected {}, got {})", 
            depth, depth as u8, sentinel_low);
    }
    if sentinel_high != (depth ^ 0xFF) as u8 {
        stem::error!("FAIL: stack_recursion sentinel_high mismatch at depth {}", depth);
    }
}

/// Test stack stability across many context switches
pub fn stack_context_stress() {
    info!("[torture] Running stack_context_stress");
    
    // Pattern that should survive context switches
    let mut local_pattern: [u64; 128] = [0; 128];
    for (i, slot) in local_pattern.iter_mut().enumerate() {
        *slot = 0xCAFE_0000 | (i as u64);
    }
    
    // Yield many times to trigger context switches
    for iteration in 0..100 {
        stem::yield_now();
        
        // Verify pattern after each yield
        for (i, slot) in local_pattern.iter().enumerate() {
            let expected = 0xCAFE_0000 | (i as u64);
            if *slot != expected {
                stem::error!("FAIL: stack_context_stress corruption at iteration {} slot {} (expected {:x}, got {:x})",
                    iteration, i, expected, *slot);
                return;
            }
        }
        
        if iteration % 25 == 0 {
            stem::info!("[torture] context_stress iteration={}", iteration);
        }
    }
    
    info!("PASS: stack_context_stress");
}

/// Intentional stack overflow to test guard page behavior
/// This should trigger a controlled fault, not a triple fault
#[allow(dead_code)]
pub fn stack_overflow() {
    info!("[torture] Running stack_overflow (expect guard fault)");
    
    fn infinite_recurse(depth: usize) {
        let mut buf = [0u8; 4096];
        unsafe { core::ptr::write_volatile(buf.as_mut_ptr(), depth as u8); }
        if depth % 64 == 0 {
            stem::info!("[torture] overflow depth={}", depth);
        }
        infinite_recurse(depth + 1);
        // Prevent tail-call optimization
        unsafe { core::ptr::read_volatile(buf.as_ptr()); }
    }
    
    infinite_recurse(0);
    // Should never reach here
    stem::error!("FAIL: stack_overflow did not trigger guard page");
}
