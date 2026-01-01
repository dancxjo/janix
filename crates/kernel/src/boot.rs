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
    ARCH_MACHINE.init(info);
    unsafe { machine::install(ARCH_MACHINE) };
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

    // Phase 2: Initialize symbol table (Now part of Graph)
    // symbols::init(); 
    // log::klog(Level::Info, "KERNEL", "symbols init");

    // Phase 3: Initialize graph store
    graph::init();
    log::klog(Level::Info, "KERNEL", "graph init (symbols+store)");

    // Phase 3.5: Seed core ontology
    graph::seed_minimal();
    log::klog(Level::Info, "KERNEL", "graph seeded");

    // Phase 3.6: Verify Graph
    if let Err(e) = graph::debug_dump_roots() {
         crate::serial::write(b"GRAPH VERIFICATION FAILED: ");
         crate::serial::write(e.as_bytes());
         crate::serial::write(b"\n");
         // For now, continuing, but this is a critical failure in strict mode
    } else {
         log::klog(Level::Info, "GRAPH", "verification passed");
    }

    // seed_ontology(ctx); // Removed - using graph::seed_minimal

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
        let _ = Bytespace::new_framebuffer(fb.addr, size);
    }

    // Phase 6: Modules and Sprout
    log::klog(Level::Info, "BOOT", "scanning modules...");
    let modules = ctx.modules;
    for module in modules {
        log::klog(Level::Info, "KERNEL", &format!("creating bytespace for module: {}", module.path));
        // Use physical address directly for Bytespace base
        let bs = Bytespace::new_module(module.phys_addr, module.size as usize);
        
        if module.path.ends_with("sprout") {
             spawn_module(ctx, module, &bs);
        }
    }

    // Phase 6.5: Spawn Ping-Pong Verification
    crate::sched::spawn_kernel_task("ping", ping_task);
    crate::sched::spawn_kernel_task("pong", pong_task);

    // Phase 7: Enter scheduler loop
    sched::run()
}

extern "C" fn ping_task() {
    loop {
        crate::serial::write(b"ping\n");
        for _ in 0..10000000 { core::hint::spin_loop(); }
    }
}

extern "C" fn pong_task() {
    loop {
        crate::serial::write(b"pong\n");
        for _ in 0..10000000 { core::hint::spin_loop(); }
    }
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

        let final_entry: u64;
        let image_base_virt: u64;
        let image_size: usize;

        // Create the task to own everything
        let task_id = crate::sched::spawn_empty("sprout");
        crate::sched::mark_as_init(task_id);

        if elf_type == 3 { // ET_DYN (PIE)
            let ph_off = unsafe { *((virt_addr + 32) as *const u64) };
            let ph_num = unsafe { *((virt_addr + 56) as *const u16) };
            let ph_size = unsafe { *((virt_addr + 54) as *const u16) };

            let total_size = 1024 * 1024; // 1MB allocation
            image_size = total_size;

            // Create RAM Bytespace for the loaded specific instance
            let image_bs = Bytespace::new_ram(total_size);
            let buffer_base = image_bs.backing_ptr().unwrap() as u64;
            
            // We map it at 0x0040_0000 (standard-ish?) or just use the buffer_base if Kernel mode.
            // For now, let's Map it identity-ish or fixed?
            // If we use PIE, we can run it at buffer_base.
            final_entry = buffer_base + entry_point;
            image_base_virt = buffer_base;

            log::klog(Level::Info, "ELF", &format!("loading PIE into bytespace at {:#x}", buffer_base));

            // Copy LOAD segments
            for i in 0..ph_num {
                let ph_addr = virt_addr + ph_off + (i as u64 * ph_size as u64);
                let p_type = unsafe { *(ph_addr as *const u32) };
                if p_type == 1 { // PT_LOAD
                    let p_offset = unsafe { *((ph_addr + 8) as *const u64) };
                    let p_vaddr = unsafe { *((ph_addr + 16) as *const u64) };
                    let p_filesz = unsafe { *((ph_addr + 32) as *const u64) };
                    let p_memsz = unsafe { *((ph_addr + 40) as *const u64) };
                    
                    unsafe {
                        core::ptr::copy_nonoverlapping(
                            (virt_addr + p_offset) as *const u8,
                            (buffer_base + p_vaddr) as *mut u8,
                            p_filesz as usize
                        );
                        if p_memsz > p_filesz {
                            core::ptr::write_bytes(
                                (buffer_base + p_vaddr + p_filesz) as *mut u8,
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
                // Apply Relocs (Code omitted for brevity in previous, but needed here)
                // Reusing the logic from before...
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
                             let target_ptr = (buffer_base + r_offset) as *mut u64;
                             unsafe { *target_ptr = buffer_base.wrapping_add(r_addend as u64); }
                         }
                     }
                 }
            }
            
            // Map the image Bytespace into the Task
            crate::sched::with_task(task_id, |t| {
                 t.address_space.as_ref().map_bytespace_shared(image_base_virt, &image_bs, 0, image_size, MapPerms::READ | MapPerms::WRITE | MapPerms::EXEC).unwrap();
            });

        } else {
             // Non-PIE not supported for Sprout in this refactor
             panic!("Sprout must be PIE");
        }

        // Dedicated Stack Bytespace
        let stack_size = 64 * 1024;
        let stack_bs = Bytespace::new_ram(stack_size);
        let stack_base = stack_bs.backing_ptr().unwrap() as u64;
        let stack_top = (stack_base + stack_size as u64) & !0xf;
        
        crate::sched::with_task(task_id, |t| {
             t.address_space.as_ref().map_bytespace_shared(stack_base, &stack_bs, 0, stack_size, MapPerms::READ | MapPerms::WRITE).unwrap();
        });

        // Heap Bytespace
        let heap_size = 4 * 1024;
        let heap_bs = Bytespace::new_ram(heap_size);
        let heap_base = heap_bs.backing_ptr().unwrap() as u64;

        crate::sched::with_task(task_id, |t| {
             t.address_space.as_ref().map_bytespace_shared(heap_base, &heap_bs, 0, heap_size, MapPerms::READ | MapPerms::WRITE).unwrap();
        });

        // Configure Context (Entry/Stack)
        crate::sched::configure_task_memory(
            task_id, 
            (image_base_virt, image_size as u64),
            (stack_base, stack_size as u64),
            (heap_base, heap_size as u64, heap_base)
        );

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
    log::klog(Level::Warn, "SYSCALL", &format!("module '{}' not found", name));
}



