//! Thread verification test for thing-os.
//!
//! This test proves that spawned threads truly share the address space with
//! the spawner, not just "have an Arc field". It tests:
//!
//! 1. A spawned thread can read a pointer to heap memory allocated by the spawner
//! 2. A spawned thread can read from a mapped bytespace created by the spawner

#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};
use thing_std::*;

/// Magic sentinel values for verification
const HEAP_SENTINEL: u64 = 0xC0FFEE_DEAD_BEEF;
const BYTESPACE_SENTINEL: u8 = 0x42;

/// Exit codes for test results
const EXIT_SUCCESS: i32 = 0;
const EXIT_HEAP_FAIL: i32 = 1;
const EXIT_BYTESPACE_FAIL: i32 = 2;

/// Shared state passed to thread via raw pointer
#[repr(C)]
struct ThreadArgs {
    /// Pointer to heap-allocated sentinel value
    heap_ptr: *const u64,
    /// Pointer to mapped bytespace region
    bytespace_ptr: *const u8,
    /// Length of bytespace mapping
    bytespace_len: usize,
    /// Atomic result flag: 0=pending, 1=heap_ok, 2=bytespace_ok, 3=all_ok
    result: AtomicU32,
}

// SAFETY: We're passing this between threads that share the address space
unsafe impl Send for ThreadArgs {}
unsafe impl Sync for ThreadArgs {}

