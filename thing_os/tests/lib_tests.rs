use std::cell::RefCell;
use std::collections::VecDeque;

use abi::{
    graph_kinds, KernelRequest, KernelResponse, MemorySummary, PropKey, PropType, PropValue,
    SchedulerSummary, ThingId,
};
use runtime::Sys;
use thing_os::{
    add_link, alloc_frame, create_thing, create_thread, create_transaction, default_mode,
    demo_shared::DemoState, demo_shared::DEMO_NAME_VAL, find_thing, free_frame,
    is_console_mode_active, link_targets, list_things_by_kind, load_thing, memory_summary,
    register_schema_for, scheduler_summary, scheduler_tick, spawn_program, update_props, Alarm,
    AlarmRequest, Mode, Thing,
};

#[derive(Default)]
struct MockSys {
    responses: RefCell<VecDeque<KernelResponse>>,
    requests: RefCell<Vec<KernelRequest>>,
    time: RefCell<u64>,
}

impl MockSys {
    fn with_responses(resps: Vec<KernelResponse>) -> Self {
        Self {
            responses: RefCell::new(resps.into()),
            ..Default::default()
        }
    }
}

impl Sys for MockSys {
    fn syscall(&self, request: KernelRequest) -> KernelResponse {
        self.requests.borrow_mut().push(request);
        self.responses
            .borrow_mut()
            .pop_front()
            .expect("mock response exhausted")
    }

    fn time_now_ns(&mut self) -> u64 {
        let mut t = self.time.borrow_mut();
        *t += 1;
        *t
    }

    fn time_monotonic_ns(&mut self) -> u64 {
        self.time_now_ns()
    }

    fn time_system_ns(&mut self) -> u64 {
        self.time_now_ns()
    }

    fn sleep_for_ns(&mut self, _delta_ns: u64) {}

    fn sleep_until_ns(&mut self, _deadline_ns: u64) {}

    fn yield_now(&mut self) {}

    fn exit_thread(&mut self) -> ! {
        panic!("exit_thread called in mock")
    }
}

#[derive(Clone, Debug)]
struct DummyThing {
    id: ThingId,
    flag: bool,
}

impl Thing for DummyThing {
    const KIND: &'static str = "Dummy";
    const DESCRIPTION: &'static str = "dummy";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("flag", PropValue::Bool(self.flag)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut flag = false;
        for (k, v) in props.iter().flatten() {
            if *k == "flag" {
                if let PropValue::Bool(b) = v {
                    flag = *b;
                }
            }
        }
        DummyThing { id, flag }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[("flag", PropType::Bool)]
    }
}

fn props_slice(props: Vec<(PropKey, PropValue)>) -> &'static [Option<(PropKey, PropValue)>] {
    Box::leak(
        props
            .into_iter()
            .map(Some)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    )
}

#[test]
fn create_and_load_roundtrip() {
    let props = props_slice(vec![("flag", PropValue::Bool(true))]);
    let sys = MockSys::with_responses(vec![
        KernelResponse::ThingCreated { id: ThingId(1) },
        KernelResponse::ThingData {
            id: ThingId(1),
            kind: DummyThing::KIND,
            props,
        },
    ]);

    let dummy = DummyThing {
        id: ThingId(0),
        flag: true,
    };
    let id = create_thing(&sys, &dummy).expect("thing id");
    assert_eq!(id, ThingId(1));

    let loaded: DummyThing = load_thing(&sys, id).expect("load");
    assert!(loaded.flag);
}

#[test]
fn list_and_find_things() {
    let props = props_slice(vec![("flag", PropValue::Bool(true))]);
    let mut sys = MockSys::with_responses(vec![
        KernelResponse::SchemaRegistered {
            kind: DummyThing::KIND,
        },
        KernelResponse::ThingListEntry {
            id: Some(ThingId(2)),
        },
        KernelResponse::ThingData {
            id: ThingId(2),
            kind: DummyThing::KIND,
            props,
        },
        KernelResponse::ThingListEntry { id: None },
    ]);

    assert!(register_schema_for::<DummyThing>(&sys));
    let listed: Vec<DummyThing> = list_things_by_kind(&mut sys);
    assert_eq!(listed.len(), 1);
}

