use x86_64::instructions::segmentation::{Segment, CS, DS, ES, SS};
use x86_64::instructions::tables::load_tss;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

pub struct GdtTss {
    pub gdt: GlobalDescriptorTable,
    pub tss: TaskStateSegment,
    pub selectors: Selectors,
}

#[derive(Clone, Copy, Debug)]
pub struct Selectors {
    pub kernel_code: SegmentSelector,
    pub kernel_data: SegmentSelector,
    pub user_code: SegmentSelector,
    pub user_data: SegmentSelector,
    pub tss: SegmentSelector,
}

impl GdtTss {
    pub const fn new() -> Self {
        Self {
            gdt: GlobalDescriptorTable::new(),
            tss: TaskStateSegment::new(),
            selectors: Selectors {
                kernel_code: SegmentSelector::new(0, x86_64::PrivilegeLevel::Ring0),
                kernel_data: SegmentSelector::new(0, x86_64::PrivilegeLevel::Ring0),
                user_code: SegmentSelector::new(0, x86_64::PrivilegeLevel::Ring3),
                user_data: SegmentSelector::new(0, x86_64::PrivilegeLevel::Ring3),
                tss: SegmentSelector::new(0, x86_64::PrivilegeLevel::Ring0),
            },
        }
    }
}

pub unsafe fn init(gdt_tss: &'static mut GdtTss) {
    // 1. Setup TSS
    // TODO: Allocate dedicated IST stack properly
    static mut IST_STACK: [u8; 4096] = [0; 4096];
    let stack_start = VirtAddr::from_ptr(core::ptr::addr_of!(IST_STACK));
    let stack_end = stack_start + 4096u64;
    gdt_tss.tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = stack_end;
    gdt_tss.tss.privilege_stack_table[0] = stack_end;

    // 2. Setup GDT
    let k_code = gdt_tss.gdt.add_entry(Descriptor::kernel_code_segment());
    let k_data = gdt_tss.gdt.add_entry(Descriptor::kernel_data_segment());
    let u_data = gdt_tss.gdt.add_entry(Descriptor::user_data_segment());
    let u_code = gdt_tss.gdt.add_entry(Descriptor::user_code_segment());
    let tss = gdt_tss.gdt.add_entry(Descriptor::tss_segment(&gdt_tss.tss));

    gdt_tss.selectors = Selectors {
        kernel_code: k_code,
        kernel_data: k_data,
        user_code: u_code,
        user_data: u_data,
        tss,
    };

    gdt_tss.gdt.load();

    // 3. Reload Segments
    CS::set_reg(k_code);
    DS::set_reg(k_data);
    ES::set_reg(k_data);
    SS::set_reg(k_data);

    load_tss(tss);
}
