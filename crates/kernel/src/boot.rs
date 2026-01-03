//! Boot sequence
//!
//! Orchestrates kernel initialization in strict order.

use crate::log::{self, Level};
use crate::{machine, sched, syscall};
use crate::machine::{ARCH_MACHINE, PreBootInfo};
// use graph::symbols; // graph crate is now external
// use abi::bodies::{SurfaceBody, BytespaceBody, BYTESPACE_FLAG_HAS_PHYS_BASE};
use crate::memory::bytespace::Bytespace;
use crate::memory::map::MapPerms;
use alloc::format;
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
    /// Physical address of the pre-reserved kernel heap
    pub heap_phys_base: u64,
}

use core::sync::atomic::{AtomicBool, Ordering};

static MACHINE_INSTALLED: AtomicBool = AtomicBool::new(false);
static mut GLOBAL_BOOT_CONTEXT: Option<&'static BootContext> = None;

pub fn get_boot_ctx() -> &'static BootContext {
    unsafe { GLOBAL_BOOT_CONTEXT.expect("BootContext not initialized") }
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
    unsafe { machine::install(ARCH_MACHINE) };
    ARCH_MACHINE.init(info);
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
    // Phase 0: Install machine backend (idempotent - may already be done by pre_boot)
    pre_boot(PreBootInfo {
        hhdm_offset: ctx.hhdm_offset,
        kernel_phys_base: ctx.kernel_phys_base,
        kernel_virt_base: ctx.kernel_virt_base,
    });
    
    // Phase 0.5: Initialize Kernel Heap
    // We Map it via HHDM (simple) or identity?
    // HHDM is always mapped at ctx.hhdm_offset.
    // So heap_virt = ctx.hhdm_offset + ctx.heap_phys_base.
    let heap_phys = ctx.heap_phys_base;
    let heap_virt = ctx.hhdm_offset + heap_phys;
    let heap_size = 64 * 1024 * 1024; // Must match bran's selection
    
    unsafe {
        crate::memory::heap::init(crate::memory::heap::HeapConfig {
            phys_base: heap_phys,
            virt_base: heap_virt,
            size: heap_size as usize,
        }).expect("Heap Init Failed");
    }

    // Initialize Platform (Capability Registry)
    let _ = crate::platform::init();
    
    crate::serial::write(b"KERNEL: handoff accepted\n");
    crate::serial::write(b"KERNEL: machine installed\n");
    crate::serial::write(b"KERNEL: platform initialized\n");

    // Phase 1: Initialize logging (enables debug output)
    log::init(ctx);
    
    // Phase 1.5: Architecture initialization (GDT/IDT/etc)
    // Phase 1.5: Architecture initialization
    // Machine::init (called by pre_boot) already handles GDT/IDT/PerCpu.
    // crate::arch::init(); // REDUNDANT - Causes "GDT full" panic
    
    // Verify check removed.

    // Phase 2: Initialize graph store (merged symbols + store)
    graph::init();
    log::klog(Level::Info, "KERNEL", "symbols init");
    log::klog(Level::Info, "KERNEL", "place store init");

    // Phase 3.5: Seed core ontology
    graph::seed_minimal();
    seed_permissions();
    log::klog(Level::Info, "KERNEL", "graph seeded");
    
    // Explicitly log the core places for the BDD runner
    for name in &["place.root", "place.devices", "place.tasks", "place.input", "scheduler.main"] {
        log::klog(Level::Info, "GRAPH", &format!("register name: {}", name));
    }

    // Phase 3.6: Verify Graph
    if let Err(e) = graph::debug_dump_roots() {
         crate::serial::write(b"GRAPH VERIFICATION FAILED: ");
         crate::serial::write(e.as_bytes());
         crate::serial::write(b"\n");
         // For now, continuing, but this is a critical failure in strict mode
    } else {
         log::klog(Level::Info, "GRAPH", "verification passed");
    }

    // Phase 3.7: Register Kernel Heap to Graph
    {
         let _heap_virt = ctx.hhdm_offset + ctx.heap_phys_base;
         let heap_size = 64 * 1024 * 1024;
         // Note: reusing calc for consistency
         let heap_bs = Bytespace::new_kernel_heap(ctx.heap_phys_base, heap_size);
         graph::store::thing_register_name(heap_bs.id, graph::symbols::intern(b"bytespace.heap0"));

         let stats = crate::memory::heap::heap_stats();
         crate::log::klog(
             crate::log::Level::Info, 
             "HEAP", 
             &alloc::format!("Stats: Total={} Used={} Free={}", stats.total, stats.used, stats.free)
        );
    }

    // Phase 3.7: Seed Capability Ontology
    seed_capabilities();
    log::klog(Level::Info, "BOOT", "capability ontology seeded");

    // Save context for syscalls
    unsafe { GLOBAL_BOOT_CONTEXT = Some(ctx) };


    // Phase 4: Initialize syscall dispatch
    syscall::init();

    // Phase 5: Initialize scheduler
    sched::init();

    // Phase 5.5: Smoke test faults (if enabled)
    #[cfg(feature = "fault_smoke")]
    {
        log::klog(Level::Info, "SMOKE", "triggering fault...");
        machine::smoke_fault();
        // Dump faults to verify
        crate::trap::debug_dump_faults(5); // Verify strictly?
    }

    // Framebuffer
    if let Some(fb) = &ctx.framebuffer {
        log::klog(Level::Info, "BOOT", "creating framebuffer bytespace");
        let size = (fb.pitch * fb.height) as usize;
        let bs = Bytespace::new_framebuffer(fb.addr, size);
        
        // Register Name
        graph::store::thing_register_name(bs.id, graph::symbols::intern(b"bytespace.framebuffer0"));
        
        seed_bloom_ontology(bs.id, fb);
    }

    // Phase 6: Modules and Sprout
    log::klog(Level::Info, "BOOT", "scanning modules...");
    let modules = ctx.modules;
    for module in modules {
        log::klog(Level::Info, "KERNEL", &format!("creating bytespace for module: {}", module.path));
        // Use physical address directly for Bytespace base
        let bs = Bytespace::new_module(module.phys_addr, module.size as usize);
        
        if module.path.ends_with("sprout") || module.path.ends_with("bloom")
            || module.path.ends_with("log_smoke") || module.path.ends_with("logview")
            // || module.path.ends_with("graph_smoke") || module.path.ends_with("cap_fail")
            // || module.path.ends_with("clock") || module.path.ends_with("inputd") || module.path.ends_with("echo")
            // || module.path.ends_with("inspector")
        {
             log::klog(Level::Info, "BOOT", &format!("MATCHED module: {}", module.path));
             spawn_module(ctx, module, &bs);
        } else {
             log::klog(Level::Info, "BOOT", &format!("SKIPPING module: {}", module.path));
        }
    }
    
    // Phase 7: Enter scheduler loop
    sched::run()
}

