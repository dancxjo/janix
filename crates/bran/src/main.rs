//! Bran - ThingOS Bootloader Glue (v0.3)
//!
//! Bran is thin boot glue that:
//! 1. Handles Limine protocol parsing
//! 2. Assembles a BootContext (Bag of Facts)
//! 3. Calls kernel::boot(BootContext) and never returns

#![no_std]
#![no_main]

use core::arch::asm;
use core::cell::UnsafeCell;
use limine::request::{
    FramebufferRequest, HhdmRequest, KernelAddressRequest, MemoryMapRequest, ModuleRequest,
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
static KERNEL_ADDRESS_REQUEST: KernelAddressRequest = KernelAddressRequest::new();

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

// =============================================================================
// Global Allocator (minimal bump allocator)
// =============================================================================

use core::alloc::{GlobalAlloc, Layout};

struct BumpAllocator;

const HEAP_SIZE: usize = 4 * 1024 * 1024;

#[repr(align(16))]
struct HeapBuf<const N: usize>(UnsafeCell<[u8; N]>);

unsafe impl<const N: usize> Sync for HeapBuf<N> {}

struct HeapState<const N: usize> {
    pos: UnsafeCell<usize>,
    buf: HeapBuf<N>,
}

unsafe impl<const N: usize> Sync for HeapState<N> {}

static HEAP: HeapState<{ HEAP_SIZE }> = HeapState {
    pos: UnsafeCell::new(0),
    buf: HeapBuf(UnsafeCell::new([0; HEAP_SIZE])),
};

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pos = HEAP.pos.get();
        let heap_base = HEAP.buf.0.get() as *mut u8;

        let align = layout.align();
        let size = layout.size();

        let current = *pos;
        let aligned_pos = (current + align - 1) & !(align - 1);
        if aligned_pos + size > HEAP_SIZE {
            return core::ptr::null_mut();
        }
        let ptr = heap_base.add(aligned_pos);
        *pos = aligned_pos + size;
        ptr
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator;

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    // Verify Limine protocol
    if !BASE_REVISION.is_supported() {
        // Protocol mismatch - halt immediately (can't log safely)
        loop {
            #[cfg(target_arch = "x86_64")]
            asm!("cli; hlt");
            #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
            asm!("wfi");
            #[cfg(target_arch = "loongarch64")]
            asm!("idle 0");
        }
    }

    // Get HHDM offset first - needed for MMIO mapping
    let hhdm_offset = HHDM_REQUEST.get_response()
        .map(|h| h.offset())
        .unwrap_or(0);

    // Get kernel physical/virtual base addresses for MMIO page table setup
    let (kernel_phys_base, kernel_virt_base) = KERNEL_ADDRESS_REQUEST.get_response()
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

    if let Some(mmap) = MEMORY_MAP_REQUEST.get_response() {
        let mut total_mem = 0;
        for entry in mmap.entries() {
            total_mem += entry.length;
        }
        BOOT_CTX.physical_memory = total_mem;
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
