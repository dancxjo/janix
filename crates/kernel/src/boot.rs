//! Boot sequence
//!
//! Orchestrates kernel initialization in strict order.

use crate::log::{self, Level};
use crate::{graph, machine, sched, symbols, syscall};
use crate::arch::machine::ARCH_MACHINE;
use abi::ids::SymbolId;

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

    // Phase 3: Initialize place store
    graph::init();
    log::klog(Level::Info, "KERNEL", "place store init");

    // Phase 3.5: Seed core ontology
    seed_ontology();


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

use crate::place;

/// Seed the core ontology: root place, kernel identity, and containment
fn seed_ontology() {
    let kind_place = symbols::well_known(b"kind.Place");
    let kind_thing = symbols::well_known(b"kind.Thing");
    let pred_contains = symbols::well_known(b"predicate.contains");

    // Create root place
    let root_id = graph::thing_create(kind_place, SymbolId::INVALID, 1);
    let root_name = symbols::intern(b"place.root");
    // PlaceBody { name: root_name }
    let mut payload = alloc::vec::Vec::new();
    payload.extend_from_slice(&root_name.0.to_le_bytes());
    graph::thing_set_inline_payload(root_id, &payload);

    log::klog(Level::Info, "PLACE", &format!("root created: {:?}", root_id));

    // Create kernel identity
    let kernel_id = graph::thing_create(kind_thing, SymbolId::INVALID, 1);
    log::klog(Level::Info, "THING", &format!("kernel identity created: {:?}", kernel_id));

    // Relate kernel to root (root contains kernel)
    let rel_id = graph::relationship_create(root_id, kernel_id, pred_contains);
    log::klog(Level::Info, "REL", &format!("contains created: {:?} from={:?} to={:?}", rel_id, root_id, kernel_id));

    // Verify containment
    let contained = place::contained_in(root_id);
    log::klog(Level::Info, "PLACE", &format!("root contains {} things", contained.len()));
    
    // Tiny self-test
    if contained.len() == 1 && contained[0] == kernel_id {
        log::klog(Level::Info, "KERNEL", "ontology self-test passed");
    } else {
        log::klog(Level::Error, "KERNEL", "ontology self-test FAILED");
    }
}

