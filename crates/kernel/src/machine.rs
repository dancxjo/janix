//! Kernel domain models and errors
//!
//! This module contains shared types that are not tied to a specific
//! bootloader or architecture.

use core::fmt;

/// Opaque token for IRQ state save/restore
#[derive(Clone, Copy)]
pub struct IrqToken(pub u64);

/// Error type for kernel operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Operation not implemented
    NotImplemented,
    /// Module not found
    NotFound,
    /// Invalid argument
    InvalidArgument,
    /// Out of memory
    OutOfMemory,
    /// Permission denied
    PermissionDenied,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotImplemented => write!(f, "not implemented"),
            Error::NotFound => write!(f, "not found"),
            Error::InvalidArgument => write!(f, "invalid argument"),
            Error::OutOfMemory => write!(f, "out of memory"),
            Error::PermissionDenied => write!(f, "permission denied"),
        }
    }
}
