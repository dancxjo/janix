#![allow(unused)]

use core::fmt::Debug;

/// Minimal user page flags used across architectures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UserPageFlags(pub u64);

impl UserPageFlags {
    pub const NONE: Self = Self(0);
    pub const READ: Self = Self(1 << 0);
    pub const WRITE: Self = Self(1 << 1);
    pub const EXEC: Self = Self(1 << 2);
    pub const USER: Self = Self(1 << 3);
    pub const DEVICE: Self = Self(1 << 4);

    pub const RW: Self = Self(Self::READ.0 | Self::WRITE.0);

    pub fn bits(self) -> u64 {
        self.0
    }

    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl core::ops::BitOr for UserPageFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        UserPageFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for UserPageFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for UserPageFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        UserPageFlags(self.0 & rhs.0)
    }
}

impl core::ops::BitAndAssign for UserPageFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

/// Per-architecture user address space control used by the kernel spawn path.
pub trait UserAddressSpace {
    type Root;

    /// Create a fresh user root page table / address space anchor.
    unsafe fn create_user_root() -> Self::Root;

    /// Map a single user page with the requested flags.
    unsafe fn map_user_page(root: &mut Self::Root, vaddr: u64, paddr: u64, flags: UserPageFlags);

    /// Allocate a zeroed physical frame.
    unsafe fn alloc_frame() -> u64;

    /// Activate the given user root (CR3 / TTBR0).
    unsafe fn activate_user_root(root: &Self::Root);

    /// Write bytes into user space, mapping pages as needed with the provided flags.
    unsafe fn write_user(
        root: &mut Self::Root,
        vaddr: u64,
        bytes: &[u8],
        writable_flags: UserPageFlags,
    );

    /// Ensure instruction cache coherency for the given range.
    unsafe fn sync_icache(vaddr: u64, len: usize);
}

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
pub trait FullMachineBridge:
    MachineBridge + PortIo + VmMapper + Power + Rtc + UserAddressSpace
{
}

impl<T: MachineBridge + PortIo + VmMapper + Power + Rtc + UserAddressSpace> FullMachineBridge
    for T
{
}

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
