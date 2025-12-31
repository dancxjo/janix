use crate::bringup::interrupts::trap::{self, TrapFrame};
use core::arch::naked_asm;
use kernel::bridge::CpuBridge;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use x86_64::VirtAddr;

// Debug counter for IRQ1
pub static IRQ1_COUNT: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

use core::sync::atomic::{AtomicU64, Ordering};
use lazy_static::lazy_static;

// Debug guard to log only a few kernel-mode timer interrupts
static TIMER_KERNEL_LOG: AtomicU64 = AtomicU64::new(0);

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.double_fault.set_handler_fn(double_fault_handler_naked);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.general_protection_fault.set_handler_fn(gp_handler_naked);
        idt.page_fault.set_handler_fn(page_fault_handler_naked);

        // Timer Interrupt (IRQ 0 = 32)
        unsafe {
            idt[32].set_handler_addr(VirtAddr::new(timer_interrupt_naked as *const () as u64));
            idt[33].set_handler_addr(VirtAddr::new(keyboard_interrupt_naked as *const () as u64));
            idt[44].set_handler_addr(VirtAddr::new(mouse_interrupt_naked as *const () as u64));
        }

        idt
    };
}

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    kernel::diag::record_fault(
        stack_frame.instruction_pointer.as_u64(),
        stack_frame.stack_pointer.as_u64(),
        stack_frame.cpu_flags.bits(),
        0,
        0,
        3, // Breakpoint trap #3
        "BREAKPOINT",
    );
}

#[no_mangle]
pub extern "C" fn double_fault_handler(frame: &mut TrapFrame, error_code: u64) -> ! {
    // Direct output to debugcon before anything else
    unsafe {
        let bridge = crate::Bridge;
        bridge.log("\n!!! DOUBLE FAULT !!!\n");
        bridge.log("RIP: ");
        crate::print_hex(&bridge, frame.rip);
        bridge.log(" RSP: ");
        crate::print_hex(&bridge, frame.rsp);
        bridge.log(" ERR: ");
        crate::print_hex(&bridge, error_code);
        bridge.log("\n");
    }

    kernel::diag::record_fault(
        frame.rip,
        frame.rsp,
        frame.rflags,
        0,
        error_code,
        8, // Double Fault #8
        "DOUBLE FAULT",
    );
    loop {}
}

#[no_mangle]
pub extern "C" fn gp_handler(frame: &mut TrapFrame, error_code: u64) {
    use x86_64::registers::control::Cr2;
    let cr2 = Cr2::read().unwrap_or(VirtAddr::zero()).as_u64();

    unsafe {
        // Raw 'G' to 0xE9
        core::arch::asm!(
            "out dx, al",
            in("dx") 0xe9u16,
            in("al") b'G',
            options(nomem, nostack, preserves_flags)
        );
    }

    kernel::diag::record_fault(
        frame.rip,
        frame.rsp,
        frame.rflags,
        cr2,
        error_code,
        13, // GPF #13
        "GENERAL PROTECTION FAULT",
    );
    panic!("GPF");
}

#[no_mangle]
pub extern "C" fn page_fault_handler(frame: &mut TrapFrame, error_code: PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;
    let cr2 = Cr2::read().unwrap_or(VirtAddr::zero()).as_u64();
    // Debug dump of fault frame to help root-cause early boot faults
    {
        let bridge = crate::Bridge;
        bridge.log("PAGE FAULT: rip=");
        crate::print_hex(&bridge, frame.rip);
        bridge.log(" cs=");
        crate::print_hex(&bridge, frame.cs);
        bridge.log(" rsp=");
        crate::print_hex(&bridge, frame.rsp);
        bridge.log(" ss=");
        crate::print_hex(&bridge, frame.ss);
        bridge.log(" err=");
        crate::print_hex(&bridge, error_code.bits() as u64);
        bridge.log(" cr2=");
        crate::print_hex(&bridge, cr2);
        // Peek a couple of user stack slots to see call chain
        let rsp_val = frame.rsp;
        if rsp_val != 0 {
            let _ptr = rsp_val as *const u64;
            // Safety: We are in the page fault handler. Peeking might cause another fault?
            // Usually fine if we are careful.
            // let slot0 = unsafe { core::ptr::read(ptr) };
            // bridge.log(" stack[0]=");
            // crate::print_hex(&bridge, slot0);
        }
        bridge.log("\n");
    }

    // Check hook first
    unsafe {
        if let Some(hook) = crate::bridge::PAGE_FAULT_HOOK {
            // Note: Hook takes &InterruptStackFrame. Passing a dummy for now.
            let dummy = core::mem::zeroed::<InterruptStackFrame>();
            if hook(&dummy, cr2, error_code) {
                return;
            }
        }
    }

    kernel::diag::record_fault(
        frame.rip,
        frame.rsp,
        frame.rflags,
        cr2,
        error_code.bits() as u64,
        14, // Page Fault #14
        "PAGE FAULT",
    );

    panic!("Page Fault");
}

