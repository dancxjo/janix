#![allow(unused)]

use core::fmt::Debug;

/// Tier-0 CPU bridge: minimal primitives required by the kernel core.
pub trait CpuBridge {
    /// Best-effort logging; should be IRQ-safe.
    fn log(&self, msg: &str);

    /// Interrupt mask token used for nested-safe restore.
    type IrqState: Copy + Clone + Debug + Default;
    fn irq_disable(&self) -> Self::IrqState;
    fn irq_restore(&self, state: Self::IrqState);
    fn irq_enable(&self) {
        self.irq_restore(Self::IrqState::default())
    }

    /// Monotonic time base expressed in ticks plus its rate.
    fn ticks(&self) -> u64;
    fn ticks_per_second(&self) -> u64;

    /// Idle until the next interrupt; must return after an interrupt.
    fn idle(&self);

    /// Context bookkeeping.
    const CONTEXT_WORDS: usize;
    type Context: Copy + Clone + Debug + Default + Send + Sync + 'static;
    fn init_thread_context(&self, entry: u64, stack_top: u64, arg: u64) -> Self::Context;
    fn switch(&self, from: &mut Self::Context, to: &Self::Context);

    /// Kernel stack to use on trap/syscall entry from user.
    fn set_kernel_stack(&self, stack_top: u64);

    /// FPU/SIMD state management.
    type FpuState: Default + Copy + Clone + Debug;
    fn save_fpu(&self, out: &mut Self::FpuState);
    fn restore_fpu(&self, state: &Self::FpuState);

    /// Helper: convert ticks to nanoseconds using the reported rate.
    fn monotonic_nanos(&self) -> u64 {
        let tps = self.ticks_per_second();
        if tps == 0 {
            return 0;
        }
        let ticks = self.ticks() as u128;
        ((ticks * 1_000_000_000u128) / tps as u128) as u64
    }
    fn monotonic_now(&self) -> u64 {
        self.monotonic_nanos()
    }
}

/// Optional Tier-1 machine power control.
pub trait Power {
    fn shutdown(&self) -> !;
    fn reboot(&self) -> ! {
        self.shutdown()
    }
}

/// Optional Tier-1 RTC access.
pub trait Rtc {
    fn rtc_read(&self, out: &mut abi::wire::time::RtcSample);
}

/// Optional Tier-1 port I/O primitives.
pub trait PortIo: CpuBridge {
    fn port_outb(&self, port: u16, val: u8);
    fn port_inb(&self, port: u16) -> u8;
    fn port_outw(&self, port: u16, val: u16);
    fn port_inw(&self, port: u16) -> u16;
    fn port_outd(&self, port: u16, val: u32);
    fn port_ind(&self, port: u16) -> u32;
}

/// Optional Tier-1 machine glue.
pub trait MachineBridge: CpuBridge {
    fn hhdm_offset(&self) -> u64;
}

/// Optional Tier-1 memory helpers.
pub trait VmMapper: MachineBridge {
    fn map_new_user_page(&self, virt_addr: u64, flags: u64) -> Result<(), ()>;
    fn map_user_mmio(&self, virt_addr: u64, phys_addr: u64, flags: u64) -> Result<(), ()>;
}

/// Compatibility: combine common Tier-1 extensions.
pub trait FullMachineBridge: MachineBridge + PortIo + VmMapper + Power + Rtc {}

impl<T: MachineBridge + PortIo + VmMapper + Power + Rtc> FullMachineBridge for T {}

/// Provider-side bridge view (used by machine providers).
pub trait ProviderBridge {
    type IrqState: Copy + Clone + Debug + Default;
    fn monotonic_nanos(&self) -> u64;
    fn monotonic_now(&self) -> u64 {
        self.monotonic_nanos()
    }
    fn irq_disable(&self) -> Self::IrqState;
    fn irq_restore(&self, state: Self::IrqState);
}

impl<T: CpuBridge> ProviderBridge for T {
    type IrqState = T::IrqState;

    fn monotonic_nanos(&self) -> u64 {
        CpuBridge::monotonic_nanos(self)
    }

    fn irq_disable(&self) -> Self::IrqState {
        CpuBridge::irq_disable(self)
    }

    fn irq_restore(&self, state: Self::IrqState) {
        CpuBridge::irq_restore(self, state)
    }
}
