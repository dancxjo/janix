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
static mut GLOBAL_BOOT_CONTEXT: Option<&'static BootContext> = None;

pub fn get_boot_ctx() -> &'static BootContext {
    unsafe { GLOBAL_BOOT_CONTEXT.expect("BootContext not initialized") }
}

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

#[link_section = ".text"]
static mut SPROUT_BUFFER: [u8; 1024 * 1024] = [0u8; 1024 * 1024];

use alloc::format;

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

                    let buffer_base = unsafe { SPROUT_BUFFER.as_mut_ptr() as u64 };
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
                let stack_top = (stack.as_mut_ptr() as u64 + STACK_SIZE as u64) & !0xf;
                core::mem::forget(stack);

                log::klog(Level::Info, "SPROUT", &format!("jumping to {:#x} with stack {:#x}", final_entry, stack_top));

                unsafe {
                    #[cfg(target_arch = "x86_64")]
                    core::arch::asm!(
                        "mov rsp, {stack_top}",
                        "xor rbp, rbp",
                        "jmp {entry}",
                        stack_top = in(reg) stack_top,
                        entry = in(reg) final_entry,
                        in("rdi") crate::syscall::dispatch as *mut () as u64,
                        options(noreturn)
                    );

                    #[cfg(target_arch = "aarch64")]
                    core::arch::asm!(
                        "mov sp, {stack_top}",
                        "mov x29, xzr",
                        "br {entry}",
                        stack_top = in(reg) stack_top,
                        entry = in(reg) final_entry,
                        in("x0") crate::syscall::dispatch as *mut () as u64,
                        options(noreturn)
                    );

                    #[cfg(target_arch = "riscv64")]
                    core::arch::asm!(
                        "mv sp, {stack_top}",
                        "mv s0, zero",
                        "jr {entry}",
                        stack_top = in(reg) stack_top,
                        entry = in(reg) final_entry,
                        in("a0") crate::syscall::dispatch as *mut () as u64,
                        options(noreturn)
                    );

                    #[cfg(target_arch = "loongarch64")]
                    core::arch::asm!(
                        "move $sp, {stack_top}",
                        "move $fp, $zero",
                        "jirl $zero, {entry}, 0",
                        stack_top = in(reg) stack_top,
                        entry = in(reg) final_entry,
                        in("$a0") crate::syscall::dispatch as *mut () as u64,
                        options(noreturn)
                    );
                }
            }
            break;
        }
    }

    if !found {
        log::klog(Level::Warn, "KERNEL", &format!("module '{}' not found", name));
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
    graph::thing_register_name(root_id, root_name);

    log::klog(Level::Info, "PLACE", &format!("root created: {:?}", root_id));

    // Create kernel identity
    let kernel_id = graph::thing_create(kind_thing, SymbolId::INVALID, 1);
    log::klog(Level::Info, "THING", &format!("kernel identity created: {:?}", kernel_id));

    // Relate kernel to root (root contains kernel)
    let rel_id = graph::relationship_create(root_id, kernel_id, pred_contains);
    log::klog(Level::Info, "REL", &format!("contains created: {:?} from={:?} to={:?}", rel_id, root_id, kernel_id));

    // Index rebuild test
    graph::rebuild_indexes();
    log::klog(Level::Info, "KERNEL", "place store indexes rebuilt");

    // Verify containment
    let contained = place::contained_in(root_id);
    log::klog(Level::Info, "PLACE", &format!("root contains {} things", contained.len()));
    
    if contained.len() == 1 && contained[0] == kernel_id {
        log::klog(Level::Info, "KERNEL", "ontology self-test passed");
    } else {
        log::klog(Level::Error, "KERNEL", "ontology self-test FAILED");
    }
}

