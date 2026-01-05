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
            IRQ_COUNT.fetch_add(1, Ordering::Relaxed);
            let id = unsafe { gic::ack_irq() };
            if id == timer::TIMER_IRQ {
                let new_sp = crate::sched::tick(ctx as *mut _ as u64);
                unsafe { timer::ack() };
                return new_sp;
            } else if id == 32 {
                // xHCI Interrupt
                unsafe {
                    super::usb::xhci_irq_handler();
                    gic::eoi(id);
                }
            } else if id < 1022 {
                unsafe { gic::eoi(id) };
            }
            0
        }
        8 => {
            let ec = (ctx.esr_el1 >> 26) & 0x3F;
            if ec == 0x15 {
                let res = crate::syscall::dispatch::dispatch(
                    ctx.x[8] as u32,
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
            panic_exception(ctx, vector);
        }
        _ => {
            panic_exception(ctx, vector);
        }
    }
}

fn decode_ec(ec: u64) -> &'static str {
    match ec {
        0x00 => "Unknown reason",
        0x07 => "SVE/SIMD/FP trap",
        0x0E => "Illegal Execution state",
        0x15 => "SVC from AArch64",
        0x20 => "Instruction Abort (lower EL)",
        0x21 => "Instruction Abort (same EL)",
        0x22 => "PC alignment fault",
        0x24 => "Data Abort (lower EL)",
        0x25 => "Data Abort (same EL)",
        0x26 => "SP alignment fault",
        0x2C => "FP exception",
        0x2F => "SError interrupt",
        0x30 => "Breakpoint (lower EL)",
        0x31 => "Breakpoint (same EL)",
        0x3C => "BRK (AArch64)",
        _ => "Reserved/Unknown",
    }
}

fn decode_dfsc(dfsc: u64) -> &'static str {
    match dfsc & 0x3F {
        0x04 => "Translation fault, level 0",
        0x05 => "Translation fault, level 1",
        0x06 => "Translation fault, level 2",
        0x07 => "Translation fault, level 3",
        0x09 => "Access flag fault, level 1",
        0x0A => "Access flag fault, level 2",
        0x0B => "Access flag fault, level 3",
        0x0D => "Permission fault, level 1",
        0x0E => "Permission fault, level 2",
        0x0F => "Permission fault, level 3",
        0x10 => "Synchronous External abort",
        0x21 => "Alignment fault",
        _ => "Unknown fault",
    }
}

fn vector_name(v: u64) -> &'static str {
    match v {
        4 => "Sync EL1h",
        5 => "IRQ EL1h",
        7 => "SError EL1h",
        8 => "Sync EL0 (64-bit)",
        9 => "IRQ EL0 (64-bit)",
        11 => "SError EL0 (64-bit)",
        _ => "Unknown",
    }
}

fn panic_exception(ctx: &ExceptionContext, vector: u64) -> ! {
    use crate::serial::{write, write_hex};

    write(b"\n\n========== AArch64 FATAL EXCEPTION ==========\n");
    write(b"Vector: ");
    write_hex(vector);
    write(b" (");
    write(vector_name(vector).as_bytes());
    write(b")\n");

    let ec = (ctx.esr_el1 >> 26) & 0x3F;
    let iss = ctx.esr_el1 & 0x1FFFFFF;

    write(b"ESR_EL1: ");
    write_hex(ctx.esr_el1);
    write(b"\n");
    write(b"  EC: ");
    write_hex(ec);
    write(b" -> ");
    write(decode_ec(ec).as_bytes());
    write(b"\n");
    write(b"  ISS: ");
    write_hex(iss);
    write(b"\n");

    if ec == 0x20 || ec == 0x21 || ec == 0x24 || ec == 0x25 {
        let dfsc = iss & 0x3F;
        let wnr = (iss >> 6) & 1;
        write(b"  DFSC: ");
        write_hex(dfsc);
        write(b" -> ");
        write(decode_dfsc(dfsc).as_bytes());
        write(b"\n");
        if ec == 0x24 || ec == 0x25 {
            write(b"  Access: ");
            if wnr != 0 {
                write(b"WRITE\n");
            } else {
                write(b"READ\n");
            }
        }
    }

    write(b"\nELR_EL1: ");
    write_hex(ctx.elr_el1);
    write(b"\nFAR_EL1: ");
    write_hex(ctx.far_el1);
    write(b"\nSPSR: ");
    write_hex(ctx.spsr_el1);
    write(b"\n");

    write(b"\n--- Registers ---\n");
    for i in 0..30 {
        write(b"x");
        if i < 10 {
            write(&[b'0' + i as u8]);
        } else {
            write(&[b'0' + (i / 10) as u8, b'0' + (i % 10) as u8]);
        }
        write(b"=");
        write_hex(ctx.x[i as usize]);
        if (i + 1) % 4 == 0 {
            write(b"\n");
        } else {
            write(b" ");
        }
    }
    write(b"\nLR=");
    write_hex(ctx.lr);
    write(b" SP=");
    write_hex(ctx.sp);
    write(b"\n");
    write(b"==============================================\n");

    panic!("AArch64 Exception");
}
