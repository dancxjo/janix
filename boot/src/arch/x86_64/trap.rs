use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use lazy_static::lazy_static;
use crate::gdt;
use super::syscall;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            let handler_addr = x86_64::VirtAddr::new(syscall::syscall_handler_asm as u64);
            idt[0x80].set_handler_addr(handler_addr)
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
    loop {}
}

extern "x86-interrupt" fn gp_fault_handler(
    stack_frame: InterruptStackFrame, error_code: u64)
{
    kernel_core::log("GENERAL PROTECTION FAULT");
    loop {}
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode)
{
    use x86_64::registers::control::Cr2;
    use x86_64::structures::paging::{PageTable, OffsetPageTable, Page, PageTableFlags, Size4KiB, Mapper};
    use x86_64::VirtAddr;
    use x86_64::structures::paging::mapper::MapperAllSizes;

    let addr = Cr2::read();
    
    // Lazy map as user accessible on protection violation
    if error_code.contains(PageFaultErrorCode::PROTECTION_VIOLATION) && error_code.contains(PageFaultErrorCode::USER_MODE) {
        if let Some(hhdm) = crate::boot_model::HHDM_REQUEST.get_response() {
             let phys_mem_offset = hhdm.offset();
             let level_4_table_ptr = x86_64::registers::control::Cr3::read().0.start_address().as_u64();
             let level_4_table_ptr = VirtAddr::new(level_4_table_ptr + phys_mem_offset);
             let level_4_table: &mut PageTable = unsafe { &mut *level_4_table_ptr.as_mut_ptr() };
             let mut mapper = unsafe { OffsetPageTable::new(level_4_table, VirtAddr::new(phys_mem_offset)) };
             
             let page = Page::<Size4KiB>::containing_address(addr);
             let new_flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
             
             unsafe {
                 if let Ok(flush) = mapper.update_flags(page, new_flags) {
                     flush.flush();
                     return;
                 }
             }
        }
    }

    kernel_core::log("PAGE FAULT");
    loop {}
}
