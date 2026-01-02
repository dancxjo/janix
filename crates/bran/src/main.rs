//! Bran - ThingOS Bootloader Glue (v0.3)
//!
//! Bran is thin boot glue that:
//! 1. Handles Limine protocol parsing
//! 2. Assembles a BootContext (Bag of Facts)
//! 3. Calls kernel::boot(BootContext) and never returns

#![no_std]
#![no_main]

use core::arch::asm;
use limine::request::{
    FramebufferRequest, HhdmRequest, ExecutableAddressRequest, MemoryMapRequest, ModuleRequest,
    RequestsEndMarker, RequestsStartMarker,
};
use limine::BaseRevision;

use kernel::boot::{BootContext, FramebufferInfo, ModuleInfo};
use kernel::PreBootInfo;

// =============================================================================
// Limine Requests
// =============================================================================

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
static MODULE_REQUEST: ModuleRequest = ModuleRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
static EXECUTABLE_ADDRESS_REQUEST: ExecutableAddressRequest = ExecutableAddressRequest::new();

#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

// =============================================================================
// Static Storage for Boot Facts
// =============================================================================

static mut MODULE_LIST: [ModuleInfo; 64] = [ModuleInfo {
    index: 0,
    path: "",
    phys_addr: 0,
    size: 0,
}; 64];

static mut BOOT_CTX: BootContext = BootContext {
    hhdm_offset: 0,
    physical_memory: 0,
    cmdline: None,
    framebuffer: None,
    modules: &[],
    early_putc: Some(early_putc),
    kernel_phys_base: 0,
    kernel_virt_base: 0,
    heap_phys_base: 0,
};

// =============================================================================
// Early Bringup Utilities
// =============================================================================

/// Write a byte to the console via the kernel's Machine interface.
///
/// After pre_boot() is called, this routes through proper MMIO mappings.
fn early_putc(c: u8) {
    kernel::serial::putc(c);
}

fn bran_log(msg: &str) {
    for &b in msg.as_bytes() {
        early_putc(b);
    }
    early_putc(b'\n');
}

// =============================================================================
// Entry Point
// =============================================================================

// Global Allocator removed - provided by kernel

// =============================================================================
// Stack and Entry
// =============================================================================

#[repr(align(16))]
struct Stack([u8; 262144]); // 256KB

#[used]
#[unsafe(no_mangle)]
static mut BOOT_STACK: Stack = Stack([0; 262144]);

#[cfg(target_arch = "x86_64")]
core::arch::global_asm!(
    ".section .text",
    ".global _start",
    "_start:",
    "mov rsp, offset BOOT_STACK + 262144",
    "jmp kmain"
);

#[cfg(target_arch = "aarch64")]
core::arch::global_asm!(
    ".section .text",
    ".global _start",
    "_start:",
    "ldr x9, =BOOT_STACK",
    "add x9, x9, #262144",
    "mov sp, x9",
    "b kmain"
);

#[cfg(target_arch = "riscv64")]
core::arch::global_asm!(
    ".section .text",
    ".global _start",
    "_start:",
    "la sp, BOOT_STACK",
    "li t0, 262144",
    "add sp, sp, t0",
    "tail kmain"
);

