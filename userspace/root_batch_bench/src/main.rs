#![no_std]
#![no_main]

extern crate alloc;
// use stem::prelude::*;
use abi::root::*;
use abi::syscall::*;
use stem::println;

#[stem::main]
fn main() -> ! {
    stem::info!("Root Batch Bench Starting...");
    let mut graph = [0u8; 128]; // dummy buffer
    // ... logic ...
    
    // Instead of return 0, we must exit.
    // stem::process::exit(0); // If exists
    // For now, loop forever to satisfy ! if we don't have exit.
    // But benchmarks should print results then exit.
    // Check stem source? I'll assume stem::process::exit exists or I loop.
    test_main();
    loop { stem::yield_now(); }
}

fn test_main() -> i32 {    
    // 1. Intern some symbols to use
    // For benchmarks we can use fixed bytes since we manipulate raw batch
    let kind_id = [0xAA; 16]; // Fake SymbolId
    let rel_id = [0xBB; 16]; 

    // 2. Build a batch
    // Header + (CreateNode x 100) + (PutEdge x 99)
    // CreateNode: 1(Tag) + 16(Kind) + 2(OutRef) = 19 bytes
    // PutEdge: 1(Tag) + Ref(3) + Rel(16) + Ref(3) = 23 bytes (Using Local Refs)
    // Ref Local: 1(Tag=1) + 2(Idx) = 3 bytes
    
    let mut batch = alloc::vec::Vec::with_capacity(1024 * 64);
    
    // Header
    let magic = BATCH_MAGIC;
    let version = BATCH_VERSION;
    let ops = 200u16; 
    
    batch.extend_from_slice(&magic.to_le_bytes());
    batch.extend_from_slice(&version.to_le_bytes());
    batch.extend_from_slice(&ops.to_le_bytes());
    
    // Add 100 CreateNode ops
    for i in 0..100 {
        batch.push(OP_CREATE_NODE);
        batch.extend_from_slice(&kind_id);
        let out_idx = i as u16;
        batch.extend_from_slice(&out_idx.to_le_bytes());
    }
    
    // Add 100 PutEdge ops (Chain them: 0->1, 1->2 ...)
    for i in 0..100 {
        batch.push(OP_PUT_EDGE);
        
        // Subject: Local Ref i
        batch.push(REF_LOCAL);
        batch.extend_from_slice(&(i as u16).to_le_bytes());
        
        // Pred
        batch.extend_from_slice(&rel_id);
        
        // Object: Local Ref (i+1) % 100
        batch.push(REF_LOCAL);
        let next = ((i + 1) % 100) as u16;
        batch.extend_from_slice(&next.to_le_bytes()); 
    }
    
    println!("Batch size: {} bytes", batch.len());
    
    // 3. Run Benchmark
    let start = stem::time::monotonic_ns();
    let loops = 100;
    
    for _ in 0..loops {
        unsafe {
             let res = stem::syscall::syscall6(
                 SYS_ROOT_APPLY_BATCH,
                 batch.as_ptr() as usize,
                 batch.len() as usize,
                 0, 0, 0, 0
             );
             if (res as isize) < 0 {
                 println!("Syscall failed: {}", res);
                 return -1;
             }
        }
    }
    
    let end = stem::time::monotonic_ns();
    let elapsed_ns = end - start;
    let total_ops = ops as u64 * loops as u64;
    
    println!("Applied {} ops in {} ns", total_ops, elapsed_ns);
    let ops_per_sec = total_ops * 1_000_000_000 / elapsed_ns;
    println!("Throughput: {} ops/sec", ops_per_sec);

    0
}
