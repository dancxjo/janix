use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use crate::log::{self, Level};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub unsafe fn init() {
    IDT.breakpoint.set_handler_fn(breakpoint_handler);
    IDT.double_fault.set_handler_fn(double_fault_handler)
        .set_stack_index(super::gdt::DOUBLE_FAULT_IST_INDEX);
    IDT.general_protection_fault.set_handler_fn(gp_handler);
    IDT.page_fault.set_handler_fn(page_fault_handler);
    
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    log::klog(Level::Info, "IDT", "BREAKPOINT");
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    panic!("DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn gp_handler(
    stack_frame: InterruptStackFrame, error_code: u64)
{
    panic!("GENERAL PROTECTION FAULT: error_code={}\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode)
{
    use x86_64::registers::control::Cr2;
    panic!("PAGE FAULT: accessed {:?}\nerror code: {:?}\n{:#?}", Cr2::read(), error_code, stack_frame);
}
