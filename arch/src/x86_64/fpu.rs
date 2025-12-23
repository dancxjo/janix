use kernel::sched::FpuContext;
use x86_64::registers::control::{Cr0, Cr0Flags, Cr4, Cr4Flags};

pub fn init() {
    unsafe {
        let mut cr0 = Cr0::read();
        cr0.remove(Cr0Flags::EMULATE_COPROCESSOR);
        cr0.insert(Cr0Flags::MONITOR_COPROCESSOR);
        Cr0::write(cr0);

        let mut cr4 = Cr4::read();
        cr4.insert(Cr4Flags::OSFXSR | Cr4Flags::OSXMMEXCPT_ENABLE);
        Cr4::write(cr4);
    }
}

pub fn save_fpu(ctx: &mut FpuContext) {
    // FpuContext is aligned to 16 bytes, so data array should be aligned.
    let ptr = ctx.data.as_mut_ptr();
    unsafe {
        core::arch::x86_64::_fxsave(ptr);
    }
}
