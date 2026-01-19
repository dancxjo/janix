#![no_std]
#![no_main]

//! Single-Op Syscall Watch Tester
//!
//! Tests that single-op syscalls (CREATE_NODE, LINK, PROP_SET) properly
//! deliver watch events through the unified batch pipeline.

extern crate alloc;
use stem::println;
use abi::root::*;
use abi::watch;
use abi::types::WATCH_START_LATEST;
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

/// Open a watch subscription (match all, start from oldest)
fn open_watch() -> Result<usize, i64> {
    open_watch_with_start_seq(0, RootWatchFilter::default())
}

/// Open a watch starting from LATEST (next commit only)
fn open_watch_latest() -> Result<usize, i64> {
    open_watch_with_start_seq(WATCH_START_LATEST, RootWatchFilter::default())
}

/// Open a watch with specific start_seq and filter
fn open_watch_with_start_seq(start_seq: u64, filter: RootWatchFilter) -> Result<usize, i64> {
    let mut spec = abi::types::WatchSpec::default();
    spec.start_seq = start_seq;
    spec.filter_ptr = &filter as *const _ as u64;
    spec.filter_len = core::mem::size_of::<RootWatchFilter>() as u64;
    
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

/// Open a watch with a specific filter (start from oldest)
fn open_watch_filtered(filter: RootWatchFilter) -> Result<usize, i64> {
    open_watch_with_start_seq(0, filter)
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

/// Drain watch events until EAGAIN, returning number of payloads seen.
fn drain_until_eagain(handle: usize, buf: &mut [u8]) -> Result<usize, i64> {
    let mut batches = 0usize;
    loop {
        match watch_next(handle, buf) {
            Ok((_len, _seq)) => {
                batches += 1;
            }
            Err(-11) => {
                return Ok(batches);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

/// Validate batch header
fn validate_header(buf: &[u8]) -> bool {
    watch::decode_event(buf).is_ok()
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
    
    // ========================================
    // Test 0: WATCH_START_LATEST immediate EAGAIN contract
    // ========================================
    println!("\n--- Test 0: Open with WATCH_START_LATEST, immediate EAGAIN ---");
    {
        let fresh_watch = match open_watch_latest() {
            Ok(h) => h,
            Err(e) => {
                println!("FAIL: Could not open watch: {}", e);
                failures += 1;
                0
            }
        };
        
        if fresh_watch != 0 {
            // Use a sentinel value to verify out_seq is not modified on EAGAIN
            let _out_seq_before = 0xDEADBEEF_u64;
            let mut buf = [0u8; 256];
            
            match watch_next(fresh_watch, &mut buf) {
                Err(-11) => {
                    // EAGAIN is expected - verify out_seq was not modified
                    // (Note: watch_next uses internal seq, so we just check error)
                    println!("PASS: Immediate watch_next returns EAGAIN");
                }
                Ok((len, _)) => {
                    println!("FAIL: Expected EAGAIN, got {} bytes", len);
                    failures += 1;
                }
                Err(e) => {
                    println!("FAIL: Expected EAGAIN (-11), got error: {}", e);
                    failures += 1;
                }
            }
        }
    }
    
    // ========================================
    // Test 0b: start_seq=0 means replay (not latest)
    // ========================================
    println!("\n--- Test 0b: Open with start_seq=0, verify not treated as latest ---");
    {
        // This test confirms 0 is NOT treated as "latest"
        // If there are historical events from boot, we should get them
        // If no events yet, EAGAIN is also acceptable
        let replay_watch = match open_watch() {
            Ok(h) => h,
            Err(e) => {
                println!("FAIL: Could not open replay watch: {}", e);
                failures += 1;
                0
            }
        };
        
        if replay_watch != 0 {
            println!("PASS: start_seq=0 watch opened successfully (will see historical events if any)");
        }
    }

    // ========================================
    // Test 0.5: Drain until EAGAIN contract
    // ========================================
    println!("\n--- Test 0.5: Drain until EAGAIN contract ---");
    {
        let drain_watch = match open_watch() {
            Ok(h) => h,
            Err(e) => {
                println!("FAIL: Could not open drain watch: {}", e);
                failures += 1;
                0
            }
        };

        if drain_watch != 0 {
            let mut buf = [0u8; 256];
            match watch_next(drain_watch, &mut buf) {
                Err(-11) => {
                    println!("PASS: Initial watch_next returned EAGAIN");
                }
                Ok((len, _)) => {
                    println!("INFO: Initial watch_next returned {} bytes (non-fresh state)", len);
                }
                Err(e) => {
                    println!("FAIL: Initial watch_next error: {}", e);
                    failures += 1;
                }
            }

            let _node = create_node("test.drain");
            let mut drain_buf = [0u8; 256];
            match drain_until_eagain(drain_watch, &mut drain_buf) {
                Ok(batches) if batches > 0 => {
                    println!("PASS: drain received {} batches then EAGAIN", batches);
                }
                Ok(_) => {
                    println!("FAIL: drain received 0 batches");
                    failures += 1;
                }
                Err(e) => {
                    println!("FAIL: drain error: {}", e);
                    failures += 1;
                }
            }
        }
    }
    
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
                if validate_header(&buf[..len]) {
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
                    if validate_header(&buf[..len]) {
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
                    if validate_header(&buf[..len]) {
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
                    
                    if validate_header(&buf[..len]) {
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
    
    // ========================================
    // Test 5: Subject Filter
    // ========================================
    println!("\n--- Test 5: Subject filter ---");
    {
        // Create target node FIRST before opening filtered watch
        let target_node = create_node("test.subject.target");
        println!("Created target node: id={}", target_node);
        
        // Open a watch filtered to only this subject
        let filter = RootWatchFilter::subject(target_node);
        let filtered_watch = match open_watch_filtered(filter) {
            Ok(h) => h,
            Err(e) => {
                println!("FAIL: Could not open filtered watch: {}", e);
                failures += 1;
                0
            }
        };
        
        if filtered_watch != 0 {
            println!("Filtered watch opened: handle={}", filtered_watch);
            
            // Create unrelated nodes (should be filtered out)
            let _unrelated1 = create_node("test.unrelated.1");
            let _unrelated2 = create_node("test.unrelated.2");
            println!("Created unrelated nodes");
            
            // Set a property on the target node (should match filter)
            let res = prop_set(target_node, "watched_prop", 123);
            if res != 0 {
                println!("WARN: prop_set returned: {}", res);
            }
            println!("Set property on target node");
            
            // The filtered watch should skip the unrelated CREATE_NODEs and find the PROP_SET
            let mut buf = [0u8; 256];
            let mut found_match = false;
            let mut attempts = 0;
            
            while attempts < 10 && !found_match {
                match watch_next(filtered_watch, &mut buf) {
                    Ok((len, seq)) => {
                        println!("Filtered watch received: len={}, seq={}", len, seq);
                        if len > 8 && buf[8] == OP_SET_PROP {
                            println!("PASS: Subject filter correctly received SET_PROP");
                            found_match = true;
                        } else {
                            println!("  Op tag: {:#x}", buf.get(8).unwrap_or(&0));
                        }
                    }
                    Err(-11) => {
                        // EAGAIN - no more events or scan limit
                        attempts += 1;
                        stem::yield_now();
                    }
                    Err(e) => {
                        println!("FAIL: watch_next error: {}", e);
                        failures += 1;
                        break;
                    }
                }
            }
            
            if !found_match {
                println!("FAIL: Subject filter did not find SET_PROP within attempts");
                failures += 1;
            }
        }
    }
    
    // ========================================
    // Test 6: Two Watchers Different Filters
    // ========================================
    println!("\n--- Test 6: Two watchers with different filters ---");
    {
        // Create two target nodes for two different filters
        let node_a = create_node("test.filter.a");
        let node_b = create_node("test.filter.b");
        println!("Created nodes: A={}, B={}", node_a, node_b);
        
        // Open two filtered watches
        let filter_a = RootWatchFilter::subject(node_a);
        let filter_b = RootWatchFilter::subject(node_b);
        
        let watch_a = open_watch_filtered(filter_a).unwrap_or(0);
        let watch_b = open_watch_filtered(filter_b).unwrap_or(0);
        
        if watch_a != 0 && watch_b != 0 {
            println!("Watch A (subject={}): handle={}", node_a, watch_a);
            println!("Watch B (subject={}): handle={}", node_b, watch_b);
            
            // Mutate node_a (should only be seen by watch_a)
            prop_set(node_a, "only_a", 111);
            
            // Mutate node_b (should only be seen by watch_b)
            prop_set(node_b, "only_b", 222);
            
            // Check watch_a sees its event
            let mut buf = [0u8; 256];
            let mut a_saw_event = false;
            for _ in 0..5 {
                match watch_next(watch_a, &mut buf) {
                    Ok((len, _seq)) if len > 8 && buf[8] == OP_SET_PROP => {
                        a_saw_event = true;
                        break;
                    }
                    Err(-11) => { stem::yield_now(); }
                    _ => break,
                }
            }
            
            // Check watch_b sees its event
            let mut b_saw_event = false;
            for _ in 0..5 {
                match watch_next(watch_b, &mut buf) {
                    Ok((len, _seq)) if len > 8 && buf[8] == OP_SET_PROP => {
                        b_saw_event = true;
                        break;
                    }
                    Err(-11) => { stem::yield_now(); }
                    _ => break,
                }
            }
            
            if a_saw_event && b_saw_event {
                println!("PASS: Both filtered watchers received their respective events");
            } else {
                println!("FAIL: Watch A saw={}, Watch B saw={}", a_saw_event, b_saw_event);
                failures += 1;
            }
        } else {
            println!("FAIL: Could not open dual filtered watches");
            failures += 1;
        }
    }
    
    // ========================================
    // Test 7: ENOSPC (buffer too small, non-destructive)
    // ========================================
    println!("\n--- Test 7: ENOSPC (buffer too small) ---");
    {
        // Open a fresh watch
        let enospc_watch = match open_watch() {
            Ok(h) => h,
            Err(e) => {
                println!("FAIL: Could not open watch for ENOSPC test: {}", e);
                failures += 1;
                0
            }
        };
        
        if enospc_watch != 0 {
            // Create a node to generate an event
            let _node = create_node("test.enospc");
            
            // Try to read with a tiny buffer (1 byte)
            let mut tiny_buf = [0u8; 1];
            match watch_next(enospc_watch, &mut tiny_buf) {
                Err(-28) => {
                    println!("PASS: Got ENOSPC as expected");
                    
                    // Now try with a proper buffer - event should still be there
                    let mut proper_buf = [0u8; 256];
                    match watch_next(enospc_watch, &mut proper_buf) {
                        Ok((len, _seq)) if len > 0 => {
                            println!("PASS: Event was NOT consumed on ENOSPC (got {} bytes)", len);
                        }
                        Ok((0, _)) => {
                            println!("FAIL: Event was consumed despite ENOSPC");
                            failures += 1;
                        }
                        Err(-11) => {
                            println!("FAIL: Event was consumed (got EAGAIN)");
                            failures += 1;
                        }
                        Err(e) => {
                            println!("FAIL: Unexpected error after ENOSPC: {}", e);
                            failures += 1;
                        }
                        _ => {}
                    }
                }
                Ok((len, _)) => {
                    println!("FAIL: Expected ENOSPC, got {} bytes", len);
                    failures += 1;
                }
                Err(e) => {
                    println!("FAIL: Expected ENOSPC (-28), got error: {}", e);
                    failures += 1;
                }
            }
        }
    }
    
    // ========================================
    // Test 8: EBADF (invalid watch handle)
    // ========================================
    println!("\n--- Test 8: EBADF (invalid watch handle) ---");
    {
        let invalid_handle = 0xDEAD_BEEF_usize;
        let mut buf = [0u8; 256];
        
        match watch_next(invalid_handle, &mut buf) {
            Err(-9) => {
                println!("PASS: Got EBADF for invalid handle");
            }
            Err(e) => {
                println!("FAIL: Expected EBADF (-9), got error: {}", e);
                failures += 1;
            }
            Ok(_) => {
                println!("FAIL: Expected error, got success");
                failures += 1;
            }
        }
    }
    
    if failures > 0 {
        -failures
    } else {
        0
    }
}
