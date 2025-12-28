use core::arch::global_asm;
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::ToString;
use hw::HardwareBridge; // For logging

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
    unsafe { core::arch::asm!("mrs {}, esr_el1", out(reg) esr) };
    let ec = (esr >> 26) & 0x3F;

    if ec != 0x15 {
        let far: u64;
        unsafe { core::arch::asm!("mrs {}, far_el1", out(reg) far) };
        log!("EXCEPTION: AArch64 Trap (Not SVC)");
        log!("ESR: {:#x}", esr);
        log!("FAR: {:#x}", far);
        log!("{:#?}", tf);
        loop {}
    }

    // Stub implementation due to ABI drift
    log!("AArch64 Syscall handler not implemented yet due to ABI drift.");
    log!("Syscall number: {}", tf.x8);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn invalid_exception(tf: &TrapFrame, kind: usize, source: usize) {
    let esr: u64;
    let far: u64;
    unsafe {
        core::arch::asm!("mrs {}, esr_el1", out(reg) esr);
        core::arch::asm!("mrs {}, far_el1", out(reg) far);
    }
    log!("EXCEPTION: AArch64 Trap");
    log!("Kind: {}, Source: {}", kind, source);
    log!("ESR: {:#x}, FAR: {:#x}", esr, far);
    log!("{:#?}", tf);
    loop {}
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
fn save_current_thread_context(tf: &TrapFrame) {
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
