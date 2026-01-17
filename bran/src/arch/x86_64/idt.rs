use core::mem::size_of;

pub const IRQ_TIMER_VECTOR: u8 = 0xFE;

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    type_attr: u8,
    offset_middle: u16,
    offset_high: u32,
    reserved: u32,
}

impl IdtEntry {
    const fn missing() -> Self {
        Self {
            offset_low: 0,
            selector: 0,
            ist: 0,
            type_attr: 0,
            offset_middle: 0,
            offset_high: 0,
            reserved: 0,
        }
    }

    fn set_handler(&mut self, handler: u64, sel: u16, ist: u8, type_attr: u8) {
        self.offset_low = (handler & 0xFFFF) as u16;
        self.selector = sel;
        self.ist = ist;
        self.type_attr = type_attr;
        self.offset_middle = ((handler >> 16) & 0xFFFF) as u16;
        self.offset_high = (handler >> 32) as u32;
        self.reserved = 0;
    }
}

#[repr(C, align(16))]
struct Idt {
    entries: [IdtEntry; 256],
}

static mut IDT: Idt = Idt {
    entries: [IdtEntry::missing(); 256],
};

#[repr(C, packed)]
struct IdtDescriptor {
    size: u16,
    offset: u64,
}

unsafe extern "C" {
    fn breakpoint_handler_shim();
    fn double_fault_handler_shim();
    fn gp_handler_shim();
    fn pf_handler_shim();
    fn generic_handler_shim();
    fn irq_common_handler_shim();
    fn irq_timer_handler_shim();
}

core::arch::global_asm!(
    r#"
    .global breakpoint_handler_shim
    breakpoint_handler_shim:
        int3
        iretq

    .global double_fault_handler_shim
    double_fault_handler_shim:
        mov $0x3f8, %dx
        mov $0x44, %al
        out %al, %dx
        mov $0x46, %al
        out %al, %dx
        testb $3, 16(%rsp)
        jz 1f
        swapgs
    1:
        cli
        mov %rsp, %rdi
        call rust_double_fault_handler
    2:  hlt
        jmp 2b

    .global gp_handler_shim
    gp_handler_shim:
        mov $0x3f8, %dx
        mov $0x47, %al
        out %al, %dx
        testb $3, 16(%rsp)
        jz 1f
        swapgs
    1:
        cli
        mov %rsp, %rdi
        call rust_gp_handler
    2:  hlt
        jmp 2b

    .global pf_handler_shim
    pf_handler_shim:
        mov $0x3f8, %dx
        mov $0x50, %al
        out %al, %dx
        testb $3, 16(%rsp)
        jz 1f
        swapgs
    1:
        cli
        mov %rsp, %rdi
        call rust_pf_handler
    2:  hlt
        jmp 2b

    .global generic_handler_shim
    generic_handler_shim:
        mov $0x3f8, %dx
        mov $0x3F, %al
        out %al, %dx
    2:  hlt
        jmp 2b

    .global irq_common_handler_shim
    irq_common_handler_shim:
        push %rax
        push %rcx
        push %rdx
        push %rsi
        push %rdi
        push %r8
        push %r9
        push %r10
        push %r11

        mov $0, %rdi
        call rust_irq_handler

        pop %r11
        pop %r10
        pop %r9
        pop %r8
        pop %rdi
        pop %rsi
        pop %rdx
        pop %rcx
        pop %rax

        iretq

    .global irq_timer_handler_shim
    irq_timer_handler_shim:
        push %rax
        push %rcx
        push %rdx
        push %rsi
        push %rdi
        push %r8
        push %r9
        push %r10
        push %r11

        mov $0xFE, %rdi
        call rust_irq_handler

        pop %r11
        pop %r10
        pop %r9
        pop %r8
        pop %rdi
        pop %rsi
        pop %rdx
        pop %rcx
        pop %rax

        iretq
"#,
    options(att_syntax)
);

pub unsafe fn init() {
    let handler = generic_handler_shim as *const () as u64;
    unsafe {
        let base = core::ptr::addr_of_mut!(IDT.entries) as *mut IdtEntry;
        for i in 0..256 {
            (*base.add(i)).set_handler(
                handler,
                crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
                0,
                0x8E,
            );
        }

        // Exceptions
        IDT.entries[3].set_handler(
            breakpoint_handler_shim as *const () as u64,
            crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
            0,
            0x8E,
        );
        IDT.entries[8].set_handler(
            double_fault_handler_shim as *const () as u64,
            crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
            1,
            0x8E,
        );
        IDT.entries[13].set_handler(
            gp_handler_shim as *const () as u64,
            crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
            0,
            0x8E,
        );
        IDT.entries[14].set_handler(
            pf_handler_shim as *const () as u64,
            crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
            0,
            0x8E,
        );

        // Hardware IRQs/MSI vectors
        for vector in 0x20..=0xEF {
            IDT.entries[vector as usize].set_handler(
                irq_common_handler_shim as *const () as u64,
                crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
                0,
                0x8E,
            );
        }

        // Dedicated Timer Vector
        IDT.entries[IRQ_TIMER_VECTOR as usize].set_handler(
            irq_timer_handler_shim as *const () as u64,
            crate::arch::x86_64::gdt::KERNEL_CODE_SEL,
            0,
            0x8E,
        );

        let idtr = IdtDescriptor {
            size: (size_of::<Idt>() - 1) as u16,
            offset: core::ptr::addr_of!(IDT) as u64,
        };

        core::arch::asm!("lidt [{}]", in(reg) &idtr);
    }
}

#[repr(C)]
pub struct InterruptStackFrame {
    pub error_code: u64,
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

/// Hardware IRQ handler - dispatches to kernel and sends EOI
#[unsafe(no_mangle)]
pub extern "C" fn rust_irq_handler(vector: u64) {
    let resolved = crate::arch::x86_64::ioapic::lapic_in_service_vector().unwrap_or(vector as u8);
    
    // Send EOI to Local APIC early to avoid wedging during context switch
    crate::arch::x86_64::ioapic::send_eoi();

    // IRQ_TIMER_VECTOR is our preemption heartbeat
    if resolved == IRQ_TIMER_VECTOR {
        kernel::task::resched_if_needed::<crate::arch::CurrentRuntime>();
    } else {
        kernel::irq::dispatch_irq(resolved);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_pf_handler(frame: &InterruptStackFrame) -> ! {
    let cr2: u64;
    unsafe {
        core::arch::asm!("mov {}, cr2", out(reg) cr2);
    }

    if frame.cs & 3 == 3 {
        unsafe {
            unsafe extern "C" {
                fn kernel_handle_page_fault(rip: u64, addr: u64, err: u64);
            }
            kernel_handle_page_fault(frame.rip, cr2, frame.error_code);
        }
    }

    panic!(
        "PAGE FAULT at 0x{:x} RIP=0x{:x} CS=0x{:x} ERR=0x{:x}",
        cr2, frame.rip, frame.cs, frame.error_code
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_gp_handler(frame: &InterruptStackFrame) -> ! {
    panic!(
        "GPF at RIP=0x{:x} CS=0x{:x} ERR=0x{:x} RSP=0x{:x}",
        frame.rip, frame.cs, frame.error_code, frame.rsp
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_double_fault_handler(frame: &InterruptStackFrame) -> ! {
    panic!(
        "DOUBLE FAULT at RIP=0x{:x} CS=0x{:x} ERR=0x{:x} RSP=0x{:x}",
        frame.rip, frame.cs, frame.error_code, frame.rsp
    );
}
