//! Bran - ThingOS Bootloader Glue (v0.3)

#![no_std]
#![no_main]

use limine::request::{
    ExecutableAddressRequest, FramebufferRequest, HhdmRequest, MemoryMapRequest, ModuleRequest,
    RequestsEndMarker, RequestsStartMarker,
};
use limine::BaseRevision;

use kernel::boot::{BootContext, FramebufferInfo, ModuleInfo};
use kernel::PreBootInfo;

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

static mut MODULE_LIST: [ModuleInfo; 64] = [ModuleInfo {
    index: 0,
    path: "",
    cmdline: "",
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

fn early_putc(c: u8) {
    kernel::serial::putc(c);
}

fn bran_log(msg: &str) {
    for &b in msg.as_bytes() {
        early_putc(b);
    }
}

fn bran_logln(msg: &str) {
    bran_log(msg);
    early_putc(b'\n');
}

#[repr(align(16))]
struct Stack([u8; 262144]);

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
    if !BASE_REVISION.is_supported() {
        loop {}
    }

    let hhdm_offset = HHDM_REQUEST.get_response().map(|h| h.offset()).unwrap_or(0);

    let (kernel_phys_base, kernel_virt_base) = EXECUTABLE_ADDRESS_REQUEST
        .get_response()
        .map(|r| (r.physical_base(), r.virtual_base()))
        .unwrap_or((0, 0));

    kernel::boot::pre_boot(PreBootInfo {
        hhdm_offset,
        kernel_phys_base,
        kernel_virt_base,
    });

    bran_logln("BRAN: starting V2");

    unsafe {
        BOOT_CTX.hhdm_offset = hhdm_offset;
        BOOT_CTX.kernel_phys_base = kernel_phys_base;
        BOOT_CTX.kernel_virt_base = kernel_virt_base;
    }

    let mut heap_found = false;
    let heap_size_req = 64 * 1024 * 1024;

    if let Some(mmap) = MEMORY_MAP_REQUEST.get_response() {
        let mut total_mem = 0;
        for entry in mmap.entries() {
            total_mem += entry.length;
            if !heap_found
                && entry.entry_type == limine::memory_map::EntryType::USABLE
                && entry.length >= heap_size_req
            {
                unsafe { BOOT_CTX.heap_phys_base = entry.base; }
                heap_found = true;
            }
        }
        unsafe { BOOT_CTX.physical_memory = total_mem; }
    }

    if !heap_found {
        bran_logln("BRAN: PANIC: Could not find 64MB for kernel heap!");
        loop {}
    }

    // Collect Framebuffer info with color format
    if let Some(fb_res) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(fb) = fb_res.framebuffers().next() {
            let fb_virt = fb.addr() as u64;
            let fb_phys = fb_virt.wrapping_sub(hhdm_offset);
            unsafe {
                BOOT_CTX.framebuffer = Some(FramebufferInfo {
                    addr: fb_phys,
                    width: fb.width(),
                    height: fb.height(),
                    pitch: fb.pitch(),
                    bpp: fb.bpp(),
                    red_mask_size: fb.red_mask_size(),
                    red_mask_shift: fb.red_mask_shift(),
                    green_mask_size: fb.green_mask_size(),
                    green_mask_shift: fb.green_mask_shift(),
                    blue_mask_size: fb.blue_mask_size(),
                    blue_mask_shift: fb.blue_mask_shift(),
                });
            }
        }
    }

    // Collect Modules
    if let Some(mod_res) = MODULE_REQUEST.get_response() {
        let mut count = 0;
        let mods = mod_res.modules();
        bran_log("BRAN: Limine reported module count: ");
        kernel::serial::write_hex(mods.len() as u64);
        bran_logln("");

        for (i, m) in mods.iter().enumerate() {
            if count >= 64 { break; }
            unsafe {
                MODULE_LIST[count] = ModuleInfo {
                    index: i,
                    path: m.path().to_str().unwrap_or("unknown"),
                    cmdline: core::str::from_utf8(m.cmdline()).unwrap_or(""),
                    phys_addr: (m.addr() as u64).wrapping_sub(BOOT_CTX.hhdm_offset),
                    size: m.size() as u64,
                };
                bran_log("BRAN: module[");
                kernel::serial::write_hex(count as u64);
                bran_log("] path: ");
                bran_logln(MODULE_LIST[count].path);
            }
            count += 1;
        }
        unsafe { BOOT_CTX.modules = &MODULE_LIST[..count]; }
    } else {
        bran_logln("BRAN: No Module Request response from Limine!");
    }

    bran_logln("BRAN: handoff to kernel");

    unsafe { kernel::boot::boot(&raw mut BOOT_CTX) }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    bran_logln("\n========== BRAN PANIC ==========");
    if let Some(loc) = info.location() {
        bran_log("Location: ");
        bran_log(loc.file());
        bran_log(":");
        bran_print_num(loc.line() as u64);
        bran_logln("");
    }
    bran_logln("=================================");
    loop {}
}

fn bran_print_num(v: u64) {
    if v == 0 { early_putc(b'0'); return; }
    let mut buf = [0u8; 20];
    let mut n = v;
    let mut i = 19;
    while n > 0 && i > 0 { buf[i] = b'0' + (n % 10) as u8; n /= 10; i -= 1; }
    for b in &buf[i+1..] { early_putc(*b); }
}
