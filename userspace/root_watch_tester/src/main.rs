#![no_std]
#![no_main]

extern crate alloc;
use stem::println;
use abi::root::*;
use abi::syscall::*;

// Helper to make a simple batch (creates 1 node)
fn make_batch(tag: u8) -> alloc::vec::Vec<u8> {
    let mut batch = alloc::vec::Vec::new();
    batch.extend_from_slice(&BATCH_MAGIC.to_le_bytes());
    batch.extend_from_slice(&BATCH_VERSION.to_le_bytes());
    batch.extend_from_slice(&1u16.to_le_bytes()); // 1 op
    
    // CreateNode
    batch.push(OP_CREATE_NODE);
    let mut kind = [0u8; 16];
    kind[0] = tag; // Use tag to identify batch
    batch.extend_from_slice(&kind);
    batch.extend_from_slice(&0u16.to_le_bytes()); // out ref 0
    
    batch
}

#[stem::main]
fn main() -> ! {
    stem::info!("Root Watch Tester Starting...");
    test_main();
    loop { stem::yield_now(); }
}

fn test_main() -> i32 {
    
    // 1. Open Watch
    // Query spec: empty (match all) or bytespace filter.
    // For v0, kernel usually defaults to "all" if query matches "bytespace" logic or just subscribes?
    // handle_watch_open implementation does filter for "bytespace".
    // Let's rely on internal behavior or just pass empty/basic query.
    // Since we didn't implement robust query parsing in v0 logic in handle_watch_open (it scans for Scan op),
    // we should provide a query that triggers the subscription.
    // However, implementation says "if none, fallback to bytespace". 
    // So passing empty query works? Or invalid query?
    // Let's pass a dummy spec.
    
    // Spec layout: WatchSpec { query_ptr, query_len, mode }
    // We pass raw pointer to syscall.
    
    let spec = abi::types::WatchSpec {
        query_ptr: 0,
        query_len: 0,
        mode: 0, 
        start_seq: 0,
    };
    println!("Spec ptr: {:x}", &spec as *const _ as usize);
    
    let watch_handle = unsafe {
        let res = stem::syscall::syscall6(SYS_ROOT_WATCH_OPEN, &spec as *const _ as usize, 0, 0, 0, 0, 0);
        if (res as isize) < 0 {
             println!("Watch open failed: {}", res);
             return -1;
        }
        res as usize
    };
    
    println!("Watch opened, handle: {}", watch_handle);
    
    // 2. Apply Batch 1
    let b1 = make_batch(0xA1);
    unsafe {
        let res = stem::syscall::syscall6(SYS_ROOT_APPLY_BATCH, b1.as_ptr() as usize, b1.len() as usize, 0, 0, 0, 0);
        if (res as isize) < 0 {
             println!("Batch 1 failed: {}", res);
             return -1;
        }
        println!("Applied Batch 1, seq: {}", res);
    }
    
    // 3. Read Batch 1
    let mut buf = [0u8; 1024];
    let mut seq_out = 0u64;
    unsafe {
        let res = stem::syscall::syscall6(
            SYS_ROOT_WATCH_NEXT,
            watch_handle,
            &mut seq_out as *mut _ as usize,
            buf.as_mut_ptr() as usize,
            buf.len(),
            0, 0
        );
        
        if (res as isize) < 0 {
            println!("Watch read failed: {}", res);
            return -1;
        }
        println!("Received event, len: {}, seq: {}", res, seq_out);
        
        if buf[8] == OP_CREATE_NODE && buf[9] == 0xA1 {
             println!("PASS: Batch content match.");
        } else {
             println!("FAIL: Batch content mismatch.");
        }
    }
    
    // 4. Test Buffer Too Small
    let b2 = make_batch(0xB2); // length ~27 bytes
    unsafe { stem::syscall::syscall6(SYS_ROOT_APPLY_BATCH, b2.as_ptr() as usize, b2.len() as usize, 0, 0, 0, 0); }
    
    let mut tiny_buf = [0u8; 10]; // Too small
    unsafe {
        let res = stem::syscall::syscall6(
            SYS_ROOT_WATCH_NEXT,
            watch_handle,
            &mut seq_out as *mut _ as usize,
            tiny_buf.as_mut_ptr() as usize,
            tiny_buf.len(),
            0, 0
        );
        // Expect -28 (ENOSPC)
        // Note: isize cast. -28.
        // We print it.
        if (res as isize) == -28 {
             println!("PASS: Got ENOSPC for small buffer.");
        } else {
             println!("FAIL: Expected ENOSPC (-28), got {}", res as isize);
        }
    }
    
    // 5. Test Overflow
    // Max pending commits = 256.
    // We already have 1 pending (b2, which wasn't popped because of small buffer).
    // Let's push 300 more.
    println!("Generating overflow...");
    for i in 0..300 {
        let b = make_batch((i % 255) as u8);
        unsafe { stem::syscall::syscall6(SYS_ROOT_APPLY_BATCH, b.as_ptr() as usize, b.len() as usize, 0, 0, 0, 0); }
    }
    
    // Read -> Expect overflow error
    unsafe {
        let res = stem::syscall::syscall6(
            SYS_ROOT_WATCH_NEXT,
            watch_handle,
            &mut seq_out as *mut _ as usize,
            buf.as_mut_ptr() as usize,
            buf.len(),
            0, 0
        );
        
        if (res as isize) == -75 {
             println!("PASS: Got EOVERFLOW.");
        } else {
             println!("FAIL: Expected EOVERFLOW (-75), got {}", res as isize);
        }
        
        // Next read should succeed (recovering)
        let res2 = stem::syscall::syscall6(
            SYS_ROOT_WATCH_NEXT,
            watch_handle,
            &mut seq_out as *mut _ as usize,
            buf.as_mut_ptr() as usize,
            buf.len(),
            0, 0
        );
        if (res2 as isize) > 0 {
             println!("PASS: Recovered after overflow, got seq: {}", seq_out);
        } else {
             println!("FAIL: Did not recover, got {}", res2 as isize);
        }
    }

    0
}
