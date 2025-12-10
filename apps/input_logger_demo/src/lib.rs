#![no_std]

use thing_models::InputCharEvent;
use userland::prelude::*;

const POLL_NS: u64 = 5_000_000;

pub fn run<S: Sys>(sys: &mut S) -> ! {
    println(sys, "input_logger_demo: starting");
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
            format!(
                "input_logger_demo: InputCharEvent '{}' (seq={})",
                label,
                event.sequence_index
            ),
        );
        return;
    }

    log_dynamic(
        sys,
        format!(
            "input_logger_demo: InputCharEvent '{}' (seq={})",
            event.ch,
            event.sequence_index
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
