pub trait HardwareBridge {
    fn log(&self, msg: &str);
    fn hhdm_offset(&self) -> u64;
    fn ticks(&self) -> u64;
    fn system_now(&self) -> u64;
    fn idle(&self);
    fn shutdown(&self) -> !;
    fn irq_disable(&self);
    fn irq_enable(&self);
    type Context: Copy + Clone + core::fmt::Debug + Default + Send + Sync + 'static;

    // Context size is bridge-specific but we use fixed 20 u64s for now as per Scheduler struct
    fn init_thread_context(&self, entry: u64, stack: u64, arg: u64) -> Self::Context;
    fn resume_user_mode(&self, context: &Self::Context) -> !;
    fn set_kernel_stack(&self, stack_top: u64);

    // Time
    fn rtc_read(&self, out: &mut abi::wire::time::RtcSample);

    /// Returns high-resolution monotonic time in nanoseconds.
    /// Returns 0 if not available/calibrated yet.
    fn monotonic_now(&self) -> u64;

    // IO
    fn port_outb(&self, port: u16, val: u8);
    fn port_inb(&self, port: u16) -> u8;
    fn port_outw(&self, port: u16, val: u16);
    fn port_inw(&self, port: u16) -> u16;
    fn port_outd(&self, port: u16, val: u32);
    fn port_ind(&self, port: u16) -> u32;

    fn save_fpu(&self, area: &mut [u8; 512]);
    fn restore_fpu(&self, area: &[u8; 512]);
}
