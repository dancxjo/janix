#![no_std]

extern crate alloc;

use alloc::format;
use thing_os::prelude::*;

pub fn main<S: Sys>(sys: &mut S) -> ! {
    println(sys, "window_demo: starting");
    let handle = match create_window(sys, "Demo") {
        Some(h) => h,
        None => {
            println(sys, "window_demo: failed to create window");
            sys.exit_thread();
        }
    };

    let mut counter = 0_u64;
    loop {
        let text = format!("Hello from windowed world!\nCounter: {}", counter);
        set_window_text(sys, handle, &text);
        counter = counter.saturating_add(1);
        sys.sleep_for_ns(1_000_000_000);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::{KernelRequest, KernelResponse, PropKey, PropValue, ThingId};
    use alloc::vec::Vec;
    use userland::ui::{WindowHandle, create_window, set_window_text};
    use thing_os::{Mode, Place, Surface, Thing, Window, doc_helpers::DocSys, graph_kinds};

    fn thing_props<T: Thing>(thing: &T) -> &'static [Option<(PropKey, PropValue)>] {
        let mut props = Vec::new();
        thing.to_props(&mut props);
        DocSys::props_slice(props)
    }

    #[test]
    fn create_window_emits_schema_and_links() {
        let mode = Mode {
            id: ThingId(5),
            index: 1,
            name: "Desktop".to_string(),
            place_id: Some(ThingId(13)),
            active: true,
            layout_policy: None,
        };
        let mut responses = Vec::new();
        responses.push(KernelResponse::SchemaRegistered { kind: Place::KIND });
        responses.push(KernelResponse::SchemaRegistered { kind: Mode::KIND });
        responses.push(KernelResponse::SchemaRegistered { kind: Window::KIND });
        responses.push(KernelResponse::SchemaRegistered {
            kind: Surface::KIND,
        });
        responses.push(KernelResponse::ThingListEntry { id: Some(mode.id) });
        responses.push(KernelResponse::ThingData {
            id: mode.id,
            kind: Mode::KIND,
            props: thing_props(&mode),
        });
        responses.push(KernelResponse::ThingListEntry { id: None });
        responses.push(KernelResponse::ThingCreated { id: ThingId(99) });
        responses.push(KernelResponse::Success { data: None });

        let mut sys = DocSys::with_responses(responses);
        let handle = create_window(&mut sys, "Demo").expect("window handle");
        assert_eq!(handle.id, ThingId(99));

        let requests = sys.requests.borrow();
        assert!(requests.iter().any(|request| match request {
            KernelRequest::AddLink { src, pred, dst } =>
                *pred == graph_kinds::LINK_PLACE_WINDOW
                    && *src == mode.place_id.unwrap_or(ThingId(0))
                    && *dst == handle.id,
            _ => false,
        }));
        assert!(requests.iter().any(|request| match request {
            KernelRequest::ThingCreate { kind, .. } => *kind == Window::KIND,
            _ => false,
        }));
    }

    #[test]
    fn set_window_text_creates_surface_when_missing() {
        let mut responses = Vec::new();
        responses.push(KernelResponse::SchemaRegistered { kind: Place::KIND });
        responses.push(KernelResponse::SchemaRegistered { kind: Mode::KIND });
        responses.push(KernelResponse::SchemaRegistered { kind: Window::KIND });
        responses.push(KernelResponse::SchemaRegistered {
            kind: Surface::KIND,
        });
        responses.push(KernelResponse::ThingListEntry { id: None });
        responses.push(KernelResponse::ThingCreated { id: ThingId(42) });
        responses.push(KernelResponse::Success { data: None });

        let mut sys = DocSys::with_responses(responses);
        let handle = WindowHandle { id: ThingId(99) };
        set_window_text(&mut sys, handle, "hello");

        let requests = sys.requests.borrow();
        assert!(requests.iter().any(|request| match request {
            KernelRequest::ThingCreate { kind, .. } => *kind == Surface::KIND,
            _ => false,
        }));
        assert!(requests.iter().any(|request| match request {
            KernelRequest::AddLink { src, pred, dst } =>
                *pred == graph_kinds::LINK_WINDOW_SURFACE
                    && *src == handle.id
                    && *dst == ThingId(42),
            _ => false,
        }));
    }
}