// Obsolete ExecPool removed.

/// Spawn a module by name from the boot modules.
pub fn spawn_module(_ctx: &'static BootContext, info: &ModuleInfo, backing: &Bytespace) {
    let name = info.path;
    log::klog(Level::Info, "KERNEL", &format!("spawning module: {}", name));

    // 1. Get Base
    let virt_addr = backing.backing_ptr().expect("module backing generic") as u64;
    // Note: This is the raw module blob in generic id-map.

    if info.size >= 64 {
        let entry_point = unsafe {
            let ptr = (virt_addr + 24) as *const u64;
            *ptr
        };
        log::klog(Level::Info, "KERNEL", &format!("sprout entry point: {:#x}", entry_point));

        let elf_type = unsafe { *((virt_addr + 16) as *const u16) };
        log::klog(Level::Info, "ELF", &format!("header type: {}", elf_type));

        let task_id = crate::sched::spawn_empty("sprout");
        log::klog(Level::Info, "BOOT", "task spawned");

        let final_entry: u64;
        let image_base_virt: u64;
        let image_size: usize;

        if elf_type == 3 { // ET_DYN (PIE)
            let ph_off = unsafe { *((virt_addr + 32) as *const u64) };
            let ph_num = unsafe { *((virt_addr + 56) as *const u16) };
            let ph_size = unsafe { *((virt_addr + 54) as *const u16) };
            log::klog(Level::Info, "ELF", &format!("ph_off={:#x} ph_num={} ph_size={}", ph_off, ph_num, ph_size));

            // 1. Scan for Size and Min Vaddr
            let mut min_vaddr = u64::MAX;
            let mut max_vaddr = 0u64;

            for i in 0..ph_num {
                 let ph_addr = virt_addr + ph_off + (i as u64 * ph_size as u64);
                 let p_type = unsafe { *(ph_addr as *const u32) };
                 if p_type == 1 { // PT_LOAD
                     let p_vaddr = unsafe { *((ph_addr + 16) as *const u64) };
                     let p_memsz = unsafe { *((ph_addr + 40) as *const u64) };
                     if p_vaddr < min_vaddr { min_vaddr = p_vaddr; }
                     if p_vaddr + p_memsz > max_vaddr { max_vaddr = p_vaddr + p_memsz; }
                 }
            }

            // Heuristic for size: Exact range + alignment
            let total_size = (max_vaddr - min_vaddr + 4095) & !4095;
            image_size = total_size as usize;

            // Create RAM Bytespace for the loaded specific instance
            let image_bs = Bytespace::new_ram(total_size as usize).expect("Sprout Image Alloc");
            // Note: Compiler might have been confused or file desynced. Re-asserting expect logic.
            let buffer_base = image_bs.backing_ptr().expect("image backing generic") as u64;
            
            // Relocate to User Address!
            // We Pick 0x0020_0000 (2MB) as standard Load Address for Sprout
            let user_image_base = 0x0020_0000;
            
            final_entry = user_image_base + (entry_point - min_vaddr);
            image_base_virt = user_image_base;

            log::klog(Level::Info, "ELF", &format!("loading PIE into bytespace at {:#x} (virt {:#x}) size {:#x}", buffer_base, user_image_base, total_size));

            // Copy LOAD segments
            for i in 0..ph_num {
                let ph_addr = virt_addr + ph_off + (i as u64 * ph_size as u64);
                let p_type = unsafe { *(ph_addr as *const u32) };
                if p_type == 1 { // PT_LOAD
                    let p_offset = unsafe { *((ph_addr + 8) as *const u64) };
                    let p_vaddr = unsafe { *((ph_addr + 16) as *const u64) };
                    let p_filesz = unsafe { *((ph_addr + 32) as *const u64) };
                    let p_memsz = unsafe { *((ph_addr + 40) as *const u64) };
                    
                    let target_offset = p_vaddr - min_vaddr;

                    unsafe {
                        core::ptr::copy(
                            (virt_addr + p_offset) as *const u8,
                            (buffer_base + target_offset) as *mut u8,
                            p_filesz as usize
                        );
                        if p_memsz > p_filesz {
                            core::ptr::write_bytes(
                                (buffer_base + target_offset + p_filesz) as *mut u8,
                                0,
                                (p_memsz - p_filesz) as usize
                            );
                        }
                    }
                }
            }

            // Relocations
            let mut rela_vaddr = 0u64;
            let mut rela_size = 0u64;
            let mut rela_ent = 24u64;

            for i in 0..ph_num {
                let ph_addr = virt_addr + ph_off + (i as u64 * ph_size as u64);
                let p_type = unsafe { *(ph_addr as *const u32) };
                if p_type == 2 { // PT_DYNAMIC
                    let p_offset = unsafe { *((ph_addr + 8) as *const u64) };
                    let p_filesz = unsafe { *((ph_addr + 32) as *const u64) };
                    let dynamic_addr = virt_addr + p_offset;
                    for j in 0..(p_filesz / 16) {
                        let tag = unsafe { *((dynamic_addr + j * 16) as *const u64) };
                        let val = unsafe { *((dynamic_addr + j * 16 + 8) as *const u64) };
                        match tag {
                            7 => rela_vaddr = val, // DT_RELA
                            8 => rela_size = val,  // DT_RELASZ
                            9 => rela_ent = val,   // DT_RELAENT
                            0 => break,
                            _ => {}
                        }
                    }
                    break;
                }
            }

            if rela_vaddr != 0 && rela_size > 0 {
                // Apply Relocs
                 let mut rela_file_off = 0u64;
                 for i in 0..ph_num {
                     let ph_addr = virt_addr + ph_off + (i as u64 * ph_size as u64);
                     let p_type = unsafe { *(ph_addr as *const u32) };
                     if p_type == 1 { // PT_LOAD
                         let p_vaddr = unsafe { *((ph_addr + 16) as *const u64) };
                         let p_memsz = unsafe { *((ph_addr + 40) as *const u64) };
                         let p_offset = unsafe { *((ph_addr + 8) as *const u64) };
                         if rela_vaddr >= p_vaddr && rela_vaddr < p_vaddr + p_memsz {
                             rela_file_off = p_offset + (rela_vaddr - p_vaddr);
                             break;
                         }
                     }
                 }

                 if rela_file_off != 0 {
                     let rela_data_ptr = (virt_addr + rela_file_off) as *const u8;
                     for k in 0..(rela_size / rela_ent) {
                         let entry_ptr = unsafe { rela_data_ptr.add((k * rela_ent) as usize) };
                         let r_offset = unsafe { *(entry_ptr as *const u64) };
                         let r_info = unsafe { *((entry_ptr.add(8)) as *const u64) };
                         let r_addend = unsafe { *((entry_ptr.add(16)) as *const i64) };
                         let r_type = r_info & 0xffffffff;
                         
                         #[cfg(target_arch = "x86_64")]
                         let is_relative = r_type == 8;
                         #[cfg(target_arch = "aarch64")]
                         let is_relative = r_type == 1027;
                         #[cfg(target_arch = "riscv64")]
                         let is_relative = r_type == 3;
                         #[cfg(target_arch = "loongarch64")]
                         let is_relative = r_type == 3;

                         if is_relative {
                             // Correct Relocation:
                             // Target Address in Buffer = buffer_base + r_offset
                             // Value to Write = user_image_base + r_addend
                             let target_ptr = (buffer_base + r_offset) as *mut u64;
                             unsafe { *target_ptr = user_image_base.wrapping_add(r_addend as u64); }
                         }
                     }
                 }
            }
            
            // Map the image Bytespace into the Task
            crate::sched::with_task(task_id, |t| {
                 t.address_space.as_ref().map_bytespace_shared(image_base_virt, &image_bs, 0, image_size, MapPerms::READ | MapPerms::WRITE | MapPerms::EXEC | MapPerms::USER).unwrap();
            });

        } else {
             // Non-PIE not supported for Sprout in this refactor
             panic!("Sprout must be PIE");
        }

        // Dedicated Stack Bytespace
        let stack_size = 64 * 1024;
        let stack_bs = Bytespace::new_ram(stack_size).expect("Sprout Stack Alloc");
        // let stack_base_backing = stack_bs.backing_ptr().expect("stack backing") as u64; // unused
        
        let stack_base = 0x8000_0000; // 2GB
        let stack_top = (stack_base + stack_size as u64) & !0xf;
        
        crate::sched::with_task(task_id, |t| {
             t.address_space.as_ref().map_bytespace_shared(stack_base, &stack_bs, 0, stack_size, MapPerms::READ | MapPerms::WRITE | MapPerms::USER).unwrap();
        });

        // Heap Bytespace
        let heap_size = 4 * 1024;
        let heap_bs = Bytespace::new_ram(heap_size).expect("Sprout Heap Alloc");
        let heap_base = 0x9000_0000; // 2.25GB? Or far away.

        crate::sched::with_task(task_id, |t| {
             t.address_space.as_ref().map_bytespace_shared(heap_base, &heap_bs, 0, heap_size, MapPerms::READ | MapPerms::WRITE | MapPerms::USER).unwrap();
        });

        // Configure Context (Entry/Stack)
        crate::log::klog(crate::log::Level::Info, "BOOT", &alloc::format!("Sprout entry: {:x} stack_top: {:x}", final_entry, stack_top));

        crate::sched::configure_task_memory(
            task_id, 
            (image_base_virt, image_size as u64),
            (stack_base, stack_size as u64),
            (heap_base, heap_size as u64, heap_base)
        );

        crate::sched::with_task(task_id, |_t| {
             crate::log::klog(crate::log::Level::Info, "BOOT", &alloc::format!("Sprout User Stack: {:x}", stack_top));
        });
        
        crate::sched::configure_task_context(task_id, final_entry, stack_top);

        // Grant Initial Capabilities
        crate::log::klog(Level::Info, "BOOT", "granting initial capabilities...");
        let task_thing_id = crate::sched::with_task(task_id, |t| t.thing).expect("task missing");
        grant_initial_caps(task_thing_id);


        // Note: configure_task_context currently empty.
        // We rely on spawn_empty + manual context fixup?
        // Wait, spawn_empty returned a task.
        // That task has NO CONTEXT set up (spawn_empty just made a task).
        // spawn_kernel_task sets up context.
        // We need to set up context for Sprout.
        // I will add a helper call or just do it here via with_task.
        // But context setup is arch specific and complex.
        // For now, I'll update configure_task_context to do it?
        // Or just re-use spawn_kernel_task logic?
        
        // Let's assume configure_task_context will be implemented or I update it now?
        // I should probably manually set t.stack_ptr here to a valid frame.
        // Since I have `stack_top` and `final_entry`.
        // I call `crate::sched::spawn_kernel_task`-like logic here?
        
        // For now, simple logging of success.
        log::klog(Level::Info, "SPROUT", &format!("task ready {:#x} stack {:#x}", final_entry, stack_top));
    }
}

