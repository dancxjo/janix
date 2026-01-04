//! Platform capability surface.
//!
//! The Platform layer wires machine facilities to kernel services.
//! It handles:
//! - Driver instantiation  
//! - Graph exposure of platform-specific Things
//! - Registration of capability providers

use alloc::boxed::Box;
use alloc::vec::Vec;
use spin::Mutex;

#[cfg(target_arch = "x86_64")]
mod x86_64;

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

    pub fn console_write(&self, bytes: &[u8]) {
        crate::machine::machine().console_write(bytes);
    }
}

static mut PLATFORM: Option<Platform> = None;

/// Initialize the platform layer.
///
/// This is the ONLY place where arch-specific dispatch occurs for
/// platform initialization. The kernel core calls this, not
/// arch-specific code directly.
#[allow(static_mut_refs)]
pub fn init() -> &'static Platform {
    unsafe {
        PLATFORM = Some(Platform::new());
        let platform = PLATFORM.as_ref().unwrap();
        register_bootstrap_providers(platform);

        // Dispatch to arch-specific platform init
        #[cfg(target_arch = "x86_64")]
        x86_64::init();

        platform
    }
}

#[allow(static_mut_refs)]
pub fn platform() -> &'static Platform {
    unsafe { PLATFORM.as_ref().expect("platform not initialized") }
}

fn register_bootstrap_providers(platform: &Platform) {
    platform.register(Box::new(ConsoleProvider));
}
