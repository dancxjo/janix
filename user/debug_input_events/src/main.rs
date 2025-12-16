#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
mod app {
    extern crate alloc;
    use thing_os::prelude::*;
    use thing_models::{InputCharEvent, KeyScanEvent, MousePacketEvent};
    use examples_support::{log, sleep_ms, UserlandSys};
    use alloc::format;
    use alloc::vec::Vec;
    use abi::Thing;

    #[unsafe(no_mangle)]
    pub fn main() {
        let mut sys = examples_support::init();
        if let Err(_) = run(&mut sys) {
            log(&mut sys, "debug_input_events", "Error running app");
        }
    }

    fn run(sys: &mut UserlandSys) -> Result<(), ()> {
        log(sys, "debug_input_events", "starting");

        let _ = register_schema_for::<KeyScanEvent>(sys);
        let _ = register_schema_for::<InputCharEvent>(sys);
        let _ = register_schema_for::<MousePacketEvent>(sys);

        let mut last_key_seq = max_sequence::<KeyScanEvent>(sys);
        let mut last_char_seq = max_sequence::<InputCharEvent>(sys);
        let mut last_mouse_seq = max_sequence::<MousePacketEvent>(sys);

        loop {
            process_events(sys, &mut last_key_seq, |e: &KeyScanEvent| {
                 format!("KEY: scancode={:#x} released={} extended={} (seq={})", 
                     e.scancode, e.released, e.extended, e.sequence_index)
            });

            process_events(sys, &mut last_char_seq, |e: &InputCharEvent| {
                 format!("CHAR: '{}' (seq={})", escape_char(e.ch), e.sequence_index)
            });

            process_events(sys, &mut last_mouse_seq, |e: &MousePacketEvent| {
                 format!("MOUSE: dx={} dy={} btns={:#x} (seq={})", 
                     e.delta_x, e.delta_y, e.buttons, e.sequence_index)
            });

            sleep_ms(sys, 10);
        }
    }

    trait Sequenced {
        fn sequence(&self) -> u64;
    }

    impl Sequenced for KeyScanEvent { fn sequence(&self) -> u64 { self.sequence_index } }
    impl Sequenced for InputCharEvent { fn sequence(&self) -> u64 { self.sequence_index } }
    impl Sequenced for MousePacketEvent { fn sequence(&self) -> u64 { self.sequence_index } }

    fn max_sequence<T: Thing + Sequenced>(sys: &mut UserlandSys) -> Option<u64> {
        list_things_by_kind::<UserlandSys, T>(sys)
            .iter()
            .map(|e| e.sequence())
            .max()
    }

    fn process_events<T, F>(sys: &mut UserlandSys, last_seq: &mut Option<u64>, fmt_fn: F)
    where 
        T: Thing + Sequenced,
        F: Fn(&T) -> alloc::string::String 
    {
        let events: Vec<T> = list_things_by_kind::<UserlandSys, T>(sys);
        
        let mut relevant: Vec<&T> = events.iter()
            .filter(|e| last_seq.map_or(true, |last| e.sequence() > last))
            .collect();
            
        relevant.sort_by_key(|e| e.sequence());

        for event in relevant {
            log(sys, "debug_input_events", &fmt_fn(event));
            *last_seq = Some(event.sequence());
        }
    }

    fn escape_char(c: char) -> alloc::string::String {
        match c {
            '\n' => "\\n".into(),
            '\r' => "\\r".into(),
            '\t' => "\\t".into(),
             _  => format!("{}", c),
        }
    }
}

#[cfg(not(target_os = "none"))]
fn main() {}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
