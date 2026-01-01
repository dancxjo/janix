pub mod serial;

pub static ARCH_MACHINE: &'static dyn crate::machine::Machine = &PlaceholderMachine;

struct PlaceholderMachine;

impl crate::machine::Machine for PlaceholderMachine {
    fn console_write(&self, _bytes: &[u8]) {}
    fn mmio_map(&self, _addr: usize, _size: usize) -> Option<usize> { None }
    fn irq_disable(&self) -> u64 { 0 }
    fn irq_restore(&self, _flags: u64) {}
    fn halt(&self) -> ! { loop {} }

    fn switch_to(&self, _next: &mut crate::machine::Context) {}
    fn task_entry_stub(&self) -> u64 { 0 }
}
