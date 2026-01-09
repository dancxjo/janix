//! aarch64 architecture-specific implementation.

use core::arch::asm;
use kernel::BootRuntime;
use kernel::time::MonotonicClamp;

mod simd;

/// The BootRuntime implementation for aarch64.
pub struct Runtime {
    serial: SerialPort,
}

impl Runtime {
    pub const fn new() -> Self {
        Self {
            serial: SerialPort::new(),
        }
    }
}

impl BootRuntime for Runtime {
    fn putchar(&self, c: u8) {
        self.serial.putchar(c);
    }

    fn halt(&self) -> ! {
        hcf()
    }

    fn mono_ticks(&self) -> u64 {
        let raw = read_cntvct_el0();
        self.serial.clamp.clamp(raw)
    }

    fn mono_freq_hz(&self) -> u64 {
        read_cntfrq_el0()
    }

    fn simd_init_cpu(&self) {
        simd::init_cpu();
    }

    fn simd_state_layout(&self) -> (usize, usize) {
        simd::STATE_LAYOUT
    }

    unsafe fn simd_save(&self, dst: *mut u8) {
        unsafe { simd::save(dst) };
    }

    unsafe fn simd_restore(&self, src: *const u8) {
        unsafe { simd::restore(src) };
    }

    // Stubs
    fn phys_memory_map(&self) -> &'static [kernel::PhysRange] { &[] }
    fn modules(&self) -> &'static [kernel::BootModuleDesc] { &[] }
    fn page_size(&self) -> usize { 4096 }
    fn kernel_virt_base(&self) -> u64 { 0xffff_0000_0000_0000 }
    fn phys_to_virt_offset(&self) -> u64 { 0 }
    fn framebuffer(&self) -> Option<kernel::FramebufferInfo> { None }
    fn cpu_count(&self) -> usize { 1 }
    fn boot_cpu_id(&self) -> usize { 0 }
}

/// Serial port implementation for aarch64 using Semihosting.
/// (PL011 MMIO requires identity mapping of 0x09000000 which may be missing)
pub struct SerialPort {
    pub clamp: MonotonicClamp,
}

impl SerialPort {
    pub const fn new() -> Self {
        Self {
            clamp: MonotonicClamp::new(),
        }
    }

    fn putchar(&self, c: u8) {
        let ch = c;
        unsafe {
            // Semihosting call: SYS_WRITEC (0x03)
            // W0 = Operation 0x03
            // X1 = Pointer to character
            asm!(
                "hlt #0xF000",
                in("w0") 0x03,
                in("x1") &ch,
                options(nostack, preserves_flags)
            );
        }
    }
}

/// Halt and catch fire - enters an infinite wait-for-interrupt loop.
pub fn hcf() -> ! {
    loop {
        unsafe { asm!("wfi") };
    }
}

/// Read the virtual counter frequency (CNTFRQ_EL0)
#[inline]
fn read_cntfrq_el0() -> u64 {
    let val: u64;
    unsafe {
        asm!("mrs {}, cntfrq_el0", out(reg) val, options(nomem, nostack));
    }
    val
}

/// Read the virtual counter count (CNTVCT_EL0)
#[inline]
fn read_cntvct_el0() -> u64 {
    let val: u64;
    unsafe {
        asm!("mrs {}, cntvct_el0", out(reg) val, options(nomem, nostack));
    }
    val
}
