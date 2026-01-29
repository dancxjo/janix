#![no_std]
#![no_main]

extern crate alloc;
use abi::root::*;
use abi::syscall::*;
use alloc::vec::Vec;
use stem::println;

// Configurable benchmark parameters (compile-time)
const OPS_PER_BATCH: usize = 200; // CreateNodes + PutEdges
const ITERATIONS: usize = 100; // Number of batch submissions
const NODES_PER_BATCH: usize = 100; // CreateNode ops per batch

#[stem::main]
fn main() -> ! {
    stem::info!("Root Batch Bench Starting...");
    stem::info!(
        "Config: {} ops/batch, {} iterations",
        OPS_PER_BATCH,
        ITERATIONS
    );

    run_benchmark();

    stem::info!("Benchmark complete. Halting.");
    loop {
        stem::yield_now();
    }
}

fn run_benchmark() {
    // Build a reusable batch payload
    let batch = build_batch(NODES_PER_BATCH);
    let batch_ops = NODES_PER_BATCH * 2; // CREATE_NODE + PUT_EDGE for each

    println!("Batch size: {} bytes, {} ops", batch.len(), batch_ops);

    // Warm-up: run once to trigger any lazy initialization/allocation
    let _ = submit_batch(&batch);

    // Timed benchmark run
    let start = stem::time::monotonic_ns();
    let mut success_count = 0u64;
    let mut error_count = 0u64;

    for _ in 0..ITERATIONS {
        match submit_batch(&batch) {
            Ok(_seq) => success_count += 1,
            Err(errno) => {
                error_count += 1;
                if error_count <= 5 {
                    println!("ApplyBatch error: {}", errno);
                }
            }
        }
    }

    let end = stem::time::monotonic_ns();
    let elapsed_ns = end.saturating_sub(start);

    // Compute metrics
    let total_ops = (batch_ops as u64) * (success_count as u64);
    let total_commits = success_count;

    println!("=== Benchmark Results ===");
    println!(
        "Iterations: {} success, {} errors",
        success_count, error_count
    );
    println!("Elapsed: {} ns ({} ms)", elapsed_ns, elapsed_ns / 1_000_000);

    if elapsed_ns > 0 {
        let ops_per_sec = total_ops.saturating_mul(1_000_000_000) / elapsed_ns;
        let commits_per_sec = total_commits.saturating_mul(1_000_000_000) / elapsed_ns;
        let ns_per_batch = elapsed_ns / (success_count.max(1));

        println!("Throughput: {} ops/sec", ops_per_sec);
        println!("Commits: {} commits/sec", commits_per_sec);
        println!(
            "Latency: {} ns/batch ({} us)",
            ns_per_batch,
            ns_per_batch / 1000
        );
    }
}

fn build_batch(node_count: usize) -> Vec<u8> {
    let kind_id = [0xAA; 16]; // Fixed kind hash
    let rel_id = [0xBB; 16]; // Fixed predicate hash

    // Total ops = node_count (CREATE_NODE) + node_count (PUT_EDGE)
    let op_count = (node_count * 2) as u16;

    let mut batch = Vec::with_capacity(8 + node_count * 50);

    // Header: magic (4) + version (2) + op_count (2)
    batch.extend_from_slice(&BATCH_MAGIC.to_le_bytes());
    batch.extend_from_slice(&BATCH_VERSION.to_le_bytes());
    batch.extend_from_slice(&op_count.to_le_bytes());

    // CREATE_NODE ops: tag (1) + kind (16) + out_idx (2) = 19 bytes each
    for i in 0..node_count {
        batch.push(OP_CREATE_NODE);
        batch.extend_from_slice(&kind_id);
        let out_idx = i as u16;
        batch.extend_from_slice(&out_idx.to_le_bytes());
    }

    // PUT_EDGE ops: tag + src_ref + predicate + dst_ref
    // Local ref: tag (1) + idx (2) = 3 bytes
    // Total: 1 + 3 + 16 + 3 = 23 bytes each
    for i in 0..node_count {
        batch.push(OP_PUT_EDGE);

        // Subject: local ref to node i
        batch.push(REF_LOCAL);
        batch.extend_from_slice(&(i as u16).to_le_bytes());

        // Predicate
        batch.extend_from_slice(&rel_id);

        // Object: local ref to node (i+1) % node_count
        batch.push(REF_LOCAL);
        let next = ((i + 1) % node_count) as u16;
        batch.extend_from_slice(&next.to_le_bytes());
    }

    batch
}

fn submit_batch(batch: &[u8]) -> Result<u64, i32> {
    let result = unsafe {
        stem::syscall::syscall6(
            SYS_ROOT_APPLY_BATCH,
            batch.as_ptr() as usize,
            batch.len(),
            0,
            0,
            0,
            0,
        )
    };

    let signed = result as i64;
    if signed < 0 {
        Err(signed as i32)
    } else {
        Ok(result as u64) // Returns seq number
    }
}
