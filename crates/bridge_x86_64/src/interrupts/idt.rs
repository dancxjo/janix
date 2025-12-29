use crate::interrupts::trap::{self, TrapFrame};
use core::arch::naked_asm;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use x86_64::VirtAddr;

// Debug counter for IRQ1
pub static IRQ1_COUNT: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

use lazy_static::lazy_static;

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
pub extern "C" fn double_fault_handler(
    frame: &mut TrapFrame,
    error_code: u64,
) -> ! {
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
pub extern "C" fn page_fault_handler(
    frame: &mut TrapFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    let cr2 = Cr2::read().unwrap_or(VirtAddr::zero()).as_u64();
    // Debug dump of fault frame to help root-cause early boot faults
    {
        use kernel::bridge::HardwareBridge;
        let bridge = crate::Bridge;
        bridge.log("PAGE FAULT: rip=");
        crate::print_hex(frame.rip);
        bridge.log(" cs=");
        crate::print_hex(frame.cs);
        bridge.log(" rsp=");
        crate::print_hex(frame.rsp);
        bridge.log(" ss=");
        crate::print_hex(frame.ss);
        bridge.log(" err=");
        crate::print_hex(error_code.bits() as u64);
        bridge.log(" cr2=");
        crate::print_hex(cr2);
        // Peek a couple of user stack slots to see call chain
        let rsp_val = frame.rsp;
        if rsp_val != 0 {
            let _ptr = rsp_val as *const u64;
            // Safety: We are in the page fault handler. Peeking might cause another fault?
            // Usually fine if we are careful.
            // let slot0 = unsafe { core::ptr::read(ptr) };
            // bridge.log(" stack[0]=");
            // crate::print_hex(slot0);
        }
        bridge.log("\n");
    }

    // Check hook first
    unsafe {
        if let Some(hook) = crate::PAGE_FAULT_HOOK {
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
        // 1. Check if we came from user mode (CS & 3 == 3)
        // CS is at [rsp + 8] (since HW pushed RIP, CS, RFLAGS)
        "test byte ptr [rsp + 8], 3",
        "jnz 1f",
        // --- KERNEL MODE ENTRY ---
        // Stack: [RIP, CS, RFLAGS]
        // We need to expand to [RIP, CS, RFLAGS, RSP, SS]
        // to match TrapFrame layout and prevent stack corruption when overwriting.
        "sub rsp, 16", // Create gap
        "push rax",    // Scratch
        // Correct Order: Low to High to avoid overwriting invalidating sources
        // Source RIP is at +24. Dest is at +8.
        "mov rax, [rsp + 24]", // RIP
        "mov [rsp + 8], rax",  // New RIP position
        // Source CS is at +32. Dest is at +16.
        "mov rax, [rsp + 32]", // CS
        "mov [rsp + 16], rax", // New CS position
        // Source RFLAGS is at +40. Dest is at +24.
        "mov rax, [rsp + 40]", // RFLAGS
        "mov [rsp + 24], rax", // New RFLAGS position
        // Synthesize SS and RSP
        "mov rax, ss",
        "mov [rsp + 40], rax", // SS at top
        "lea rax, [rsp + 48]", // Original RSP
        "mov [rsp + 32], rax", // RSP
        "pop rax",             // Restore scratch
        "jmp 2f",
        "1:",
        // --- USER MODE ENTRY ---
        // Stack: [RIP, CS, RFLAGS, RSP, SS] (HW Pushed 5 items)
        "swapgs",
        "2:",
        // Common: Push GPRs (TrapFrame items 0..14)
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
        // Call Handler
        "mov rdi, rsp",
        "call timer_interrupt_handler",
        // Restore GPRs
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
        // --- RETURN ---
        // Check if returning to user mode (CS & 3 == 3)
        // Stack: [RIP, CS, RFLAGS, RSP, SS]
        "test byte ptr [rsp + 8], 3",
        "jz 3f",
        // Return to User
        "swapgs",
        "iretq",
        "3:",
        // Return to Kernel: Must Pivot Stack if RSP changed!

        // 1. Save RAX (Scratch/Return Value)
        "push rax",
        // Stack: [RAX, RIP, CS, RFLAGS, RSP, SS]
        // Offsets: 0, 8, 16, 24, 32, 40

        // 2. Load Target RSP (from +32)
        "mov rax, [rsp + 32]",
        "sub rax, 40", // Reserve space for 5 items (RIP, CS, RFLAGS, RSP, SS)
        // 3. Save RBX (Scratch)
        "push rbx",
        // Stack: [RBX, RAX, RIP, CS, RFLAGS, RSP, SS]
        // Offsets: 0, 8, 16, 24, 32, 40, 48
        
        // 4. Copy Interrupt Frame to Target Stack (All 5 items)
        // Copy RIP
        "mov rbx, [rsp + 16]",
        "mov [rax], rbx",
        // Copy CS
        "mov rbx, [rsp + 24]",
        "mov [rax + 8], rbx",
        // Copy RFLAGS
        "mov rbx, [rsp + 32]",
        "mov [rax + 16], rbx",
        // Copy RSP
        "mov rbx, [rsp + 40]",
        "mov [rax + 24], rbx",
        // Copy SS
        "mov rbx, [rsp + 48]",
        "mov [rax + 32], rbx",

        // 5. Restore Saved RAX to Target Stack (Dest: [rax-8])
        "mov rbx, [rsp + 8]",
        "mov [rax - 8], rbx",
        "pop rbx", // Restore RBX
        // 6. Pivot
        "mov rsp, rax",
        "sub rsp, 8", // Point to saved RAX
        "pop rax",    // Restore RAX
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
        crate::interrupts::apic::end_of_interrupt();
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

        crate::interrupts::apic::end_of_interrupt();
    }

    // Debug: Increment and log occasionally (DISABLED)
    // use core::sync::atomic::Ordering;
    // let count = IRQ1_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    // // Only log first few or every 10th to avoid flood
    // if count < 20 || (count % 10 == 0) {
    //     use kernel::bridge::HardwareBridge;
    //     let bridge = crate::Bridge;
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
        crate::interrupts::apic::end_of_interrupt();
    }
    kernel::input::on_ps2_mouse(byte);
}

#[unsafe(naked)]
pub extern "x86-interrupt" fn page_fault_handler_naked(
    _frame: InterruptStackFrame,
    _error_code: PageFaultErrorCode,
) {
    core::arch::naked_asm!(
        "push rax",
        "mov rax, [rsp + 8]", // Error code
        "xchg rax, [rsp]",   // rax = orig_rax, [rsp] = error_code
        "xchg rax, [rsp + 8]", // rax = error_code, [rsp + 8] = orig_rax
        // Stack: [Error, OrigRAX, RIP, CS, RFLAGS, RSP, SS]
        "test byte ptr [rsp + 24], 3", // CS index
        "jz 1f",
        "swapgs",
        "1:",
        "pop rax", // RAX = ErrorCode
        // Push GPRs to form TrapFrame (rdi...rax)
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
        // TrapFrame: [rdi...rax, rip, cs, rflags, rsp, ss]
        // Wait, RAX is missing? No, orig_rax is at offset 112 if we pushed 14 regs.
        // Wait, TrapFrame has 15 GPRs.
        // Stack: [rdi...r15, orig_rax, RIP, CS...]
        // offsets: 0...112 (r15), 120 (rax), 128 (rip)
        // Let's re-verify TrapFrame order: rdi, rsi, rdx, rcx, r8, r9, r10, r11, rbx, rbp, r12, r13, r14, r15, rax.
        "mov rdi, rsp",
        "mov rsi, rax", // ErrorCode
        "call page_fault_handler",
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
        // Stack: [OrigRAX, RIP, CS, RFLAGS, RSP, SS]
        "test byte ptr [rsp + 16], 3", // CS index is 16
        "jz 2f",
        "swapgs",
        "2:",
        "pop rax",
        "iretq"
    );
}

#[unsafe(naked)]
pub extern "x86-interrupt" fn gp_handler_naked(
    _frame: InterruptStackFrame,
    _error_code: u64,
) {
    core::arch::naked_asm!(
        "push rax",
        "mov rax, [rsp + 8]",
        "xchg rax, [rsp]",
        "xchg rax, [rsp + 8]",
        "test byte ptr [rsp + 24], 3",
        "jz 1f",
        "swapgs",
        "1:",
        "pop rax", // ErrorCode
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
        "call gp_handler",
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
        "test byte ptr [rsp + 16], 3",
        "jz 2f",
        "swapgs",
        "2:",
        "pop rax",
        "iretq"
    );
}

#[unsafe(naked)]
pub extern "x86-interrupt" fn double_fault_handler_naked(
    _frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    core::arch::naked_asm!(
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