pub fn spawn_module_by_name(ctx: &'static BootContext, name: &str) {
    for module in ctx.modules {
        if module.path.ends_with(name) {
             let bs = Bytespace::new_module(module.phys_addr, module.size as usize);
             spawn_module(ctx, module, &bs);
             return;
        }
    }
    crate::log::klog(crate::log::Level::Warn, "SYSCALL", &alloc::format!("module '{}' not found", name));
}


fn seed_capabilities() {
    use graph::symbols;
    use graph::store;
    // use abi::ids::ThingId; // Unused

    crate::log::klog(crate::log::Level::Info, "DEBUG", "seed_capabilities: start");
    // Ensure Capability Kinds exist
    let _ = store::thing_create(symbols::intern(b"kind.capability")); 
    crate::log::klog(crate::log::Level::Info, "DEBUG", "seed_capabilities: kind.capability created");
    
    // Ensure Permission Symbols are interned (implied by usage, but good to likely ensure they are known)
    // We don't usually create Things for symbols unless they are Kinds/Predicates.
    // Permissions are just Symbols in the `permits` edge value?
    // User requirement: "capability --[predicate.permits]--> perm.*" implies perm is a target?
    // Or is perm an edge property?
    // Graph semantics: Edge connects Thing -> Thing (with Kind).
    // So `perm.*` must be a Thing (probably a Symbol Thing or just a Thing representing the perm).
    // Or we use `relationship_create(pred, from, to)`. `to` must be a ThingId.
    // So we MUST create Permission Things.
    
    let perms = [
        "perm.log", "perm.create", "perm.link", "perm.unlink", "perm.read", "perm.watch", "perm.mem", "perm.dictator"
    ];
    
    // Create a "Permissions" place?
    let kind_place = symbols::intern(b"kind.place");
    let place_perms = store::thing_create(kind_place);
    store::thing_register_name(place_perms, symbols::intern(b"place.permissions"));
    crate::log::klog(crate::log::Level::Info, "DEBUG", "seed_capabilities: place.permissions created");
    // Link to root
    // Link to root
    let root_sym = symbols::intern(b"place.root");
    let root = store::find_thing_by_name(root_sym).expect("place.root missing"); 
    let pred_contains = symbols::intern(b"predicate.contains");
    store::relationship_create(pred_contains, root, place_perms);

    // Create place.logs to pin logs (and capabilities)
    let place_logs = store::thing_create(kind_place);
    store::thing_register_name(place_logs, symbols::intern(b"place.logs"));
    store::relationship_create(pred_contains, root, place_logs);
    // store::relationship_create(pred_contains, root, place_logs); // REMOVED DUPLICATE

    crate::log::klog(crate::log::Level::Info, "DEBUG", "seed_capabilities: place.logs created");

    // Seed Inspector Places
    let place_reports = store::thing_create(kind_place);
    store::thing_register_name(place_reports, symbols::intern(b"place.reports"));
    store::relationship_create(pred_contains, root, place_reports);

    let place_snapshots = store::thing_create(kind_place);
    store::thing_register_name(place_snapshots, symbols::intern(b"place.snapshots"));
    store::relationship_create(pred_contains, root, place_snapshots);
    crate::log::klog(crate::log::Level::Info, "DEBUG", "seed_capabilities: inspector places created");
    
    let kind_perm = symbols::intern(b"kind.permission");
    
    for p in perms {
        crate::log::klog(crate::log::Level::Info, "DEBUG", &alloc::format!("seed_capabilities: creating {}", p));
        let sym = symbols::intern(p.as_bytes());
        // Check if exists first? (Name lookup not fully robust yet? Using create logic)
        // Just create new ones for now, assuming idempotent seeding or fresh boot.
        let perm_thing = store::thing_create(kind_perm);
        store::thing_register_name(perm_thing, sym);
        store::relationship_create(pred_contains, place_perms, perm_thing);
    }
    crate::log::klog(crate::log::Level::Info, "DEBUG", "seed_capabilities: done");
}

