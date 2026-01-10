
use super::gdt;
use super::trap::TrapFrame;

pub unsafe fn enable(handler_entry: u64) {
    // Enable SCE (System Call Extensions) in EFER (0xC0000080)
    // Bit 0 = SCE
    unsafe {
        core::arch::asm!("rdmsr", in("ecx") 0xC0000080u32, out("eax") _, out("edx") _); 
    }

    let mut lo: u32;
    let mut hi: u32;
    unsafe {
        core::arch::asm!("rdmsr", in("ecx") 0xC0000080u32, out("eax") lo, out("edx") hi);
    }
    lo |= 1; // SCE
    unsafe {
        core::arch::asm!("wrmsr", in("ecx") 0xC0000080u32, in("eax") lo, in("edx") hi);
    }

    // LSTAR (0xC0000082) = Entry Point
    let handler_lo = handler_entry as u32;
    let handler_hi = (handler_entry >> 32) as u32;
    unsafe {
        core::arch::asm!("wrmsr", in("ecx") 0xC0000082u32, in("eax") handler_lo, in("edx") handler_hi);
    }

    // STAR (0xC0000081)
    let user_base_sel = gdt::USER_CODE32_IDX << 3; 
    let kernel_base_sel = gdt::KERNEL_CODE_SEL;
    
    let star_val: u64 = ((user_base_sel as u64) << 48) | ((kernel_base_sel as u64) << 32);
    let star_lo = star_val as u32;
    let star_hi = (star_val >> 32) as u32;
    
    unsafe {
        core::arch::asm!("wrmsr", in("ecx") 0xC0000081u32, in("eax") star_lo, in("edx") star_hi);
    }

    // FMASK (0xC0000084)
    let fmask: u64 = 0x200 | 0x400; // IF | DF
    let fmask_lo = fmask as u32;
    let fmask_hi = (fmask >> 32) as u32;
    unsafe {
        core::arch::asm!("wrmsr", in("ecx") 0xC0000084u32, in("eax") fmask_lo, in("edx") fmask_hi);
    }
}

// Trampoline called from syscall_entry.S
// Casts concrete TrapFrame to &mut dyn ArchTrapFrame and calls kernel
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_dispatch_trampoline(tf: &mut TrapFrame) {
    // We need to call kernel::syscall::syscall_dispatch(tf)
    // kernel::syscall_dispatch takes &mut dyn ArchTrapFrame
    kernel::syscall::syscall_dispatch(tf);
}
