//! Bran - ThingOS Bootloader Glue (v0.3)
//!
//! Bran is thin boot glue that:
//! 1. Handles Limine protocol parsing
//! 2. Assembles a BootContext (Bag of Facts)
//! 3. Calls kernel::boot(&mut BootContext) and never returns

#![no_std]
#![no_main]

use core::arch::asm;
use limine::request::{
    FramebufferRequest, HhdmRequest, MemoryMapRequest, ModuleRequest, RequestsEndMarker,
    RequestsStartMarker,
};
use limine::BaseRevision;

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
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

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

fn get_serial_base(hhdm_offset: u64) -> Option<*mut u8> {
    unsafe {
        #[cfg(target_arch = "x86_64")]
        {
            None // Port I/O doesn't use memory addresses
        }
        #[cfg(target_arch = "aarch64")]
        {
            // QEMU virt: PL011 at 0x0900_0000
            if hhdm_offset != 0 {
                Some((0x0900_0000 + hhdm_offset) as *mut u8)
            } else {
                None
            }
        }
        #[cfg(target_arch = "riscv64")]
        {
            // QEMU virt: NS16550 at 0x1000_0000
            if hhdm_offset != 0 {
                Some((0x1000_0000 + hhdm_offset) as *mut u8)
            } else {
                None
            }
        }
        #[cfg(target_arch = "loongarch64")]
        {
            // QEMU virt: Serial at 0x1fe001e0
             if hhdm_offset != 0 {
                Some((0x1fe001e0 + hhdm_offset) as *mut u8)
            } else {
                None
            }
        }
    }
}

fn serial_init(hhdm_offset: u64) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        // Initialize COM1
        asm!("out dx, al", in("dx") 0x3F9u16, in("al") 0x00u8, options(nomem, nostack, preserves_flags)); // IER
        asm!("out dx, al", in("dx") 0x3FBu16, in("al") 0x80u8, options(nomem, nostack, preserves_flags)); // LCR (DLAB)
        asm!("out dx, al", in("dx") 0x3F8u16, in("al") 0x01u8, options(nomem, nostack, preserves_flags)); // DLL
        asm!("out dx, al", in("dx") 0x3F9u16, in("al") 0x00u8, options(nomem, nostack, preserves_flags)); // DLM
        asm!("out dx, al", in("dx") 0x3FBu16, in("al") 0x03u8, options(nomem, nostack, preserves_flags)); // LCR (8N1)
        asm!("out dx, al", in("dx") 0x3FAu16, in("al") 0x07u8, options(nomem, nostack, preserves_flags)); // FCR
        asm!("out dx, al", in("dx") 0x3FCu16, in("al") 0x03u8, options(nomem, nostack, preserves_flags)); // MCR
    }

    #[cfg(target_arch = "aarch64")]
    if let Some(base) = get_serial_base(hhdm_offset) {
        unsafe {
            let base = base as *mut u32;
            core::ptr::write_volatile(base.add(0x30 / 4), 0); // UARTCR: disable
            core::ptr::write_volatile(base.add(0x44 / 4), 0x7ff); // UARTICR: clear interrupts
            core::ptr::write_volatile(base.add(0x24 / 4), 13); // UARTIBRD
            core::ptr::write_volatile(base.add(0x28 / 4), 1); // UARTFBRD
            core::ptr::write_volatile(base.add(0x2c / 4), (3 << 5) | (1 << 4)); // UARTLCR_H
            core::ptr::write_volatile(base.add(0x30 / 4), (1 << 9) | (1 << 8) | 1); // UARTCR: enable
        }
    }

    #[cfg(target_arch = "riscv64")]
    if let Some(base) = get_serial_base(hhdm_offset) {
        unsafe {
            let base = base as *mut u8;
            core::ptr::write_volatile(base.add(1), 0x00); // IER
            core::ptr::write_volatile(base.add(3), 0x80); // LCR (DLAB)
            core::ptr::write_volatile(base.add(0), 0x01); // DLL
            core::ptr::write_volatile(base.add(1), 0x00); // DLM
            core::ptr::write_volatile(base.add(3), 0x03); // LCR (8N1)
            core::ptr::write_volatile(base.add(2), 0x07); // FCR
            core::ptr::write_volatile(base.add(4), 0x03); // MCR
        }
    }
}

fn early_putc(c: u8) {
    // Limine Console/Terminal request removed due to compilation issues.
    // We fall back to direct serial input below.

    // On MMIO architectures, we need the HHDM offset because we are in virtual mode
    // but only have the physical address of the UART.
    let hhdm_offset = if let Some(hhdm) = HHDM_REQUEST.get_response() {
        hhdm.offset()
    } else {
        0
    };

    unsafe {
        #[cfg(target_arch = "x86_64")]
        {
            // COM1 (0x3F8) - I/O ports don't use paging
            // Wait for THRE (bit 5) in LSR (port 0x3FD)
            let mut lsr: u8;
            loop {
                asm!("in al, dx", out("al") lsr, in("dx") 0x3FDu16, options(nomem, nostack, preserves_flags));
                if lsr & 0x20 != 0 { break; }
            }
            asm!("out dx, al", in("dx") 0x3F8u16, in("al") c, options(nomem, nostack, preserves_flags));
        }
        #[cfg(target_arch = "aarch64")]
        {
             if let Some(base) = get_serial_base(hhdm_offset) {
                let base = base as *mut u32;
                // Wait for TXFF (bit 5) to be clear in FR (offset 0x18)
                while core::ptr::read_volatile(base.add(0x18 / 4)) & 0x20 != 0 {}
                // Write to DR (offset 0x00)
                core::ptr::write_volatile(base, c as u32);
            }
        }
        #[cfg(target_arch = "riscv64")]
        {
             if let Some(base) = get_serial_base(hhdm_offset) {
                let base = base as *mut u8;
                // Wait for THRE (bit 5) in LSR (offset 5)
                while core::ptr::read_volatile(base.add(5)) & 0x20 == 0 {}
                // Write to THR (offset 0)
                core::ptr::write_volatile(base, c);
            }
        }
        #[cfg(target_arch = "loongarch64")]
        {
             if let Some(base) = get_serial_base(hhdm_offset) {
                let base = base as *mut u8;
                               // Wait for THRE (bit 5) in LSR (offset 5)
                while core::ptr::read_volatile(base.add(5)) & 0x20 == 0 {}
                core::ptr::write_volatile(base, c);
            }
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
        // Just try to print '!' without HHDM or init, desperate measure
        // (Might fail if HHDM is needed but we don't have it yet)
         // ...
    }

    // Initialize serial for early logging as soon as we have HHDM
    if let Some(hhdm) = HHDM_REQUEST.get_response() {
        serial_init(hhdm.offset());
    } else {
        // Emergency fallback if HHDM failed - still try to init with 0 offset?
        // On recent Limine, we should be mapped in HHDM.
        // If not, we can't really print on AArch64/RISC-V/LoongArch safe easily.
        serial_init(0);
    }

    bran_log("BRAN: starting");

    // 1. Collect HHDM and Memory Map info
    if let Some(hhdm) = HHDM_REQUEST.get_response() {
        BOOT_CTX.hhdm_offset = hhdm.offset();
    }

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
