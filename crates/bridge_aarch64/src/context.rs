#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct ArchContext(pub [u64; 34]);

impl Default for ArchContext {
    fn default() -> Self {
        Self([0; 34])
    }
}

#[cfg(target_arch = "aarch64")]
pub fn init_thread_context(entry: u64, stack: u64, arg: u64) -> ArchContext {
    // [x0..x29, x30, sp_el0, elr, spsr]
    let mut ctx = [0u64; 34];

    let _uer = crate::user::UserEntryRegs {
        entry_point: entry,
        user_stack: stack,
        arg0: arg,
    };

    // SPSR_EL1 = 0 => Return to EL0t
    ctx[33] = 0;
    // ELR_EL1 = entry
    ctx[32] = entry;
    // SP_EL0 = stack
    ctx[31] = stack;
    // x0 = arg
    ctx[0] = arg;

    ArchContext(ctx)
}

#[cfg(target_arch = "aarch64")]
pub fn resume_user_mode(context: &ArchContext) -> ! {
    crate::user::enter::resume_user_mode(&context.0, &kernel::sched::fpu::FpuContext::default())
}

#[cfg(target_arch = "aarch64")]
pub fn set_kernel_stack(_stack: u64) {}
