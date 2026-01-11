use core::mem::size_of;

pub const KERNEL_CODE_SEL: u16 = 0x08;
pub const KERNEL_DATA_SEL: u16 = 0x10;
pub const USER_CODE32_SEL: u16 = 0x18 | 3; // Dummy for sysret
pub const USER_DATA_SEL: u16 = 0x20 | 3;
pub const USER_CODE_SEL: u16 = 0x28 | 3;
pub const TSS_SEL: u16 = 0x30;

#[repr(C, packed)]
pub struct Tss {
    reserved1: u32,
    pub rsp0: u64,
    pub rsp1: u64,
    pub rsp2: u64,
    reserved2: u64,
    pub ist1: u64,
    pub ist2: u64,
    pub ist3: u64,
    pub ist4: u64,
    pub ist5: u64,
    pub ist6: u64,
    pub ist7: u64,
    reserved3: u64,
    reserved4: u16,
    pub iomap_base: u16,
}

impl Tss {
    pub const fn new() -> Self {
        Self {
            reserved1: 0,
            rsp0: 0, rsp1: 0, rsp2: 0,
            reserved2: 0,
            ist1: 0, ist2: 0, ist3: 0, ist4: 0, ist5: 0, ist6: 0, ist7: 0,
            reserved3: 0,
            reserved4: 0,
            iomap_base: size_of::<Tss>() as u16,
        }
    }
}

pub static mut TSS: Tss = Tss::new();

#[repr(C)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

#[repr(C)]
struct GdtSystemEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
    base_upper: u32,
    reserved: u32,
}

impl GdtEntry {
    const fn new(base: u64, limit: u32, access: u8, flags: u8) -> Self {
        Self {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            access,
            granularity: ((limit >> 16) & 0x0F) as u8 | (flags & 0xF0),
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }
}

#[repr(C)]
#[repr(align(4096))] // Align page for hygiene
struct Gdt {
    null: GdtEntry,
    kcode: GdtEntry,
    kdata: GdtEntry,
    ucode32: GdtEntry,
    udata: GdtEntry,
    ucode64: GdtEntry,
    tss: GdtSystemEntry,
}

static mut GDT: Gdt = Gdt {
    null: GdtEntry::new(0, 0, 0, 0),
    kcode: GdtEntry::new(0, 0, 0x9A, 0xA0), // Present, Ring 0, Code, Exec/Read, Long Mode
    kdata: GdtEntry::new(0, 0, 0x92, 0xC0), // Present, Ring 0, Data, Read/Write
    ucode32: GdtEntry::new(0, 0xFFFFF, 0xFA, 0xCF), // Present, Ring 3, Code, 32-bit (DB=1, L=0)
    udata: GdtEntry::new(0, 0, 0xF2, 0xC0), // Present, Ring 3, Data, Read/Write
    ucode64: GdtEntry::new(0, 0, 0xFA, 0xA0), // Present, Ring 3, Code, 64-bit (L=1)
    tss: GdtSystemEntry {
        limit_low: 0,
        base_low: 0,
        base_middle: 0,
        access: 0x89, // Present, Ring 0, TSS Available (0x9)
        granularity: 0,
        base_high: 0,
        base_upper: 0,
        reserved: 0,
    },
};

#[repr(C, packed)]
struct GdtDescriptor {
    size: u16,
    offset: u64,
}

pub unsafe fn init() {
    // Setup TSS entry in GDT
    let tss_base = core::ptr::addr_of!(TSS) as u64;
    let tss_limit = size_of::<Tss>() as u32 - 1;
    
    unsafe {
        GDT.tss.limit_low = (tss_limit & 0xFFFF) as u16;
        GDT.tss.base_low = (tss_base & 0xFFFF) as u16;
        GDT.tss.base_middle = ((tss_base >> 16) & 0xFF) as u8;
        GDT.tss.access = 0x89; // Present, Bit 0 cleared (Busy=0)
        GDT.tss.granularity = 0; // Byte granularity
        GDT.tss.base_high = ((tss_base >> 24) & 0xFF) as u8;
        GDT.tss.base_upper = (tss_base >> 32) as u32;
        GDT.tss.reserved = 0;
    }

    let gdtr = GdtDescriptor {
        size: (size_of::<Gdt>() - 1) as u16,
        offset: core::ptr::addr_of!(GDT) as u64,
    };

    unsafe {
        core::arch::asm!("lgdt [{}]", in(reg) &gdtr);
        
        // Reload segments
        core::arch::asm!(
            "push {sel}",
            "lea rax, [rip + 2f]",
            "push rax",
            "retfq",
            "2:",
            "mov ax, {dsel}",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            sel = const KERNEL_CODE_SEL,
            dsel = const KERNEL_DATA_SEL,
            out("rax") _,
        );

        // Load TSS
        core::arch::asm!("ltr {sel:x}", sel = in(reg) TSS_SEL);
    }
}

pub unsafe fn set_rsp0(rsp0: u64) {
    unsafe {
        TSS.rsp0 = rsp0;
    }
}

pub unsafe fn set_ist1(stack_top: u64) {
    unsafe {
        TSS.ist1 = stack_top;
    }
}