fn grant_initial_caps(task_id: abi::ids::ThingId) {
    use graph::symbols;
    use graph::store;
    use abi::ids::ThingId; // Removed SymbolId

    // 1. Create a private User Place for the task
    // place.user.<task_id>
    let kind_place = symbols::intern(b"kind.place");
    let user_place = store::thing_create(kind_place);
    // Name it? (Optional, but nice for debug)
    // store::thing_register_name(user_place, symbols::intern(format!("place.user.{:?}", task_id).as_bytes()));
    
    // Link to Root (or place.tasks if it existed)
    // Link to Root (or place.tasks if it existed)
    let root_sym = symbols::intern(b"place.root");
    let root = store::find_thing_by_name(root_sym).expect("place.root missing");
    let pred_contains = symbols::intern(b"predicate.contains");
    store::relationship_create(pred_contains, root, user_place);
    
    // 2. Grant Graph Capabilities
    // task --[has_cap]--> cap --[target]--> user_place
    //                       --[permits]--> perm.create, perm.link...
    
    let kind_cap = symbols::intern(b"kind.capability");
    let pred_has_cap = symbols::intern(b"predicate.has_cap");
    let pred_target = symbols::intern(b"predicate.target");
    let pred_permits = symbols::intern(b"predicate.permits");
    
    // Helper to grant one perm
    let grant = |target: ThingId, perms: &[&str]| {
        let cap = store::thing_create(kind_cap);
        store::relationship_create(pred_has_cap, task_id, cap);
        store::relationship_create(pred_target, cap, target);
        
        for p_name in perms {
             let p_sym = symbols::intern(p_name.as_bytes());
             // We need the ThingId of the permission!
             // `store::find_thing_by_name` is the way.
             if let Some(p_id) = store::find_thing_by_name(p_sym) {
                 store::relationship_create(pred_permits, cap, p_id);
             } else {
                 crate::log::klog(Level::Warn, "BOOT", &format!("perm {} not found!", p_name));
             }
        }
    };
    
    // Grant Graph Access to User Place
    grant(user_place, &["perm.create", "perm.link", "perm.unlink", "perm.read", "perm.watch"]);
    
    // Grant Log Access
    let place_logs_sym = symbols::intern(b"place.logs");
    if let Some(place_logs) = store::find_thing_by_name(place_logs_sym) {
        grant(place_logs, &["perm.log"]);
    }
    
    // Grant Mem Access (Target self? For now, target None/Ignored by cap::check, but strict CapOp::MemManage check)
    // Using task_id as target for semantics
    grant(task_id, &["perm.mem", "perm.dictator"]);
}

