#![no_std]

pub trait HardwareBridge {
    fn log(&self, msg: &str);
    fn ticks(&self) -> u64;
    fn system_now(&self) -> u64;
    fn idle(&self);
    fn shutdown(&self) -> !;
    fn irq_disable(&self);
    fn irq_enable(&self);
    // Context size is bridge-specific but we use fixed 20 u64s for now as per Scheduler struct
    fn init_thread_context(&self, entry: u64, stack: u64, arg: u64) -> [u64; 34];
    fn resume_user_mode(&self, context: &[u64]) -> !;
    fn set_kernel_stack(&self, stack_top: u64);
}
