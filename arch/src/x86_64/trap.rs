use super::{pic, syscall};
use crate::gdt;
use kernel::memory;
use lazy_static::lazy_static;
use x86_64::VirtAddr;
use x86_64::instructions::{hlt, interrupts};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use x86_64::structures::paging::Translate;

const KEYBOARD_VECTOR: usize = (pic::PIC_1_OFFSET as usize) + 1;
const KEYBOARD_IRQ: u8 = 1;
const MOUSE_VECTOR: usize = (pic::PIC_1_OFFSET as usize) + 12;
const MOUSE_IRQ: u8 = 12;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        let handler_addr = VirtAddr::new(syscall::syscall_handler_asm as *const () as u64);
        unsafe {
            idt[0x80]
                .set_handler_addr(handler_addr)
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
        idt[MOUSE_VECTOR].set_handler_fn(mouse_interrupt_handler);
        idt
    };
}

pub fn init() {
    IDT.load();
    pic::init();
    // Log syscall gate configuration to ensure user mode can invoke int 0x80.

    #[cfg(feature = "trace_idt")]
    {
        let entry = &IDT[0x80];
        kernel::println!("IDT[0x80]: {:?}", entry);
    }
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
    use x86_64::registers::control::{Cr2, Cr3};
    use x86_64::structures::paging::Translate;
    use x86_64::structures::paging::{OffsetPageTable, PageTable};

    let addr = Cr2::read();

    kernel::println!("EXCEPTION: PAGE FAULT");
    kernel::println!("  Accessed Address: {:?}", addr);
    kernel::println!("  Error Code: {:?}", error_code);
    kernel::println!(
        "  RIP={:#x} RSP={:#x} CR3={:#x}",
        stack_frame.instruction_pointer.as_u64(),
        stack_frame.stack_pointer.as_u64(),
        Cr3::read().0.start_address().as_u64(),
    );

    // Best-effort translation, read-only, no allocation
    let phys_mem_offset = memory::get_hhdm_offset();
    if phys_mem_offset != 0 {
        let l4_phys = Cr3::read().0.start_address().as_u64();
        let l4_virt = VirtAddr::new(l4_phys + phys_mem_offset);
        let l4: &mut PageTable = unsafe { &mut *l4_virt.as_mut_ptr() };

        let mapper = unsafe { OffsetPageTable::new(l4, VirtAddr::new(phys_mem_offset)) };

        match mapper.translate_addr(addr) {
            Some(pa) => {
                kernel::println!("  Translation: virt={:?} -> phys={:#x}", addr, pa.as_u64());
            }
            None => {
                kernel::println!("  Translation: virt={:?} unmapped", addr);
            }
        }
    }

    kernel::println!("{:#?}", stack_frame);

    // Future: convert into a fault event or thread termination
    loop {
        hlt();
    }
}
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    kernel::hw::io::handle_interrupt(KEYBOARD_IRQ);
    pic::notify_end_of_interrupt(KEYBOARD_IRQ);
}

extern "x86-interrupt" fn mouse_interrupt_handler(_stack_frame: InterruptStackFrame) {
    kernel::hw::io::handle_interrupt(MOUSE_IRQ);
    pic::notify_end_of_interrupt(MOUSE_IRQ);
}
