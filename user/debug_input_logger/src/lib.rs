#![no_std]

extern crate alloc;

use thing_models::InputCharEvent;
use thing_os::prelude::*;

const POLL_NS: u64 = 5_000_000;

pub fn run<S: Sys>(sys: &mut S) -> ! {
    println(sys, "debug_input_logger: starting");
    let _ = register_schema_for::<InputCharEvent>(sys);
    let mut last_sequence = initial_sequence(sys);
    loop {
        log_new_events(sys, &mut last_sequence);
        sys.sleep_for_ns(POLL_NS);
    }
}

fn initial_sequence<S: Sys>(sys: &mut S) -> Option<u64> {
    list_things_by_kind::<S, InputCharEvent>(sys)
        .into_iter()
        .map(|event| event.sequence_index)
        .max()
}

fn log_new_events<S: Sys>(sys: &mut S, last_sequence: &mut Option<u64>) {
    let mut events: Vec<InputCharEvent> = list_things_by_kind(sys);
    events.sort_by(|a, b| a.sequence_index.cmp(&b.sequence_index));

    for event in events {
        if let Some(last) = last_sequence {
            if event.sequence_index <= *last {
                continue;
            }
        }
        *last_sequence = Some(event.sequence_index);
        log_event(sys, &event);
    }
}

fn log_event<S: Sys>(sys: &mut S, event: &InputCharEvent) {
    if let Some(label) = special_display(event.ch) {
        log_dynamic(
            sys,
            format_args!(
                "debug_input_logger: InputCharEvent '{}' (seq={})",
                label, event.sequence_index
            ),
        );
        return;
    }

    log_dynamic(
        sys,
        format_args!(
            "debug_input_logger: InputCharEvent '{}' (seq={})",
            event.ch, event.sequence_index
        ),
    );
}

fn special_display(ch: char) -> Option<&'static str> {
    match ch {
        '\n' => Some("\\n"),
        '\u{0008}' => Some("\\b"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::{KernelRequest, KernelResponse, PropKey, PropValue, Thing, ThingId};
    use alloc::vec;
    use alloc::vec::Vec;
    use thing_models::InputCharEvent;
    use thing_os::doc_helpers::DocSys;

    fn thing_props<T: Thing>(thing: &T) -> &'static [Option<(PropKey, PropValue)>] {
        let mut props = Vec::new();
        thing.to_props(&mut props);
        DocSys::props_slice(props)
    }

    fn list_responses(events: &[InputCharEvent]) -> Vec<KernelResponse> {
        let mut responses = Vec::new();
        for event in events {
            responses.push(KernelResponse::ThingListEntry { id: Some(event.id) });
            responses.push(KernelResponse::ThingData {
                id: event.id,
                kind: InputCharEvent::KIND,
                props: thing_props(event),
            });
        }
        responses.push(KernelResponse::ThingListEntry { id: None });
        responses
    }

    #[test]
    fn special_display_handles_escape_sequences() {
        assert_eq!(special_display('\n'), Some("\\n"));
        assert_eq!(special_display('\u{0008}'), Some("\\b"));
        assert_eq!(special_display('a'), None);
    }

    #[test]
    fn initial_sequence_returns_highest_index() {
        let events = vec![
            InputCharEvent {
                id: ThingId(1),
                ch: 'x',
                source_controller: ThingId(1),
                source_port_index: 0,
                sequence_index: 7,
            },
            InputCharEvent {
                id: ThingId(2),
                ch: 'y',
                source_controller: ThingId(1),
                source_port_index: 0,
                sequence_index: 11,
            },
        ];
        let mut sys = DocSys::with_responses(list_responses(&events));
        assert_eq!(initial_sequence(&mut sys), Some(11));
    }

    #[test]
    fn log_new_events_skips_old_sequences() {
        let events = vec![
            InputCharEvent {
                id: ThingId(3),
                ch: 'a',
                source_controller: ThingId(1),
                source_port_index: 0,
                sequence_index: 5,
            },
            InputCharEvent {
                id: ThingId(4),
                ch: '\n',
                source_controller: ThingId(1),
                source_port_index: 0,
                sequence_index: 12,
            },
        ];
        let mut responses = list_responses(&events);
        responses.push(KernelResponse::Success { data: None });
        let mut sys = DocSys::with_responses(responses);
        let mut last_sequence = Some(6);
        log_new_events(&mut sys, &mut last_sequence);
        assert_eq!(last_sequence, Some(12));
        let requests = sys.requests.borrow();
        assert!(requests.iter().any(|request| match request {
            KernelRequest::Log { message } => message.contains("seq=12"),
            _ => false,
        }));
    }
}
