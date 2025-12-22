use crate::*;
use alloc::vec::Vec;

#[test]
fn cpu_core_thing_props() {
    let cpu = CpuCoreThing {
        id: ThingId(1),
        index: 0,
    };
    let mut props = Vec::new();
    cpu.to_props(&mut props);

    assert_eq!(props.len(), 1);
    assert_eq!(props[0].0, "index");
    assert_eq!(props[0].1, PropValue::U64(0));

    let cpu2 = CpuCoreThing::from_props(
        ThingId(2),
        &[Some(("index".to_string(), PropValue::U64(5)))],
    );
    assert_eq!(cpu2.id, ThingId(2));
    assert_eq!(cpu2.index, 5);
}

#[test]
fn display_thing_props() {
    let display = DisplayThing {
        id: ThingId(10),
        name: "Screen".to_string(),
        width: 1920,
        height: 1080,
        stride: 1920 * 4,
        pixel_format: Some("Bgra8888".to_string()),
        active_buffer_index: 1,
    };
    let mut props = Vec::new();
    display.to_props(&mut props);

    // Check roundtrip
    let props_opt: Vec<Option<(PropKey, PropValue)>> = props.iter().cloned().map(Some).collect();
    let display2 = DisplayThing::from_props(ThingId(20), &props_opt);

    assert_eq!(display2.name, "Screen");
    assert_eq!(display2.width, 1920);
    assert_eq!(display2.height, 1080);
    assert_eq!(display2.active_buffer_index, 1);
}

#[test]
fn test_create_process() {
    use abi::{KernelRequest, KernelResponse};

    // Setup mock response
    let expected_pid = ThingId(123);
    let expected_tid = ThingId(456);

    mock::set_responses(vec![KernelResponse::ProgramSpawned {
        process_id: expected_pid,
        thread_id: expected_tid,
    }]);

    // Call function
    let boot_prog = ThingId(1);
    let result = create_process(boot_prog);

    // Verify result
    assert_eq!(result, Ok((expected_pid, expected_tid)));

    // Verify request
    let requests = mock::get_requests();
    assert_eq!(requests.len(), 1);
    match &requests[0] {
        KernelRequest::SpawnProgram { boot_program_id } => {
            assert_eq!(*boot_program_id, boot_prog);
        }
        _ => panic!("Unexpected request"),
    }
}