struct FindSys {
    props: &'static [Option<(PropKey, PropValue)>],
}

impl Sys for FindSys {
    fn syscall(&self, request: KernelRequest) -> KernelResponse {
        match request {
            KernelRequest::ThingGet { id } if id == ThingId(5) => KernelResponse::ThingData {
                id,
                kind: DummyThing::KIND,
                props: self.props,
            },
            KernelRequest::ThingGet { .. } => KernelResponse::Error {
                message: "not found",
            },
            _ => panic!("unexpected request {:?}", request),
        }
    }

    fn time_now_ns(&mut self) -> u64 {
        0
    }
    fn sleep_until_ns(&mut self, _deadline_ns: u64) {}
    fn time_monotonic_ns(&mut self) -> u64 {
        0
    }
    fn time_system_ns(&mut self) -> u64 {
        0
    }
    fn sleep_for_ns(&mut self, _delta_ns: u64) {}
    fn yield_now(&mut self) {}
    fn exit_thread(&mut self) -> ! {
        panic!("exit")
    }
}

#[test]
fn find_thing_scans_until_match() {
    let props = props_slice(vec![("flag", PropValue::Bool(true))]);
    let sys = FindSys { props };
    let found = find_thing::<DummyThing>(&sys, |d| d.flag).expect("found");
    assert_eq!(found.id, ThingId(5));
}

#[test]
fn mode_selection_helpers() {
    let mode_active = props_slice(vec![
        (graph_kinds::PROP_MODE_INDEX, PropValue::U64(5)),
        (graph_kinds::PROP_NAME, PropValue::Str("m5".into())),
        (graph_kinds::PROP_MODE_ACTIVE, PropValue::Bool(true)),
    ]);
    let mode_inactive = props_slice(vec![
        (graph_kinds::PROP_MODE_INDEX, PropValue::U64(2)),
        (graph_kinds::PROP_NAME, PropValue::Str("m2".into())),
        (graph_kinds::PROP_MODE_ACTIVE, PropValue::Bool(false)),
    ]);
    let mut sys = MockSys::with_responses(vec![
        // active_mode list sequence
        KernelResponse::ThingListEntry {
            id: Some(ThingId(1)),
        },
        KernelResponse::ThingData {
            id: ThingId(1),
            kind: Mode::KIND,
            props: mode_active,
        },
        KernelResponse::ThingListEntry {
            id: Some(ThingId(2)),
        },
        KernelResponse::ThingData {
            id: ThingId(2),
            kind: Mode::KIND,
            props: mode_inactive,
        },
        KernelResponse::ThingListEntry { id: None },
        // default_mode list sequence
        KernelResponse::ThingListEntry {
            id: Some(ThingId(1)),
        },
        KernelResponse::ThingData {
            id: ThingId(1),
            kind: Mode::KIND,
            props: mode_active,
        },
        KernelResponse::ThingListEntry {
            id: Some(ThingId(2)),
        },
        KernelResponse::ThingData {
            id: ThingId(2),
            kind: Mode::KIND,
            props: mode_inactive,
        },
        KernelResponse::ThingListEntry { id: None },
        // is_console_mode_active calls active_mode again
        KernelResponse::ThingListEntry {
            id: Some(ThingId(1)),
        },
        KernelResponse::ThingData {
            id: ThingId(1),
            kind: Mode::KIND,
            props: mode_active,
        },
        KernelResponse::ThingListEntry {
            id: Some(ThingId(2)),
        },
        KernelResponse::ThingData {
            id: ThingId(2),
            kind: Mode::KIND,
            props: mode_inactive,
        },
        KernelResponse::ThingListEntry { id: None },
        // is_console_mode_active -> active_mode list sequence
        KernelResponse::ThingListEntry {
            id: Some(ThingId(1)),
        },
        KernelResponse::ThingData {
            id: ThingId(1),
            kind: Mode::KIND,
            props: mode_active,
        },
        KernelResponse::ThingListEntry {
            id: Some(ThingId(2)),
        },
        KernelResponse::ThingData {
            id: ThingId(2),
            kind: Mode::KIND,
            props: mode_inactive,
        },
        KernelResponse::ThingListEntry { id: None },
    ]);

    let active = thing_os::active_mode(&mut sys).expect("active");
    assert!(active.active);
    let default = default_mode(&mut sys).expect("default");
    assert_eq!(default.index, 2);
    assert!(is_console_mode_active(&mut sys) == false);
}

