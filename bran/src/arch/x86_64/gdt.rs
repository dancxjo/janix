
use core::mem::size_of;

// Descriptors
pub const KERNEL_CODE_IDX: u16 = 1;
pub const KERNEL_DATA_IDX: u16 = 2;
pub const USER_CODE32_IDX: u16 = 3;
pub const USER_DATA_IDX: u16 = 4;
pub const USER_CODE64_IDX: u16 = 5;
pub const TSS_IDX: u16 = 6;

// Selectors
pub const KERNEL_CODE_SEL: u16 = KERNEL_CODE_IDX << 3;
pub const KERNEL_DATA_SEL: u16 = KERNEL_DATA_IDX << 3;

// User selectors (RPL=3)
pub const USER_DATA_SEL: u16 = (USER_DATA_IDX << 3) | 3;
pub const USER_CODE64_SEL: u16 = (USER_CODE64_IDX << 3) | 3;

// GDT Entry
#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
pub struct Descriptor {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

impl Descriptor {
    pub const fn null() -> Self {
        Self { limit_low: 0, base_low: 0, base_middle: 0, access: 0, granularity: 0, base_high: 0 }
    }

    pub const fn kernel_code() -> Self {
        // Present, Ring0, Code/Data, Executable, Readable
        // Access: P=1, DPL=00, S=1, Type=1010 (Code, Read) -> 10011010b = 0x9A
        // Flags: L=1 (Long mode), D=0
        // Granularity: 0xA0 (L=1, D=0)
        Self {
            limit_low: 0, base_low: 0, base_middle: 0,
            access: 0x9A,
            granularity: 0x20, // Long mode
            base_high: 0,
        }
    }

    pub const fn kernel_data() -> Self {
        // Access: P=1, DPL=00, S=1, Type=0010 (Data, Write) -> 10010010b = 0x92
        // Apps don't care about limits in long mode usually
        Self {
            limit_low: 0, base_low: 0, base_middle: 0,
            access: 0x92,
            granularity: 0,
            base_high: 0,
        }
    }

    pub const fn user_code32() -> Self {
        // Dummy 32-bit user code
        // Access: P=1, DPL=11, S=1, Type=1010 -> 11111010b = 0xFA
        // Granularity: 0x40 (D=1=32bit) ? Or just 0?
        // SYSRET doesn't use it, just needs slot.
        Self {
            limit_low: 0xFFFF, base_low: 0, base_middle: 0,
            access: 0xFA,
            granularity: 0xCF, // 4KB, 32-bit
            base_high: 0,
        }
    }

    pub const fn user_data() -> Self {
        // Access: P=1, DPL=11, S=1, Type=0010 -> 11110010b = 0xF2
        Self {
            limit_low: 0, base_low: 0, base_middle: 0,
            access: 0xF2,
            granularity: 0,
            base_high: 0,
        }
    }

    pub const fn user_code64() -> Self {
        // Access: P=1, DPL=11, S=1, Type=1010 -> 11111010b = 0xFA
        // Flags: L=1
        Self {
            limit_low: 0, base_low: 0, base_middle: 0,
            access: 0xFA,
            granularity: 0x20,
            base_high: 0,
        }
    }

