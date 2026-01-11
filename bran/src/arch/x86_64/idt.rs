use core::mem::size_of;

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
            offset_low: 0, selector: 0, ist: 0, type_attr: 0,
            offset_middle: 0, offset_high: 0, reserved: 0,
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
    fn gp_handler_shim();
    fn pf_handler_shim();
    fn generic_handler_shim();
}

core::arch::global_asm!(r#"
    .att_syntax
    .global breakpoint_handler_shim
    breakpoint_handler_shim:
        int3
        iretq

    .global gp_handler_shim
    gp_handler_shim:
        cli
        mov $0x3f8, %dx
        mov $0x47, %al
        out %al, %dx
    2:  hlt
        jmp 2b

    .global pf_handler_shim
    pf_handler_shim:
        cli
        // Read CR2 (Fault Address)
        mov %cr2, %rax
        
        // Check Code (0x200000)
        mov $0x200000, %rbx
        // Mask offset to check page
        and $0xFFFFFFFFFFFFF000, %rax
        cmp %rbx, %rax
        je 1f // Code

        // Check Stack (0x3FF000 - mapped page for 0x400000 SP)
        mov $0x3FF000, %rbx
        cmp %rbx, %rax
        je 2f // Stack
        
        // Other
        mov $0x4F, %al // 'O'
        jmp 3f

    1: // Code
        mov $0x43, %al // 'C'
        jmp 3f
    2: // Stack
        mov $0x53, %al // 'S'
        jmp 3f
    
    3:
        mov $0x3f8, %dx
        out %al, %dx

        // Check Error Code (Top of stack) for Present Bit (Bit 0)
        mov (%rsp), %bl
        test $1, %bl
        jnz 4f // Present -> Protection Violation
        
        mov $0x4E, %al // 'N' (Not Present)
        jmp 5f
    4:
        mov $0x50, %al // 'P' (Protection)
    5:
        out %al, %dx
        
    6:  hlt
        jmp 6b

    .global generic_handler_shim
    generic_handler_shim:
        mov $0x3f8, %dx
        mov $0x3F, %al
        out %al, %dx
    2:  hlt
        jmp 2b
"#);

pub unsafe fn init() {
    // Fill all with generic handler for now
    let handler = generic_handler_shim as u64; 
    
    unsafe {
        for i in 0..32 {
            IDT.entries[i].set_handler(handler, crate::arch::x86_64::gdt::KERNEL_CODE_SEL, 0, 0x8E);
        }
    
        // Specifically GPF (13) and PF (14)
        IDT.entries[3].set_handler(breakpoint_handler_shim as u64, crate::arch::x86_64::gdt::KERNEL_CODE_SEL, 0, 0x8E);
        IDT.entries[13].set_handler(gp_handler_shim as u64, crate::arch::x86_64::gdt::KERNEL_CODE_SEL, 0, 0x8E);
        IDT.entries[14].set_handler(pf_handler_shim as u64, crate::arch::x86_64::gdt::KERNEL_CODE_SEL, 0, 0x8E);

        let idtr = IdtDescriptor {
            size: (size_of::<Idt>() - 1) as u16,
            offset: core::ptr::addr_of!(IDT) as u64,
        };

        core::arch::asm!("lidt [{}]", in(reg) &idtr);
    }
}