#[test]
fn link_helpers_and_updates() {
    let mut sys = MockSys::with_responses(vec![
        KernelResponse::Success { data: None }, // add_link
        KernelResponse::LinkTarget {
            target: Some(ThingId(5)),
        },
        KernelResponse::LinkTarget { target: None },
        KernelResponse::Success { data: None }, // update_props
    ]);

    assert!(add_link(
        &sys,
        ThingId(1),
        graph_kinds::LINK_RUNS_ON,
        ThingId(2)
    ));
    let neighbors = link_targets(&mut sys, ThingId(1), graph_kinds::LINK_RUNS_ON);
    assert_eq!(neighbors, vec![ThingId(5)]);
    assert!(update_props(
        &sys,
        ThingId(1),
        &[("flag", PropValue::Bool(true))]
    ));
}

#[test]
fn summaries_and_frames() {
    let sys = MockSys::with_responses(vec![
        KernelResponse::MemorySummary {
            summary: MemorySummary {
                total_frames: 3,
                used_frames: 1,
                free_frames: 2,
            },
        },
        KernelResponse::SchedulerSummary {
            summary: SchedulerSummary {
                process_count: 1,
                thread_count: 2,
                runnable_threads: 1,
            },
        },
        KernelResponse::FrameAllocated {
            frame: abi::FrameInfo {
                id: abi::FrameId(9),
                base: 0x1000,
                size: 4096,
            },
        },
        KernelResponse::FrameFreed {
            frame_id: abi::FrameId(9),
        },
    ]);

    assert_eq!(memory_summary(&sys).unwrap().free_frames, 2);
    assert_eq!(scheduler_summary(&sys).unwrap().thread_count, 2);
    let frame = alloc_frame(&sys).unwrap();
    assert_eq!(frame.id.0, 9);
    assert!(free_frame(&sys, abi::FrameId(9)));
}

#[test]
fn process_and_thread_ops() {
    let mut sys = MockSys::with_responses(vec![
        KernelResponse::ProcessCreated { pid: 10 },
        KernelResponse::ThreadCreated { tid: 20 },
        KernelResponse::SchedulerTicked {
            current: Some(abi::ThreadInfo {
                tid: 20,
                state: 1,
                priority: 5,
            }),
        },
        KernelResponse::ProgramSpawned {
            process_id: ThingId(3),
            thread_id: ThingId(4),
        },
    ]);

    assert_eq!(thing_os::create_process(&sys, "p").unwrap(), 10);
    assert_eq!(create_thread(&sys, 10, "t", 1, 5).unwrap(), 20);
    assert_eq!(scheduler_tick(&sys).unwrap().tid, 20);
    assert_eq!(spawn_program(&mut sys, ThingId(1)).unwrap().0, ThingId(3));
}

