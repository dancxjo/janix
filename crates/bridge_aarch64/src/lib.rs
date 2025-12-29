#![no_std]
#![allow(unused)]
#![allow(clippy::missing_safety_doc)]

extern crate alloc;

use kernel::bridge::HardwareBridge;

pub mod interrupts;
pub mod paging;
pub mod user;

mod context;
pub use context::ArchContext;

#[cfg(target_arch = "aarch64")]
mod cpu;
#[cfg(target_arch = "aarch64")]
mod fpu;
#[cfg(target_arch = "aarch64")]
mod ports;
#[cfg(target_arch = "aarch64")]
mod rtc;
#[cfg(target_arch = "aarch64")]
mod uart;
#[cfg(target_arch = "aarch64")]
pub use uart::set_uart_base;

pub struct Bridge;

#[cfg(target_arch = "aarch64")]
impl Bridge {
    pub unsafe fn init(hhdm: u64) {
        uart::init(hhdm);
        interrupts::trap::init();
    }
}

#[cfg(not(target_arch = "aarch64"))]
impl Bridge {
    pub unsafe fn init(_hhdm: u64) {}
}

#[cfg(target_arch = "aarch64")]
impl HardwareBridge for Bridge {
    type Context = ArchContext;

    fn log(&self, msg: &str) {
        uart::log(msg);
    }

    fn ticks(&self) -> u64 {
        cpu::ticks()
    }

    fn idle(&self) {
        cpu::idle();
    }

    fn shutdown(&self) -> ! {
        cpu::shutdown()
    }

    fn irq_disable(&self) {
        cpu::irq_disable();
    }

    fn irq_enable(&self) {
        cpu::irq_enable();
    }

    fn system_now(&self) -> u64 {
        self.ticks()
    }

    fn monotonic_now(&self) -> u64 {
        cpu::monotonic_now()
    }

    fn init_thread_context(&self, entry: u64, stack: u64, arg: u64) -> Self::Context {
        context::init_thread_context(entry, stack, arg)
    }

    fn resume_user_mode(&self, context: &Self::Context) -> ! {
        context::resume_user_mode(context)
    }

    fn set_kernel_stack(&self, stack: u64) {
        context::set_kernel_stack(stack);
    }

    fn rtc_read(&self, out: &mut abi::wire::time::RtcSample) {
        rtc::rtc_read(out);
    }

    fn save_fpu(&self, area: &mut [u8; 512]) {
        fpu::save(area);
    }

    fn restore_fpu(&self, area: &[u8; 512]) {
        fpu::restore(area);
    }

    fn hhdm_offset(&self) -> u64 {
        uart::hhdm_offset()
    }

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

#[cfg(not(target_arch = "aarch64"))]
impl HardwareBridge for Bridge {
    type Context = ArchContext;

    fn log(&self, _msg: &str) {}
    fn ticks(&self) -> u64 {
        0
    }
    fn idle(&self) {}
    fn shutdown(&self) -> ! {
        #[allow(clippy::empty_loop)]
        loop {}
    }
    fn irq_disable(&self) {}
    fn irq_enable(&self) {}
    fn system_now(&self) -> u64 {
        0
    }
    fn monotonic_now(&self) -> u64 {
        0
    }
    fn init_thread_context(&self, _entry: u64, _stack: u64, _arg: u64) -> Self::Context {
        ArchContext([0; 34])
    }
    fn resume_user_mode(&self, _context: &Self::Context) -> ! {
        loop {}
    }
    fn set_kernel_stack(&self, _stack: u64) {}
    fn rtc_read(&self, _out: &mut abi::wire::time::RtcSample) {}

    fn hhdm_offset(&self) -> u64 { 0 }
    fn port_outb(&self, _port: u16, _val: u8) {}
    fn port_inb(&self, _port: u16) -> u8 { 0 }
    fn port_outw(&self, _port: u16, _val: u16) {}
    fn port_inw(&self, _port: u16) -> u16 { 0 }
    fn port_outd(&self, _port: u16, _val: u32) {}
    fn port_ind(&self, _port: u16) -> u32 { 0 }
    fn save_fpu(&self, _area: &mut [u8; 512]) {}
    fn restore_fpu(&self, _area: &[u8; 512]) {}
}
