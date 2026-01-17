#![no_std]
#![no_main]

use stem::info;

#[stem::main]
fn main() -> ! {
    let mut last_print = 0;
    let mut iter = 0;
    loop {
        let now = stem::syscall::monotonic_ns();
        if now.saturating_sub(last_print) >= 1_000_000_000 {
            info!("hogger alive: {}", iter);
            last_print = now;
        }
        iter += 1;
        core::hint::black_box(());
    }
}
