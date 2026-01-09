use core::arch::asm;
use kernel::{BootRuntime, PhysRange, PhysRangeKind, BootModuleDesc, BootModuleKind, FramebufferInfo, PixelFormat, IrqState};
use crate::requests::{MEMORY_MAP_REQUEST, HHDM_REQUEST, FRAMEBUFFER_REQUEST, SMP_REQUEST, MODULE_REQUEST};

use super::simd;
use super::serial::{SerialPort, rdtsc};

/// The BootRuntime implementation for x86_64.
pub struct Runtime {
    serial: SerialPort,
}

static mut MEMORY_MAP_CACHE: [PhysRange; 128] = [PhysRange { start: 0, end: 0, kind: PhysRangeKind::Other }; 128];
static mut MEMORY_MAP_LEN: usize = 0;
static mut MEMORY_MAP_INIT: bool = false;

static mut MODULES_CACHE: [BootModuleDesc; 32] = [BootModuleDesc { 
    name: "", 
    bytes: &[], 
    phys_start: 0, 
    phys_end: 0, 
    kind: BootModuleKind::Unknown 
}; 32];
static mut MODULES_LEN: usize = 0;
static mut MODULES_INIT: bool = false;

impl Runtime {
    pub const fn new() -> Self {
        Self {
            serial: SerialPort::new(),
        }
    }

    fn init_memory_map(&self) {
        unsafe {
            if *core::ptr::addr_of!(MEMORY_MAP_INIT) {
                return;
            }
            
            if let Some(resp) = MEMORY_MAP_REQUEST.get_response() {
                let mut idx = 0;
                // Use raw pointers to avoid creating checking references to static mut
                let cache_ptr = core::ptr::addr_of_mut!(MEMORY_MAP_CACHE);
                let capacity = 128; // Hardcoded size matches declaration

                for entry in resp.entries() {
                    if idx >= capacity {
                        break;
                    }

                    let kind = match entry.entry_type {
                        limine::memory_map::EntryType::USABLE => PhysRangeKind::Usable,
                        limine::memory_map::EntryType::RESERVED => PhysRangeKind::Reserved,
                        limine::memory_map::EntryType::ACPI_RECLAIMABLE => PhysRangeKind::Acpi,
                        limine::memory_map::EntryType::ACPI_NVS => PhysRangeKind::Acpi,
                        limine::memory_map::EntryType::BAD_MEMORY => PhysRangeKind::Other,
                        limine::memory_map::EntryType::BOOTLOADER_RECLAIMABLE => PhysRangeKind::Reserved, // Can be reclaimed later if we want
                        limine::memory_map::EntryType::EXECUTABLE_AND_MODULES => PhysRangeKind::KernelImage,
                        limine::memory_map::EntryType::FRAMEBUFFER => PhysRangeKind::Framebuffer,
                        _ => PhysRangeKind::Other,
                    };

                    (*cache_ptr)[idx] = PhysRange {
                        start: entry.base,
                        end: entry.base + entry.length,
                        kind,
                    };
                    idx += 1;
                }
                *core::ptr::addr_of_mut!(MEMORY_MAP_LEN) = idx;
            }

            *core::ptr::addr_of_mut!(MEMORY_MAP_INIT) = true;
        }
    }

    fn init_modules(&self) {
        unsafe {
            if *core::ptr::addr_of!(MODULES_INIT) {
                return;
            }

            if let Some(resp) = MODULE_REQUEST.get_response() {
                let mut idx = 0;
                let cache_ptr = core::ptr::addr_of_mut!(MODULES_CACHE);
                let capacity = 32;

                // get HHDM offset for physical address calculation
                // If HHDM failed, we assume identity (0), but it shouldn't fail if we are here usually.
                let offset = self.phys_to_virt_offset();

                for module in resp.modules() {
                     if idx >= capacity {
                        break;
                    }

                    let path_cstr = module.path();
                    let name = path_cstr.to_str().unwrap_or("unknown");
                    
                    let addr = module.addr(); // This is virtual (HHDM)
                    let len = module.size() as usize;
                    let bytes = core::slice::from_raw_parts(addr, len);

                    // Phys = Virt - Offset
                    // Note: addr is a pointer, cast to u64
                    let virt_addr = addr as u64;
                    let phys_start = virt_addr.saturating_sub(offset);
                    let phys_end = phys_start + len as u64;

                    (*cache_ptr)[idx] = BootModuleDesc {
                        name,
                        bytes,
                        phys_start,
                        phys_end,
                        kind: BootModuleKind::Unknown, // Could parse extension here
                    };

                    idx += 1;
                }
                *core::ptr::addr_of_mut!(MODULES_LEN) = idx;
            }
            *core::ptr::addr_of_mut!(MODULES_INIT) = true;
        }
    }
}

