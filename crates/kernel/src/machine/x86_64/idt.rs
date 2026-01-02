use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

#[allow(static_mut_refs)]
pub unsafe fn init() {
    IDT.breakpoint.set_handler_fn(breakpoint_handler);
    IDT.double_fault.set_handler_fn(double_fault_handler)
        .set_stack_index(super::gdt::DOUBLE_FAULT_IST_INDEX);
    IDT.general_protection_fault.set_handler_fn(gp_handler);
    IDT.page_fault.set_handler_fn(page_fault_handler);

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
    crate::log::klog(crate::log::Level::Info, "TRAP", &alloc::format!("Breakpoint at {:?}", stack_frame.instruction_pointer));
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    crate::serial::write(b"DOUBLE FAULT\n");
    panic!("DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn gp_handler(
    stack_frame: InterruptStackFrame, error_code: u64)
{
    crate::serial::write(b"GENERAL PROTECTION FAULT\n");
    panic!("GENERAL PROTECTION FAULT: error_code={}\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    let addr = Cr2::read().as_u64();
    
    crate::serial::write(b"PAGE FAULT\n");
    panic!("PAGE FAULT: accessed {:#x} \nerror code: {:?}\n{:#?}", addr, error_code, stack_frame);
}

