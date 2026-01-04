//! Interrupt controller abstraction.
//!
//! This trait defines the interface that the kernel uses to interact with
//! interrupt controllers. Implementations (LAPIC, PIC, GIC) live in the
//! machine layer.

/// Timer operating mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerMode {
    /// Timer fires once after the specified ticks.
    OneShot,
    /// Timer fires repeatedly at the specified interval.
    Periodic,
}

/// Abstract interrupt controller interface.
///
/// The kernel core interacts with interrupt hardware only through this trait.
/// No APIC/PIC/GIC register knowledge escapes the machine layer.
pub trait InterruptController: Send + Sync {
    /// Initialize the interrupt controller hardware.
    fn init(&self);

    /// Enable interrupt delivery from this controller.
    fn enable(&self);

    /// Disable interrupt delivery from this controller.
    fn disable(&self);

    /// Send End-Of-Interrupt signal.
    fn eoi(&self);

    /// Configure the timer with the specified mode and tick count.
    fn set_timer(&self, mode: TimerMode, ticks: u32);

    /// Get the timer frequency in Hz.
    fn timer_frequency_hz(&self) -> u32;

    /// Get the timer resolution in nanoseconds.
    fn timer_resolution_ns(&self) -> u64 {
        let freq = self.timer_frequency_hz() as u64;
        if freq > 0 { 1_000_000_000 / freq } else { 0 }
    }

    /// Get the controller's local ID (e.g., APIC ID for LAPIC).
    fn local_id(&self) -> u32 { 0 }
}

// Global interrupt controller reference
static mut INTERRUPT_CONTROLLER: Option<&'static dyn InterruptController> = None;

/// Install the interrupt controller implementation.
pub unsafe fn install(controller: &'static dyn InterruptController) {
    INTERRUPT_CONTROLLER = Some(controller);
}

/// Get the installed interrupt controller.
pub fn controller() -> &'static dyn InterruptController {
    unsafe { INTERRUPT_CONTROLLER.expect("interrupt controller not installed") }
}

/// Send EOI to the interrupt controller.
pub fn eoi() {
    controller().eoi();
}