#[test]
fn shared_buffer_and_display_open() {
    let display_props = props_slice(vec![
        (graph_kinds::PROP_NAME, PropValue::Str("display0".into())),
        (graph_kinds::PROP_WIDTH, PropValue::U64(640)),
        (graph_kinds::PROP_HEIGHT, PropValue::U64(480)),
        (graph_kinds::PROP_STRIDE, PropValue::U64(640 * 4)),
        (
            graph_kinds::PROP_DISPLAY_ACTIVE_BUFFER_INDEX,
            PropValue::I64(0),
        ),
    ]);
    let mut sys = MockSys::with_responses(vec![
        // ThingList/ThingGet for Display
        KernelResponse::ThingListEntry {
            id: Some(ThingId(1)),
        },
        KernelResponse::ThingData {
            id: ThingId(1),
            kind: graph_kinds::KIND_DISPLAY,
            props: display_props,
        },
        KernelResponse::ThingListEntry { id: None },
        // Link targets for display buffers
        KernelResponse::LinkTarget {
            target: Some(ThingId(2)),
        },
        KernelResponse::LinkTarget { target: None },
        KernelResponse::LinkTarget {
            target: Some(ThingId(3)),
        },
        KernelResponse::LinkTarget { target: None },
        // Shared buffer info/mapping for front
        KernelResponse::SharedBufferInfoResponse {
            info: abi::SharedBufferInfo {
                width: 640,
                height: 480,
                stride: 640 * 4,
                pixel_format: abi::PixelFormat::Bgra8888,
            },
        },
        KernelResponse::SharedBufferMapped {
            vaddr: 0x1000,
            size: 640 * 480 * 4,
        },
        // Shared buffer info/mapping for back
        KernelResponse::SharedBufferInfoResponse {
            info: abi::SharedBufferInfo {
                width: 640,
                height: 480,
                stride: 640 * 4,
                pixel_format: abi::PixelFormat::Bgra8888,
            },
        },
        KernelResponse::SharedBufferMapped {
            vaddr: 0x2000,
            size: 640 * 480 * 4,
        },
    ]);

    let buf = thing_os::open_primary_display_buffer(&mut sys).expect("open display");
    assert_eq!(buf.front_buffer().id, ThingId(2));
    assert_eq!(buf.back_buffer().id, ThingId(3));
    assert_eq!(buf.front_buffer().info.width, 640);
}

#[test]
fn transaction_and_graph_queries() {
    let sys = MockSys::with_responses(vec![
        KernelResponse::TransactionCreated {
            tx_id: abi::TransactionId(7),
        },
        KernelResponse::Success { data: None }, // commit
        KernelResponse::NodeData {
            node_id: abi::NodeId(1),
            value: 55,
        },
    ]);

    assert_eq!(create_transaction(&sys).unwrap(), abi::TransactionId(7));
    assert!(thing_os::commit_transaction(&sys, abi::TransactionId(7)));
    assert_eq!(thing_os::graph_query(&sys, abi::NodeId(1)).unwrap(), 55);
}

#[test]
fn demo_state_updates_and_read() {
    let props = props_slice(vec![
        ("name", PropValue::U64(DEMO_NAME_VAL)),
        ("hello_ticks", PropValue::U64(9)),
        ("heartbeat_ticks", PropValue::U64(10)),
    ]);
    let sys = MockSys::with_responses(vec![
        KernelResponse::Success { data: None },
        KernelResponse::Success { data: None },
        KernelResponse::ThingData {
            id: ThingId(7),
            kind: DemoState::KIND,
            props,
        },
    ]);

    let state = DemoState {
        id: ThingId(7),
        name: DEMO_NAME_VAL,
        hello_ticks: 0,
        heartbeat_ticks: 0,
    };

    assert!(state.update_hello_ticks(&sys, 9));
    assert!(state.update_heartbeat_ticks(&sys, 10));
    assert_eq!(state.read(&sys), Some((9, 10)));
}

#[test]
fn alarm_request_and_state() {
    let alarm_req = AlarmRequest {
        id: ThingId(0),
        target_unix_seconds: 0,
        target_unix_nanos: 0,
        owner_process: ThingId(0),
        owner_thread: ThingId(0),
        state: "Fired".to_string(),
        target_ticks: None,
    };
    let mut props_vec = Vec::new();
    alarm_req.to_props(&mut props_vec);
    let props = props_slice(props_vec);
    let mut sys = MockSys::with_responses(vec![
        KernelResponse::ThingCreated { id: ThingId(5) },
        KernelResponse::ThingData {
            id: ThingId(5),
            kind: AlarmRequest::KIND,
            props,
        },
    ]);

    let alarm = Alarm::request_at(&mut sys, 0, 0).expect("alarm");
    assert_eq!(alarm.id, ThingId(5));
    assert_eq!(alarm.state(&mut sys), Some("Fired".to_string()));
}
