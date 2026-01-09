use core::arch::asm;
use kernel::{BootRuntime, PhysRange, PhysRangeKind};
use crate::requests::MEMORY_MAP_REQUEST;

/// The BootRuntime implementation for loongarch64.
pub struct Runtime {
    serial: SerialPort,
}

static mut MEMORY_MAP_CACHE: [PhysRange; 128] = [PhysRange { start: 0, end: 0, kind: PhysRangeKind::Other }; 128];
static mut MEMORY_MAP_LEN: usize = 0;
static mut MEMORY_MAP_INIT: bool = false;

impl Runtime {
    pub const fn new() -> Self {
        Self {
            serial: SerialPort,
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
}

impl BootRuntime for Runtime {
    fn putchar(&self, c: u8) {
        self.serial.putchar(c);
    }

    fn halt(&self) -> ! {
        hcf()
    }

    fn mono_ticks(&self) -> u64 {
        let mut count: u64;
        unsafe { asm!("rdtime.d {}, $r0", out(reg) count) };
        count
    }

    fn mono_freq_hz(&self) -> u64 {
        100_000_000
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

    // Stubs
    fn modules(&self) -> &'static [kernel::BootModuleDesc] { &[] }
    fn page_size(&self) -> usize { 4096 }
    fn kernel_virt_base(&self) -> u64 { 0xffffffff80000000 }
    fn phys_to_virt_offset(&self) -> u64 { 0 }
    fn framebuffer(&self) -> Option<kernel::FramebufferInfo> { None }
    fn cpu_count(&self) -> usize { 1 }
    fn boot_cpu_id(&self) -> usize { 0 }
}

/// Serial port implementation for loongarch64 using NS16550A-compatible UART.
pub struct SerialPort;

impl SerialPort {
    pub const fn new() -> Self {
        Self
    }
}

impl SerialPort {
    fn putchar(&self, c: u8) {
        unsafe {
            // LoongArch QEMU virt machine UART base (NS16550A compatible)
            let base = 0x1fe001e0 as *mut u8;
            base.write_volatile(c);
        }
    }
}

/// Halt and catch fire - enters an infinite idle loop.
pub fn hcf() -> ! {
    loop {
        unsafe { asm!("idle 0") };
    }
}
