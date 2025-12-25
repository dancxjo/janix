use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use x86_64::VirtAddr;
use crate::interrupts::pic;
use crate::interrupts::trap::{self, TrapFrame};
use core::arch::naked_asm;

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

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    // Breakpoint
}

extern "x86-interrupt" fn double_fault_handler(
    _stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    loop {}
}

extern "x86-interrupt" fn gp_handler(
    _stack_frame: InterruptStackFrame, _error_code: u64)
{
    // GP Fault
    loop {}
}

extern "x86-interrupt" fn page_fault_handler(
    _stack_frame: InterruptStackFrame, _error_code: PageFaultErrorCode)
{
    // Page Fault
    loop {}
}

/// Naked trampoline for Timer Interrupt.
#[unsafe(naked)]
unsafe extern "C" fn timer_interrupt_naked() {
    naked_asm!(
        "test byte ptr [rsp + 8], 3",
        "jz 2f", 
        "push rax", "push rdi", "push rsi", "push rdx", "push rcx",
        "push r8", "push r9", "push r10", "push r11", "push rbx",
        "push rbp", "push r12", "push r13", "push r14", "push r15",
        
        "mov rdi, rsp",
        "call timer_interrupt_handler",
        
        "pop r15", "pop r14", "pop r13", "pop r12", "pop rbp",
        "pop rbx", "pop r11", "pop r10", "pop r9", "pop r8",
        "pop rcx", "pop rdx", "pop rsi", "pop rdi", "pop rax",
        
        "swapgs",
        "iretq",

        "2:", // From Kernel
        // Push GPRs.
        "push rax", "push rdi", "push rsi", "push rdx", "push rcx",
        "push r8", "push r9", "push r10", "push r11", "push rbx",
        "push rbp", "push r12", "push r13", "push r14", "push r15",
        
        "mov rdi, rsp",
        "call timer_interrupt_handler",
        
        "pop r15", "pop r14", "pop r13", "pop r12", "pop rbp",
        "pop rbx", "pop r11", "pop r10", "pop r9", "pop r8",
        "pop rcx", "pop rdx", "pop rsi", "pop rdi", "pop rax",
        
        "iretq",
    );
}

/// Naked trampoline for Keyboard Interrupt (IRQ 1).
#[unsafe(naked)]
unsafe extern "C" fn keyboard_interrupt_naked() {
    naked_asm!(
        "test byte ptr [rsp + 8], 3",
        "jz 2f", 
        // User
        "push rax", "push rdi", "push rsi", "push rdx", "push rcx",
        "push r8", "push r9", "push r10", "push r11", "push rbx",
        "push rbp", "push r12", "push r13", "push r14", "push r15",
        
        "mov rdi, rsp",
        "call keyboard_interrupt_handler",
        
        "pop r15", "pop r14", "pop r13", "pop r12", "pop rbp",
        "pop rbx", "pop r11", "pop r10", "pop r9", "pop r8",
        "pop rcx", "pop rdx", "pop rsi", "pop rdi", "pop rax",
        
        "swapgs",
        "iretq",

        "2:", // Kernel
        "push rax", "push rdi", "push rsi", "push rdx", "push rcx",
        "push r8", "push r9", "push r10", "push r11", "push rbx",
        "push rbp", "push r12", "push r13", "push r14", "push r15",
        
        "mov rdi, rsp",
        "call keyboard_interrupt_handler",
        
        "pop r15", "pop r14", "pop r13", "pop r12", "pop rbp",
        "pop rbx", "pop r11", "pop r10", "pop r9", "pop r8",
        "pop rcx", "pop rdx", "pop rsi", "pop rdi", "pop rax",
        
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
    
    // Pass to kernel input system
    kernel_core::input::on_ps2_scancode(scancode);
}

