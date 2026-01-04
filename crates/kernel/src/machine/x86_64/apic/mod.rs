//! Local APIC module for x86_64.
//!
//! This module contains all LAPIC-specific code including register access,
//! MSR manipulation, and timer configuration. No APIC knowledge escapes
//! this module.

pub mod lapic;
pub mod msr;
pub mod registers;

pub use lapic::{Lapic, LAPIC};
