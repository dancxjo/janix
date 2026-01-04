use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

#[allow(static_mut_refs)]
pub unsafe fn init() {
    IDT.breakpoint.set_handler_fn(breakpoint_handler);
    IDT.double_fault.set_handler_fn(double_fault_handler).set_stack_index(super::gdt::DOUBLE_FAULT_IST_INDEX);
    IDT.general_protection_fault.set_handler_fn(gp_handler);
    IDT.page_fault.set_handler_fn(page_fault_handler);
    IDT.invalid_opcode.set_handler_fn(ud_handler);
    IDT[32].set_handler_addr(x86_64::VirtAddr::new(timer_interrupt_trampoline as *const () as u64));
    IDT[33].set_handler_addr(x86_64::VirtAddr::new(keyboard_interrupt_trampoline as *const () as u64));
    IDT[44].set_handler_addr(x86_64::VirtAddr::new(mouse_interrupt_trampoline as *const () as u64));
    IDT.load();
}

extern "C" {
    fn timer_interrupt_trampoline();
    fn keyboard_interrupt_trampoline();
    fn mouse_interrupt_trampoline();
}

extern "x86-interrupt" fn breakpoint_handler(_sf: InterruptStackFrame) {
    crate::serial::write(b"BREAKPOINT\n");
}

extern "x86-interrupt" fn double_fault_handler(sf: InterruptStackFrame, ec: u64) -> ! {
    use crate::serial::{write, write_hex};
    write(b"\n\n========== x86_64 DOUBLE FAULT ==========\n");
    write(b"Error Code: "); write_hex(ec); write(b"\n");
    write(b"RIP: "); write_hex(sf.instruction_pointer.as_u64()); write(b"\n");
    write(b"CS:  "); write_hex(sf.code_segment as u64); write(b"\n");
    write(b"RFLAGS: "); write_hex(sf.cpu_flags); write(b"\n");
    write(b"RSP: "); write_hex(sf.stack_pointer.as_u64()); write(b"\n");
    write(b"SS:  "); write_hex(sf.stack_segment as u64); write(b"\n");
    write(b"==========================================\n");
    panic!("DOUBLE FAULT");
}

extern "x86-interrupt" fn gp_handler(sf: InterruptStackFrame, ec: u64) {
    use crate::serial::{write, write_hex};
    write(b"\n\n========== x86_64 GENERAL PROTECTION FAULT ==========\n");
    write(b"Error Code: "); write_hex(ec); write(b"\n");
    if ec != 0 {
        let ext = ec & 1;
        let idt = (ec >> 1) & 1;
        let ti = (ec >> 2) & 1;
        let idx = (ec >> 3) & 0x1FFF;
        write(b"  External: "); write_hex(ext); write(b"\n");
        write(b"  Table: ");
        if idt != 0 { write(b"IDT"); } else if ti != 0 { write(b"LDT"); } else { write(b"GDT"); }
        write(b"\n  Selector: "); write_hex(idx); write(b"\n");
    }
    write(b"RIP: "); write_hex(sf.instruction_pointer.as_u64()); write(b"\n");
    write(b"CS:  "); write_hex(sf.code_segment as u64); write(b"\n");
    write(b"RFLAGS: "); write_hex(sf.cpu_flags); write(b"\n");
    write(b"RSP: "); write_hex(sf.stack_pointer.as_u64()); write(b"\n");
    write(b"SS:  "); write_hex(sf.stack_segment as u64); write(b"\n");
    write(b"======================================================\n");
    panic!("GPF");
}

extern "x86-interrupt" fn page_fault_handler(sf: InterruptStackFrame, ec: PageFaultErrorCode) {
    use crate::serial::{write, write_hex};
    use x86_64::registers::control::Cr2;
    let addr = Cr2::read().as_u64();
    write(b"\n\n========== x86_64 PAGE FAULT ==========\n");
    write(b"CR2 (Fault Addr): "); write_hex(addr); write(b"\n");
    write(b"Error Code: "); write_hex(ec.bits()); write(b"\n");
    write(b"  Cause: ");
    if ec.contains(PageFaultErrorCode::PROTECTION_VIOLATION) { write(b"Protection\n"); }
    else { write(b"Not present\n"); }
    write(b"  Access: ");
    if ec.contains(PageFaultErrorCode::CAUSED_BY_WRITE) { write(b"WRITE\n"); } else { write(b"READ\n"); }
    write(b"  Mode: ");
    if ec.contains(PageFaultErrorCode::USER_MODE) { write(b"User\n"); } else { write(b"Kernel\n"); }
    if ec.contains(PageFaultErrorCode::INSTRUCTION_FETCH) { write(b"  Instruction fetch (NX?)\n"); }
    write(b"RIP: "); write_hex(sf.instruction_pointer.as_u64()); write(b"\n");
    write(b"CS:  "); write_hex(sf.code_segment as u64); write(b"\n");
    write(b"RFLAGS: "); write_hex(sf.cpu_flags); write(b"\n");
    write(b"RSP: "); write_hex(sf.stack_pointer.as_u64()); write(b"\n");
    write(b"SS:  "); write_hex(sf.stack_segment as u64); write(b"\n");
    write(b"========================================\n");
    panic!("PAGE FAULT");
}

extern "x86-interrupt" fn ud_handler(sf: InterruptStackFrame) {
    use crate::serial::{write, write_hex};
    write(b"\n\n========== x86_64 INVALID OPCODE (#UD) ==========\n");
    write(b"Likely causes: executing data, unsupported insn, bad ptr\n");
    write(b"RIP: "); write_hex(sf.instruction_pointer.as_u64()); write(b"\n");
    write(b"CS:  "); write_hex(sf.code_segment as u64); write(b"\n");
    write(b"RFLAGS: "); write_hex(sf.cpu_flags); write(b"\n");
    write(b"RSP: "); write_hex(sf.stack_pointer.as_u64()); write(b"\n");
    write(b"SS:  "); write_hex(sf.stack_segment as u64); write(b"\n");
    write(b"=================================================\n");
    panic!("UD");
}
