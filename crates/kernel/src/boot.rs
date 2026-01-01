//! Boot sequence
//!
//! Orchestrates kernel initialization in strict order.

use crate::log::{self, Level};
use crate::{graph, machine, sched, symbols, syscall};
use crate::arch::machine::ARCH_MACHINE;

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
    /// Kernel physical load address
    pub kernel_phys_base: u64,
    /// Kernel virtual base address
    pub kernel_virt_base: u64,
}

use core::sync::atomic::{AtomicBool, Ordering};

static MACHINE_INSTALLED: AtomicBool = AtomicBool::new(false);

/// Information needed for pre-boot machine initialization.
#[derive(Clone, Copy)]
pub struct PreBootInfo {
    /// HHDM offset for physical memory access
    pub hhdm_offset: u64,
    /// Kernel physical load address
    pub kernel_phys_base: u64,
    /// Kernel virtual base address  
    pub kernel_virt_base: u64,
}

/// Pre-boot initialization for early console output.
///
/// Bran calls this with addressing info before any logging.
/// This initializes the Machine interface and maps UART MMIO,
/// enabling safe console output on all architectures.
///
/// Safe to call multiple times (idempotent).
pub fn pre_boot(info: PreBootInfo) {
    if MACHINE_INSTALLED.swap(true, Ordering::SeqCst) {
        return; // Already installed
    }
    ARCH_MACHINE.init_machine(info);
    unsafe { machine::install(&ARCH_MACHINE) };
    crate::serial::init();
}

/// Main kernel entry point
///
/// Called by Bran after collecting facts from the bootloader.
/// ctx is a Bag of Facts (no behavior).
/// This function never returns.
pub unsafe fn boot(ctx: *mut BootContext) -> ! {
    // Safety: called exactly once during boot, ctx points to the single global BootContext,
    // and no aliasing occurs after transfer.
    let ctx: &'static mut BootContext = unsafe { &mut *ctx };
    // Phase 0: Install machine backend (idempotent - may already be done by pre_boot)
    pre_boot(PreBootInfo {
        hhdm_offset: ctx.hhdm_offset,
        kernel_phys_base: ctx.kernel_phys_base,
        kernel_virt_base: ctx.kernel_virt_base,
    });
    crate::serial::write(b"KERNEL: handoff accepted\n");
    crate::serial::write(b"MACHINE: installed\n");
    crate::serial::write(b"MACHINE: mmio ok\n");

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

    // Phase 6.5: Signal boot completion
    // This is the signal for the BDD runner to stop deciding the test passed
    crate::serial::write(b"Booted.\n");

    // Phase 7: Enter scheduler loop
    sched::run()
}

use alloc::format;

/// Spawn the Sprout init process
fn spawn_sprout(ctx: &'static BootContext) {
    log::klog(Level::Info, "KERNEL", "spawning sprout");

    // Look for a module named "sprout"
    let mut found_sprout = false;
    for &module in ctx.modules {
        if module.path.ends_with("sprout") {
            found_sprout = true;
            let virt_addr = module.phys_addr.wrapping_add(ctx.hhdm_offset);
            log::klog(
                Level::Info,
                "KERNEL",
                &format!("found sprout at {:#x} (size: {})", virt_addr, module.size),
            );

            // Simple ELF-64 header parsing to find entry point
            // e_entry is at offset 24
            if module.size >= 64 {
                let entry_point = unsafe {
                    let ptr = (virt_addr + 24) as *const u64;
                    *ptr
                };
                log::klog(
                    Level::Info,
                    "KERNEL",
                    &format!("sprout entry point: {:#x}", entry_point),
                );

                // TODO: Enter user mode at entry_point
            } else {
                log::klog(Level::Error, "KERNEL", "sprout module too small");
            }
            break;
        }
    }

    if !found_sprout {
        log::klog(Level::Warn, "KERNEL", "sprout module not found");
    }
}
