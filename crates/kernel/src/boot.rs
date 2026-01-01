//! Boot sequence
//!
//! Orchestrates kernel initialization in strict order.

use crate::log::{self, Level};
use crate::{machine, sched, syscall};
use crate::machine::{ARCH_MACHINE, PreBootInfo};
// use graph::symbols; // graph crate is now external
// use abi::bodies::{SurfaceBody, BytespaceBody, BYTESPACE_FLAG_HAS_PHYS_BASE};

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
    
    unsafe { verify_exec_pool_is_executable(); }

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

    // Phase 6: Spawn Sprout
    spawn_module(ctx, "sprout");

    // Phase 6.5: Signal boot completion
    // Handed off to Sprout - never returns

    // Phase 7: Enter scheduler loop
    sched::run()
}

use alloc::format;

// Executable memory pool (placed in .text to ensure/hope it's executable)
// We rely on the linker keeping this writable or Limine mapping it RWX.
const EXEC_POOL_SIZE: usize = 4 * 1024 * 1024;

#[repr(align(4096))]
struct ExecPool([u8; EXEC_POOL_SIZE]);

#[link_section = ".text"]
static mut EXEC_POOL_STORAGE: ExecPool = ExecPool([0u8; EXEC_POOL_SIZE]);
static mut EXEC_POS: usize = 0;

unsafe fn alloc_exec(size: usize) -> Option<(&'static mut [u8], u64)> {
    // Safety: we are single threaded during boot (mostly) or we hope nothing races here.
    // Align to 16 bytes to satisfy instruction alignment requirements on all archs
    let pos = (EXEC_POS + 15) & !15;
    
    if pos + size > EXEC_POOL_SIZE {
        return None;
    }
    EXEC_POS = pos + size;
    
    // Avoid creating a mutable reference to the static array
    let pool_base = core::ptr::addr_of_mut!(EXEC_POOL_STORAGE.0) as *mut u8;
    let ptr = pool_base.add(pos);
    
    let slice = core::slice::from_raw_parts_mut(ptr, size);
    Some((slice, ptr as u64))
}

unsafe fn verify_exec_pool_is_executable() {
    log::klog(Level::Info, "EXEC", "verifying execution permission...");
    
    #[cfg(target_arch = "x86_64")]
    let ret_opcode: &[u8] = &[0xC3]; // ret

    #[cfg(target_arch = "aarch64")]
    let ret_opcode: &[u8] = &[0xC0, 0x03, 0x5F, 0xD6]; // ret

    #[cfg(target_arch = "riscv64")]
    let ret_opcode: &[u8] = &[0x67, 0x80, 0x00, 0x00]; // ret (jalr x0, x1, 0)

    #[cfg(target_arch = "loongarch64")]
    let ret_opcode: &[u8] = &[0x20, 0x00, 0x00, 0x4C]; // jirl $r0, $r1, 0

    // 1. Alloc a tiny slice
    let (slice, addr) = alloc_exec(ret_opcode.len()).unwrap();
    
    // 2. Write 'ret'
    slice[..ret_opcode.len()].copy_from_slice(ret_opcode);
    
    // 3. Flush cache if necessary (Architecture dependent)
    // For now, we rely on the fact that this memory is likely cold or coherent enough.
    // If this fails on LoongArch/RISCV, we need I-Bar/Fence.I here.
    #[cfg(any(target_arch = "riscv64", target_arch = "loongarch64", target_arch = "aarch64"))]
    {
         // Simple fence if possible, but without asm! we risk it.
         // Given this is a kernel, assume asm! is available if we needed it.
         // For now, try just the opcode fix.
    }

    // 4. Jump to it
    let func: extern "C" fn() = core::mem::transmute(addr);
    func();
    log::klog(Level::Info, "EXEC", "verification passed!");
}

