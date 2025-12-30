pub use crate::arch::bridge::{
    CpuBridge, FullMachineBridge, MachineBridge, PortIo, Power, ProviderBridge, Rtc, VmMapper,
};

/// Compatibility shim: legacy HardwareBridge name maps to the full in-kernel bridge stack.
#[deprecated(note = "Use CpuBridge / MachineBridge / Rtc / Power instead")]
pub trait HardwareBridge: FullMachineBridge {}

impl<T: FullMachineBridge> HardwareBridge for T {}
