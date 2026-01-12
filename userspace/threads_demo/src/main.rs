#![no_std]
#![no_main]

use stem::{error, info, thread, time};

#[stem::main]
fn main() -> ! {
    let cpu = stem::arch::whoami();
    info!(
        "whoami: cs=0x{:x} ss=0x{:x} cpl={} rsp=0x{:x} rip=0x{:x} rflags=0x{:x}",
        cpu.cs, cpu.ss, cpu.cpl, cpu.rsp, cpu.rip, cpu.rflags
    );

    info!("THREADS: starting");

    match thread::spawn(thread_b) {
        Ok(tid) => info!("Spawned thread B with ID {}", tid),
        Err(e) => error!("Failed to spawn thread B: {:?}", e),
    }

    tick("A");
}

fn tick(tag: &str) -> ! {
    let mut i: usize = 0;
    loop {
        info!("{}: tick {}", tag, i);
        i = i.wrapping_add(1);
        thread::yield_now();
        time::sleep_ms(1000);
    }
}

extern "C" fn thread_b() -> ! {
    let cpu = stem::arch::whoami();
    info!(
        "whoami: cs=0x{:x} ss=0x{:x} cpl={} rsp=0x{:x} rip=0x{:x} rflags=0x{:x}",
        cpu.cs, cpu.ss, cpu.cpl, cpu.rsp, cpu.rip, cpu.rflags
    );

    tick("B")
}
