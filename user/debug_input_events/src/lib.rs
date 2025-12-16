#![no_std]

extern crate alloc;

use thing_models::{InputCharEvent, KeyScanEvent, MousePacketEvent};
use thing_os::prelude::*;

const POLL_NS: u64 = 10_000_000; // 10ms

pub fn main<S: Sys>(sys: &mut S) -> ! {
    println(sys, "debug_input_events: starting");

    // Register interest in schemas
    let _ = register_schema_for::<KeyScanEvent>(sys);
    let _ = register_schema_for::<InputCharEvent>(sys);
    let _ = register_schema_for::<MousePacketEvent>(sys);

    let mut last_key_seq = initial_sequence::<S, KeyScanEvent>(sys);
    let mut last_char_seq = initial_sequence::<S, InputCharEvent>(sys);
    let mut last_mouse_seq = initial_sequence::<S, MousePacketEvent>(sys);

    loop {
        // Log KeyScanEvents
        let keys: Vec<KeyScanEvent> = list_things_by_kind(sys);
        for event in keys {
            if is_new(event.sequence_index, &last_key_seq) {
                log_dynamic(
                    sys,
                    format_args!(
                        "KEY: scancode={:#x} released={} extended={} (seq={})",
                        event.scancode, event.released, event.extended, event.sequence_index
                    ),
                );
                update_seq(&mut last_key_seq, event.sequence_index);
            }
        }

        // Log InputCharEvents
        let chars: Vec<InputCharEvent> = list_things_by_kind(sys);
        for event in chars {
            if is_new(event.sequence_index, &last_char_seq) {
                log_dynamic(
                    sys,
                    format_args!(
                        "CHAR: '{}' (seq={})",
                        escape_char(event.ch),
                        event.sequence_index
                    ),
                );
                update_seq(&mut last_char_seq, event.sequence_index);
            }
        }

        // Log MousePacketEvents
        let mice: Vec<MousePacketEvent> = list_things_by_kind(sys);
        for event in mice {
            if is_new(event.sequence_index, &last_mouse_seq) {
                log_dynamic(
                    sys,
                    format_args!(
                        "MOUSE: dx={} dy={} btns={:#x} (seq={})",
                        event.delta_x, event.delta_y, event.buttons, event.sequence_index
                    ),
                );
                update_seq(&mut last_mouse_seq, event.sequence_index);
            }
        }

        sys.sleep_for_ns(POLL_NS);
    }
}

fn is_new(seq: u64, last: &Option<u64>) -> bool {
    match last {
        Some(l) => seq > *l,
        None => true,
    }
}

fn update_seq(last: &mut Option<u64>, current: u64) {
    *last = Some(match *last {
        Some(l) => l.max(current),
        None => current,
    });
}

// Helper trait to get sequence index generically would be nice, but explicit for now is fine.
fn initial_sequence<S: Sys, T: Thing + HasSequence>(sys: &mut S) -> Option<u64> {
    // This is a bit inefficient to list all just to find max, but acceptable for debug tool
    // Actually, we can just rely on the helper function logic if we implemented a trait.
    // But `Thing` doesn't enforce `sequence_index`.
    // We'll just define the trait locally.

    // Correct implementation:
    let items: Vec<T> = list_things_by_kind(sys);
    items.into_iter().map(|i| i.sequence_index()).max()
}

trait HasSequence {
    fn sequence_index(&self) -> u64;
}

impl HasSequence for KeyScanEvent {
    fn sequence_index(&self) -> u64 {
        self.sequence_index
    }
}

impl HasSequence for InputCharEvent {
    fn sequence_index(&self) -> u64 {
        self.sequence_index
    }
}

impl HasSequence for MousePacketEvent {
    fn sequence_index(&self) -> u64 {
        self.sequence_index
    }
}

fn escape_char(c: char) -> alloc::string::String {
    match c {
        '\n' => "\\n".into(),
        '\r' => "\\r".into(),
        '\t' => "\\t".into(),
        _ => alloc::format!("{}", c),
    }
}
