//! Platform capability surface.
//!
//! The Platform trait represents the registry of capabilities available
//! to the kernel and drivers.


use alloc::vec::Vec;

/// A handle to a capability provider.
use spin::Mutex;
use alloc::boxed::Box;

/// A handle to a capability provider.
pub trait Provider: Send + Sync {
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

struct ConsoleProvider;

impl Provider for ConsoleProvider {
    fn invoke(&self, _op: u32, payload: &[u8]) -> Result<Vec<u8>, PlatformError> {
        // Op 0: write
        crate::machine::machine().console_write(payload);
        Ok(Vec::new())
    }
}

/// The Platform capability registry.
pub struct Platform {
    providers: Mutex<Vec<Box<dyn Provider>>>,
}

impl Platform {
    pub const fn new() -> Self {
        Self {
            providers: Mutex::new(Vec::new()),
        }
    }

    pub fn register(&self, provider: Box<dyn Provider>) {
        self.providers.lock().push(provider);
    }
    
    // Simple helper for now, usually we'd dispatch by ID
    pub fn console_write(&self, bytes: &[u8]) {
        // Optimization: direct machine call for now, or find console provider
        // For strict correctness with "delegate":
        crate::machine::machine().console_write(bytes);
    }
}

static mut PLATFORM: Option<Platform> = None;

pub fn init() -> &'static Platform {
    unsafe {
        PLATFORM = Some(Platform::new());
        let platform = PLATFORM.as_ref().unwrap();
        register_bootstrap_providers(platform);
        platform
    }
}

pub fn platform() -> &'static Platform {
    unsafe { PLATFORM.as_ref().expect("platform not initialized") }
}

fn register_bootstrap_providers(platform: &Platform) {
    platform.register(Box::new(ConsoleProvider));
}