/// Naked trampoline for Timer Interrupt.
/// 
/// CRITICAL: x86_64 hardware frame layout differs by privilege:
/// - User mode (CPL 3→0): SS, RSP, RFLAGS, CS, RIP (5 values)  
/// - Kernel mode (CPL 0→0): RFLAGS, CS, RIP only (3 values, NO RSP/SS!)
///
/// To enable context switching between kernel and user threads, we NORMALIZE
/// the stack to always have a 5-element frame on entry.
#[unsafe(naked)]
unsafe extern "C" fn timer_interrupt_naked() {
    naked_asm!(
        // DEBUG: Very first thing - output 'X' to debugcon
        "push rax",
        "push rdx",
        "mov al, 0x58", // 'X'
        "mov dx, 0xe9",
        "out dx, al",
        "pop rdx",
        "pop rax",
        
        // Check CPL from hardware frame CS (at [rsp + 8] for no-error interrupts)
        // If from user mode, swapgs and skip stack normalization
        "test byte ptr [rsp + 8], 3",
        "jnz 10f",
        
        // === KERNEL MODE INTERRUPT ===
        // CPU only pushed RIP, CS, RFLAGS (3 values at rsp+0, rsp+8, rsp+16)
        // We need to expand to 5-element frame: RIP, CS, RFLAGS, RSP, SS
        //
        // CRITICAL: We can ONLY use rax as scratch, and we must save it first!
        // We absolutely cannot touch ANY other register before saving the TrapFrame.
        
        // Save rax to stack
        "push rax",
        // Stack: [saved_rax] [RIP] [CS] [RFLAGS] ...
        //        rsp+0       rsp+8 rsp+16 rsp+24
        
        // Calculate what RSP was before the interrupt (for iretq)
        // Pre-interrupt RSP = current_rsp + 8(saved_rax) + 24(CPU frame) = rsp + 32
        // But we need 16 more bytes for RSP+SS slots
        // After expansion, we want RSP slot to hold: current_rsp + 8 + 24 + 16 = current_rsp + 48
        
        // Expand stack by 16 bytes
        "sub rsp, 16",
        // Stack: [??] [??] [saved_rax] [RIP] [CS] [RFLAGS] ...
        //        rsp+0 rsp+8 rsp+16    rsp+24 rsp+32 rsp+40
        
        // Move saved_rax down to rsp+0
        "mov rax, [rsp + 16]",
        "mov [rsp + 0], rax",
        
        // Move RIP down to rsp+8
        "mov rax, [rsp + 24]",
        "mov [rsp + 8], rax",
        
        // Move CS down to rsp+16
        "mov rax, [rsp + 32]",
        "mov [rsp + 16], rax",
        
        // Move RFLAGS down to rsp+24
        "mov rax, [rsp + 40]",
        "mov [rsp + 24], rax",
        
        // Synthesize RSP at rsp+32 (pre-interrupt RSP)
        // Original RSP = current_rsp + 16 (our gap) + 8 (saved_rax) + 24 (orig CPU frame) = current_rsp + 48
        "lea rax, [rsp + 48]",
        "mov [rsp + 32], rax",
        
        // Synthesize SS at rsp+40
        "mov rax, ss",
        "mov [rsp + 40], rax",
        
        // Restore original rax
        "mov rax, [rsp]",
        
        // Adjust stack to pop saved_rax placeholder
        "add rsp, 8",
        // Stack now: [RIP] [CS] [RFLAGS] [RSP] [SS] - normalized!
        //            rsp+0 rsp+8 rsp+16  rsp+24 rsp+32
        
        "jmp 11f",
        
        // === USER MODE INTERRUPT ===
        "10:",
        "swapgs",
        // Stack already has 5-element frame from CPU
        
        "11:",
        // === COMMON PATH: Build TrapFrame ===
        // Stack now has: [RIP] [CS] [RFLAGS] [RSP] [SS] for both cases
        
        // Save RAX then allocate TrapFrame
        "push rax",
        "sub rsp, 160",
        
        // Save GPRs r15..rdi into TrapFrame
        "mov [rsp + 0], r15",
        "mov [rsp + 8], r14",
        "mov [rsp + 16], r13",
        "mov [rsp + 24], r12",
        "mov [rsp + 32], rbp",
        "mov [rsp + 40], rbx",
        "mov [rsp + 48], r11",
        "mov [rsp + 56], r10",
        "mov [rsp + 64], r9",
        "mov [rsp + 72], r8",
        "mov [rsp + 80], rcx",
        "mov [rsp + 88], rdx",
        "mov [rsp + 96], rsi",
        "mov [rsp + 104], rdi",
        
        // Saved orig RAX is at rsp + 160
        "mov rax, [rsp + 160]",
        "mov [rsp + 112], rax",
        
        // Hardware frame: rip=+168, cs=+176, rflags=+184, rsp=+192, ss=+200
        "mov rax, [rsp + 168]",
        "mov [rsp + 120], rax", // rip
        "mov rax, [rsp + 176]",
        "mov [rsp + 128], rax", // cs
        "mov rax, [rsp + 184]",
        "mov [rsp + 136], rax", // rflags
        "mov rax, [rsp + 192]",
        "mov [rsp + 144], rax", // rsp
        "mov rax, [rsp + 200]",
        "mov [rsp + 152], rax", // ss
        
        // Call Handler
        "mov rdi, rsp",
        "call timer_interrupt_handler",
        
        // === RETURN PATH ===
        // Copy TrapFrame back to hardware frame
        "mov rax, [rsp + 120]",
        "mov [rsp + 168], rax", // RIP
        "mov rax, [rsp + 128]",
        "mov [rsp + 176], rax", // CS
        "mov rax, [rsp + 136]",
        "mov [rsp + 184], rax", // RFLAGS
        "mov rax, [rsp + 144]",
        "mov [rsp + 192], rax", // RSP
        "mov rax, [rsp + 152]",
        "mov [rsp + 200], rax", // SS
        
        // Restore GPRs
        "mov r15, [rsp + 0]",
        "mov r14, [rsp + 8]",
        "mov r13, [rsp + 16]",
        "mov r12, [rsp + 24]",
        "mov rbp, [rsp + 32]",
        "mov rbx, [rsp + 40]",
        "mov r11, [rsp + 48]",
        "mov r10, [rsp + 56]",
        "mov r9,  [rsp + 64]",
        "mov r8,  [rsp + 72]",
        "mov rcx, [rsp + 80]",
        "mov rdx, [rsp + 88]",
        "mov rsi, [rsp + 96]",
        "mov rdi, [rsp + 104]",
        "mov rax, [rsp + 112]",
        
        // Tear down TrapFrame and saved rax
        "add rsp, 160",
        "pop rax",
        
        // Now at the normalized 5-element hardware frame
        // Check the CS we are ABOUT TO return to (at rsp + 8)
        "test byte ptr [rsp + 8], 3",
        "jz 12f",
        
        // === RETURNING TO USER MODE ===
        // DEBUG: Output 'U' to debugcon
        "push rax",
        "push rdx",
        "mov al, 0x55", // 'U'
        "mov dx, 0xe9",
        "out dx, al",
        "pop rdx",
        "pop rax",
        
        // iretq will pop all 5 values: RIP, CS, RFLAGS, RSP, SS
        "swapgs",
        "iretq",
        
        // === RETURNING TO KERNEL MODE ===
        "12:",
        // DEBUG: Output 'K' to debugcon
        "push rax",
        "push rdx",
        "mov al, 0x4b", // 'K'
        "mov dx, 0xe9",
        "out dx, al",
        "pop rdx",
        "pop rax",
        
        // For kernel-to-kernel return, iretq only pops RIP/CS/RFLAGS (not RSP/SS).
        // We must manually set up the stack so RSP ends up at the correct location.
        //
        // Current layout: [RIP] [CS] [RFLAGS] [target_RSP] [target_SS]
        //                 rsp+0 rsp+8 rsp+16   rsp+24       rsp+32
        //
        // Strategy: 
        // 1. Save rax and r11 (they'll be our scratch registers)
        // 2. Copy RIP/CS/RFLAGS to just below target_RSP
        // 3. Copy saved rax and r11 to new stack so we can restore them
        // 4. Point RSP to new location, restore regs, iretq
        
        // Save scratch registers
        "push rax",
        "push r11",
        // Stack: [r11] [rax] [RIP] [CS] [RFLAGS] [target_RSP] [target_SS]
        //        rsp+0 rsp+8 rsp+16 rsp+24 rsp+32  rsp+40       rsp+48
        
        "mov rax, [rsp + 40]",  // target_RSP
        
        // Copy iret frame to just below target_RSP using r11 as scratch
        "mov r11, [rsp + 16]",  // RIP
        "mov [rax - 24], r11",
        "mov r11, [rsp + 24]",  // CS
        "mov [rax - 16], r11",
        "mov r11, [rsp + 32]",  // RFLAGS
        "mov [rax - 8], r11",
        
        // Copy saved r11 and rax to new stack (in reverse order for pop)
        "mov r11, [rsp + 8]",   // saved rax
        "mov [rax - 32], r11",  
        "mov r11, [rsp + 0]",   // saved r11
        "mov [rax - 40], r11",
        
        // Switch to new stack location
        "lea rsp, [rax - 40]",
        // New stack: [saved_r11] [saved_rax] [RIP] [CS] [RFLAGS]
        //            rsp+0       rsp+8       rsp+16 rsp+24 rsp+32
        
        // Restore r11 and rax, then iretq
        "pop r11",
        "pop rax",
        "iretq",
    );
}
#[unsafe(naked)]
unsafe extern "C" fn keyboard_interrupt_naked() {
    naked_asm!(
        "test byte ptr [rsp + 8], 3",
        "jz 1f",
        "swapgs",
        "1:",
        "push rax",
        "push rdi",
        "push rsi",
        "push rdx",
        "push rcx",
        "push r8",
        "push r9",
        "push r10",
        "push r11",
        "push rbx",
        "push rbp",
        "push r12",
        "push r13",
        "push r14",
        "push r15",
        "mov rdi, rsp",
        "call keyboard_interrupt_handler",
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop rbp",
        "pop rbx",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        "pop rcx",
        "pop rdx",
        "pop rsi",
        "pop rdi",
        "pop rax",
        "test byte ptr [rsp + 8], 3",
        "jz 2f",
        "swapgs",
        "2:",
        "iretq",
    );
}

