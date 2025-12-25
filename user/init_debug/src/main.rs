#![no_std]
#![no_main]

extern crate alloc;

use thing_models::BootProgram;
use thing_os::{create_process, prelude::*};

const IDLE_SLEEP_MS: u64 = 1_000;

#[thing_os::main]
fn main() {
    println!("init_debug: starting debug init (clock/taskman disabled)");

    if !ensure_schema_exists_for::<BootProgram>() {
        println!("init_debug: BootProgram schema missing");
        idle();
    }

    launch_program("framebuffer");
    launch_program("compositor");
    launch_program("ps2_keyboard_driver");
    launch_program("debug_input_logger");
    launch_program("debug_input_events");
    launch_program("ps2_mouse_driver");
    launch_program("watch_test");
    launch_program("window_demo");

    idle();
}

fn launch_program(binary: &str) {
    let Some(program) = find_thing::<BootProgram>(|program| program.binary == binary) else {
        println!("init_debug: {} BootProgram missing", binary);
        return;
    };

    println!("init_debug: launching {}", binary);
    if create_process(program.id).is_err() {
        println!("init_debug: failed to spawn {}", binary);
    }
}

fn idle() -> ! {
    loop {
        sleep(Duration::from_millis(IDLE_SLEEP_MS));
    }
}