/// Spawn a module by name from the boot modules.
pub fn spawn_module(ctx: &'static BootContext, name: &str) {
    log::klog(Level::Info, "KERNEL", &format!("spawning module: {}", name));

    let mut found = false;
    for module in ctx.modules {
        if module.path.ends_with(name) {
            found = true;
            let virt_addr = module.phys_addr.wrapping_add(ctx.hhdm_offset);
            log::klog(
                Level::Info,
                "KERNEL",
                &format!("found {} at {:#x} (size: {})", name, virt_addr, module.size),
            );

            if module.size >= 64 {
                let entry_point = unsafe {
                    let ptr = (virt_addr + 24) as *const u64;
                    *ptr
                };
                log::klog(Level::Info, "KERNEL", &format!("sprout entry point: {:#x}", entry_point));

                let elf_type = unsafe { *((virt_addr + 16) as *const u16) };
                log::klog(Level::Info, "ELF", &format!("header type: {}", elf_type));

                let final_entry: u64;

                if elf_type == 3 { // ET_DYN (PIE)
                    let ph_off = unsafe { *((virt_addr + 32) as *const u64) };
                    let ph_num = unsafe { *((virt_addr + 56) as *const u16) };
                    let ph_size = unsafe { *((virt_addr + 54) as *const u16) };

                    let image_size = 1024 * 1024; // 1MB buffer
                    let (_image_slice, buffer_addr) = unsafe { 
                         alloc_exec(image_size).expect("Out of executable memory") 
                    };
                    let buffer_base = buffer_addr;
                    // Note: image_slice is already valid &mut [u8]. We don't need to forget it because it's a reference to static pool.
                    
                    log::klog(Level::Info, "ELF", &format!("loading PIE into executable buffer at {:#x}", buffer_base));

                    // Copy LOAD segments and zero BSS
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
                        log::klog(Level::Info, "ELF", &format!("applying {} rels", rela_size / rela_ent));
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
                                let is_relative = r_type == 8; // R_X86_64_RELATIVE
                                #[cfg(target_arch = "aarch64")]
                                let is_relative = r_type == 1027; // R_AARCH64_RELATIVE
                                #[cfg(target_arch = "riscv64")]
                                let is_relative = r_type == 3; // R_RISCV_RELATIVE
                                #[cfg(target_arch = "loongarch64")]
                                let is_relative = r_type == 3; // R_LARCH_RELATIVE

                                if is_relative {
                                    let target_ptr = (buffer_base + r_offset) as *mut u64;
                                    unsafe { *target_ptr = buffer_base.wrapping_add(r_addend as u64); }
                                }
                            }
                        }
                    }
                    final_entry = buffer_base + entry_point;
                } else {
                    final_entry = virt_addr + entry_point;
                }

                // Dedicated stack
                const STACK_SIZE: usize = 64 * 1024;
                let mut stack = alloc::vec![0u8; STACK_SIZE];
                let stack_base = stack.as_ptr() as u64;
                let _stack_top = (stack.as_mut_ptr() as u64 + STACK_SIZE as u64) & !0xf;
                core::mem::forget(stack);
                
                // Create Task for tracking
                // Note: name string lifetime is tricky here, but "sprout" is static str literal usually
                // or we just trust it lives long enough (it's from boot context modules).
                // Actually kernel task name is &'static str in struct.
                let task_id = crate::sched::spawn_kernel_task("sprout");
                crate::sched::mark_as_init(task_id);
                // crate::sched::set_current_task(task_id); // Removed to let scheduler pick it up
                
                // Configure memory
                
                // Heap: Allocate from kernel heap
                const HEAP_SIZE: usize = 4 * 1024; // 4KB
                let heap = alloc::vec![0u8; HEAP_SIZE];
                let heap_base = heap.as_ptr() as u64;
                let heap_size = HEAP_SIZE as u64;
                core::mem::forget(heap);

                // Image base - derived from final entry
                // We allocated this earlier in the ELF loading block.
                let image_base = final_entry - entry_point;
                let image_size = 1024 * 1024; // 1MB fixed
                crate::sched::configure_task_memory(
                    task_id, 
                    (image_base, image_size as u64),
                    (stack_base, STACK_SIZE as u64),
                    (heap_base, heap_size, heap_base) // brk starts at base
                );

                // Configure context (trampoline)
                crate::sched::configure_task_context(task_id, final_entry);

                log::klog(Level::Info, "SPROUT", &format!("task ready {:#x}", final_entry));
                
                // No jump! We return and let the scheduler pick it up.
            }
            break;
        }
    }

    if !found {
        log::klog(Level::Warn, "KERNEL", &format!("module '{}' not found", name));
    }
}



