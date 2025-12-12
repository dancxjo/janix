use core::arch::asm;

/// Switch from the temporary boot stack to a kernel-owned stack and invoke the main entry point.
pub unsafe fn enter_kernel_stack(stack_base: u64, stack_size: u64) -> ! {

    #[cfg(target_arch = "aarch64")]
    {
        let stack_top = stack_base + stack_size;
        super::aarch64::trap::init();
        let entry = kmain_inner_asm;
        super::aarch64::trap::jump_to_el1_stack(stack_top, entry);
    }

    #[cfg(target_arch = "x86_64")]
    {
        // Use the full stack buffer (grows downward) instead of starting halfway
        // through it, which risked overrunning into the heap below when call
        // depth temporarily spiked during boot.
        let adjusted_stack_top = stack_base + stack_size;
        unsafe {
            asm!(
                "mov rsp, {stack}",
                "xor rbp, rbp",
                "call {entry}",
                stack = in(reg) adjusted_stack_top,
                entry = sym kmain_inner,
                options(noreturn)
            );
        }
    }

    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    {
        kmain_inner();
    }
}

unsafe extern "C" {
    fn kmain_inner() -> !;
}

#[cfg(target_arch = "aarch64")]
unsafe extern "C" {
    fn kmain_inner_asm() -> !;
}
