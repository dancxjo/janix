#![no_std]
#![no_main]

extern crate alloc;

use thing_models::BootProgram;
use thing_os::{create_process, prelude::*};

const IDLE_SLEEP_MS: u64 = 1_000;

#[thing_os::main]
fn main() {
    println!("init_debug: starting clock+debug_alloc init");

    if !ensure_schema_exists_for::<BootProgram>() {
        println!("init_debug: BootProgram schema missing");
        idle();
    }

    launch_program("clock");
    launch_program("debug_alloc");
    launch_program("debug_thread");

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