/// Naked trampoline for Mouse Interrupt (IRQ 12).
#[unsafe(naked)]
unsafe extern "C" fn mouse_interrupt_naked() {
    naked_asm!(
        "test byte ptr [rsp + 8], 3",
        "jz 1f",
        "swapgs",
        "1:",
        "push rax",
        "push rdi",
        "push rsi",
        "push rdx",
        "push rcx",
        "push r8",
        "push r9",
        "push r10",
        "push r11",
        "push rbx",
        "push rbp",
        "push r12",
        "push r13",
        "push r14",
        "push r15",
        "mov rdi, rsp",
        "call mouse_interrupt_handler",
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop rbp",
        "pop rbx",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        "pop rcx",
        "pop rdx",
        "pop rsi",
        "pop rdi",
        "pop rax",
        "test byte ptr [rsp + 8], 3",
        "jz 2f",
        "swapgs",
        "2:",
        "iretq",
    );
}

#[no_mangle]
extern "C" fn timer_interrupt_handler(frame: &mut TrapFrame) {
    unsafe {
        // Debug: Early log to see if we even reach the handler
        core::arch::asm!(
            "out dx, al",
            in("dx") 0xe9u16,
            in("al") b'T',
            options(nomem, nostack, preserves_flags)
        );
        crate::bringup::interrupts::apic::end_of_interrupt();
    }
    trap::timer_tick(frame);
}

