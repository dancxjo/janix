use super::{pic, syscall};
use crate::gdt;
use kernel::memory;
use lazy_static::lazy_static;
use x86_64::instructions::{hlt, interrupts};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use x86_64::structures::paging::Translate;

const KEYBOARD_VECTOR: usize = (pic::PIC_1_OFFSET as usize) + 1;
const KEYBOARD_IRQ: u8 = 1;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt[0x80]
                .set_handler_fn(syscall::syscall_handler_naked)
                .set_privilege_level(x86_64::PrivilegeLevel::Ring3);
        }
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt.general_protection_fault
            .set_handler_fn(gp_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt[KEYBOARD_VECTOR].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub fn init() {
    IDT.load();
    pic::init();
    // Log syscall gate configuration to ensure user mode can invoke int 0x80.
    let entry = &IDT[0x80];
    kernel::println!("IDT[0x80]: {:?}", entry);
    interrupts::enable();
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    kernel::println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    kernel::log("Double fault occurred; halting CPU");
    loop {
        hlt();
    }
}

extern "x86-interrupt" fn gp_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    use core::ptr;
    use x86_64::registers::control::Cr3;

    let rsp = stack_frame.stack_pointer.as_u64();
    let mut words = [0u64; 6];
    // SAFETY: We only read a small number of words for debugging.
    for (i, slot) in words.iter_mut().enumerate() {
        let ptr = (rsp as *const u64).wrapping_add(i);
        unsafe {
            *slot = ptr::read_volatile(ptr);
        }
    }

    kernel::println!(
        "EXCEPTION: GENERAL PROTECTION FAULT\nError Code: {:#x}\nCR3={:#x}\nRSP={:#x}\nStack top: [{:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}]\n{:#?}",
        error_code,
        Cr3::read().0.start_address().as_u64(),
        rsp,
        words[0],
        words[1],
        words[2],
        words[3],
        words[4],
        words[5],
        stack_frame
    );
    loop {}
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::VirtAddr;
    use x86_64::registers::control::Cr2;
    use x86_64::structures::paging::mapper::MapperFlush;
    use x86_64::structures::paging::{
        Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, Size4KiB,
    };

    let addr = Cr2::read();

    let phys_mem_offset = memory::get_hhdm_offset();
    if phys_mem_offset != 0 {
        let level_4_table_ptr = x86_64::registers::control::Cr3::read()
            .0
            .start_address()
            .as_u64();
        let level_4_table_ptr = VirtAddr::new(level_4_table_ptr + phys_mem_offset);
        let level_4_table: &mut PageTable = unsafe { &mut *level_4_table_ptr.as_mut_ptr() };
        let mut mapper =
            unsafe { OffsetPageTable::new(level_4_table, VirtAddr::new(phys_mem_offset)) };

        let _translation = mapper.translate_addr(addr);
        // match translation {
        //     Some(pa) => {
        //         kernel::println!("Page fault translation: virt={:?} -> phys={:?}", addr, pa)
        //     }
        //     None => kernel::println!("Page fault translation: virt={:?} unmapped", addr),
        // }

        // Lazy map as user accessible on protection violation
        if error_code.contains(PageFaultErrorCode::PROTECTION_VIOLATION)
            && error_code.contains(PageFaultErrorCode::USER_MODE)
        {
            let page = Page::<Size4KiB>::containing_address(addr);
            let new_flags = PageTableFlags::PRESENT
                | PageTableFlags::WRITABLE
                | PageTableFlags::USER_ACCESSIBLE;

            unsafe {
                if let Ok(flush) = mapper.update_flags(page, new_flags) {
                    let flush: MapperFlush<Size4KiB> = flush;
                    flush.flush();
                    return;
                }
            }
        }
    }

    kernel::println!("EXCEPTION: PAGE FAULT");
    kernel::println!("Accessed Address: {:?}", addr);
    kernel::println!("Error Code: {:?}", error_code);
    kernel::println!("{:#?}", stack_frame);
    loop {}
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    kernel::hw::io::handle_interrupt(KEYBOARD_IRQ);
    pic::notify_end_of_interrupt(KEYBOARD_IRQ);
}