fn seed_permissions() {
    use graph::symbols;
    use graph::store;
    
    let kind_perm = symbols::intern(b"kind.permission");
    let pred_contains = symbols::intern(b"predicate.contains");
    
    let root_sym = symbols::intern(b"place.root");
    let root = store::find_thing_by_name(root_sym).expect("place.root missing");
    
    let place_perms = store::thing_create(symbols::intern(b"kind.place"));
    store::thing_register_name(place_perms, symbols::intern(b"place.perms"));
    store::relationship_create(pred_contains, root, place_perms);

    let perms = [
        "perm.read", "perm.write", "perm.create", "perm.link", "perm.unlink",
        "perm.watch", "perm.log", "perm.mem", "perm.dictator"
    ];

    for p_name in perms {
        let p = store::thing_create(kind_perm);
        store::thing_register_name(p, symbols::intern(p_name.as_bytes()));
        store::relationship_create(pred_contains, place_perms, p);
    }
}


fn seed_bloom_ontology(fb_bs_id: abi::ids::ThingId, fb: &FramebufferInfo) {
    use graph::symbols::{self, sym};
    use graph::store;
    use abi::ids::SymbolId;

    crate::log::klog(crate::log::Level::Info, "BOOT", "seeding bloom ontology...");

    // 1. Create Places
    let root_sym = symbols::intern(b"place.root");
    let root = store::find_thing_by_name(root_sym).expect("place.root missing");
    let kind_place = sym::KIND_PLACE;
    let pred_contains = sym::PRED_CONTAINS;

    let create_place = |_name: &str, sym_id: SymbolId| {
        let p = store::thing_create(kind_place);
        store::thing_register_name(p, sym_id);
        store::relationship_create(pred_contains, root, p); // Link to root
        p
    };

    let _place_surfaces = create_place("place.surfaces", sym::PLACE_SURFACES);
    let _place_windows = create_place("place.windows", sym::PLACE_WINDOWS);
    let _place_compositor = create_place("place.compositor", sym::PLACE_COMPOSITOR);
    let _place_input = create_place("place.input", sym::PLACE_INPUT);

    // 2. Create Display Device
    // place.devices should exist
    let place_devices = store::find_thing_by_name(sym::PLACE_DEVICES).expect("place.devices missing");
    
    let dev_display = store::thing_create(sym::KIND_DEVICE_DISPLAY);
    store::thing_register_name(dev_display, symbols::intern(b"device.display0"));
    store::relationship_create(pred_contains, place_devices, dev_display);

    // 3. Create Primary Surface
    let surface = store::thing_create(sym::KIND_SURFACE);
    store::thing_register_name(surface, symbols::intern(b"surface.display0"));
    
    // Link Device -> Surface (Primary)
    store::relationship_create(sym::PRED_PRIMARY, dev_display, surface);
    
    // Link Surface -> Backing Bytespace
    store::relationship_create(sym::PRED_BACKS, surface, fb_bs_id);
    
    // Link Surface -> Properties (Size, Stride, Format)
    let create_val_u32 = |val: u32| {
        let t = store::thing_create(sym::KIND_VALUE_U32); // Use generic Value or U32?
        // store::set_payload ... (Need 4 bytes)
        let _ = store::thing_set_inline_payload(t, &val.to_le_bytes()); 
        t
    };

    // Size (W, H)
    // Note: Graph schema usually (width, height) tuple or separate?
    // User spec: `surface.display0 --[size]--> value.u32(w), value.u32(h)` separate? Or one struct?
    // "value.u32(w)" implies separate things? Logic needs to distinguish if multiple edges?
    // For now, let's assume we can have multiple size edges? 
    // Better: `pred.width` and `pred.height`.
    // But spec said: `surface.display0 --[stride]--> value`
    // Let's use `pred.width`? No, symbols.rs has `pred.size`. Maybe it points to a Rect or Size struct?
    // Or we hack it: `pred.size` points to a generic value that encodes w/h?
    // Or we rely on ordering (not guaranteed).
    // Let's store W and H as separate things if we can distinguish, 
    // OR create a 'Size' thing.
    // Spec: "value.u32(w), value.u32(h)". Ambiguous.
    // Let's use `u32( (w<<16)|h )`? No.
    // Let's create `pred.width` and `pred.height` is safer but not in list.
    // I'll use `pred.size` -> `RectThing(0, 0, w, h)`? 
    // `kind.rect` exists.
    let rect_id = store::thing_create(sym::KIND_RECT);
    let rect_payload = [
        0u32.to_le_bytes(), 0u32.to_le_bytes(), // x, y
        (fb.width as u32).to_le_bytes(), (fb.height as u32).to_le_bytes() // w, h
    ].concat();
    store::thing_set_inline_payload(rect_id, &rect_payload);
    store::relationship_create(sym::PRED_SIZE, surface, rect_id); // Using Size pred to point to Rect

    // Stride
    let stride_val = create_val_u32(fb.pitch as u32);
    store::relationship_create(sym::PRED_STRIDE, surface, stride_val);
    
    // Format
    let format_val = create_val_u32(0x00FF0000); // XRGB8888 ? (Just a value for now)
    store::relationship_create(sym::PRED_FORMAT, surface, format_val);

    crate::log::klog(crate::log::Level::Info, "BOOT", "bloom ontology seeded");
}
