use crate::user;
use abi::{KernelRequest, KernelResponse, SyscallNumber};
use core::arch::global_asm;
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::ToString;

global_asm!(include_str!("trap.S"));

#[unsafe(no_mangle)]
pub extern "C" fn syscall_handler_rust(tf: &mut TrapFrame) -> u64 {
    let esr: u64;
    unsafe { core::arch::asm!("mrs {}, esr_el1", out(reg) esr) };
    let ec = (esr >> 26) & 0x3F;

    if ec != 0x15 {
        kernel_core::println!("EXCEPTION: AArch64 Trap (Not SVC)");
        kernel_core::println!("ESR: {:#x}", esr);
        kernel_core::println!("{:#?}", tf);
        loop {}
    }

    let num = tf.x8;
    let arg1 = tf.x0;
    let arg2 = tf.x1;
    let arg3 = tf.x2;
    let arg4 = tf.x3;
    let arg5 = tf.x4;
    let arg6 = tf.x5;

    if num == SyscallNumber::Yield as u64 {
        {
            let mut sched = kernel_core::sched::SCHEDULER.lock();
            if let Some(tid) = sched.current_id() {
                if let Some(thread) = sched.thread_mut(tid) {
                    let regs_ptr = tf as *const TrapFrame as *const u64;
                    let regs_slice = unsafe { core::slice::from_raw_parts(regs_ptr, 34) };
                    thread.context.copy_from_slice(regs_slice);
                    thread.started = true;
                }
            }
        }
        kernel_core::sched::yield_current_thread();
        user::schedule_next();
        0
    } else if num == SyscallNumber::Log as u64 {
        let ptr = arg1 as *const u8;
        let len = arg2 as usize;
        if let Ok(s) = unsafe { core::str::from_utf8(core::slice::from_raw_parts(ptr, len)) } {
            let leaked: &'static str = Box::leak(s.to_string().into_boxed_str());
            kernel_core::log(leaked);
        }
        0
    } else if num == SyscallNumber::ExitThread as u64 {
        kernel_core::log("Thread exited via syscall");
        kernel_core::sched::exit_current_thread();
        user::schedule_next();
        0
    } else if num == SyscallNumber::AllocFrame as u64 {
        let pool_index = arg1;
        let frame_info_ptr = arg2 as *mut abi::FrameInfo;

        let req = KernelRequest::AllocFrame {
            pool_index: pool_index,
        };
        match kernel_core::handle_request(req) {
            KernelResponse::FrameAllocated { frame } => {
                unsafe { *frame_info_ptr = frame };
                0
            }
            _ => 1,
        }
    } else if num == SyscallNumber::FreeFrame as u64 {
        let frame_id = abi::FrameId(arg1);
        let req = KernelRequest::FreeFrame { frame_id };
        match kernel_core::handle_request(req) {
            KernelResponse::FrameFreed { .. } => 0,
            _ => 1,
        }
    } else if num == SyscallNumber::SpawnProgram as u64 {
        kernel_core::log("SpawnProgram syscall not implemented for aarch64");
        1
    } else {
        kernel_core::log("Unknown syscall");
        1
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn invalid_exception(tf: &TrapFrame, kind: usize, source: usize) {
    let esr: u64;
    let far: u64;
    unsafe {
        core::arch::asm!("mrs {}, esr_el1", out(reg) esr);
        core::arch::asm!("mrs {}, far_el1", out(reg) far);
    }
    kernel_core::println!("EXCEPTION: AArch64 Trap");
    kernel_core::println!("Kind: {}, Source: {}", kind, source);
    kernel_core::println!("ESR: {:#x}, FAR: {:#x}", esr, far);
    kernel_core::println!("{:#?}", tf);
    loop {}
}

#[repr(C)]
#[derive(Debug)]
pub struct TrapFrame {
    pub x0: u64,
    pub x1: u64,
    pub x2: u64,
    pub x3: u64,
    pub x4: u64,
    pub x5: u64,
    pub x6: u64,
    pub x7: u64,
    pub x8: u64,
    pub x9: u64,
    pub x10: u64,
    pub x11: u64,
    pub x12: u64,
    pub x13: u64,
    pub x14: u64,
    pub x15: u64,
    pub x16: u64,
    pub x17: u64,
    pub x18: u64,
    pub x19: u64,
    pub x20: u64,
    pub x21: u64,
    pub x22: u64,
    pub x23: u64,
    pub x24: u64,
    pub x25: u64,
    pub x26: u64,
    pub x27: u64,
    pub x28: u64,
    pub x29: u64,
    pub x30: u64,
    pub sp_el0: u64,
    pub elr: u64,
    pub spsr: u64,
}

pub fn init() {
    unsafe extern "C" {
        static exception_vector_table: u8;
    }
    unsafe {
        core::arch::asm!(
            "msr vbar_el1, {}",
            in(reg) &exception_vector_table,
        );
    }
}

/// Switches to a dedicated EL1 kernel stack and jumps to the given entry point.
/// This is necessary because we cannot return to the caller after switching stacks
/// (the return address would be on the old stack).
pub unsafe fn jump_to_el1_stack(stack_top: u64, entry: unsafe extern "C" fn() -> !) -> ! {
    // Ensure stack is 16-byte aligned
    let stack_top = stack_top & !0xf;

    // kernel_core::println!("Switching to SP_EL1. Stack: {:#x}, Entry: {:#x}", stack_top, entry as usize);
    unsafe {
        core::arch::asm!(
            "msr spsel, #1",
            "mov sp, {stack}",
            "mov x29, xzr", // Clear FP
            "mov x30, xzr", // Clear LR
            "isb",
            "br {entry}",
            "b .",
            stack = in(reg) stack_top,
            entry = in(reg) entry,
            options(noreturn)
        );
    }
}
