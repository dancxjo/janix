use core::arch::asm;
use core::sync::atomic::{AtomicU64, Ordering};
use kernel::sched::TICKS;
use kernel::time::HardwareTimer;

// Default to 2GHz if calibration fails or before calibration
static TSC_FREQUENCY: AtomicU64 = AtomicU64::new(2_000_000_000);

pub struct X86HardwareTimer;

impl HardwareTimer for X86HardwareTimer {
    fn init(&self) {
        // Calibrate TSC using PIT-driven TICKS
        // We assume interrupts are enabled and PIT is firing at 1kHz.

        let start_tick = TICKS.load(Ordering::Relaxed);

        // Wait for the start of a new tick to align
        // Add a safety limit to avoid infinite loop if interrupts are broken
        let mut loop_limit = 10_000_000;
        let mut tick_started = false;
        while loop_limit > 0 {
            if TICKS.load(Ordering::Relaxed) != start_tick {
                tick_started = true;
                break;
            }
            core::hint::spin_loop();
            loop_limit -= 1;
        }

        if !tick_started {
            kernel::println!("TSC Calibration failed: TICKS not incrementing. Using default 2GHz.");
        } else {
            let tsc_start = rdtsc();
            let start_tick_aligned = TICKS.load(Ordering::Relaxed);

            // Wait for 50 ticks (50ms)
            let calibration_ticks = 50;
            let target_tick = start_tick_aligned + calibration_ticks;

            // No strict loop limit here as 50ms takes time.
            // We rely on interrupts working (verified by first loop).
            while TICKS.load(Ordering::Relaxed) < target_tick {
                core::hint::spin_loop();
            }

            let tsc_end = rdtsc();

            let delta_tsc = tsc_end - tsc_start;
            // Frequency = (delta_tsc * 1000) / calibration_ticks
            let frequency = (delta_tsc * 1000) / calibration_ticks;

            TSC_FREQUENCY.store(frequency, Ordering::Relaxed);
            kernel::println!("TSC Calibrated: {} Hz", frequency);
        }

        // Initialize LAPIC Timer
        super::apic::init();
        super::apic::set_timer_vector(super::trap::LAPIC_TIMER_VECTOR as u8);
    }

    fn now_ns(&self) -> u64 {
        let tsc = rdtsc();
        let freq = TSC_FREQUENCY.load(Ordering::Relaxed);
        if freq == 0 {
            return 0; // Should not happen with default
        }
        // ns = cycles * 1_000_000_000 / freq
        ((tsc as u128 * 1_000_000_000) / freq as u128) as u64
    }

    fn set_deadline_ns(&self, deadline_ns: u64) {
        let freq = TSC_FREQUENCY.load(Ordering::Relaxed);
        if freq == 0 {
            return;
        }

        // deadline_tsc = deadline_ns * freq / 1_000_000_000
        let deadline_tsc = (deadline_ns as u128 * freq as u128) / 1_000_000_000;
        super::apic::set_deadline_tsc(deadline_tsc as u64);
    }
}

#[inline(always)]
fn rdtsc() -> u64 {
    let tsc: u64;
    unsafe {
        asm!(
            "rdtsc",
            "shl rdx, 32",
            "or rax, rdx",
            out("rax") tsc,
            out("rdx") _,
            options(nomem, nostack)
        );
    }
    tsc
}

pub fn init_arch_timer() {
    static TIMER: X86HardwareTimer = X86HardwareTimer;
    kernel::time::register_timer(&TIMER);
    TIMER.init();
}
