#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use thing_models::InputCharEvent;
use thing_os::prelude::*;

#[thing_os::main]
fn main() {
    println!("debug_input_logger: starting");

    if !ensure_schema_exists_for::<InputCharEvent>() {
        println!("debug_input_logger: InputCharEvent schema missing");
        return;
    }

    let mut last_seq = list_things_by_kind::<InputCharEvent>()
        .iter()
        .map(|e| e.sequence_index)
        .max();

    loop {
        let mut events: Vec<InputCharEvent> = list_things_by_kind();
        events.sort_by_key(|e| e.sequence_index);

        for event in events {
            if last_seq.map_or(true, |last| event.sequence_index > last) {
                log_event(&event);
                last_seq = Some(event.sequence_index);
            }
        }

        sleep_ms(5);
    }
}

fn log_event(event: &InputCharEvent) {
    let display = match event.ch {
        '\n' => "\\n",
        '\u{0008}' => "\\b",
        _ => {
            println!(
                "debug_input_logger: InputCharEvent '{}' (seq={})",
                event.ch, event.sequence_index
            );
            return;
        }
    };
    println!(
        "debug_input_logger: InputCharEvent '{}' (seq={})",
        display, event.sequence_index
    );
}
