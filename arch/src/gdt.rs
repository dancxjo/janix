use lazy_static::lazy_static;
use x86_64::VirtAddr;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;

// IST index 0 is a special value that means "no IST" in the IDT entry.
// Use index 1 to reserve the first IST entry for double-fault handling.
pub const DOUBLE_FAULT_IST_INDEX: u16 = 1;

const PRIVILEGE_STACK_GUARD_BYTES: usize = 16 * 1024;
const PRIVILEGE_STACK_BYTES: usize = 256 * 1024;

#[repr(align(16))]
struct Stack<const N: usize>([u8; N]);

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: Stack<STACK_SIZE> = Stack([0; STACK_SIZE]);

            let stack_start = VirtAddr::from_ptr(core::ptr::addr_of!(STACK));
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        // Set privilege stack for ring 0 (needed for ring 3 -> ring 0 transition)
        tss.privilege_stack_table[0] = {
            static mut STACK: Stack<{ PRIVILEGE_STACK_GUARD_BYTES + PRIVILEGE_STACK_BYTES }> =
                Stack([0; PRIVILEGE_STACK_GUARD_BYTES + PRIVILEGE_STACK_BYTES]);

            // Leave a guard region at the bottom so accidental underflow from the
            // kernel privilege stack won't immediately trample adjacent globals
            // (like the heap allocator) in .bss.
            let stack_start = VirtAddr::from_ptr(core::ptr::addr_of!(STACK))
                + PRIVILEGE_STACK_GUARD_BYTES;
            let stack_end = stack_start + PRIVILEGE_STACK_BYTES;
            stack_end
        };
        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let kcode = gdt.add_entry(Descriptor::kernel_code_segment());
        let kdata = gdt.add_entry(Descriptor::kernel_data_segment());
        let tss = gdt.add_entry(Descriptor::tss_segment(&TSS));
        let udata = gdt.add_entry(Descriptor::user_data_segment());
        let ucode = gdt.add_entry(Descriptor::user_code_segment());
        (
            gdt,
            Selectors {
                kcode,
                kdata,
                tss,
                ucode,
                udata,
            },
        )
    };
}

pub struct Selectors {
    pub kcode: SegmentSelector,
    pub kdata: SegmentSelector,
    pub tss: SegmentSelector,
    pub ucode: SegmentSelector,
    pub udata: SegmentSelector,
}

pub fn init() {
    use x86_64::instructions::segmentation::{CS, DS, ES, FS, GS, SS, Segment};
    use x86_64::instructions::tables::load_tss;

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.kcode);
        DS::set_reg(GDT.1.kdata);
        ES::set_reg(GDT.1.kdata);
        SS::set_reg(GDT.1.kdata);
        FS::set_reg(SegmentSelector(0));
        GS::set_reg(SegmentSelector(0));
        load_tss(GDT.1.tss);
    }
    kernel::println!(
        "GDT initialized. TSS RSP0: {:#x}",
        TSS.privilege_stack_table[0].as_u64()
    );
}

pub fn get_selectors() -> &'static Selectors {
    &GDT.1
}

pub fn kernel_stack_top() -> u64 {
    TSS.privilege_stack_table[0].as_u64()
}

pub fn kernel_stack_ist1_top() -> u64 {
    TSS.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize].as_u64()
}

#[repr(C)]
pub struct PerCpu {
    pub kernel_rsp: u64,
    pub user_rsp: u64,
    pub scratch: u64,
}

pub static mut PER_CPU: PerCpu = PerCpu {
    kernel_rsp: 0,
    user_rsp: 0,
    scratch: 0,
};

pub unsafe fn init_per_cpu() {
    use x86_64::registers::model_specific::KernelGsBase;

    // Set up the PerCpu structure
    PER_CPU.kernel_rsp = kernel_stack_top();

    let addr = x86_64::VirtAddr::from_ptr(core::ptr::addr_of!(PER_CPU));
    let rsp_val = core::ptr::addr_of!(PER_CPU.kernel_rsp).read();
    kernel::println!("PerCpu init: addr={:?} kernel_rsp={:#x}", addr, rsp_val);
    KernelGsBase::write(addr);
    let check = KernelGsBase::read();
    if check.as_u64() != addr.as_u64() {
        kernel::println!(
            "FATAL: KernelGsBase write failed! read={:?} expected={:?}",
            check,
            addr
        );
        loop {}
    }
    kernel::println!(
        "PerCpu init: written to MSR 0xC0000102. Verified: {:?}",
        check
    );

    // Switch to Kernel GS immediately so we run with correct context.
    // This ensures enter_user_mode (which does swapgs) sees Kernel GS as active.
    x86_64::instructions::segmentation::swap_gs();
}
