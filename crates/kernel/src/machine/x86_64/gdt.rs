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

    // 2. Setup GDT in canonical architecture order for syscall/sysretq
    // 0: Null
    // 8: Kernel Code
    // 16: Kernel Data
    // 24: User Code 32 (Dummy, required for sysretq alignment)
    // 32: User Data 64
    // 40: User Code 64
    let k_code = gdt_tss.gdt.add_entry(Descriptor::kernel_code_segment());
    let k_data = gdt_tss.gdt.add_entry(Descriptor::kernel_data_segment());

    // User segments (ordered for sysretq)
    // Base selector for STAR.UserBase will be u_code32
    let _u_code32 = gdt_tss.gdt.add_entry(Descriptor::user_code_segment()); // Dummy but present
    let u_data = gdt_tss.gdt.add_entry(Descriptor::user_data_segment());
    let u_code64 = gdt_tss.gdt.add_entry(Descriptor::user_code_segment());

    let tss = gdt_tss.gdt.add_entry(Descriptor::tss_segment(&gdt_tss.tss));

    crate::serial::write(b"GDT: k_code="); crate::serial::write_hex(k_code.0 as u64); crate::serial::write(b" k_data="); crate::serial::write_hex(k_data.0 as u64); crate::serial::write(b" u_data="); crate::serial::write_hex(u_data.0 as u64); crate::serial::write(b" u_code64="); crate::serial::write_hex(u_code64.0 as u64); crate::serial::write(b"\n"); gdt_tss.selectors = Selectors {
        kernel_code: k_code,
        kernel_data: k_data,
        user_code: u_code64,
        user_data: u_data,
        tss,
    };

    gdt_tss.gdt.load();

    // 3. Reload Segments
    CS::set_reg(k_code);
    DS::set_reg(k_data);
    ES::set_reg(k_data);
    SS::set_reg(k_data);
    use x86_64::instructions::segmentation::{FS, GS};
    FS::set_reg(k_data);
    GS::set_reg(k_data);

    load_tss(tss);
}

pub unsafe fn set_tss_rsp0(top: u64) {
    use crate::machine::x86_64::BSP_GDT;
    BSP_GDT.tss.privilege_stack_table[0] = VirtAddr::new(top);
}
