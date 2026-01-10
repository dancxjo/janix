use core::mem::size_of;

/// Interrupt Descriptor Table (IDT) entry.
/// 16 bytes.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
pub struct IdtEntry {
    offset_low: u16,
    selector: u16,
    options: u16,
    offset_middle: u16,
    offset_high: u32,
    reserved: u32,
}

impl IdtEntry {
    pub const fn missing() -> Self {
        Self {
            offset_low: 0,
            selector: 0,
            options: 0,
            offset_middle: 0,
            offset_high: 0,
            reserved: 0,
        }
    }

    pub fn set_handler(&mut self, handler: u64, selector: u16, flags: u16) {
        self.offset_low = handler as u16;
        self.selector = selector;
        // options: P(1) | DPL(2) | 0 | Type(4) | 000 | IST(3)
        // Present=1, Interrupt Gate=0xE (1110)
        // options = (P << 15) | (DPL << 13) | (Type << 8) | IST
        self.options = flags;
        self.offset_middle = (handler >> 16) as u16;
        self.offset_high = (handler >> 32) as u32;
        self.reserved = 0;
    }
}

/// Interrupt Descriptor Table.
#[repr(C, align(16))]
pub struct Idt {
    pub entries: [IdtEntry; 256],
}

impl Idt {
    pub const fn new() -> Self {
        Self {
            entries: [IdtEntry::missing(); 256],
        }
    }
}

#[repr(C, packed)]
struct IdtPtr {
    limit: u16,
    base: u64,
}

pub unsafe fn lidt(idt: &Idt) {
    let ptr = IdtPtr {
        limit: (size_of::<Idt>() - 1) as u16,
        base: idt as *const _ as u64,
    };
    unsafe { core::arch::asm!("lidt [{}]", in(reg) &ptr, options(nostack, preserves_flags)); }
}
