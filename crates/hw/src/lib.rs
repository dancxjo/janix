#![no_std]

//! Hardware Bridge trait - the canonical definition is in `kernel::bridge`.
//! This crate is deprecated; use `kernel::bridge::HardwareBridge` instead.

/// See `kernel::bridge::HardwareBridge` for the canonical definition.
/// This is a minimal stub for backwards compatibility.
#[deprecated(note = "Use kernel::bridge::HardwareBridge instead")]
pub trait HardwareBridge {
    fn log(&self, msg: &str);
    fn ticks(&self) -> u64;
    fn idle(&self);
    fn shutdown(&self) -> !;

    type IrqState: Copy + Clone + core::fmt::Debug + Default;
    fn irq_disable(&self) -> Self::IrqState;
    fn irq_restore(&self, state: Self::IrqState);

    type Context: Copy + Clone + core::fmt::Debug + Default + Send + Sync + 'static;
    fn init_thread_context(&self, entry: u64, stack: u64, arg: u64) -> Self::Context;
    fn resume_user_mode(&self, context: &Self::Context) -> !;
    fn set_kernel_stack(&self, stack_top: u64);

    fn rtc_read(&self, out: &mut abi::wire::time::RtcSample);
    fn monotonic_now(&self) -> u64;

    type FpuState: Default + Copy + Clone + core::fmt::Debug;
    fn save_fpu(&self, out: &mut Self::FpuState);
    fn restore_fpu(&self, state: &Self::FpuState);
}
