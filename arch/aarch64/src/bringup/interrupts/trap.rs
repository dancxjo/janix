use core::arch::global_asm;
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::ToString;
use core::arch::asm;
use kernel::bridge::HardwareBridge; // For logging

global_asm!(include_str!("trap.S"));

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

// Temporary logging helper
macro_rules! log {
    ($($arg:tt)*) => ({
        let s = alloc::format!($($arg)*);
        crate::Bridge.log(&s);
        crate::Bridge.log("\n");
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn syscall_handler_rust(tf: &mut TrapFrame) -> u64 {
    let esr: u64;
    unsafe {
        core::arch::asm!("mrs {}, esr_el1", out(reg) esr);
    }
    let ec = (esr >> 26) & 0x3F;

    // EC 0x15 = SVC in AArch64
    if ec == 0x15 {
        // x8 is syscall number in AArch64 ABI
        let num = tf.x8 as usize;
        let a1 = tf.x0 as usize;
        let a2 = tf.x1 as usize;
        let a3 = tf.x2 as usize;
        let a4 = tf.x3 as usize;
        let a5 = tf.x4 as usize;
        let a6 = tf.x5 as usize;

        unsafe {
            if let Some(hook) = super::syscall::SYSCALL_HOOK {
                hook(num, a1, a2, a3, a4, a5, a6) as u64
            } else {
                // -1 (isize) cast to u64 is usually all 1s
                (!0u64)
            }
        }
    } else {
        // Not a syscall (e.g. Data Abort, Instruction Abort)

        let far: u64;
        unsafe {
            asm!("mrs {}, far_el1", out(reg) far);
        }

        // EC 0x20 = I-Abort Lower EL, 0x21 = I-Abort Curr EL
        // EC 0x24 = D-Abort Lower EL, 0x25 = D-Abort Curr EL
        if ec == 0x20 || ec == 0x24 {
            unsafe {
                if let Some(hook) = crate::PAGE_FAULT_HOOK {
                    hook(tf, far, esr);
                    // If hook returns, we resume
                    return 0;
                }
            }
        }

        // Delegate to invalid_exception(tf, kind=2 (Lower EL), source=0 (Sync))
        invalid_exception(tf, 2, 0);
        0
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
    // Simplified logging to avoid alloc/format panic loops
    unsafe {
        crate::Bridge.log("EXCEPTION: AArch64 Trap (Raw)\n");
    }
    // log!("Kind: {}, Source: {}", kind, source);
    // log!("ESR: {:#x}, FAR: {:#x}", esr, far);
    // log!("{:#?}", tf);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn irq_handler(tf: &mut TrapFrame) {
    let id = unsafe { super::gic::acknowledge_irq() };

    // Spurious check (1023)
    if id >= 1020 {
        return;
    }

    if id == 30 {
        // Timer
        crate::timer::next_match();

        unsafe {
            if let Some(hook) = crate::TICK_HOOK {
                hook(tf);
            }
        }
    } else {
        log!("IRQ: {}", id as u64);
    }

    unsafe { super::gic::end_of_irq(id) };
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

pub unsafe fn jump_to_el1_stack(stack_top: u64, entry: unsafe extern "C" fn() -> !) -> ! {
    let stack_top = stack_top & !0xf;
    unsafe {
        core::arch::asm!(
            "msr spsel, #1",
            "mov sp, {stack}",
            "mov x29, xzr",
            "mov x30, xzr",
            "isb",
            "br {entry}",
            "b .",
            stack = in(reg) stack_top,
            entry = in(reg) entry,
            options(noreturn)
        );
    }
}

// NOTE: We might need to expose this for scheduler later
fn save_current_thread_context(_tf: &TrapFrame) {
    /*
    let mut sched = kernel::sched::SCHEDULER.lock();
    if let Some(tid) = sched.current_id() {
        if let Some(thread) = sched.thread_mut(tid) {
            let regs_ptr = tf as *const TrapFrame as *const u64;
            let regs_slice = unsafe { core::slice::from_raw_parts(regs_ptr, 34) };
            thread.context.copy_from_slice(regs_slice);
            thread.started = true;
        }
    }
    */
}

fn leak_user_str(ptr: u64, len: usize) -> Option<&'static str> {
    if len == 0 {
        return Some("");
    }
    let bytes = unsafe { user_slice(ptr as *const u8, len) };
    core::str::from_utf8(bytes).ok().map(|s| {
        let leaked: &'static mut str = Box::leak(s.to_string().into_boxed_str());
        leaked as &'static str
    })
}

unsafe fn user_slice<'a, T>(ptr: *const T, len: usize) -> &'a [T] {
    if len == 0 || ptr.is_null() {
        return &[];
    }
    let align = core::mem::align_of::<T>();
    if align > 1 && (ptr as usize) % align != 0 {
        &[]
    } else {
        core::slice::from_raw_parts(ptr, len)
    }
}
