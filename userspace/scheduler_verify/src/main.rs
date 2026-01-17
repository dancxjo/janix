#![no_std]
#![no_main]

use stem::{info, thread, time};

#[stem::main]
fn main() -> ! {
    info!("SCHEDULER_VERIFY: starting diagnostic app");

    // Spawn Thread B (The Printer)
    // This thread should print something every 100ms
    match thread::spawn(printer_thread) {
        Ok(tid) => info!("Spawned Printer Thread (TID {})", tid),
        Err(e) => info!("ERROR: failed to spawn Printer Thread: {:?}", e),
    }

    // Thread A (The Busy Bee)
    // This thread will busy-loop WITHOUT yielding.
    // In a cooperative scheduler, this will block the Printer Thread.
    // In a preemptive scheduler, both will run.
    info!("Busy Bee: starting infinite loop (no yields)");
    loop {
        core::hint::black_box(());
    }
}

extern "C" fn printer_thread() -> ! {
    let mut i = 0;
    loop {
        info!("Printer: heart-beat {}", i);
        i += 1;
        time::sleep_ms(100);
    }
}
