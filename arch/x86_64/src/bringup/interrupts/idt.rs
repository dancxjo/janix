use crate::bringup::interrupts::trap::{self, TrapFrame};
use core::arch::naked_asm;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use x86_64::VirtAddr;
use kernel::bridge::CpuBridge;

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
#[unsafe(naked)]
unsafe extern "C" fn timer_interrupt_naked() {
    naked_asm!(
        // DEBUG: Very first thing - output 'X' to debugcon
        "push rax",
        "push rdx",
        "mov al, 0x58",  // 'X'
        "mov dx, 0xe9",
        "out dx, al",
        "pop rdx",
        "pop rax",
        // Check CPL from hardware frame CS (at [rsp + 8] for no-error interrupts)
        "test byte ptr [rsp + 8], 3",
        "jz 0f",
        "swapgs",
        "0:",
        // Save RAX then allocate TrapFrame
        "push rax",
        "sub rsp, 160",
        // Save GPRs r15..rdi
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
        // Hardware frame (no error): rip=+168, cs=+176, rflags=+184, rsp=+192, ss=+200
        "mov rax, [rsp + 168]",
        "mov [rsp + 120], rax", // rip
        "mov rax, [rsp + 176]",
        "mov [rsp + 128], rax", // cs
        "mov rax, [rsp + 184]",
        "mov [rsp + 136], rax", // rflags
        // Saved RSP/SS depend on CPL
        "mov rax, [rsp + 176]", // cs
        "test al, 3",
        "jnz 1f",
        // Kernel mode interrupt: synthesize RSP/SS
        "lea rcx, [rsp + 192]", // pre-interrupt RSP
        "mov [rsp + 144], rcx",
        "mov rcx, ss",
        "mov [rsp + 152], rcx",
        "jmp 2f",
        "1:",
        // User mode interrupt: hardware provided RSP/SS
        "mov rcx, [rsp + 192]",
        "mov [rsp + 144], rcx",
        "mov rcx, [rsp + 200]",
        "mov [rsp + 152], rcx",
        "2:",
        // Call Handler
        "mov rdi, rsp",
        "call timer_interrupt_handler",

        // IMPORTANT: The handler (Scheduler) may have modified the TrapFrame (rsp)
        // to switch contexts. We MUST copy the potentially modified RIP, CS, RFLAGS, RSP, SS
        // back to the Hardware Frame (at rsp + 168) so iretq executes the switch.

        // 1. RIP (Offset 120 -> 168)
        "mov rax, [rsp + 120]",
        "mov [rsp + 168], rax",
        // 2. CS (Offset 128 -> 176)
        "mov rax, [rsp + 128]",
        "mov [rsp + 176], rax",
        // 3. RFLAGS (Offset 136 -> 184)
        "mov rax, [rsp + 136]",
        "mov [rsp + 184], rax",

        // Check CS (now in rax) for CPL. If Kernel (0), skip RSP/SS restore.
        "test al, 3",
        "jz 4f",

        // 4. RSP (Offset 144 -> 192)
        "mov rax, [rsp + 144]",
        "mov [rsp + 192], rax",
        // 5. SS (Offset 152 -> 200)
        "mov rax, [rsp + 152]",
        "mov [rsp + 200], rax",
        "4:",

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
        // Return: swapgs only if returning to USER mode.
        // We check the CS we are ABOUT TO POP (at rsp + 8)
        "test byte ptr [rsp + 8], 3",
        "jz 3f",
        "swapgs",
        "iretq",
        "3:",
        "iretq",
    );
}

/// Naked trampoline for Keyboard Interrupt (IRQ 1).
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
        "mov al, 0x44",  // 'D'
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
