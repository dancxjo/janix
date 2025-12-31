//! Machine abstraction layer
//!
//! Defines the arch-neutral contract between Bran (bootloader glue) and the kernel.
//! These traits contain NO Limine types and NO arch-specific details.

use core::fmt;

/// Opaque token for IRQ state save/restore
#[derive(Clone, Copy)]
pub struct IrqToken(pub u64);

/// Boot information extracted from bootloader
#[derive(Clone)]
pub struct BootInfo {
    /// Higher-half direct map offset
    pub hhdm_offset: u64,
    /// Physical memory available (in bytes)
    pub physical_memory: u64,
    /// Kernel command line (if any)
    pub cmdline: Option<&'static str>,
}

impl Default for BootInfo {
    fn default() -> Self {
        Self {
            hhdm_offset: 0,
            physical_memory: 0,
            cmdline: None,
        }
    }
}

/// Information about a boot module
#[derive(Clone)]
pub struct ModuleInfo {
    /// Module index
    pub index: usize,
    /// Module path/name
    pub path: &'static str,
    /// Physical address
    pub phys_addr: u64,
    /// Size in bytes
    pub size: u64,
}

/// A mapped module ready for reading
pub struct MappedModule {
    /// Virtual address of mapping
    pub virt_addr: *const u8,
    /// Size in bytes
    pub size: usize,
}

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

/// The primary machine abstraction provided by Bran
///
/// This trait bridges the bootloader-specific world (Limine) to the
/// arch-neutral kernel core. Bran constructs a concrete implementation
/// and passes it to `kernel::boot()`.
pub trait Machine: Sync {
    /// Get the architecture abstraction
    fn arch(&self) -> &dyn Architecture;
    
    /// Get the module provider
    fn modules(&self) -> &dyn ModuleProvider;
    
    /// Get boot information
    fn boot_info(&self) -> &BootInfo;
}

/// CPU-level primitives (no device drivers)
///
/// Provides the minimal hardware abstraction needed by the kernel:
/// - IRQ management
/// - CPU identification
/// - Halt/idle primitives
/// - (Future: context switching hooks, timer tick source)
pub trait Architecture: Sync {
    /// Disable interrupts and return a token for restore
    fn irq_disable(&self) -> IrqToken;
    
    /// Restore interrupts using the saved token
    fn irq_restore(&self, token: IrqToken);
    
    /// Get the current CPU ID (0 for uniprocessor)
    fn cpu_id(&self) -> u32;
    
    /// Halt the CPU forever (no return)
    fn halt(&self) -> !;
    
    /// Idle the CPU until next interrupt
    fn idle(&self);
    
    /// Write a byte to the debug serial port (for early logging)
    fn debug_putc(&self, c: u8);
}

/// Module listing/mapping abstraction
///
/// Provides access to boot modules loaded by the bootloader.
/// The kernel uses this to find and map Sprout and other modules.
pub trait ModuleProvider: Sync {
    /// Iterate over all available modules
    fn list(&self, out: &mut dyn FnMut(&ModuleInfo));
    
    /// Map a module read-only by index
    fn map_ro(&self, index: usize) -> Result<MappedModule, Error>;
    
    /// Get the number of modules
    fn count(&self) -> usize;
}