#[no_mangle]
extern "C" fn keyboard_interrupt_handler(_frame: &mut TrapFrame) {
    // Read Scan Code
    let scancode: u8;
    unsafe {
        use x86_64::instructions::port::Port;
        let mut port = Port::new(0x60);
        scancode = port.read();

        crate::bringup::interrupts::apic::end_of_interrupt();
    }

    // Debug: Increment and log occasionally (DISABLED)
    // use core::sync::atomic::Ordering;
    // let count = IRQ1_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    // // Only log first few or every 10th to avoid flood
    // if count < 20 || (count % 10 == 0) {
    //         //     let bridge = crate::Bridge;
    //     // Manual formatting since we can't easily use format! here without alloc
    //     bridge.log("IRQ1: count=");
    //     crate::print_u64(count); // We need a helper, or just hacking it
    //     bridge.log(" scancode=");
    //     crate::print_hex(scancode as u64);
    //     bridge.log("\n");
    // }

    // Pass to kernel input system
    kernel::input::on_ps2_scancode(scancode);
}

#[no_mangle]
extern "C" fn mouse_interrupt_handler(_frame: &mut TrapFrame) {
    let byte: u8;
    unsafe {
        use x86_64::instructions::port::Port;
        let mut port = Port::new(0x60);
        byte = port.read();
        crate::bringup::interrupts::apic::end_of_interrupt();
    }
    kernel::input::on_ps2_mouse(byte);
}

