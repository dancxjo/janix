use crate::bringup::{context, fpu, interrupts};
use core::arch::asm;
use kernel::bridge::{CpuBridge, MachineBridge, PortIo, Power, Rtc, VmMapper};

pub mod ports;
pub mod rtc;
pub mod timer;
pub mod uart;
pub mod user;

// Re-export specific used items
pub use uart::set_uart_base;

// Hook for scheduler. Only set by kernel binary.
pub static mut TICK_HOOK: Option<fn(&mut interrupts::trap::TrapFrame)> = None;
pub static mut PAGE_FAULT_HOOK: Option<fn(&mut interrupts::trap::TrapFrame, u64, u64)> = None;

pub fn set_tick_hook(hook: fn(&mut interrupts::trap::TrapFrame)) {
    unsafe {
        TICK_HOOK = Some(hook);
    }
}

pub fn set_page_fault_hook(hook: fn(&mut interrupts::trap::TrapFrame, u64, u64)) {
    unsafe {
        PAGE_FAULT_HOOK = Some(hook);
    }
}

pub struct Bridge;

/// aarch64 interrupt state: stores whether I-bit was clear (interrupts enabled)
#[derive(Copy, Clone, Debug)]
pub struct IrqState(pub bool);

impl Default for IrqState {
    fn default() -> Self {
        Self(true)
    }
}

/// aarch64 FPU/SIMD state: 512-byte area for 32 Q registers (128-bit each)
#[repr(C, align(16))]
#[derive(Copy, Clone, Debug)]
pub struct FpuState(pub [u8; 512]);

impl Default for FpuState {
    fn default() -> Self {
        Self([0u8; 512])
    }
}

#[cfg(target_arch = "aarch64")]
impl Bridge {
    pub unsafe fn init(hhdm: u64) {
        uart::init(hhdm);
        interrupts::trap::init();
    }

    pub unsafe fn init_platform(hhdm: u64) {
        interrupts::gic::init(hhdm);
        timer::init();
    }
}

#[cfg(not(target_arch = "aarch64"))]
impl Bridge {
    pub unsafe fn init(_hhdm: u64) {}
    pub unsafe fn init_platform(_hhdm: u64) {}
}

#[cfg(target_arch = "aarch64")]
impl CpuBridge for Bridge {
    const CONTEXT_WORDS: usize = 34;

    type Context = context::ArchContext;
    type IrqState = IrqState;
    type FpuState = FpuState;

    fn log(&self, msg: &str) {
        uart::log(msg);
    }

    fn ticks(&self) -> u64 {
        // Read CNTPCT_EL0 (physical counter)
        let cntpct: u64;
        unsafe {
            asm!("mrs {}, cntpct_el0", out(reg) cntpct);
        }
        cntpct
    }

    fn ticks_per_second(&self) -> u64 {
        let cntfrq: u64;
        unsafe {
            asm!("mrs {}, cntfrq_el0", out(reg) cntfrq);
        }
        cntfrq
    }

    fn idle(&self) {
        unsafe {
            asm!("wfi");
        }
    }

    fn irq_disable(&self) -> Self::IrqState {
        let daif: u64;
        unsafe {
            asm!("mrs {}, daif", out(reg) daif);
            asm!("msr daifset, #2"); // Set I bit (disable IRQ)
        }
        // I bit is bit 7 in DAIF; 0 = interrupts enabled
        IrqState((daif & (1 << 7)) == 0)
    }

    fn irq_restore(&self, state: Self::IrqState) {
        if state.0 {
            unsafe {
                asm!("msr daifclr, #2");
            } // Clear I bit (enable IRQ)
        }
        // If state.0 is false, leave interrupts disabled
    }

    fn init_thread_context(&self, entry: u64, stack: u64, arg: u64) -> Self::Context {
        context::init_thread_context(entry, stack, arg)
    }

    fn switch(&self, _from: &mut Self::Context, to: &Self::Context) {
        context::resume_user_mode(to)
    }

    fn set_kernel_stack(&self, stack: u64) {
        context::set_kernel_stack(stack);
    }

    fn save_fpu(&self, out: &mut Self::FpuState) {
        fpu::save(&mut out.0);
    }

