use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use lazy_static::lazy_static;
use crate::gdt;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt[0x80].set_handler_fn(syscall_handler)
                .set_privilege_level(x86_64::PrivilegeLevel::Ring3);
        }
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt.general_protection_fault.set_handler_fn(gp_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt
    };
}

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    kernel_core::log("DOUBLE FAULT");
    // panic!("DOUBLE FAULT\n{:#?}", stack_frame);
    loop {}
}

extern "x86-interrupt" fn gp_fault_handler(
    stack_frame: InterruptStackFrame, error_code: u64)
{
    kernel_core::log("GENERAL PROTECTION FAULT");
    // panic!("GP FAULT: error_code={}\n{:#?}", error_code, stack_frame);
    loop {}
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode)
{
    use x86_64::registers::control::Cr2;
    let addr = Cr2::read();
    kernel_core::log("PAGE FAULT");
    // panic!("PAGE FAULT: accessed {:?}\nerror code: {:?}\n{:#?}", addr, error_code, stack_frame);
    loop {}
}

extern "x86-interrupt" fn syscall_handler(stack_frame: InterruptStackFrame) {
    kernel_core::log("syscall from user mode!");
    // Note: To read RAX we would need a naked function or assembly wrapper.
    // For now, just proving we got here is enough.
}
