use abi::PropValue;
use kernel::sched_types::ThreadState;

#[test]
fn test_boot_graph_initialization() {
    let _guard = kernel::test_lock();
    kernel::init();
    kernel::init_boot_graph();

    // The boot graph should have created various Things
    // We can't directly query by kind, but we can verify that Things were created
    // by checking that ThingIds 0-N exist

    // Try to get first several Things (CpuCore, Process, Thread, AddressSpace, etc.)
    for i in 0..10 {
        let thing_id = abi::ThingId(i);
        if let Some((kind, props)) = kernel::graph::get_thing(thing_id) {
            println!("Thing {}: kind = {}", i, kind);
            for prop in props.iter() {
                if let Some((key, value)) = prop {
                    println!("  {} = {:?}", key, value);
                }
            }
        }
    }
}

#[test]
fn test_boot_graph_has_process() {
    let _guard = kernel::test_lock();
    kernel::init();
    kernel::init_boot_graph();

    // Look for Process Thing
    let mut found_process = false;
    for i in 0..20 {
        let thing_id = abi::ThingId(i);
        if let Some((kind, props)) = kernel::graph::get_thing(thing_id) {
            if kind == "Process" {
                found_process = true;
                // Check that it has a pid property
                let mut has_pid = false;
                for prop in props.iter() {
                    if let Some((key, _value)) = prop {
                        if *key == "pid" {
                            has_pid = true;
                        }
                    }
                }

                assert!(has_pid, "Process should have pid property");
                break;
            }
        }
    }

    assert!(
        found_process,
        "Boot graph should contain at least one Process"
    );
}

#[test]
fn test_boot_graph_has_thread() {
    let _guard = kernel::test_lock();
    kernel::init();
    kernel::init_boot_graph();

    // Look for Thread Thing
    let mut found_thread = false;
    let mut thread_state = None;

    for i in 0..20 {
        let thing_id = abi::ThingId(i);
        if let Some((kind, props)) = kernel::graph::get_thing(thing_id) {
            if kind == "Thread" {
                found_thread = true;

                // Check properties
                let mut has_tid = false;
                let mut has_state = false;
                let mut has_priority = false;
                let mut has_runtime = false;
                let mut has_last_started = false;

                for prop in props.iter() {
                    if let Some((key, value)) = prop {
                        match *key {
                            "tid" => has_tid = true,
                            "state" => {
                                has_state = true;
                                if let PropValue::Str(state) = value {
                                    thread_state = Some(state.clone());
                                }
                            }
                            "priority" => has_priority = true,
                            "runtime_ns" => has_runtime = true,
                            "last_started_ns" => has_last_started = true,
                            _ => {}
                        }
                    }
                }

                assert!(has_tid, "Thread should have tid property");
                assert!(has_state, "Thread should have state property");
                assert!(has_priority, "Thread should have priority property");
                assert!(has_runtime, "Thread should have runtime_ns property");
                assert!(has_last_started, "Thread should track last_started_ns");
                break;
            }
        }
    }

    assert!(
        found_thread,
        "Boot graph should contain at least one Thread"
    );
    assert_eq!(
        thread_state.as_deref(),
        Some(ThreadState::Running.as_str()),
        "Thread should be in Running state"
    );
}

#[test]
fn test_boot_graph_has_cpu_core() {
    let _guard = kernel::test_lock();
    kernel::init();
    kernel::init_boot_graph();

    // Look for CpuCore Thing
    let mut found_cpu = false;

    for i in 0..20 {
        let thing_id = abi::ThingId(i);
        if let Some((kind, props)) = kernel::graph::get_thing(thing_id) {
            if kind == "CpuCore" {
                found_cpu = true;

                // Check that it has an index property
                let mut has_index = false;
                for prop in props.iter() {
                    if let Some((key, _value)) = prop {
                        if *key == "index" {
                            has_index = true;
                        }
                    }
                }

                assert!(has_index, "CpuCore should have index property");
                break;
            }
        }
    }

    assert!(found_cpu, "Boot graph should contain at least one CpuCore");
}