#[unsafe(naked)]
pub extern "x86-interrupt" fn page_fault_handler_naked(
    _frame: InterruptStackFrame,
    _error_code: PageFaultErrorCode,
) {
    core::arch::naked_asm!(
        // Stack on entry (CPU): [Error, RIP, CS, RFLAGS, (RSP, SS)?]
        // Swap GS if coming from user mode
        "test byte ptr [rsp + 16], 3",
        "jz 0f",
        "swapgs",
        "0:",
        // Save original RAX
        "push rax",
        // Allocate TrapFrame (20 * 8 = 160 bytes)
        "sub rsp, 160",
        // Save GPRs into TrapFrame order r15..rax
        "mov [rsp + 0], r15",
        "mov [rsp + 8], r14",
        "mov [rsp + 16], r13",
        "mov [rsp + 24], r12",
        "mov [rsp + 32], rbp",
        "mov [rsp + 40], rbx",
        "mov [rsp + 48], r11",
        "mov [rsp + 56], r10",
        "mov [rsp + 64], r9",
        "mov [rsp + 72], r8",
        "mov [rsp + 80], rcx",
        "mov [rsp + 88], rdx",
        "mov [rsp + 96], rsi",
        "mov [rsp + 104], rdi",
        // Saved orig RAX is at rsp + 160
        "mov rax, [rsp + 160]",
        "mov [rsp + 112], rax", // rax slot
        // Hardware frame offsets relative to current rsp:
        // 168=error, 176=rip, 184=cs, 192=rflags, 200=rsp?, 208=ss?
        "mov rax, [rsp + 176]",
        "mov [rsp + 120], rax", // rip
        "mov rax, [rsp + 184]",
        "mov [rsp + 128], rax", // cs
        "mov rax, [rsp + 192]",
        "mov [rsp + 136], rax", // rflags
        // Determine saved RSP / SS
        "mov rax, [rsp + 184]", // cs
        "test al, 3",
        "jnz 1f",
        // Kernel mode fault: synthesize RSP/SS
        "lea rcx, [rsp + 200]", // original rsp (before CPU push)
        "mov [rsp + 144], rcx",
        "mov rcx, ss",
        "mov [rsp + 152], rcx",
        "jmp 2f",
        "1:",
        // User mode fault: hardware provided RSP/SS
        "mov rcx, [rsp + 200]",
        "mov [rsp + 144], rcx",
        "mov rcx, [rsp + 208]",
        "mov [rsp + 152], rcx",
        "2:",
        // Call handler
        "mov rdi, rsp",         // &TrapFrame
        "mov rsi, [rsp + 168]", // error code
        "call page_fault_handler",
        // Restore GPRs
        "mov r15, [rsp + 0]",
        "mov r14, [rsp + 8]",
        "mov r13, [rsp + 16]",
        "mov r12, [rsp + 24]",
        "mov rbp, [rsp + 32]",
        "mov rbx, [rsp + 40]",
        "mov r11, [rsp + 48]",
        "mov r10, [rsp + 56]",
        "mov r9,  [rsp + 64]",
        "mov r8,  [rsp + 72]",
        "mov rcx, [rsp + 80]",
        "mov rdx, [rsp + 88]",
        "mov rsi, [rsp + 96]",
        "mov rdi, [rsp + 104]",
        "mov rax, [rsp + 112]",
        // Tear down TrapFrame and saved rax
        "add rsp, 160",
        "pop rax", // saved orig rax
        // Drop error code to get to hardware frame RIP
        "add rsp, 8",
        "test byte ptr [rsp + 8], 3", // CS
        "jnz 3f",
        "iretq",
        "3:",
        "swapgs",
        "iretq"
    );
}

