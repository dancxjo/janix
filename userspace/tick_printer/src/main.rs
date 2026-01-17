#![no_std]
#![no_main]

use stem::info;
use stem::syscall::sleep_ms;

#[stem::main]
fn main() -> ! {
    let mut iter = 0;
    loop {
        let now = stem::syscall::monotonic_ns() / 1_000_000;
        info!("tick_printer: t={} iter={}", now, iter);
        iter += 1;
        sleep_ms(10);
    }
}