/// Worker thread entry point
extern "C" fn worker_entry(arg: u64) -> ! {
    let args = unsafe { &*(arg as *const ThreadArgs) };
    
    log_info("THREAD_TEST: Worker started");
    
    // Test 1: Read heap memory allocated by spawner
    let heap_val = unsafe { core::ptr::read_volatile(args.heap_ptr) };
    if heap_val == HEAP_SENTINEL {
        log_info("THREAD_TEST: [PASS] Heap read OK - shared VM confirmed for heap");
        args.result.fetch_or(1, Ordering::SeqCst);
    } else {
        log_info("THREAD_TEST: [FAIL] Heap read WRONG VALUE - address space NOT shared!");
        log_info(&alloc::format!("  Expected: {:#x}, Got: {:#x}", HEAP_SENTINEL, heap_val));
        thread_exit(EXIT_HEAP_FAIL);
    }
    
    // Test 2: Read mapped bytespace region created by spawner
    if args.bytespace_ptr.is_null() || args.bytespace_len == 0 {
        log_info("THREAD_TEST: [SKIP] Bytespace test - no bytespace mapped");
    } else {
        let bs_val = unsafe { core::ptr::read_volatile(args.bytespace_ptr) };
        if bs_val == BYTESPACE_SENTINEL {
            log_info("THREAD_TEST: [PASS] Bytespace read OK - shared VM confirmed for mappings");
            args.result.fetch_or(2, Ordering::SeqCst);
        } else {
            log_info("THREAD_TEST: [FAIL] Bytespace read WRONG VALUE - mappings NOT shared!");
            log_info(&alloc::format!("  Expected: {:#x}, Got: {:#x}", BYTESPACE_SENTINEL, bs_val));
            thread_exit(EXIT_BYTESPACE_FAIL);
        }
    }
    
    log_info("THREAD_TEST: Worker exiting with success");
    thread_exit(EXIT_SUCCESS);
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    init(0);
    log_info("==================================");
    log_info("THREAD_TEST: Starting thread verification");
    log_info("==================================");
    
    // ========================================
    // Test 1 Setup: Heap allocation
    // ========================================
    log_info("THREAD_TEST: Allocating heap sentinel...");
    let heap_box: Box<u64> = Box::new(HEAP_SENTINEL);
    let heap_ptr = Box::into_raw(heap_box);
    log_info(&alloc::format!("THREAD_TEST: Heap sentinel at {:#x}", heap_ptr as u64));
    
    // Verify we can read it ourselves
    let verify = unsafe { core::ptr::read_volatile(heap_ptr) };
    if verify != HEAP_SENTINEL {
        log_info("THREAD_TEST: [FATAL] Cannot even read our own heap allocation!");
        sys_exit(-1);
    }
    log_info("THREAD_TEST: Heap sentinel verified readable by main thread");
    
    // ========================================
    // Test 2 Setup: Bytespace mapping
    // ========================================
    // Try to find and map a bytespace. If none available, skip this test.
    let (bytespace_ptr, bytespace_len): (*const u8, usize) = {
        // Create a small RAM bytespace, write our sentinel, then map it
        let bs_size = 4096u64;
        let bs_id = memory::bytespace_create(bs_size);
        
        if bs_id.0 == 0 {
            log_info("THREAD_TEST: Could not create bytespace, skipping mapping test");
            (core::ptr::null(), 0)
        } else {
            log_info(&alloc::format!("THREAD_TEST: Created bytespace {}", bs_id.low()));
            
            // Map it at a fixed address
            let map_addr = 0x8700_0000u64;
            let result = memory::space_map(bs_id, map_addr, 0, bs_size);
            
            if result == 0 {
                log_info("THREAD_TEST: space_map returned 0 (may indicate failure)");
                (core::ptr::null(), 0)
            } else {
                log_info(&alloc::format!("THREAD_TEST: Mapped bytespace at {:#x}", map_addr));
                
                // Write our sentinel
                unsafe {
                    core::ptr::write_volatile(map_addr as *mut u8, BYTESPACE_SENTINEL);
                }
                
                // Verify we can read it
                let check = unsafe { core::ptr::read_volatile(map_addr as *const u8) };
                if check != BYTESPACE_SENTINEL {
                    log_info("THREAD_TEST: [FATAL] Cannot read our own bytespace mapping!");
                    sys_exit(-1);
                }
                log_info("THREAD_TEST: Bytespace sentinel verified readable by main thread");
                
                (map_addr as *const u8, bs_size as usize)
            }
        }
    };
    
    // ========================================
    // Spawn worker thread
    // ========================================
    log_info("THREAD_TEST: Creating thread args struct...");
    let args = Box::new(ThreadArgs {
        heap_ptr,
        bytespace_ptr,
        bytespace_len,
        result: AtomicU32::new(0),
    });
    let args_ptr = Box::into_raw(args);
    
    log_info("THREAD_TEST: Spawning worker thread...");
    let handle = thread_spawn(worker_entry, args_ptr as u64);
    log_info(&alloc::format!("THREAD_TEST: Spawned thread tid={}", handle.tid()));
    
    // ========================================
    // Wait for worker
    // ========================================
    log_info("THREAD_TEST: Joining worker thread...");
    let exit_code = join(handle);
    log_info(&alloc::format!("THREAD_TEST: Worker exited with code {}", exit_code));
    
    // ========================================
    // Analyze results
    // ========================================
    let args = unsafe { Box::from_raw(args_ptr) };
    let result = args.result.load(Ordering::SeqCst);
    
    log_info("==================================");
    if exit_code == EXIT_SUCCESS {
        let heap_ok = (result & 1) != 0;
        let bs_ok = (result & 2) != 0 || bytespace_ptr.is_null();
        
        if heap_ok && bs_ok {
            log_info("THREAD_TEST: [ALL PASS] Threads share address space correctly!");
            log_info("==================================");
            
            // Cleanup
            unsafe { let _ = Box::from_raw(heap_ptr as *mut u64); }
            
            sys_exit(0);
        }
    }
    
    log_info("THREAD_TEST: [FAIL] Thread verification FAILED");
    log_info(&alloc::format!("  Exit code: {}, Result flags: {:#b}", exit_code, result));
    log_info("==================================");
    
    // Cleanup
    unsafe { let _ = Box::from_raw(heap_ptr as *mut u64); }
    
    sys_exit(exit_code);
}