#[unsafe(naked)]
pub extern "x86-interrupt" fn gp_handler_naked(_frame: InterruptStackFrame, _error_code: u64) {
    core::arch::naked_asm!(
        "test byte ptr [rsp + 16], 3",
        "jz 0f",
        "swapgs",
        "0:",
        "push rax",
        "sub rsp, 160",
        "mov [rsp + 0], r15",
        "mov [rsp + 8], r14",
        "mov [rsp + 16], r13",
        "mov [rsp + 24], r12",
        "mov [rsp + 32], rbp",
        "mov [rsp + 40], rbx",
        "mov [rsp + 48], r11",
        "mov [rsp + 56], r10",
        "mov [rsp + 64], r9",
        "mov [rsp + 72], r8",
        "mov [rsp + 80], rcx",
        "mov [rsp + 88], rdx",
        "mov [rsp + 96], rsi",
        "mov [rsp + 104], rdi",
        "mov rax, [rsp + 160]",
        "mov [rsp + 112], rax",
        "mov rax, [rsp + 176]",
        "mov [rsp + 120], rax",
        "mov rax, [rsp + 184]",
        "mov [rsp + 128], rax",
        "mov rax, [rsp + 192]",
        "mov [rsp + 136], rax",
        "mov rax, [rsp + 184]",
        "test al, 3",
        "jnz 1f",
        "lea rcx, [rsp + 200]",
        "mov [rsp + 144], rcx",
        "mov rcx, ss",
        "mov [rsp + 152], rcx",
        "jmp 2f",
        "1:",
        "mov rcx, [rsp + 200]",
        "mov [rsp + 144], rcx",
        "mov rcx, [rsp + 208]",
        "mov [rsp + 152], rcx",
        "2:",
        "mov rdi, rsp",
        "mov rsi, [rsp + 168]",
        "call gp_handler",
        "mov r15, [rsp + 0]",
        "mov r14, [rsp + 8]",
        "mov r13, [rsp + 16]",
        "mov r12, [rsp + 24]",
        "mov rbp, [rsp + 32]",
        "mov rbx, [rsp + 40]",
        "mov r11, [rsp + 48]",
        "mov r10, [rsp + 56]",
        "mov r9,  [rsp + 64]",
        "mov r8,  [rsp + 72]",
        "mov rcx, [rsp + 80]",
        "mov rdx, [rsp + 88]",
        "mov rsi, [rsp + 96]",
        "mov rdi, [rsp + 104]",
        "mov rax, [rsp + 112]",
        "add rsp, 160",
        "pop rax",
        "add rsp, 8", // drop error
        "test byte ptr [rsp + 8], 3",
        "jnz 3f",
        "iretq",
        "3:",
        "swapgs",
        "iretq"
    );
}

#[unsafe(naked)]
pub extern "x86-interrupt" fn double_fault_handler_naked(
    _frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    core::arch::naked_asm!(
        // DEBUG: Output 'D' for double fault
        "push rax",
        "push rdx",
        "mov al, 0x44", // 'D'
        "mov dx, 0xe9",
        "out dx, al",
        "pop rdx",
        "pop rax",
        "push rax",
        "mov rax, [rsp + 16]", // CS
        "test rax, 3",
        "jz 1f",
        "swapgs",
        "1:",
        "pop rax",
        "push rdi",
        "push rsi",
        "push rdx",
        "push rcx",
        "push r8",
        "push r9",
        "push r10",
        "push r11",
        "push rbx",
        "push rbp",
        "push r12",
        "push r13",
        "push r14",
        "push r15",
        "mov rdi, rsp",
        "mov rsi, rax",
        "call double_fault_handler",
        "ud2"
    );
}
