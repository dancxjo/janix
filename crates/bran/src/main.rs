//! Bran - ThingOS Bootloader Glue (v0.3)
//!
//! Bran is thin boot glue that:
//! 1. Handles Limine protocol parsing
//! 2. Assembles a BootContext (Bag of Facts)
//! 3. Calls kernel::boot(&mut BootContext) and never returns

#![no_std]
#![no_main]

use core::arch::asm;
use limine::BaseRevision;
use limine::request::{
    FramebufferRequest, MemoryMapRequest, ModuleRequest,
    RequestsEndMarker, RequestsStartMarker,
};

use kernel::boot::{BootContext, FramebufferInfo, ModuleInfo};

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
static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
static MODULE_REQUEST: ModuleRequest = ModuleRequest::new();

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
};

// =============================================================================
// Early Bringup Utilities
// =============================================================================

fn early_putc(c: u8) {
    unsafe {
        #[cfg(target_arch = "x86_64")]
        {
            // COM1 (0x3F8)
            asm!("out dx, al", in("dx") 0x3F8u16, in("al") c, options(nomem, nostack, preserves_flags));
        }
        #[cfg(target_arch = "aarch64")]
        {
            // PL011 (0x0900_0000)
            core::ptr::write_volatile(0x0900_0000 as *mut u32, c as u32);
        }
        #[cfg(target_arch = "riscv64")]
        {
            // 16550 (0x1000_0000)
            core::ptr::write_volatile(0x1000_0000 as *mut u8, c);
        }
        #[cfg(target_arch = "loongarch64")]
        {
            // 16550 (0x1fe001e0)
            core::ptr::write_volatile(0x1fe001e0 as *mut u8, c);
        }
    }
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

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        static mut HEAP_POS: usize = 0;
        static mut HEAP: [u8; 1024 * 1024] = [0; 1024 * 1024]; // 1MB heap

        unsafe {
            let align = layout.align();
            let size = layout.size();

            let aligned_pos = (HEAP_POS + align - 1) & !(align - 1);
            if aligned_pos + size > HEAP.len() {
                return core::ptr::null_mut();
            }
            let ptr = HEAP.as_mut_ptr().add(aligned_pos);
            HEAP_POS = aligned_pos + size;
            ptr
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator;

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {

    // Verify Limine protocol
    if !BASE_REVISION.is_supported() {
        early_putc(b'!');
        loop {
            #[cfg(target_arch = "x86_64")]
            asm!("cli; hlt");
            #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
            asm!("wfi");
            #[cfg(target_arch = "loongarch64")]
            asm!("idle 0");
        }
    }

    bran_log("BRAN: starting");

    // 1. Collect HHDM and Memory Map info
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
            if count >= 64 { break; }
            MODULE_LIST[count] = ModuleInfo {
                index: i,
                path: m.path().to_str().unwrap_or("unknown"),
                phys_addr: m.addr() as u64,
                size: m.size() as u64,
            };
            count += 1;
        }
        BOOT_CTX.modules = &MODULE_LIST[..count];
    }

    bran_log("BRAN: handoff to kernel");

    // Hand off to kernel - never returns
    kernel::boot(&mut BOOT_CTX)
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    bran_log("BRAN: PANIC!");
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
