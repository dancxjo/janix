#![no_std]
#![no_main]

use stem::{println, thread, time};

#[stem::main]
fn main() -> ! {
    println!("THREADS: starting");

    match thread::spawn(thread_b) {
        Ok(tid) => println!("Spawned thread B with ID {}", tid),
        Err(e) => println!("Failed to spawn thread B: {:?}", e),
    }

    tick("A");
}

fn tick(tag: &str) -> ! {
    let mut i: usize = 0;
    loop {
        println!("{}: tick {}", tag, i);
        i = i.wrapping_add(1);
        thread::yield_now();
        time::sleep_ms(1000);
    }
}

extern "C" fn thread_b() -> ! {
    tick("B")
}
