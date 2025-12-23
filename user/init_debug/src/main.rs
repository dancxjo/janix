#![no_std]
#![no_main]

extern crate alloc;

use thing_models::BootProgram;
use thing_os::{create_process, prelude::*};

const IDLE_SLEEP_MS: u64 = 1_000;

#[thing_os::main]
fn main() {
    println!("init_debug: starting clock-only init");

    if !ensure_schema_exists_for::<BootProgram>() {
        println!("init_debug: BootProgram schema missing");
        idle();
    }

    let Some(clock_program) = find_thing::<BootProgram>(|program| program.binary == "clock") else {
        println!("init_debug: clock BootProgram missing");
        idle();
    };

    println!("init_debug: launching clock");
    if create_process(clock_program.id).is_err() {
        println!("init_debug: failed to spawn clock");
    }

    idle();
}

fn idle() -> ! {
    loop {
        sleep(Duration::from_millis(IDLE_SLEEP_MS));
    }
}
