#![no_std]
#![no_main]

use stem::info;
use stem::syscall::{self, sleep_ms};
use abi::trace::TraceEvent;

#[stem::main]
fn main() -> ! {
    let mut buf = [TraceEvent::Empty; 256];
    info!("irqdump: starting");
    
    let mut last_second = syscall::monotonic_ns();
    let mut timer_irqs = 0;
    let mut mouse_irqs = 0;
    let mut max_timer_gap = 0;
    let mut last_timer_ts = 0;

    loop {
        sleep_ms(100); 
        loop {
            match syscall::trace_read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                     for event in &buf[..n] {
                         match event {
                             TraceEvent::Irq { vector, timestamp } => {
                                 // Vector 32 = Timer (usually), Vector 44 (0x2C) = Mouse (IRQ12 + 32)
                                 // Vector 33 (0x21) = Kbd (IRQ1 + 32)
                                 // ThingOS is likely using standard IOAPIC/LAPIC mapping (IRQ0->Vec32).
                                 if *vector == 32 { 
                                     timer_irqs += 1; 
                                     if last_timer_ts != 0 {
                                         let gap = timestamp.saturating_sub(last_timer_ts);
                                         if gap > max_timer_gap { max_timer_gap = gap; }
                                     }
                                     last_timer_ts = *timestamp;
                                 }
                                 if *vector == 44 { mouse_irqs += 1; }
                             }
                             _ => {}
                         }
                     }
                     if n < buf.len() { break; }
                }
                Err(_) => break,
            }
        }
        
        let now = syscall::monotonic_ns();
        if now.saturating_sub(last_second) >= 1_000_000_000 {
            info!("irqdump: timer={} mouse={} max_gap_us={}", timer_irqs, mouse_irqs, max_timer_gap / 1000);
            timer_irqs = 0;
            mouse_irqs = 0;
            max_timer_gap = 0;
            last_second = now;
        }
    }
}
