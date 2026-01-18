#![no_std]
#![no_main]

//! Single-Op Syscall Watch Tester
//!
//! Tests that single-op syscalls (CREATE_NODE, LINK, PROP_SET) properly
//! deliver watch events through the unified batch pipeline.

extern crate alloc;
use stem::println;
use abi::root::*;
use abi::syscall::*;

/// Helper to intern a symbol and get its ID
fn intern_symbol(name: &str) -> u32 {
    unsafe {
        stem::syscall::syscall6(
            SYS_ROOT_INTERN,
            name.as_ptr() as usize,
            name.len(),
            0, 0, 0, 0
        ) as u32
    }
}

/// Helper to create a node using single-op syscall
fn create_node(kind_name: &str) -> u64 {
    let kind_sym = make_symbol_ref(kind_name);
    unsafe {
        stem::syscall::syscall6(
            SYS_ROOT_CREATE_NODE,
            &kind_sym as *const _ as usize,
            0, 0, 0, 0, 0
        ) as u64
    }
}

/// Helper to create an edge using single-op syscall
fn link_nodes(src: u64, rel_name: &str, dst: u64) -> i64 {
    let rel_sym = make_symbol_ref(rel_name);
    unsafe {
        stem::syscall::syscall6(
            SYS_ROOT_LINK,
            src as usize,
            &rel_sym as *const _ as usize,
            dst as usize,
            0, 0, 0
        ) as i64
    }
}

/// Helper to set a property using single-op syscall
fn prop_set(id: u64, key_name: &str, value: u64) -> i64 {
    let key_sym = make_symbol_ref(key_name);
    unsafe {
        stem::syscall::syscall6(
            SYS_ROOT_PROP_SET,
            id as usize,
            &key_sym as *const _ as usize,
            value as usize,
            0, 0, 0
        ) as i64
    }
}

/// Create a SymbolRefWire for a string
fn make_symbol_ref(name: &str) -> abi::symbols::SymbolRefWire {
    abi::symbols::SymbolRefWire {
        tag: abi::symbols::SYMBOL_REF_TAG_STR,
        ptr_or_id: name.as_ptr() as u64,
        len: name.len() as u64,
    }
}

/// Open a watch subscription
fn open_watch() -> Result<usize, i64> {
    let spec = abi::types::WatchSpec {
        query_ptr: 0,
        query_len: 0,
        mode: 0,
        start_seq: 0,
    };
    
    let res = unsafe {
        stem::syscall::syscall6(
            SYS_ROOT_WATCH_OPEN,
            &spec as *const _ as usize,
            0, 0, 0, 0, 0
        )
    };
    
    if (res as isize) < 0 {
        Err(res as i64)
    } else {
        Ok(res as usize)
    }
}

/// Read next watch event
fn watch_next(handle: usize, buf: &mut [u8]) -> Result<(usize, u64), i64> {
    let mut seq_out = 0u64;
    
    let res = unsafe {
        stem::syscall::syscall6(
            SYS_ROOT_WATCH_NEXT,
            handle,
            &mut seq_out as *mut _ as usize,
            buf.as_mut_ptr() as usize,
            buf.len(),
            0, 0
        )
    };
    
    if (res as isize) < 0 {
        Err(res as i64)
    } else {
        Ok((res as usize, seq_out))
    }
}

/// Validate batch header
fn validate_header(buf: &[u8]) -> bool {
    if buf.len() < 8 {
        return false;
    }
    let magic = u32::from_le_bytes(buf[0..4].try_into().unwrap());
    let version = u16::from_le_bytes(buf[4..6].try_into().unwrap());
    let op_count = u16::from_le_bytes(buf[6..8].try_into().unwrap());
    
    magic == BATCH_MAGIC && version == BATCH_VERSION && op_count == 1
}

#[stem::main]
fn main() -> ! {
    stem::info!("Single-Op Watch Tester Starting...");
    let result = test_main();
    if result == 0 {
        println!("=== ALL TESTS PASSED ===");
    } else {
        println!("=== TESTS FAILED (code: {}) ===", result);
    }
    loop { stem::yield_now(); }
}

