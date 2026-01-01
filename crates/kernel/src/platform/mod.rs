//! Platform capability surface.
//!
//! The Platform trait represents the registry of capabilities available
//! to the kernel and drivers.


use alloc::vec::Vec;

/// A handle to a capability provider.
pub trait Provider: Sync {
    /// Invoke an operation on this provider.
    fn invoke(&self, op: u32, payload: &[u8]) -> Result<Vec<u8>, PlatformError>;
}

/// Error returned by platform calls.
#[derive(Debug)]
pub enum PlatformError {
    NotFound,
    BadVersion,
    ProviderError(u32),
}

/// The Platform capability registry.
pub struct Platform {
    // For now, use a simple static/global approach if std::collections not avail?
    // User said "spin::Mutex<Vec<Entry>>".
    // Keep it minimal.
}

impl Platform {
    pub const fn new() -> Self {
        Self {}
    }
}

static mut PLATFORM: Option<Platform> = None;

pub fn init() -> &'static Platform {
    unsafe {
        PLATFORM = Some(Platform::new());
        PLATFORM.as_ref().unwrap()
    }
}

pub fn platform() -> &'static Platform {
    unsafe { PLATFORM.as_ref().expect("platform not initialized") }
}

pub fn register_bootstrap_providers() {
    // Delegate to machine().console_write
    // Delegate to machine().mmio_map
}
