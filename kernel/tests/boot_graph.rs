use kernel::sched_types::ThreadState;
use thing_models::PropValue;

#[test]
#[ignore]
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
        kernel::graph::with_thing(thing_id, |thing| {
            // We can't easily print symbol names without reverse lookup which might not be exposed in tests easily
            // But we can check they exist
            // println!("Thing {}: kind = {:?}", i, thing.kind);
            for (key, value) in &thing.props {
                // println!("  {:?} = {:?}", key, value);
            }
        });
    }
}

#[test]
#[ignore]
fn test_boot_graph_has_process() {
    let _guard = kernel::test_lock();
    kernel::init();
    kernel::init_boot_graph();

    // Look for Process Thing
    let mut found_process = false;
    let kind_process = kernel::symbols::intern("Process");
    let pid_sym = kernel::symbols::intern("pid");

    for i in 0..20 {
        let thing_id = abi::ThingId(i);
        if let Some(res) = kernel::graph::with_thing(thing_id, |thing| {
            if thing.kind == kind_process {
                // Check that it has a pid property
                let has_pid = thing.props.iter().any(|(k, _)| *k == pid_sym);
                return (true, has_pid);
            }
            (false, false)
        }) {
            if res.0 {
                found_process = true;
                assert!(res.1, "Process should have pid property");
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
#[ignore]
fn test_boot_graph_has_thread() {
    let _guard = kernel::test_lock();
    kernel::init();
    kernel::init_boot_graph();

    // Look for Thread Thing
    let mut found_thread = false;
    let mut thread_state = None;
    let kind_thread = kernel::symbols::intern("Thread");

    let sym_tid = kernel::symbols::intern("tid");
    let sym_state = kernel::symbols::intern("state");
    let sym_priority = kernel::symbols::intern("priority");
    let sym_runtime_ns = kernel::symbols::intern("runtime_ns");
    let sym_last_started_ns = kernel::symbols::intern("last_started_ns");

    for i in 0..20 {
        let thing_id = abi::ThingId(i);

        let result = kernel::graph::with_thing(thing_id, |thing| {
            if thing.kind == kind_thread {
                let mut has_tid = false;
                let mut state_val = None;
                let mut has_priority = false;
                let mut has_runtime = false;
                let mut has_last_started = false;

                for (key, value) in &thing.props {
                    if *key == sym_tid {
                        has_tid = true;
                    } else if *key == sym_state {
                        if let PropValue::Str(state) = value {
                            state_val = Some(state.clone());
                        }
                    } else if *key == sym_priority {
                        has_priority = true;
                    } else if *key == sym_runtime_ns {
                        has_runtime = true;
                    } else if *key == sym_last_started_ns {
                        has_last_started = true;
                    }
                }

                return Some((
                    has_tid,
                    state_val,
                    has_priority,
                    has_runtime,
                    has_last_started,
                ));
            }
            None
        });

        if let Some(Some((has_tid, state, has_priority, has_runtime, has_last_started))) = result {
            found_thread = true;
            thread_state = state;
            assert!(has_tid, "Thread should have tid property");
            assert!(thread_state.is_some(), "Thread should have state property");
            assert!(has_priority, "Thread should have priority property");
            assert!(has_runtime, "Thread should have runtime_ns property");
            assert!(has_last_started, "Thread should track last_started_ns");
            break;
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
#[ignore]
fn test_boot_graph_has_cpu_core() {
    let _guard = kernel::test_lock();
    kernel::init();
    kernel::init_boot_graph();

    // Look for CpuCore Thing
    let mut found_cpu = false;
    let kind_cpu = kernel::symbols::intern("CpuCore");
    let sym_index = kernel::symbols::intern("index");

    for i in 0..20 {
        let thing_id = abi::ThingId(i);
        if let Some(is_cpu) = kernel::graph::with_thing(thing_id, |thing| {
            if thing.kind == kind_cpu {
                // Check that it has an index property
                let has_index = thing.props.iter().any(|(k, _)| *k == sym_index);
                return Some(has_index);
            }
            None
        }) {
            if let Some(has_index) = is_cpu {
                found_cpu = true;
                assert!(has_index, "CpuCore should have index property");
                break;
            }
        }
    }

    assert!(found_cpu, "Boot graph should contain at least one CpuCore");
}

#[test]
#[ignore]
fn test_boot_graph_has_address_space() {
    let _guard = kernel::test_lock();
    kernel::init();
    kernel::init_boot_graph();

    // Look for AddressSpace Thing
    let mut found_addr_space = false;
    let kind_addr_space = kernel::symbols::intern("AddressSpace");
    let sym_asid = kernel::symbols::intern("asid");

    for i in 0..20 {
        let thing_id = abi::ThingId(i);
        if let Some(is_as) = kernel::graph::with_thing(thing_id, |thing| {
            if thing.kind == kind_addr_space {
                let has_asid = thing.props.iter().any(|(k, _)| *k == sym_asid);
                return Some(has_asid);
            }
            None
        }) {
            if let Some(has_asid) = is_as {
                found_addr_space = true;
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
#[ignore]
fn test_boot_graph_has_frame_pool() {
    let _guard = kernel::test_lock();
    kernel::init();
    kernel::init_boot_graph();

    // Look for FramePool Thing
    let mut found_pool = false;
    let kind_frame_pool = kernel::symbols::intern("FramePool");
    let sym_start = kernel::symbols::intern("start");
    let sym_end = kernel::symbols::intern("end");
    let sym_frame_size = kernel::symbols::intern("frame_size");

    for i in 0..20 {
        let thing_id = abi::ThingId(i);
        if let Some(res) = kernel::graph::with_thing(thing_id, |thing| {
            if thing.kind == kind_frame_pool {
                let mut has_start = false;
                let mut has_end = false;
                let mut has_frame_size = false;

                for (key, _value) in &thing.props {
                    if *key == sym_start {
                        has_start = true;
                    } else if *key == sym_end {
                        has_end = true;
                    } else if *key == sym_frame_size {
                        has_frame_size = true;
                    }
                }
                return Some((has_start, has_end, has_frame_size));
            }
            None
        }) {
            if let Some((has_start, has_end, has_frame_size)) = res {
                found_pool = true;
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
#[ignore]
fn test_boot_graph_has_phys_frames() {
    let _guard = kernel::test_lock();
    kernel::init();
    kernel::init_boot_graph();

    // Look for PhysFrame Things
    let mut frame_count = 0;
    let kind_phys_frame = kernel::symbols::intern("PhysFrame");

    for i in 0..20 {
        let thing_id = abi::ThingId(i);
        if let Some(is_frame) =
            kernel::graph::with_thing(thing_id, |thing| thing.kind == kind_phys_frame)
        {
            if is_frame {
                frame_count += 1;
            }
        }
    }

    assert!(
        frame_count >= 3,
        "Boot graph should contain at least 3 PhysFrame things, found {}",
        frame_count
    );
}

#[test]
#[ignore]
fn test_boot_graph_has_virt_regions() {
    let _guard = kernel::test_lock();
    kernel::init();
    kernel::init_boot_graph();

    // Look for VirtRegion Things
    let mut region_count = 0;
    let kind_virt_region = kernel::symbols::intern("VirtRegion");
    let sym_base = kernel::symbols::intern("base");
    let sym_len = kernel::symbols::intern("len");
    let sym_flags = kernel::symbols::intern("flags");

    for i in 0..20 {
        let thing_id = abi::ThingId(i);
        if let Some(res) = kernel::graph::with_thing(thing_id, |thing| {
            if thing.kind == kind_virt_region {
                let mut has_base = false;
                let mut has_len = false;
                let mut has_flags = false;

                for (key, _value) in &thing.props {
                    if *key == sym_base {
                        has_base = true;
                    } else if *key == sym_len {
                        has_len = true;
                    } else if *key == sym_flags {
                        has_flags = true;
                    }
                }
                return Some((has_base, has_len, has_flags));
            }
            None
        }) {
            if let Some((has_base, has_len, has_flags)) = res {
                region_count += 1;
                assert!(has_base, "VirtRegion should have base property");
                assert!(has_len, "VirtRegion should have len property");
                assert!(has_flags, "VirtRegion should have flags property");
            }
        }
    }

    assert!(
        region_count >= 3,
        "Boot graph should contain at least 3 VirtRegion things, found {}",
        region_count
    );
}

#[test]
#[ignore]
fn test_model_create_functions() {
    let _guard = kernel::test_lock();
    kernel::init();

    // Test each create function directly
    let cpu = kernel::model::create_cpu_core(0);
    assert!(cpu.is_some(), "Should create CpuCore");

    let process = kernel::model::create_process(100, kernel::symbols::intern("test"));
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
