use crate::machine::Machine;

// Delegate to the existing internal implementation in crate::machine::x86_64
use crate::machine::x86_64::machine::ARCH_MACHINE as INNER;

// This wrapper is needed because ARCH_MACHINE must be a reference to a dyn Machine,
// and we want to unify the export location.
// Actually, INNER is ArchMachine struct. We can just export a reference to it.

pub static ARCH_MACHINE: &'static dyn Machine = &INNER;
