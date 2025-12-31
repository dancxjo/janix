//! Capability system (stub)
//!
//! Will provide capability-based access control.
//! Currently returns ENOSYS for all operations.

use crate::syscall::err::ENOSYS;

/// Capability identifier
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CapId(pub u64);

/// Capability type
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CapType {
    /// Read-only access
    Read,
    /// Write access
    Write,
    /// Execute access
    Execute,
    /// Full access
    Full,
}

/// Grant a capability
pub fn grant(_target: u64, _cap_type: CapType) -> Result<CapId, i32> {
    Err(ENOSYS)
}

/// Revoke a capability
pub fn revoke(_cap: CapId) -> Result<(), i32> {
    Err(ENOSYS)
}

/// Check if a capability is valid
pub fn check(_cap: CapId, _required: CapType) -> Result<bool, i32> {
    Err(ENOSYS)
}
