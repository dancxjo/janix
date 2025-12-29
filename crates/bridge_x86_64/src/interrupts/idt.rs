
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
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.general_protection_fault.set_handler_fn(gp_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);

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

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    kernel::diag::record_fault(
        stack_frame.instruction_pointer.as_u64(),
        stack_frame.stack_pointer.as_u64(),
        stack_frame.cpu_flags.bits(),
        0,
        error_code,
        8, // Double Fault #8
        "DOUBLE FAULT",
    );
    loop {}
}

extern "x86-interrupt" fn gp_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    use x86_64::registers::control::Cr2;
    let cr2 = Cr2::read().unwrap_or(VirtAddr::zero()).as_u64();

    kernel::diag::record_fault(
        stack_frame.instruction_pointer.as_u64(),
        stack_frame.stack_pointer.as_u64(),
        stack_frame.cpu_flags.bits(),
        cr2,
        error_code,
        13, // GPF #13
        "GENERAL PROTECTION FAULT",
    );
    panic!("GPF");
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    let cr2 = Cr2::read().unwrap_or(VirtAddr::zero()).as_u64();
    // Debug dump of fault frame to help root-cause early boot faults
    {
        use kernel::bridge::HardwareBridge;
        let bridge = crate::Bridge;
        bridge.log("PAGE FAULT: rip=");
        crate::print_hex(stack_frame.instruction_pointer.as_u64());
        bridge.log(" cs=");
        crate::print_hex(stack_frame.code_segment.0 as u64);
        bridge.log(" rsp=");
        crate::print_hex(stack_frame.stack_pointer.as_u64());
        bridge.log(" ss=");
        crate::print_hex(stack_frame.stack_segment.0 as u64);
        bridge.log(" err=");
        crate::print_hex(error_code.bits() as u64);
        bridge.log(" cr2=");
        crate::print_hex(cr2);
        bridge.log("\n");
    }
    
    // Check hook first
    unsafe {
        if let Some(hook) = crate::PAGE_FAULT_HOOK {
            if hook(&stack_frame, cr2, error_code) {
                return;
            }
        }
    }

    kernel::diag::record_fault(
        stack_frame.instruction_pointer.as_u64(),
        stack_frame.stack_pointer.as_u64(),
        stack_frame.cpu_flags.bits(),
        cr2,
        error_code.bits(),
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
        
        "sub rsp, 16",          // Create gap
        "push rax",             // Scratch
        
        // Correct Order: Low to High to avoid overwriting invalidating sources
        // Source RIP is at +24. Dest is at +8.
        "mov rax, [rsp + 24]",  // RIP
        "mov [rsp + 8], rax",   // New RIP position
        
        // Source CS is at +32. Dest is at +16.
        "mov rax, [rsp + 32]",  // CS
        "mov [rsp + 16], rax",  // New CS position
        
        // Source RFLAGS is at +40. Dest is at +24.
        "mov rax, [rsp + 40]",  // RFLAGS
        "mov [rsp + 24], rax",  // New RFLAGS position
        
        // Synthesize SS and RSP
        "mov rax, ss",
        "mov [rsp + 40], rax",  // SS at top
        
        "lea rax, [rsp + 48]",  // Original RSP
        "mov [rsp + 32], rax",  // RSP
        
        "pop rax",              // Restore scratch
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
        "sub rax, 24",         // Reserve space for RIP, CS, RFLAGS
        
        // 3. Save RBX (Scratch)
        "push rbx", 
        // Stack: [RBX, RAX, RIP, CS, RFLAGS, RSP, SS]
        // Offsets: 0, 8, 16, 24, 32, 40, 48
        
        // 4. Copy Interrupt Frame to Target Stack
        // Copy RIP (Src: +16 -> Dest: [rax])
        "mov rbx, [rsp + 16]",
        "mov [rax], rbx",
        
        // Copy CS (Src: +24 -> Dest: [rax+8])
        "mov rbx, [rsp + 24]",
        "mov [rax + 8], rbx",
        
        // Copy RFLAGS (Src: +32 -> Dest: [rax+16])
        "mov rbx, [rsp + 32]",
        "mov [rax + 16], rbx",
        
        // 5. Restore Saved RAX to Target Stack (Src: +8 -> Dest: [rax-8])
        // We want to simulate that RAX was pushed *before* the interrupt frame on the new stack?
        // No, we just want to restore RAX register.
        // We will do: mov rsp, rax; sub rsp, 8; pop rax.
        // So we need to write RAX to [rax - 8].
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