#[cfg(target_arch = "loongarch64")]
core::arch::global_asm!(
    ".section .text",
    ".global _start",
    "_start:",
    "la.global $sp, BOOT_STACK",
    "li.d $t0, 262144",
    "add.d $sp, $sp, $t0",
    "b kmain"
);

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    // Verify Limine protocol
    if !BASE_REVISION.is_supported() {
        loop {}
    }

    // Get HHDM offset first - needed for MMIO mapping
    let hhdm_offset = HHDM_REQUEST.get_response()
        .map(|h| h.offset())
        .unwrap_or(0);

    // Get kernel physical/virtual base addresses for MMIO page table setup
    let (kernel_phys_base, kernel_virt_base) = EXECUTABLE_ADDRESS_REQUEST.get_response()
        .map(|r| (r.physical_base(), r.virtual_base()))
        .unwrap_or((0, 0));

    // Initialize Machine interface before ANY logging
    // This maps UART MMIO on AArch64/RISC-V, making serial output safe
    kernel::pre_boot(PreBootInfo {
        hhdm_offset,
        kernel_phys_base,
        kernel_virt_base,
    });

    bran_log("BRAN: starting");

    // 1. Collect HHDM and Memory Map info
    // (already obtained above for pre_boot)
    BOOT_CTX.hhdm_offset = hhdm_offset;
    BOOT_CTX.kernel_phys_base = kernel_phys_base;
    BOOT_CTX.kernel_virt_base = kernel_virt_base;

    let mut heap_found = false;
    let heap_size_req = 64 * 1024 * 1024; // 64 MiB

    if let Some(mmap) = MEMORY_MAP_REQUEST.get_response() {
        let mut total_mem = 0;
        for entry in mmap.entries() {
            total_mem += entry.length;
            
            // Look for a usable region for the heap
            // Must be USABLE, big enough, and ideally not overlapping with kernel (Limine shouldn't mark kernel as usable)
            if !heap_found && entry.entry_type == limine::memory_map::EntryType::USABLE && entry.length >= heap_size_req {
                // Check alignment? 4k is fine.
                // We pick the first suitable hole.
                // Note: In a real PMM, we would claim this frame. 
                // Here, we just "take" it and tell the kernel.
                // Does Limine modify the map if we take it? No.
                // The kernel PMM (if it scans this later) must know we took it.
                // For now, ThingOS typically claims all USABLE memory into the PMM.
                // We are stealing a chunk BEFORE PMM init.
                // But wait, the kernel heap IS the allocator.
                // So this region BECOMES the kernel heap.
                
                BOOT_CTX.heap_phys_base = entry.base;
                heap_found = true;
                
                // We should technically ensure we don't clobber modules if they are in "USABLE" space?
                // Limine usually marks modules as KERNEL_AND_MODULES or similar, not USABLE.
                // So this should be safe.
            }
        }
        BOOT_CTX.physical_memory = total_mem;
    }
    
    if !heap_found {
        bran_log("BRAN: PANIC: Could not find 64MB for kernel heap!");
        loop {}
    }

    // 2. Collect Framebuffer info
    if let Some(fb_res) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(fb) = fb_res.framebuffers().next() {
            BOOT_CTX.framebuffer = Some(FramebufferInfo {
                addr: fb.addr() as u64,
                width: fb.width(),
                height: fb.height(),
                pitch: fb.pitch(),
                bpp: fb.bpp(),
            });
        }
    }

    // 3. Collect Modules
    if let Some(mod_res) = MODULE_REQUEST.get_response() {
        let mut count = 0;
        for (i, m) in mod_res.modules().iter().enumerate() {
            if count >= 64 {
                break;
            }
            MODULE_LIST[count] = ModuleInfo {
                index: i,
                path: m.path().to_str().unwrap_or("unknown"),
                phys_addr: (m.addr() as u64).wrapping_sub(BOOT_CTX.hhdm_offset),
                size: m.size() as u64,
            };
            count += 1;
        }
        BOOT_CTX.modules = &MODULE_LIST[..count];
    }

    bran_log("BRAN: handoff to kernel");

    // Hand off to kernel - never returns
    unsafe { kernel::boot(&raw mut BOOT_CTX) }
}

use core::fmt::{self, Write};

struct SerialWriter;

impl fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            early_putc(b);
        }
        Ok(())
    }
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    let mut writer = SerialWriter;
    let _ = writeln!(writer, "\nBRAN: PANIC: {}", info);

    loop {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            asm!("cli; hlt");
            #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
            asm!("wfi");
            #[cfg(target_arch = "loongarch64")]
            asm!("idle 0");
        }
    }
}
