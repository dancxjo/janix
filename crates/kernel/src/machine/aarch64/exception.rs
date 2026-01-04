use super::{gic, timer};
use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExceptionContext {
    pub x: [u64; 30],
    pub lr: u64,
    pub sp: u64,
    pub elr_el1: u64,
    pub spsr_el1: u64,
    pub esr_el1: u64,
    pub far_el1: u64,
}

pub static IRQ_COUNT: AtomicU64 = AtomicU64::new(0);

#[no_mangle]
pub extern "C" fn aarch64_handle_exception(ctx: &mut ExceptionContext, vector: u64) -> u64 {
    match vector {
        5 | 9 => {
            // IRQ from EL1 or EL0
            IRQ_COUNT.fetch_add(1, Ordering::Relaxed);
            let id = unsafe { gic::ack_irq() };
            if id == timer::TIMER_IRQ {
                let new_sp = crate::sched::tick(ctx as *mut _ as u64);
                unsafe { timer::ack() };
                return new_sp;
            } else if id < 1022 {
                // Handle other IRQs if needed
                unsafe { gic::eoi(id) };
            }
            0
        }
        8 => {
            // Sync from EL0 (Syscall)
            let ec = (ctx.esr_el1 >> 26) & 0x3F;
            if ec == 0x15 {
                // SVC
                let res = crate::syscall::dispatch(
                    ctx.x[8] as u32, // nr in x8
                    ctx.x[0],
                    ctx.x[1],
                    ctx.x[2],
                    ctx.x[3],
                    ctx.x[4],
                    ctx.x[5],
                );
                ctx.x[0] = res.status;
                ctx.x[1] = res.val0;
                ctx.x[2] = res.val1;
                return 0;
            }

            // Not a syscall, fall through to panic
            panic_exception(ctx, vector);
        }
        _ => {
            panic_exception(ctx, vector);
        }
    }
}

fn panic_exception(ctx: &ExceptionContext, vector: u64) -> ! {
    unsafe {
        crate::serial::write(b"AArch64 FATAL EXCEPTION: ");
        crate::serial::write_hex(vector);
        crate::serial::write(b"\nELR=");
        crate::serial::write_hex(ctx.elr_el1);
        crate::serial::write(b" ESR=");
        crate::serial::write_hex(ctx.esr_el1);
        crate::serial::write(b" FAR=");
        crate::serial::write_hex(ctx.far_el1);
        crate::serial::write(b"\n");
    }
    panic!("AArch64 Exception");
}