    pub fn tss(tss: &'static TaskStateSegment) -> (Self, Self) {
        let ptr = tss as *const _ as u64;
        let size = size_of::<TaskStateSegment>() as u64 - 1;

        let low = Self {
            limit_low: size as u16,
            base_low: ptr as u16,
            base_middle: (ptr >> 16) as u8,
            access: 0x89, // Present, Ring0, System, Type=9 (Available TSS) -> 10001001
            granularity: 0x00, // No specific granularity for TSS limit usually bytes
            base_high: (ptr >> 24) as u8,
        };
        
        let high = Self {
            limit_low: (ptr >> 32) as u16,
            base_low: (ptr >> 48) as u16,
            base_middle: 0,
            access: 0,
            granularity: 0,
            base_high: 0,
        };

        (low, high)
    }
}

// Minimal TSS
#[repr(C, packed)]
pub struct TaskStateSegment {
    reserved1: u32,
    pub rsp0_low: u32,
    pub rsp0_high: u32,
    pub rsp1_low: u32,
    pub rsp1_high: u32,
    pub rsp2_low: u32,
    pub rsp2_high: u32,
    reserved2: [u64; 10], // ISTs etc (simplified)
    iomap_base: u16,
}

impl TaskStateSegment {
    pub const fn new() -> Self {
        Self {
            reserved1: 0,
            rsp0_low: 0, rsp0_high: 0,
            rsp1_low: 0, rsp1_high: 0,
            rsp2_low: 0, rsp2_high: 0,
            reserved2: [0; 10],
            iomap_base: 0,
        }
    }
}

pub struct Gdt {
    entries: [Descriptor; 8],
    pub tss: TaskStateSegment,
}

pub static mut BSP_GDT: Gdt = Gdt {
    entries: [Descriptor::null(); 8],
    tss: TaskStateSegment::new(),
};

#[repr(align(16))]
struct Stack([u8; 4096]);
static mut DOUBLE_FAULT_STACK: Stack = Stack([0; 4096]);

pub unsafe fn init() {
    // Avoid mutable reference to mutable static (UB)
    let gdt = unsafe { &mut *core::ptr::addr_of_mut!(BSP_GDT) };
    
    // Fill entries
    gdt.entries[KERNEL_CODE_IDX as usize] = Descriptor::kernel_code();
    gdt.entries[KERNEL_DATA_IDX as usize] = Descriptor::kernel_data();
    gdt.entries[USER_CODE32_IDX as usize] = Descriptor::user_code32();
    gdt.entries[USER_DATA_IDX as usize] = Descriptor::user_data();
    gdt.entries[USER_CODE64_IDX as usize] = Descriptor::user_code64();

    // TSS
    let (tss_low, tss_high) = Descriptor::tss(&gdt.tss);
    gdt.entries[TSS_IDX as usize] = tss_low;
    gdt.entries[TSS_IDX as usize + 1] = tss_high;

    // Load GDT
    let limit = (size_of::<[Descriptor; 8]>() - 1) as u16;
    let base = gdt.entries.as_ptr() as u64;
    
    let gdtr = DescriptorPtr { limit, base };
    unsafe { lgdt(&gdtr) };

    // Reload segments via ASM
    unsafe { reload_segments(KERNEL_CODE_SEL, KERNEL_DATA_SEL) };

    // Load TSS
    unsafe { ltr(TSS_IDX << 3) };
    
    // Set Double Fault Stack
    unsafe {
        let stack_top = core::ptr::addr_of_mut!(DOUBLE_FAULT_STACK.0) as u64 + 4096;
        set_double_fault_stack(stack_top);
    }
}

pub unsafe fn set_tss_rsp0(stack: u64) {
    unsafe {
        BSP_GDT.tss.rsp0_low = stack as u32;
        BSP_GDT.tss.rsp0_high = (stack >> 32) as u32;
    }
}

pub unsafe fn set_kernel_stack(stack: u64) {
    unsafe { set_tss_rsp0(stack) }
}

pub unsafe fn set_double_fault_stack(stack: u64) {
    unsafe {
        // IST 1 is the first entry in reserved2 (index 0)
        // reserved2 is [u64; 10]
        BSP_GDT.tss.reserved2[0] = stack;
    }
}

#[repr(C, packed)]
struct DescriptorPtr {
    limit: u16,
    base: u64,
}

unsafe fn lgdt(gdtr: &DescriptorPtr) {
    unsafe { core::arch::asm!("lgdt [{}]", in(reg) gdtr) };
}

unsafe fn ltr(sel: u16) {
    unsafe { core::arch::asm!("ltr {:x}", in(reg) sel) };
}

unsafe fn reload_segments(cs: u16, ds: u16) {
    unsafe {
        core::arch::asm!(
            "push {0:r}",
            "lea {1:r}, [2f + rip]",
            "push {1:r}",
            "retfq",
            "2:",
            "mov ds, {2:x}",
            "mov es, {2:x}",
            "mov ss, {2:x}",
            in(reg) cs as u64,
            out(reg) _,
            in(reg) ds,
        );
    }
}
