//! Boot sequence
//!
//! Orchestrates kernel initialization in strict order.

use crate::log::{self, Level};
use crate::{graph, sched, symbols, syscall};

/// Information about a boot module
#[derive(Clone, Copy)]
pub struct ModuleInfo {
    pub index: usize,
    pub path: &'static str,
    pub phys_addr: u64,
    pub size: u64,
}

/// A mapped module ready for reading
pub struct MappedModule {
    pub virt_addr: *const u8,
    pub size: usize,
}

/// Information about the framebuffer provided by bootloader
pub struct FramebufferInfo {
    pub addr: u64,
    pub width: u64,
    pub height: u64,
    pub pitch: u64,
    pub bpp: u16,
}


/// Bag of facts provided by the bootloader
pub struct BootContext {
    pub hhdm_offset: u64,
    pub physical_memory: u64,
    pub cmdline: Option<&'static str>,
    pub framebuffer: Option<FramebufferInfo>,
    pub modules: &'static [ModuleInfo],
    /// Optional early serial output function provided by Bran
    pub early_putc: Option<fn(u8)>,
}

/// Main kernel entry point
///
/// Called by Bran after collecting facts from the bootloader.
/// ctx is a Bag of Facts (no behavior).
/// This function never returns.
pub fn boot(ctx: &'static mut BootContext) -> ! {
    // Phase 1: Initialize logging (enables debug output)
    log::init(ctx);
    
    // Phase 2: Initialize symbol table
    symbols::init();
    log::klog(Level::Info, "KERNEL", "symbols init");
    
    // Phase 3: Initialize graph store
    graph::init();
    log::klog(Level::Info, "KERNEL", "graph init");
    
    // Phase 4: Initialize syscall dispatch
    syscall::init();
    
    // Phase 5: Initialize scheduler
    sched::init();
    
    // Phase 6: Spawn Sprout
    spawn_sprout(ctx);
    
    // Phase 7: Enter scheduler loop
    sched::run()
}

/// Spawn the Sprout init process
fn spawn_sprout(ctx: &'static BootContext) {
    log::klog(Level::Info, "KERNEL", "spawning sprout");
    
    // Look for a module named "sprout"
    let mut found_sprout = false;
    for &module in ctx.modules {
        if module.path.contains("sprout") {
            found_sprout = true;
            // TODO: Actually spawn the process
            break;
        }
    }
    
    if !found_sprout {
        log::klog(Level::Warn, "KERNEL", "sprout module not found");
    }
}
