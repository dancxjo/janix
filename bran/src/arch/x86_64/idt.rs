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
    .global breakpoint_handler_shim
    breakpoint_handler_shim:
        int3
        iretq

    .global gp_handler_shim
    gp_handler_shim:
        cli
    2:  hlt
        jmp 2b

    .global pf_handler_shim
    pf_handler_shim:
        cli
    2:  hlt
        jmp 2b

    .global generic_handler_shim
    generic_handler_shim:
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
