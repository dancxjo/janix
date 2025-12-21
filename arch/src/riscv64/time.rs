use core::arch::asm;
use core::sync::atomic::{AtomicU64, Ordering};
use kernel::time::HardwareTimer;

// Default to 10MHz (QEMU virt)
// This can be updated by boot code if FDT is parsed.
pub static FREQUENCY: AtomicU64 = AtomicU64::new(10_000_000);

pub unsafe fn init_frequency_from_dtb(dtb_ptr: *const u8) {
    let fdt = match unsafe { fdt::Fdt::from_ptr(dtb_ptr) } {
        Ok(f) => f,
        Err(_e) => {
             return;
        }
    };

    let mut found = false;
    for cpu in fdt.cpus() {
        let freq = cpu.timebase_frequency();
        if freq > 0 {
            let freq = freq as u64;
            // Set the frequency
            FREQUENCY.store(freq, Ordering::Relaxed);
            found = true;
            break; // Assuming all CPUs have same timebase
        }
    }

    if !found {
        if let Some(cpus_node) = fdt.find_node("/cpus") {
             if let Some(prop) = cpus_node.property("timebase-frequency") {
                 let freq = prop.as_usize().unwrap_or(0) as u64;
                 if freq > 0 {
                     FREQUENCY.store(freq, Ordering::Relaxed);
                 }
             }
        }
    }
}

pub struct RiscvHardwareTimer;

impl HardwareTimer for RiscvHardwareTimer {
    fn init(&self) {
        // Enable Supervisor Timer Interrupt (STIE) in sie CSR.
        // Bit 5 is STIE.
        unsafe {
            asm!(
                "csrs sie, {}",
                in(reg) 1 << 5,
                options(nostack)
            );
        }
    }

    fn now_ns(&self) -> u64 {
        let cycles: u64;
        unsafe {
            asm!("rdtime {}", out(reg) cycles, options(nomem, nostack));
        }

        let freq = FREQUENCY.load(Ordering::Relaxed);
        if freq == 0 {
             return 0;
        }
        // cycles * 1e9 / freq
        ((cycles as u128 * 1_000_000_000) / freq as u128) as u64
    }

    fn set_deadline_ns(&self, deadline_ns: u64) {
        let freq = FREQUENCY.load(Ordering::Relaxed);
        // deadline_ns * freq / 1e9
        let cycles = ((deadline_ns as u128 * freq as u128) / 1_000_000_000) as u64;

        // Program the timer using SBI.
        super::sbi::set_timer(cycles);
    }
}

pub fn init_arch_timer() {
    static TIMER: RiscvHardwareTimer = RiscvHardwareTimer;
    kernel::time::register_timer(&TIMER);
    TIMER.init();
}