impl BootRuntime for Runtime {
    fn putchar(&self, c: u8) {
        self.serial.putchar(c);
    }

    fn halt(&self) -> ! {
        hcf()
    }

    fn mono_ticks(&self) -> u64 {
        unsafe {
            let raw = rdtsc();
            self.serial.clamp.clamp(raw)
        }
    }

    fn mono_freq_hz(&self) -> u64 {
        self.serial.calibrate()
    }

    fn simd_init_cpu(&self) {
        simd::init_cpu();
    }

    fn simd_state_layout(&self) -> (usize, usize) {
        simd::STATE_LAYOUT
    }

    unsafe fn simd_save(&self, dst: *mut u8) {
        unsafe { simd::save(dst) };
    }

    unsafe fn simd_restore(&self, src: *const u8) {
        unsafe { simd::restore(src) };
    }

    fn phys_memory_map(&self) -> &'static [PhysRange] {
        self.init_memory_map();
        unsafe {
            let ptr = core::ptr::addr_of!(MEMORY_MAP_CACHE);
            let len = *core::ptr::addr_of!(MEMORY_MAP_LEN);
            // Cast: *const [PhysRange; 128] -> *const PhysRange
            let slice_ptr = ptr as *const PhysRange;
            core::slice::from_raw_parts(slice_ptr, len)
        }
    }

    fn modules(&self) -> &'static [BootModuleDesc] {
        self.init_modules();
        unsafe {
            let ptr = core::ptr::addr_of!(MODULES_CACHE);
            let len = *core::ptr::addr_of!(MODULES_LEN);
            let slice_ptr = ptr as *const BootModuleDesc;
            core::slice::from_raw_parts(slice_ptr, len)
        }
    }

    fn page_size(&self) -> usize {
        4096
    }

    fn kernel_virt_base(&self) -> u64 {
        // Assume -2GB (0xFFFFFFFF80000000) if not provided.
        0xffffffff80000000
    }

    fn phys_to_virt_offset(&self) -> u64 {
         HHDM_REQUEST.get_response()
            .map(|r| r.offset())
            .unwrap_or(0)
    }

    fn framebuffer(&self) -> Option<FramebufferInfo> {
        FRAMEBUFFER_REQUEST.get_response()?.framebuffers().next().map(|fb| {
            FramebufferInfo {
                addr: fb.addr() as u64,
                byte_len: (fb.pitch() * fb.height()) as usize, 
                width: fb.width() as u32,
                height: fb.height() as u32,
                pitch: fb.pitch() as u32,
                bpp: fb.bpp() as u16,
                format: match (fb.red_mask_shift(), fb.green_mask_shift(), fb.blue_mask_shift(), fb.bpp()) {
                     (16, 8, 0, 32) => PixelFormat::Xrgb8888,
                     _ => PixelFormat::Unknown,
                }
            }
        })
    }

    fn cpu_count(&self) -> usize {
         SMP_REQUEST.get_response().map(|r| r.cpus().len()).unwrap_or(1)
    }

    fn boot_cpu_id(&self) -> usize {
        SMP_REQUEST.get_response()
            .map(|r| r.bsp_lapic_id() as usize)
            .unwrap_or(0)
    }
    
    fn irq_disable(&self) -> IrqState {
        let rflags: usize;
        unsafe {
            asm!("pushfq; pop {}", out(reg) rflags, options(nomem, preserves_flags));
            asm!("cli", options(nomem, nostack));
        }
        IrqState((rflags >> 9) & 1) 
    }

    fn irq_restore(&self, state: IrqState) {
        if state.0 != 0 {
            unsafe { asm!("sti", options(nomem, nostack)) };
        } else {
             unsafe { asm!("cli", options(nomem, nostack)) };
        }
    }
    
    fn fence_full(&self) {
        // mfence for x86_64 full barrier
        unsafe { asm!("mfence", options(nostack, preserves_flags)) };
    }
    
    fn icache_invalidate(&self) {
        // x86_64 is coherent, usually no-op.
    }
}

/// Halt and catch fire - enters an infinite halt loop.
pub fn hcf() -> ! {
    loop {
        unsafe { asm!("hlt") };
    }
}
