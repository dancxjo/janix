#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
mod app {
    extern crate alloc;
    use thing_os::prelude::*;
    use thing_models::InputCharEvent;
    use examples_support::{log, sleep_ms, UserlandSys};
    use alloc::format;
    use alloc::vec::Vec;

    #[unsafe(no_mangle)]
    pub fn main() {
        let mut sys = examples_support::init();
        if let Err(_) = run(&mut sys) {
            log(&mut sys, "debug_input_logger", "Error running app");
        }
    }

    fn run(sys: &mut UserlandSys) -> Result<(), ()> {
        log(sys, "debug_input_logger", "starting");

        let _ = register_schema_for::<InputCharEvent>(sys);

        let mut last_seq = list_things_by_kind::<UserlandSys, InputCharEvent>(sys)
            .iter()
            .map(|e| e.sequence_index)
            .max();

        loop {
            let mut events: Vec<InputCharEvent> = list_things_by_kind::<UserlandSys, InputCharEvent>(sys);
            events.sort_by_key(|e| e.sequence_index);

            for event in events {
                if last_seq.map_or(true, |last| event.sequence_index > last) {
                    log_event(sys, &event);
                    last_seq = Some(event.sequence_index);
                }
            }

            sleep_ms(sys, 5);
        }
    }

    fn log_event(sys: &mut UserlandSys, event: &InputCharEvent) {
        let display_str = match event.ch {
            '\n' => "\\n",
            '\u{0008}' => "\\b",
            _ => return log(sys, "debug_input_logger", &format!("InputCharEvent '{}' (seq={})", event.ch, event.sequence_index)),
        };
        log(sys, "debug_input_logger", &format!("InputCharEvent '{}' (seq={})", display_str, event.sequence_index));
    }
}

#[cfg(not(target_os = "none"))]
fn main() {}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
