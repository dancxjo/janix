//! Interrupt abstraction layer.
//!
//! This module provides architecture-independent abstractions for interrupt
//! controllers and timers. The kernel core uses these traits without knowing
//! the underlying hardware (APIC, PIC, GIC, etc.).

pub mod controller;
pub mod timer;

pub use controller::{InterruptController, TimerMode};
pub use timer::timer_tick;
