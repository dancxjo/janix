
#[repr(C)]
pub struct PerCpu {
    pub self_ptr: u64,
    pub kernel_rsp0: u64,
    pub user_rsp: u64,
    pub scratch_rax: u64,
}

pub static mut BSP_PERCPU: PerCpu = PerCpu {
    self_ptr: 0,
    kernel_rsp0: 0,
    user_rsp: 0,
    scratch_rax: 0,
};

pub unsafe fn init_gs_base() {
    let ptr = &raw mut BSP_PERCPU;
    
    unsafe {
        // Set self pointer
        (*ptr).self_ptr = ptr as u64;
    }

    // Load GS base
    let lo = ptr as u32;
    let hi = (ptr as u64 >> 32) as u32;
    
    unsafe {
        core::arch::asm!(
            "wrmsr",
            in("ecx") 0xC0000101u32, // IA32_GS_BASE
            in("eax") lo,
            in("edx") hi,
        );
    }
}

pub unsafe fn set_kernel_rsp0(rsp: u64) {
    unsafe {
        BSP_PERCPU.kernel_rsp0 = rsp;
    }
}
