#![no_std]

//! Deprecated: this crate now only re-exports the canonical bridge traits.

pub use kernel::bridge::{
    CpuBridge, FullMachineBridge, HardwareBridge, MachineBridge, PortIo, Power, ProviderBridge,
    Rtc, VmMapper,
};