#[test]
fn test_boot_graph_has_address_space() {
    let _guard = kernel::test_lock();
    kernel::init();
    kernel::init_boot_graph();

    // Look for AddressSpace Thing
    let mut found_addr_space = false;

    for i in 0..20 {
        let thing_id = abi::ThingId(i);
        if let Some((kind, props)) = kernel::graph::get_thing(thing_id) {
            if kind == "AddressSpace" {
                found_addr_space = true;

                // Check that it has an asid property
                let mut has_asid = false;
                for prop in props.iter() {
                    if let Some((key, _value)) = prop {
                        if *key == "asid" {
                            has_asid = true;
                        }
                    }
                }

                assert!(has_asid, "AddressSpace should have asid property");
                break;
            }
        }
    }

    assert!(
        found_addr_space,
        "Boot graph should contain at least one AddressSpace"
    );
}

#[test]
fn test_boot_graph_has_frame_pool() {
    let _guard = kernel::test_lock();
    kernel::init();
    kernel::init_boot_graph();

    // Look for FramePool Thing
    let mut found_pool = false;

    for i in 0..20 {
        let thing_id = abi::ThingId(i);
        if let Some((kind, props)) = kernel::graph::get_thing(thing_id) {
            if kind == "FramePool" {
                found_pool = true;

                // Check required properties
                let mut has_start = false;
                let mut has_end = false;
                let mut has_frame_size = false;

                for prop in props.iter() {
                    if let Some((key, _value)) = prop {
                        match *key {
                            "start" => has_start = true,
                            "end" => has_end = true,
                            "frame_size" => has_frame_size = true,
                            _ => {}
                        }
                    }
                }

                assert!(has_start, "FramePool should have start property");
                assert!(has_end, "FramePool should have end property");
                assert!(has_frame_size, "FramePool should have frame_size property");
                break;
            }
        }
    }

    assert!(
        found_pool,
        "Boot graph should contain at least one FramePool"
    );
}

#[test]
fn test_boot_graph_has_phys_frames() {
    let _guard = kernel::test_lock();
    kernel::init();
    kernel::init_boot_graph();

    // Look for PhysFrame Things
    let mut frame_count = 0;

    for i in 0..20 {
        let thing_id = abi::ThingId(i);
        if let Some((kind, _props)) = kernel::graph::get_thing(thing_id) {
            if kind == "PhysFrame" {
                frame_count += 1;
            }
        }
    }

    assert!(
        frame_count >= 3,
        "Boot graph should contain at least 3 PhysFrame nodes, found {}",
        frame_count
    );
}

#[test]
fn test_boot_graph_has_virt_regions() {
    let _guard = kernel::test_lock();
    kernel::init();
    kernel::init_boot_graph();

    // Look for VirtRegion Things
    let mut region_count = 0;

    for i in 0..20 {
        let thing_id = abi::ThingId(i);
        if let Some((kind, props)) = kernel::graph::get_thing(thing_id) {
            if kind == "VirtRegion" {
                region_count += 1;

                // Verify required properties exist
                let mut has_base = false;
                let mut has_len = false;
                let mut has_flags = false;

                for prop in props.iter() {
                    if let Some((key, _value)) = prop {
                        match *key {
                            "base" => has_base = true,
                            "len" => has_len = true,
                            "flags" => has_flags = true,
                            _ => {}
                        }
                    }
                }

                assert!(has_base, "VirtRegion should have base property");
                assert!(has_len, "VirtRegion should have len property");
                assert!(has_flags, "VirtRegion should have flags property");
            }
        }
    }

    assert!(
        region_count >= 3,
        "Boot graph should contain at least 3 VirtRegion nodes, found {}",
        region_count
    );
}

#[test]
fn test_model_create_functions() {
    let _guard = kernel::test_lock();
    kernel::init();

    // Test each create function directly
    let cpu = kernel::model::create_cpu_core(0);
    assert!(cpu.is_some(), "Should create CpuCore");

    let process = kernel::model::create_process(100);
    assert!(process.is_some(), "Should create Process");

    let thread = kernel::model::create_thread(200, 50);
    assert!(thread.is_some(), "Should create Thread");

    let addr_space = kernel::model::create_address_space(10);
    assert!(addr_space.is_some(), "Should create AddressSpace");

    let frame_pool = kernel::model::create_frame_pool(0x1000, 0x2000, 4096);
    assert!(frame_pool.is_some(), "Should create FramePool");

    let phys_frame = kernel::model::create_phys_frame(0x1000, 4096);
    assert!(phys_frame.is_some(), "Should create PhysFrame");

    let virt_region = kernel::model::create_virt_region(0x400000, 0x1000, 0x7);
    assert!(virt_region.is_some(), "Should create VirtRegion");
}
