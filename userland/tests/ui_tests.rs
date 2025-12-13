use std::cell::RefCell;
use std::collections::VecDeque;

use abi::{KernelRequest, KernelResponse, PropKey, PropValue, ThingId};
use userland::ui::{
    WindowHandle, append_window_text, create_window, ensure_ui_schemas, set_window_text,
};
use runtime::Sys;
use userland_std::graph_kinds as gk;

#[derive(Default)]
struct MockSys {
    responses: RefCell<VecDeque<KernelResponse>>,
    requests: RefCell<Vec<KernelRequest>>,
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
        0
    }
    fn time_monotonic_ns(&mut self) -> u64 {
        0
    }
    fn time_system_ns(&mut self) -> u64 {
        0
    }
    fn sleep_for_ns(&mut self, _delta_ns: u64) {}
    fn sleep_until_ns(&mut self, _deadline_ns: u64) {}
    fn yield_now(&mut self) {}
    fn exit_thread(&mut self) -> ! {
        panic!("exit")
    }
}

fn mode_props(place: ThingId) -> &'static [Option<(PropKey, PropValue)>] {
    Box::leak(
        vec![
            Some((gk::PROP_MODE_INDEX, PropValue::U64(1))),
            Some((gk::PROP_NAME, PropValue::Str("console".into()))),
            Some((gk::PROP_MODE_PLACE, PropValue::U64(place.0))),
            Some((gk::PROP_MODE_ACTIVE, PropValue::Bool(true))),
        ]
        .into_boxed_slice(),
    )
}

fn surface_props(window: ThingId, text: &str) -> &'static [Option<(PropKey, PropValue)>] {
    Box::leak(
        vec![
            Some((gk::PROP_WINDOW_ID, PropValue::U64(window.0))),
            Some((gk::PROP_SURFACE_KIND, PropValue::Str("text".into()))),
            Some((gk::PROP_SURFACE_TEXT, PropValue::Str(text.into()))),
        ]
        .into_boxed_slice(),
    )
}

#[test]
fn ensure_ui_schemas_registers_all() {
    let mut sys = MockSys::with_responses(vec![
        KernelResponse::SchemaRegistered { kind: "Place" },
        KernelResponse::SchemaRegistered {
            kind: gk::KIND_MODE,
        },
        KernelResponse::SchemaRegistered {
            kind: gk::KIND_WINDOW,
        },
        KernelResponse::SchemaRegistered {
            kind: gk::KIND_SURFACE,
        },
    ]);

    ensure_ui_schemas(&mut sys);
}

#[test]
fn create_window_links_surface_place() {
    let mode_props = mode_props(ThingId(10));
    let mut sys = MockSys::with_responses(vec![
        // Schema registration
        KernelResponse::SchemaRegistered { kind: "Place" },
        KernelResponse::SchemaRegistered {
            kind: gk::KIND_MODE,
        },
        KernelResponse::SchemaRegistered {
            kind: gk::KIND_WINDOW,
        },
        KernelResponse::SchemaRegistered {
            kind: gk::KIND_SURFACE,
        },
        // list_things_by_kind for Mode
        KernelResponse::ThingListEntry {
            id: Some(ThingId(2)),
        },
        KernelResponse::ThingData {
            id: ThingId(2),
            kind: gk::KIND_MODE,
            props: mode_props,
        },
        KernelResponse::ThingListEntry { id: None },
        // create window
        KernelResponse::ThingCreated { id: ThingId(20) },
        // add link window to place
        KernelResponse::Success { data: None },
    ]);

    let handle = create_window(&mut sys, "Test", 1).expect("window created");
    assert_eq!(handle.id, ThingId(20));
}

#[test]
fn set_and_append_window_text_updates_existing_surface() {
    let window = WindowHandle { id: ThingId(30) };
    let surf_props = surface_props(window.id, "hi");

    let mut sys = MockSys::with_responses(vec![
        // Schema registration for set_window_text
        KernelResponse::SchemaRegistered { kind: "Place" },
        KernelResponse::SchemaRegistered {
            kind: gk::KIND_MODE,
        },
        KernelResponse::SchemaRegistered {
            kind: gk::KIND_WINDOW,
        },
        KernelResponse::SchemaRegistered {
            kind: gk::KIND_SURFACE,
        },
        // list surfaces (one)
        KernelResponse::ThingListEntry {
            id: Some(ThingId(5)),
        },
        KernelResponse::ThingData {
            id: ThingId(5),
            kind: gk::KIND_SURFACE,
            props: surf_props,
        },
        KernelResponse::ThingListEntry { id: None },
        // update_props called by set_window_text
        KernelResponse::Success { data: None },
        // Schema registration for append_window_text
        KernelResponse::SchemaRegistered { kind: "Place" },
        KernelResponse::SchemaRegistered {
            kind: gk::KIND_MODE,
        },
        KernelResponse::SchemaRegistered {
            kind: gk::KIND_WINDOW,
        },
        KernelResponse::SchemaRegistered {
            kind: gk::KIND_SURFACE,
        },
        // list surfaces again
        KernelResponse::ThingListEntry {
            id: Some(ThingId(5)),
        },
        KernelResponse::ThingData {
            id: ThingId(5),
            kind: gk::KIND_SURFACE,
            props: surf_props,
        },
        KernelResponse::ThingListEntry { id: None },
        // update_props called by append_window_text
        KernelResponse::Success { data: None },
    ]);

    set_window_text(&mut sys, window, "hi");
    append_window_text(&mut sys, window, "!");
}
