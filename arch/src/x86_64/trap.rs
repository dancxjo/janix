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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TrapFrame {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub rbp: u64,
    pub rbx: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rax: u64,
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

core::arch::global_asm!(
    r#"
.global timer_interrupt_handler_asm
timer_interrupt_handler_asm:
    push rax
    push rdi
    push rsi
    push rdx
    push rcx
    push r8
    push r9
    push r10
    push r11
    push rbx
    push rbp
    push r12
    push r13
    push r14
    push r15

    // RDI = &TrapFrame (rsp matches struct layout now)
    mov rdi, rsp
    
    // Call Rust handler
    call timer_interrupt_handler
    
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbp
    pop rbx
    pop r11
    pop r10
    pop r9
    pop r8
    pop rcx
    pop rdx
    pop rsi
    pop rdi
    pop rax
    
    iretq
"#
);

unsafe extern "C" {
    fn timer_interrupt_handler_asm();
}


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
        unsafe {
             idt[pic::PIC_1_OFFSET as usize].set_handler_addr(VirtAddr::new(timer_interrupt_handler_asm as u64));
        }
        idt
    };
}

pub fn init() {
    IDT.load();
    pic::init();
    // Initialize PIT to 1000Hz
    super::pit::init();
    // Unmask PS/2 interrupts for device buffers (ISR now pushes to buffers)
    pic::set_irq_mask(KEYBOARD_IRQ, false);
    pic::set_irq_mask(MOUSE_IRQ, false);
    
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
    use x86_64::instructions::port::PortReadOnly;
    // Read directly from port 0x60
    let mut port = PortReadOnly::<u8>::new(0x60);
    let scancode = unsafe { port.read() };
    kernel::devices::ps2_buffers::push_keyboard_byte(scancode);
    pic::notify_end_of_interrupt(KEYBOARD_IRQ);
}

extern "x86-interrupt" fn mouse_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::PortReadOnly;
    let mut port = PortReadOnly::<u8>::new(0x60);
    let byte = unsafe { port.read() };
    kernel::devices::ps2_buffers::push_mouse_byte(byte);
    pic::notify_end_of_interrupt(MOUSE_IRQ);
}

#[unsafe(no_mangle)]
pub extern "C" fn timer_interrupt_handler(frame: &mut TrapFrame) {
    use kernel::sched::{self, TICKS, PREEMPT_COUNT, NEED_RESCHED};
    use core::sync::atomic::Ordering;

    // 1. Ack PIC
    pic::notify_end_of_interrupt(0);

    // 2. Increment ticks
    TICKS.fetch_add(1, Ordering::Relaxed);

    // 3. Request reschedule
    // For now, request every tick (or throttle if needed)
    NEED_RESCHED.store(true, Ordering::Relaxed);

    // 4. Check for preemption
    // Only preempt if:
    // - Preemption is allowed (count == 0)
    // - We are returning to User Mode (CS & 3 == 3)
    let is_user = (frame.cs & 3) == 3;
    let preempt_allowed = PREEMPT_COUNT.load(Ordering::Relaxed) == 0;
    
    
    if is_user && preempt_allowed {
        // Safe to schedule
        // Save context to current thread
        let mut sched = sched::SCHEDULER.lock();
        if let Some(mut thread) = sched.current_id().and_then(|tid| sched.thread_mut(tid)) {
             thread.context[0] = frame.r15;
             thread.context[1] = frame.r14;
             thread.context[2] = frame.r13;
             thread.context[3] = frame.r12;
             thread.context[4] = frame.rbp;
             thread.context[5] = frame.rbx;
             thread.context[6] = frame.r11;
             thread.context[7] = frame.r10;
             thread.context[8] = frame.r9;
             thread.context[9] = frame.r8;
             thread.context[10] = frame.rcx;
             thread.context[11] = frame.rdx;
             thread.context[12] = frame.rsi;
             thread.context[13] = frame.rdi;
             thread.context[14] = frame.rax;
             thread.context[15] = frame.rip;
             thread.context[16] = frame.cs;
             thread.context[17] = frame.rflags;
             thread.context[18] = frame.rsp;
             thread.context[19] = frame.ss;
             thread.started = true;
        }

        // IMPORTANT: Requeue the current thread so it's not lost!
        if let Some(tid) = sched.current_id() {
             sched.mark_yield(tid);
        }
        
        // Pick next thread
        let now = kernel::time::monotonic_now_ns();
        if let Some(next) = sched.choose_next_thread(now) {
             drop(sched); // Unlock before switch
             
             // Activate address space
             super::enter::activate_address_space(next.address_space_token);
             
             // Resume
             crate::current::resume_user_mode(&next.context);
        }
    }
}
