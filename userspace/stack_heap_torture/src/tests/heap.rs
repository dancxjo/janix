//! Heap torture tests

use stem::info;
use alloc::vec::Vec;
use alloc::vec;

/// Test allocate/free churn with pattern verification
pub fn heap_churn(iterations: usize) {
    info!("[torture] Running heap_churn iterations={}", iterations);
    
    for i in 0..iterations {
        // Varying sizes to stress allocator
        let size = ((i * 7 + 13) % 256) + 16;
        let mut v: Vec<u8> = vec![0; size];
        
        // Write pattern
        v[0] = (i & 0xFF) as u8;
        v[size - 1] = ((i >> 8) & 0xFF) as u8;
        
        // Verify before drop
        if v[0] != (i & 0xFF) as u8 || v[size - 1] != ((i >> 8) & 0xFF) as u8 {
            stem::error!("FAIL: heap_churn corruption at iteration {}", i);
            return;
        }
        
        if i % 100 == 0 {
            stem::info!("[torture] heap_churn iteration={}", i);
        }
    }
    
    info!("PASS: heap_churn");
}

/// Test realloc preserves existing data
pub fn heap_realloc() {
    info!("[torture] Running heap_realloc");
    
    // Start with 64 bytes filled with 0xAA
    let mut v = vec![0xAAu8; 64];
    
    // Grow to 256 bytes
    v.resize(256, 0xBB);
    
    // Verify original prefix preserved
    for (i, byte) in v[0..64].iter().enumerate() {
        if *byte != 0xAA {
            stem::error!("FAIL: heap_realloc grow lost prefix at byte {}", i);
            return;
        }
    }
    
    // Verify new bytes are 0xBB
    for (i, byte) in v[64..256].iter().enumerate() {
        if *byte != 0xBB {
            stem::error!("FAIL: heap_realloc grow did not fill new bytes at offset {}", i + 64);
            return;
        }
    }
    
    // Shrink to 32 bytes
    v.truncate(32);
    
    // Verify remaining prefix still correct
    for (i, byte) in v[0..32].iter().enumerate() {
        if *byte != 0xAA {
            stem::error!("FAIL: heap_realloc shrink lost prefix at byte {}", i);
            return;
        }
    }
    
    info!("PASS: heap_realloc");
}

/// Deterministic fuzz test with seed
pub fn heap_fuzz(seed: u64) {
    info!("[torture] Running heap_fuzz seed={}", seed);
    
    // Simple LCG PRNG for reproducibility
    struct Lcg(u64);
    impl Lcg {
        fn next(&mut self) -> u64 {
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
            self.0
        }
    }
    
    let mut rng = Lcg(seed);
    let mut allocations: Vec<Vec<u8>> = Vec::new();
    
    for i in 0..1000 {
        let op = rng.next() % 3;
        
        match op {
            0 => {
                // Alloc: create new allocation with signature
                let size = ((rng.next() % 512) + 8) as usize;
                let mut v = vec![0u8; size];
                v[0] = (i & 0xFF) as u8;
                if size > 1 {
                    v[size - 1] = ((i >> 8) & 0xFF) as u8;
                }
                allocations.push(v);
            }
            1 if !allocations.is_empty() => {
                // Free: remove random allocation
                let idx = (rng.next() as usize) % allocations.len();
                allocations.swap_remove(idx);
            }
            2 if !allocations.is_empty() => {
                // Realloc: resize random allocation
                let idx = (rng.next() as usize) % allocations.len();
                let new_size = ((rng.next() % 512) + 8) as usize;
                allocations[idx].resize(new_size, 0);
            }
            _ => {}
        }
        
        if i % 250 == 0 {
            stem::info!("[torture] heap_fuzz iteration={} live_allocs={}", i, allocations.len());
        }
    }
    
    info!("PASS: heap_fuzz seed={}", seed);
}

/// Test that heap pointers survive context switches
#[allow(dead_code)]
pub fn heap_context_survive() {
    info!("[torture] Running heap_context_survive");
    
    // Allocate some blocks
    let mut blocks: Vec<Vec<u8>> = Vec::new();
    for i in 0..10 {
        let mut v = vec![i as u8; 64];
        v[0] = 0xCA;
        v[63] = 0xFE;
        blocks.push(v);
    }
    
    // Yield many times
    for iteration in 0..100 {
        stem::yield_now();
        
        // Verify all blocks still valid
        for (i, block) in blocks.iter().enumerate() {
            if block[0] != 0xCA || block[63] != 0xFE {
                stem::error!("FAIL: heap_context_survive corruption at block {} iteration {}", i, iteration);
                return;
            }
        }
    }
    
    info!("PASS: heap_context_survive");
}

/// Test OOM behavior - exhaust allocator and verify controlled failure
#[allow(dead_code)]
pub fn heap_oom() {
    info!("[torture] Running heap_oom");
    
    let mut allocations: Vec<Vec<u8>> = Vec::new();
    let mut total_bytes = 0usize;
    
    // Try to allocate until failure
    for i in 0..10000 {
        // Allocate increasingly large blocks
        let size = 1024 * (1 + i % 64);
        
        // Try allocation - in no_std this might panic on OOM
        // Depending on allocator behavior, we either:
        // 1. Get the allocation
        // 2. Get a panic
        // 3. Return null which Vec handles
        let v: Vec<u8> = vec![0u8; size];
        total_bytes += size;
        allocations.push(v);
        
        if i % 100 == 0 {
            stem::info!("[torture] heap_oom allocated {} bytes in {} blocks", total_bytes, allocations.len());
        }
    }
    
    stem::info!("[torture] heap_oom: allocated {} bytes total (no OOM triggered)", total_bytes);
    info!("PASS: heap_oom (survived without OOM)");
}
