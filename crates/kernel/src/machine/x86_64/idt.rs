use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

#[allow(static_mut_refs)]
pub unsafe fn init() {
    IDT.breakpoint.set_handler_fn(breakpoint_handler);
    IDT.double_fault.set_handler_fn(double_fault_handler)
        .set_stack_index(super::gdt::DOUBLE_FAULT_IST_INDEX);
    IDT.general_protection_fault.set_handler_fn(gp_handler);
    IDT.page_fault.set_handler_fn(page_fault_handler);
    IDT.invalid_opcode.set_handler_fn(ud_handler);

    // Timer (Vector 32) and Keyboard (Vector 33)
    IDT[32].set_handler_addr(x86_64::VirtAddr::new(timer_interrupt_trampoline as *const () as u64));
    IDT[33].set_handler_addr(x86_64::VirtAddr::new(keyboard_interrupt_trampoline as *const () as u64));
    
    IDT.load();
}

extern "C" {
    fn timer_interrupt_trampoline();
    fn keyboard_interrupt_trampoline();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    crate::serial::write(b"BREAKPOINT\n");
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    crate::serial::write(b"DOUBLE FAULT\n");
    panic!("DOUBLE FAULT");
}

extern "x86-interrupt" fn gp_handler(
    stack_frame: InterruptStackFrame, error_code: u64)
{
    crate::serial::write(b"GENERAL PROTECTION FAULT\n");
    panic!("GPF");
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    let addr = Cr2::read().as_u64();
    
    crate::serial::write(b"PAGE FAULT accessed: ");
    crate::serial::write_hex(addr);
    crate::serial::write(b"\n");
    panic!("PAGE FAULT");
}

extern "x86-interrupt" fn ud_handler(
    stack_frame: InterruptStackFrame,
) {
    crate::serial::write(b"INVALID OPCODE\n");
    panic!("UD");
}

