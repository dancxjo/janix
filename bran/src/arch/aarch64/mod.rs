use core::arch::asm;
use kernel::{BootRuntime, PhysRange, PhysRangeKind, BootModuleDesc, BootModuleKind, FramebufferInfo, PixelFormat, IrqState};
use kernel::time::MonotonicClamp;
use crate::requests::{MEMORY_MAP_REQUEST, HHDM_REQUEST, FRAMEBUFFER_REQUEST, SMP_REQUEST, MODULE_REQUEST};

mod simd;

/// The BootRuntime implementation for aarch64.
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
                let cache_ptr = core::ptr::addr_of_mut!(MEMORY_MAP_CACHE);
                let capacity = 128; 

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
                        limine::memory_map::EntryType::BOOTLOADER_RECLAIMABLE => PhysRangeKind::Reserved, 
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

                let offset = self.phys_to_virt_offset();

                for module in resp.modules() {
                     if idx >= capacity {
                        break;
                    }

                    let path_cstr = module.path();
                    let name = path_cstr.to_str().unwrap_or("unknown");
                    
                    let addr = module.addr();
                    let len = module.size() as usize;
                    let bytes = core::slice::from_raw_parts(addr, len);

                    let virt_addr = addr as u64;
                    let phys_start = virt_addr.saturating_sub(offset);
                    let phys_end = phys_start + len as u64;

                    (*cache_ptr)[idx] = BootModuleDesc {
                        name,
                        bytes,
                        phys_start,
                        phys_end,
                        kind: BootModuleKind::Unknown,
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
        let raw = read_cntvct_el0();
        self.serial.clamp.clamp(raw)
    }

    fn mono_freq_hz(&self) -> u64 {
        read_cntfrq_el0()
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

    fn phys_memory_map(&self) -> &'static [kernel::PhysRange] { 
        self.init_memory_map();
        unsafe {
            let ptr = core::ptr::addr_of!(MEMORY_MAP_CACHE);
            let len = *core::ptr::addr_of!(MEMORY_MAP_LEN);
            let slice_ptr = ptr as *const PhysRange;
            core::slice::from_raw_parts(slice_ptr, len)
        }
    }

    fn modules(&self) -> &'static [kernel::BootModuleDesc] { 
        self.init_modules();
        unsafe {
            let ptr = core::ptr::addr_of!(MODULES_CACHE);
            let len = *core::ptr::addr_of!(MODULES_LEN);
            let slice_ptr = ptr as *const BootModuleDesc;
            core::slice::from_raw_parts(slice_ptr, len)
        }
    }
    
    fn page_size(&self) -> usize { 4096 }
    
    fn kernel_virt_base(&self) -> u64 { 
        // Assume -2GB (0xFFFFFFFF80000000) if not provided.
        0xffffffff80000000
    }
    
    fn phys_to_virt_offset(&self) -> u64 { 
         HHDM_REQUEST.get_response()
            .map(|r| r.offset())
            .unwrap_or(0)
    }
    
    fn framebuffer(&self) -> Option<kernel::FramebufferInfo> { 
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
            .map(|r| r.bsp_mpidr() as usize)
            .unwrap_or(0)
    }

    fn irq_disable(&self) -> IrqState {
        // AArch64: Mask DAIF
        let daif: u64;
        unsafe {
            asm!("mrs {}, daif", out(reg) daif, options(nomem, nostack));
            asm!("msr daifset, #2", options(nomem, nostack)); // Mask IRQ (bit 1)
        }
        // Extract original I bit (bit 7 of DAIF)
        IrqState(((daif >> 7) & 1) as usize)
    }

    fn irq_restore(&self, state: IrqState) {
        if state.0 == 0 {
             unsafe { asm!("msr daifclr, #2", options(nomem, nostack)); } // Unmask if it was 0
        } else {
             unsafe { asm!("msr daifset, #2", options(nomem, nostack)); } // Mask if it was 1
        }
    }
    
    fn fence_full(&self) {
         unsafe { asm!("dmb sy", options(nostack, preserves_flags)); }
    }
    
    fn icache_invalidate(&self) {
         unsafe { 
             asm!("ic ialluis", options(nostack, preserves_flags));
             asm!("dsb ish", options(nostack, preserves_flags));
             asm!("isb", options(nostack, preserves_flags));
         }
    }
}

/// Serial port implementation for aarch64 using Semihosting.
/// (PL011 MMIO requires identity mapping of 0x09000000 which may be missing)
pub struct SerialPort {
    pub clamp: MonotonicClamp,
}

impl SerialPort {
    pub const fn new() -> Self {
        Self {
            clamp: MonotonicClamp::new(),
        }
    }

    fn putchar(&self, c: u8) {
        let ch = c;
        unsafe {
            // Semihosting call: SYS_WRITEC (0x03)
            // W0 = Operation 0x03
            // X1 = Pointer to character
            asm!(
                "hlt #0xF000",
                in("w0") 0x03,
                in("x1") &ch,
                options(nostack, preserves_flags)
            );
        }
    }
}

/// Halt and catch fire - enters an infinite wait-for-interrupt loop.
pub fn hcf() -> ! {
    loop {
        unsafe { asm!("wfi") };
    }
}

/// Read the virtual counter frequency (CNTFRQ_EL0)
#[inline]
fn read_cntfrq_el0() -> u64 {
    let val: u64;
    unsafe {
        asm!("mrs {}, cntfrq_el0", out(reg) val, options(nomem, nostack));
    }
    val
}

/// Read the virtual counter count (CNTVCT_EL0)
#[inline]
fn read_cntvct_el0() -> u64 {
    let val: u64;
    unsafe {
        asm!("mrs {}, cntvct_el0", out(reg) val, options(nomem, nostack));
    }
    val
}
