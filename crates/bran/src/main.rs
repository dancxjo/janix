//! Bran - ThingOS Bootloader Glue (v0.3)
//!
//! Bran is thin boot glue that:
//! 1. Handles Limine protocol parsing
//! 2. Constructs a Machine implementation
//! 3. Calls kernel::boot(machine) and never returns

#![no_std]
#![no_main]

extern crate alloc;

use core::arch::asm;

use limine::BaseRevision;
use limine::request::{
    FramebufferRequest, MemoryMapRequest, ModuleRequest,
    RequestsEndMarker, RequestsStartMarker,
};

use kernel::machine::{
    Architecture, BootInfo, Error, IrqToken, Machine, MappedModule, ModuleInfo, ModuleProvider,
};

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
// Machine Implementation
// =============================================================================

/// Bran's Machine implementation
struct BranMachine {
    arch: BranArchitecture,
    modules: BranModuleProvider,
    boot_info: BootInfo,
}

impl BranMachine {
    const fn new() -> Self {
        Self {
            arch: BranArchitecture,
            modules: BranModuleProvider,
            boot_info: BootInfo {
                hhdm_offset: 0,
                physical_memory: 0,
                cmdline: None,
            },
        }
    }
}

impl Machine for BranMachine {
    fn arch(&self) -> &dyn Architecture {
        &self.arch
    }

    fn modules(&self) -> &dyn ModuleProvider {
        &self.modules
    }

    fn boot_info(&self) -> &BootInfo {
        &self.boot_info
    }
}

/// Bran's Architecture implementation
struct BranArchitecture;

impl Architecture for BranArchitecture {
    fn irq_disable(&self) -> IrqToken {
        let flags: u64;
        unsafe {
            #[cfg(target_arch = "x86_64")]
            {
                asm!("pushfq; pop {}; cli", out(reg) flags, options(nomem, preserves_flags));
            }
            #[cfg(not(target_arch = "x86_64"))]
            {
                flags = 0;
                // TODO: Implement for other architectures
            }
        }
        IrqToken(flags)
    }

    fn irq_restore(&self, token: IrqToken) {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            {
                if token.0 & 0x200 != 0 {
                    asm!("sti", options(nomem, preserves_flags));
                }
            }
            #[cfg(not(target_arch = "x86_64"))]
            {
                let _ = token;
                // TODO: Implement for other architectures
            }
        }
    }

    fn cpu_id(&self) -> u32 {
        0 // Uniprocessor for now
    }

    fn halt(&self) -> ! {
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

    fn idle(&self) {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            asm!("hlt");
            #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
            asm!("wfi");
            #[cfg(target_arch = "loongarch64")]
            asm!("idle 0");
        }
    }

    fn debug_putc(&self, c: u8) {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            {
                // Output to QEMU debug port (0xE9)
                asm!("out dx, al", in("dx") 0xE9u16, in("al") c, options(nomem, preserves_flags));
            }
            #[cfg(not(target_arch = "x86_64"))]
            {
                let _ = c;
                // TODO: UART output for other architectures
            }
        }
    }
}

/// Bran's ModuleProvider implementation
struct BranModuleProvider;

impl ModuleProvider for BranModuleProvider {
    fn list(&self, out: &mut dyn FnMut(&ModuleInfo)) {
        if let Some(response) = MODULE_REQUEST.get_response() {
            for (index, module) in response.modules().iter().enumerate() {
                let path = module.path().to_str().unwrap_or("unknown");
                let info = ModuleInfo {
                    index,
                    path,
                    phys_addr: module.addr() as u64,
                    size: module.size() as usize as u64,
                };
                out(&info);
            }
        }
    }

    fn map_ro(&self, index: usize) -> Result<MappedModule, Error> {
        if let Some(response) = MODULE_REQUEST.get_response() {
            if let Some(module) = response.modules().get(index) {
                return Ok(MappedModule {
                    virt_addr: module.addr(),
                    size: module.size() as usize,
                });
            }
        }
        Err(Error::NotFound)
    }

    fn count(&self) -> usize {
        MODULE_REQUEST
            .get_response()
            .map(|r| r.modules().len())
            .unwrap_or(0)
    }
}

// =============================================================================
// Global Allocator (minimal bump allocator)
// =============================================================================

use core::alloc::{GlobalAlloc, Layout};

struct BumpAllocator;

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Minimal allocator - just bump a static pointer
        static mut HEAP_POS: usize = 0;
        static mut HEAP: [u8; 1024 * 1024] = [0; 1024 * 1024]; // 1MB heap

        let align = layout.align();
        let size = layout.size();

        unsafe {
            let aligned_pos = (HEAP_POS + align - 1) & !(align - 1);
            if aligned_pos + size > HEAP.len() {
                return core::ptr::null_mut();
            }
            let ptr = HEAP.as_mut_ptr().add(aligned_pos);
            HEAP_POS = aligned_pos + size;
            ptr
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator doesn't deallocate
    }
}

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator;

// =============================================================================
// Entry Point
// =============================================================================

/// Static machine instance
static MACHINE: BranMachine = BranMachine::new();

/// Early serial logging
fn bran_log(msg: &str) {
    for c in msg.bytes() {
        MACHINE.arch.debug_putc(c);
    }
    MACHINE.arch.debug_putc(b'\n');
}

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    // Verify Limine protocol
    assert!(BASE_REVISION.is_supported());

    // Early Bran logging
    bran_log("BRAN: starting");

    // Draw diagonal line on framebuffer (visual indicator)
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            for i in 0..100_u64 {
                let pixel_offset = i * framebuffer.pitch() + i * 4;
                unsafe {
                    framebuffer
                        .addr()
                        .add(pixel_offset as usize)
                        .cast::<u32>()
                        .write(0xFFFFFFFF);
                }
            }
        }
    }

    bran_log("BRAN: handoff to kernel");

    // Hand off to kernel - never returns
    kernel::boot(&MACHINE)
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    bran_log("BRAN: PANIC!");
    if let Some(location) = info.location() {
        // Can't easily format without alloc, just halt
    }
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