    fn restore_fpu(&self, state: &Self::FpuState) {
        fpu::restore(&state.0);
    }
}

#[cfg(target_arch = "aarch64")]
impl MachineBridge for Bridge {
    fn hhdm_offset(&self) -> u64 {
        uart::hhdm_offset()
    }
}

#[cfg(target_arch = "aarch64")]
impl PortIo for Bridge {
    fn port_outb(&self, port: u16, val: u8) {
        ports::outb(port, val);
    }

    fn port_inb(&self, port: u16) -> u8 {
        ports::inb(port)
    }

    fn port_outw(&self, port: u16, val: u16) {
        ports::outw(port, val);
    }

    fn port_inw(&self, port: u16) -> u16 {
        ports::inw(port)
    }

    fn port_outd(&self, port: u16, val: u32) {
        ports::outd(port, val);
    }

    fn port_ind(&self, port: u16) -> u32 {
        ports::ind(port)
    }
}

#[cfg(target_arch = "aarch64")]
impl Rtc for Bridge {
    fn rtc_read(&self, out: &mut abi::wire::time::RtcSample) {
        rtc::rtc_read(out);
    }
}

#[cfg(target_arch = "aarch64")]
impl Power for Bridge {
    fn shutdown(&self) -> ! {
        loop {
            unsafe {
                asm!("wfi");
            }
        }
    }
}

#[cfg(target_arch = "aarch64")]
impl VmMapper for Bridge {
    fn map_new_user_page(&self, _virt_addr: u64, _flags: u64) -> Result<(), ()> {
        Err(())
    }

    fn map_user_mmio(&self, _virt_addr: u64, _phys_addr: u64, _flags: u64) -> Result<(), ()> {
        Err(())
    }
}

#[cfg(not(target_arch = "aarch64"))]
impl CpuBridge for Bridge {
    const CONTEXT_WORDS: usize = 34;

    type Context = context::ArchContext;
    type IrqState = IrqState;
    type FpuState = FpuState;

    fn log(&self, _msg: &str) {}
    fn ticks(&self) -> u64 {
        0
    }
    fn ticks_per_second(&self) -> u64 {
        0
    }
    fn idle(&self) {}
    fn irq_disable(&self) -> Self::IrqState {
        IrqState::default()
    }
    fn irq_restore(&self, _state: Self::IrqState) {}
    fn init_thread_context(&self, _entry: u64, _stack: u64, _arg: u64) -> Self::Context {
        context::ArchContext([0; 34])
    }
    fn switch(&self, _from: &mut Self::Context, _to: &Self::Context) {}
    fn set_kernel_stack(&self, _stack: u64) {}
    fn save_fpu(&self, _out: &mut Self::FpuState) {}
    fn restore_fpu(&self, _state: &Self::FpuState) {}
}

#[cfg(not(target_arch = "aarch64"))]
impl MachineBridge for Bridge {
    fn hhdm_offset(&self) -> u64 {
        0
    }
}

#[cfg(not(target_arch = "aarch64"))]
impl PortIo for Bridge {
    fn port_outb(&self, _port: u16, _val: u8) {}
    fn port_inb(&self, _port: u16) -> u8 {
        0
    }
    fn port_outw(&self, _port: u16, _val: u16) {}
    fn port_inw(&self, _port: u16) -> u16 {
        0
    }
    fn port_outd(&self, _port: u16, _val: u32) {}
    fn port_ind(&self, _port: u16) -> u32 {
        0
    }
}

#[cfg(not(target_arch = "aarch64"))]
impl Rtc for Bridge {
    fn rtc_read(&self, _out: &mut abi::wire::time::RtcSample) {}
}

#[cfg(not(target_arch = "aarch64"))]
impl Power for Bridge {
    fn shutdown(&self) -> ! {
        loop {}
    }
}

#[cfg(not(target_arch = "aarch64"))]
impl VmMapper for Bridge {
    fn map_new_user_page(&self, _virt_addr: u64, _flags: u64) -> Result<(), ()> {
        Err(())
    }
    fn map_user_mmio(&self, _virt_addr: u64, _phys_addr: u64, _flags: u64) -> Result<(), ()> {
        Err(())
    }
}
