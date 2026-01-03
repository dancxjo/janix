#![no_std]
#![cfg_attr(target_arch = "x86_64", feature(abi_x86_interrupt))]
#![feature(alloc_error_handler)]
#![feature(naked_functions)]

extern crate alloc;

pub mod boot;
pub mod bytespace;
// pub mod display;
pub mod log;
pub mod machine;
pub mod memory;
pub mod platform;
pub mod proc;
pub mod sched;
pub mod serial;
pub mod syscall;
pub mod watch;

pub use machine::PreBootInfo;

/// Global panic handler
struct SerialWriter;

impl core::fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        crate::serial::write(s.as_bytes());
        Ok(())
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;
    let mut writer = SerialWriter;
    
    let _ = writer.write_str("\nKERNEL PANIC:\n");
    let _ = write!(&mut writer, "{}\n", info);
    let _ = writer.write_str("Halting.\n");
    
    // Attempt backtrace?
    loop {
        unsafe { crate::machine::machine().irq_disable() };
        crate::machine::idle();
    }
}

// alloc_error_handler in memory/allocator.rs
