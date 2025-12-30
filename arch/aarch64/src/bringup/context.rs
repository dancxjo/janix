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

    let _uer = super::user::UserEntryRegs {
        entry_point: entry,
        user_stack: stack,
        arg0: arg,
    };

    // SPSR
    // If entry is high half (Kernel), use EL1h (0x5) to use SP_EL1 stack.
    // If entry is low half (User), use EL0t (0x0).
    if entry & (1 << 63) != 0 {
        ctx[33] = 0x05; // EL1h
                        // SP_EL0 is not used as stack in EL1h mode
        ctx[31] = 0;
    } else {
        ctx[33] = 0x00; // EL0t
                        // SP_EL0 = stack
        ctx[31] = stack;
    }
    // ELR_EL1 = entry
    ctx[32] = entry;
    // x0 = arg
    ctx[0] = arg;

    ArchContext(ctx)
}

#[cfg(target_arch = "aarch64")]
pub fn resume_user_mode(context: &ArchContext) -> ! {
    super::user::enter::resume_user_mode(&context.0)
}

#[cfg(target_arch = "aarch64")]
pub fn set_kernel_stack(_stack: u64) {}