fn test_main() -> i32 {
    let mut failures = 0;
    
    // 1. Open watch BEFORE making mutations
    let watch_handle = match open_watch() {
        Ok(h) => h,
        Err(e) => {
            println!("FAIL: Could not open watch: {}", e);
            return -1;
        }
    };
    println!("Watch opened: handle={}", watch_handle);
    
    // ========================================
    // Test 1: CREATE_NODE delivers watch event
    // ========================================
    println!("\n--- Test 1: CREATE_NODE watch delivery ---");
    let initial_seq = {
        // Create a node using single-op syscall
        let node_id = create_node("test.node.single");
        println!("Created node via single-op: id={}", node_id);
        
        // Read watch event
        let mut buf = [0u8; 256];
        match watch_next(watch_handle, &mut buf) {
            Ok((len, seq)) => {
                println!("Received watch event: len={}, seq={}", len, seq);
                
                // Validate header
                if validate_header(&buf) {
                    println!("PASS: Valid THRT header with op_count=1");
                } else {
                    println!("FAIL: Invalid batch header");
                    failures += 1;
                }
                
                // Validate op tag
                if len > 8 && buf[8] == OP_CREATE_NODE {
                    println!("PASS: Op tag is CREATE_NODE");
                } else {
                    println!("FAIL: Expected CREATE_NODE tag (0x01), got {:02x}", buf.get(8).unwrap_or(&0));
                    failures += 1;
                }
                
                seq
            }
            Err(e) => {
                println!("FAIL: watch_next failed: {}", e);
                failures += 1;
                0
            }
        }
    };
    
    // ========================================
    // Test 2: LINK delivers watch event
    // ========================================
    println!("\n--- Test 2: LINK watch delivery ---");
    {
        // Create two nodes to link
        let src = create_node("test.src");
        let dst = create_node("test.dst");
        
        // Drain the two CREATE_NODE events (they'll be in the queue)
        let mut buf = [0u8; 256];
        let _ = watch_next(watch_handle, &mut buf);
        let _ = watch_next(watch_handle, &mut buf);
        
        // Now link them
        let res = link_nodes(src, "connected_to", dst);
        if res != 0 {
            println!("FAIL: link_nodes returned error: {}", res);
            failures += 1;
        } else {
            // Read watch event
            match watch_next(watch_handle, &mut buf) {
                Ok((len, seq)) => {
                    println!("Received watch event: len={}, seq={}", len, seq);
                    
                    // Validate header
                    if validate_header(&buf) {
                        println!("PASS: Valid THRT header with op_count=1");
                    } else {
                        println!("FAIL: Invalid batch header");
                        failures += 1;
                    }
                    
                    // Validate op tag
                    if len > 8 && buf[8] == OP_PUT_EDGE {
                        println!("PASS: Op tag is PUT_EDGE");
                    } else {
                        println!("FAIL: Expected PUT_EDGE tag (0x02), got {:02x}", buf.get(8).unwrap_or(&0));
                        failures += 1;
                    }
                    
                    // Seq should be increasing
                    if seq > initial_seq {
                        println!("PASS: Sequence number increased");
                    } else {
                        println!("FAIL: Sequence number did not increase");
                        failures += 1;
                    }
                }
                Err(e) => {
                    println!("FAIL: watch_next failed: {}", e);
                    failures += 1;
                }
            }
        }
    }
    
    // ========================================
    // Test 3: PROP_SET delivers watch event
    // ========================================
    println!("\n--- Test 3: PROP_SET watch delivery ---");
    {
        // Create a node
        let node = create_node("test.props");
        
        // Drain CREATE_NODE event
        let mut buf = [0u8; 256];
        let _ = watch_next(watch_handle, &mut buf);
        
        // Set a property
        let res = prop_set(node, "counter", 42);
        if res != 0 {
            println!("FAIL: prop_set returned error: {}", res);
            failures += 1;
        } else {
            // Read watch event
            match watch_next(watch_handle, &mut buf) {
                Ok((len, seq)) => {
                    println!("Received watch event: len={}, seq={}", len, seq);
                    
                    // Validate header
                    if validate_header(&buf) {
                        println!("PASS: Valid THRT header with op_count=1");
                    } else {
                        println!("FAIL: Invalid batch header");
                        failures += 1;
                    }
                    
                    // Validate op tag
                    if len > 8 && buf[8] == OP_SET_PROP {
                        println!("PASS: Op tag is SET_PROP");
                    } else {
                        println!("FAIL: Expected SET_PROP tag (0x03), got {:02x}", buf.get(8).unwrap_or(&0));
                        failures += 1;
                    }
                }
                Err(e) => {
                    println!("FAIL: watch_next failed: {}", e);
                    failures += 1;
                }
            }
        }
    }
    
    // ========================================
    // Test 4: Consistency with APPLY_BATCH
    // ========================================
    println!("\n--- Test 4: Consistency with APPLY_BATCH ---");
    {
        // Make batch with same structure
        let mut batch = alloc::vec::Vec::new();
        batch.extend_from_slice(&BATCH_MAGIC.to_le_bytes());
        batch.extend_from_slice(&BATCH_VERSION.to_le_bytes());
        batch.extend_from_slice(&1u16.to_le_bytes()); // 1 op
        batch.push(OP_CREATE_NODE);
        let kind = [0xBBu8; 16];
        batch.extend_from_slice(&kind);
        batch.extend_from_slice(&0u16.to_le_bytes());
        
        let res = unsafe {
            stem::syscall::syscall6(
                SYS_ROOT_APPLY_BATCH,
                batch.as_ptr() as usize,
                batch.len(),
                0, 0, 0, 0
            )
        };
        
        if (res as isize) < 0 {
            println!("FAIL: APPLY_BATCH failed: {}", res);
            failures += 1;
        } else {
            let mut buf = [0u8; 256];
            match watch_next(watch_handle, &mut buf) {
                Ok((len, seq)) => {
                    println!("APPLY_BATCH event: len={}, seq={}", len, seq);
                    
                    if validate_header(&buf) && len > 8 && buf[8] == OP_CREATE_NODE {
                        println!("PASS: APPLY_BATCH and single-op both produce consistent watch events");
                    } else {
                        println!("FAIL: APPLY_BATCH event inconsistent");
                        failures += 1;
                    }
                }
                Err(e) => {
                    println!("FAIL: watch_next failed after APPLY_BATCH: {}", e);
                    failures += 1;
                }
            }
        }
    }
    
    if failures > 0 {
        -failures
    } else {
        0
    }
}
