use kernel::kerror;

#[repr(C)]
#[derive(Debug)]
pub struct ExceptionStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn exception_handler(
    stack_frame: *const ExceptionStackFrame,
    vector: u64,
    error_code: u64,
) {
    let frame = unsafe { &*stack_frame };
    let cr2: u64;
    unsafe { core::arch::asm!("mov {}, cr2", out(reg) cr2, options(nomem, nostack, preserves_flags)); }
    
    kerror!("EXCEPTION: VECTOR: {:#x} ERROR: {:#x}", vector, error_code);
    kerror!("RIP: {:#x} CS: {:#x} RFLAGS: {:#x}", frame.instruction_pointer, frame.code_segment, frame.cpu_flags);
    kerror!("RSP: {:#x} SS: {:#x}", frame.stack_pointer, frame.stack_segment);
    kerror!("CR2: {:#x}", cr2);
    
    unsafe {
         debug_page_walk(cr2);
    }
    
    panic!("Unrecoverable exception");
}

unsafe fn debug_page_walk(virt: u64) {
    use super::paging::phys_to_virt;
    let cr3: u64;
    unsafe { core::arch::asm!("mov {}, cr3", out(reg) cr3, options(nomem, nostack, preserves_flags)); }
    let pml4_phys = cr3 & !0xFFF;
    kerror!("PT DUMP: Virt {:#x} | CR3 {:#x}", virt, cr3);

    let pml4 = phys_to_virt(pml4_phys) as *const u64;
    let pml4_idx = (virt >> 39) & 0x1FF;
    let pml4e = unsafe { *pml4.add(pml4_idx as usize) };
    kerror!("PML4[{}] = {:#x}", pml4_idx, pml4e);
    
    if (pml4e & 1) == 0 { return; }
    
    let pdp_phys = pml4e & 0x000FFFFFFFFFF000;
    let pdp = phys_to_virt(pdp_phys) as *const u64;
    let pdp_idx = (virt >> 30) & 0x1FF;
    let pdpe = unsafe { *pdp.add(pdp_idx as usize) };
    kerror!("PDP[{}] = {:#x}", pdp_idx, pdpe);
    
    if (pdpe & 1) == 0 { return; }
    if (pdpe & 0x80) != 0 { kerror!("PDP Huge Page"); return; }
    
    let pd_phys = pdpe & 0x000FFFFFFFFFF000;
    let pd = phys_to_virt(pd_phys) as *const u64;
    let pd_idx = (virt >> 21) & 0x1FF;
    let pde = unsafe { *pd.add(pd_idx as usize) };
    kerror!("PD[{}] = {:#x}", pd_idx, pde);

    if (pde & 1) == 0 { return; }
    if (pde & 0x80) != 0 { kerror!("PD Huge Page"); return; }
    
    let pt_phys = pde & 0x000FFFFFFFFFF000;
    let pt = phys_to_virt(pt_phys) as *const u64;
    let pt_idx = (virt >> 12) & 0x1FF;
    let pte = unsafe { *pt.add(pt_idx as usize) };
    kerror!("PT[{}] = {:#x}", pt_idx, pte);
}
