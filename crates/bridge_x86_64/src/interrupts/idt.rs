use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use x86_64::VirtAddr;
use crate::interrupts::pic;
use crate::interrupts::trap::{self, TrapFrame};
use core::arch::naked_asm;

// Debug counter for IRQ1
pub static IRQ1_COUNT: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init() {
    unsafe {
        IDT.double_fault.set_handler_fn(double_fault_handler);
        IDT.breakpoint.set_handler_fn(breakpoint_handler);
        IDT.general_protection_fault.set_handler_fn(gp_handler);
        IDT.page_fault.set_handler_fn(page_fault_handler);
        
        // Timer Interrupt (IRQ 0 = 32)
        IDT[32].set_handler_addr(VirtAddr::new(timer_interrupt_naked as *const () as u64));

        // Keyboard Interrupt (IRQ 1 = 33)
        IDT[33].set_handler_addr(VirtAddr::new(keyboard_interrupt_naked as *const () as u64));

        IDT.load();

    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    kernel_core::diag::record_fault(
        stack_frame.instruction_pointer.as_u64(),
        stack_frame.stack_pointer.as_u64(),
        stack_frame.cpu_flags,
        0,
        0,
        3, // Breakpoint trap #3
        "BREAKPOINT"
    );
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, error_code: u64) -> !
{
    kernel_core::diag::record_fault(
        stack_frame.instruction_pointer.as_u64(),
        stack_frame.stack_pointer.as_u64(),
        stack_frame.cpu_flags,
        0,
        error_code,
        8, // Double Fault #8
        "DOUBLE FAULT"
    );
    loop {}
}

extern "x86-interrupt" fn gp_handler(
    stack_frame: InterruptStackFrame, error_code: u64)
{
    use x86_64::registers::control::Cr2;
    let cr2 = Cr2::read().as_u64();

    kernel_core::diag::record_fault(
        stack_frame.instruction_pointer.as_u64(),
        stack_frame.stack_pointer.as_u64(),
        stack_frame.cpu_flags,
        cr2,
        error_code,
        13, // GPF #13
        "GENERAL PROTECTION FAULT"
    );
    panic!("GPF");
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode)
{
    use x86_64::registers::control::Cr2;
    let cr2 = Cr2::read().as_u64();
    
    kernel_core::diag::record_fault(
        stack_frame.instruction_pointer.as_u64(),
        stack_frame.stack_pointer.as_u64(),
        stack_frame.cpu_flags,
        cr2,
        error_code.bits(),
        14, // Page Fault #14
        "PAGE FAULT"
    );

    panic!("Page Fault");
}

/// Naked trampoline for Timer Interrupt.
#[unsafe(naked)]
unsafe extern "C" fn timer_interrupt_naked() {
    naked_asm!(
        // Check if we came from user mode (CS & 3 == 3)
        "test byte ptr [rsp + 8], 3",
        "jz 1f",
        "swapgs",
        "1:",

        "push rax", "push rdi", "push rsi", "push rdx", "push rcx",
        "push r8", "push r9", "push r10", "push r11", "push rbx",
        "push rbp", "push r12", "push r13", "push r14", "push r15",
        
        "mov rdi, rsp",
        "call timer_interrupt_handler",
        
        "pop r15", "pop r14", "pop r13", "pop r12", "pop rbp",
        "pop rbx", "pop r11", "pop r10", "pop r9", "pop r8",
        "pop rcx", "pop rdx", "pop rsi", "pop rdi", "pop rax",
        
        // Check if we are returning to user mode (CS & 3 == 3)
        "test byte ptr [rsp + 8], 3",
        "jz 2f",
        "swapgs",
        "2:",
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

        "push rax", "push rdi", "push rsi", "push rdx", "push rcx",
        "push r8", "push r9", "push r10", "push r11", "push rbx",
        "push rbp", "push r12", "push r13", "push r14", "push r15",
        
        "mov rdi, rsp",
        "call keyboard_interrupt_handler",
        
        "pop r15", "pop r14", "pop r13", "pop r12", "pop rbp",
        "pop rbx", "pop r11", "pop r10", "pop r9", "pop r8",
        "pop rcx", "pop rdx", "pop rsi", "pop rdi", "pop rax",
        
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
        pic::notify_end_of_interrupt(32);
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
        
        pic::notify_end_of_interrupt(33);
    }
    
    // Debug: Increment and log occasionally
    use core::sync::atomic::Ordering;
    let count = IRQ1_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    // Only log first few or every 10th to avoid flood
    if count < 20 || (count % 10 == 0) {
        use hw::HardwareBridge;
        let bridge = crate::Bridge;
        // Manual formatting since we can't easily use format! here without alloc
        bridge.log("IRQ1: count=");
        crate::print_u64(count); // We need a helper, or just hacking it
        bridge.log(" scancode=");
        crate::print_hex(scancode as u64);
        bridge.log("\n");
    }

    // Pass to kernel input system
    kernel_core::input::on_ps2_scancode(scancode);
}

