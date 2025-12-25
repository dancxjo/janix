use x86_64::VirtAddr;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::instructions::segmentation::{CS, DS, ES, SS, Segment};
use x86_64::instructions::tables::load_tss;
use spin::{Mutex, Lazy};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

static mut TSS: TaskStateSegment = TaskStateSegment::new();
static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

// Selectors
pub static mut KERNEL_CODE_SELECTOR: SegmentSelector = SegmentSelector::new(0, x86_64::PrivilegeLevel::Ring0);
pub static mut KERNEL_DATA_SELECTOR: SegmentSelector = SegmentSelector::new(0, x86_64::PrivilegeLevel::Ring0);
pub static mut USER_CODE_SELECTOR: SegmentSelector = SegmentSelector::new(0, x86_64::PrivilegeLevel::Ring3);
pub static mut USER_DATA_SELECTOR: SegmentSelector = SegmentSelector::new(0, x86_64::PrivilegeLevel::Ring3);
pub static mut TSS_SELECTOR: SegmentSelector = SegmentSelector::new(0, x86_64::PrivilegeLevel::Ring0);

pub unsafe fn init() {
    // 1. Setup TSS
    // Allocate a stack for Double Faults?
    // We need a static buffer for IST.
    static mut IST_STACK: [u8; 4096] = [0; 4096];
    let stack_start = VirtAddr::from_ptr(&IST_STACK);
    let stack_end = stack_start + 4096u64;
    TSS.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = stack_end;
    TSS.privilege_stack_table[0] = stack_end; // Also use for Ring 0 stack? No, that's dangerous if shared.

    // 2. Setup GDT
    let k_code = GDT.add_entry(Descriptor::kernel_code_segment());
    let k_data = GDT.add_entry(Descriptor::kernel_data_segment());
    let u_data = GDT.add_entry(Descriptor::user_data_segment());
    let u_code = GDT.add_entry(Descriptor::user_code_segment());
    let tss = GDT.add_entry(Descriptor::tss_segment(&TSS));
    
    KERNEL_CODE_SELECTOR = k_code;
    KERNEL_DATA_SELECTOR = k_data;
    USER_DATA_SELECTOR = u_data;
    USER_CODE_SELECTOR = u_code;
    TSS_SELECTOR = tss;

    GDT.load();

    // 3. Reload Segments
    CS::set_reg(k_code);
    DS::set_reg(k_data);
    ES::set_reg(k_data);
    SS::set_reg(k_data);
    
    load_tss(tss);
}

pub unsafe fn set_kernel_stack(stack_top: u64) {
    let virt = VirtAddr::new(stack_top);
    // Writes to static mut TSS are unsafe
    TSS.privilege_stack_table[0] = virt;
}

