#![no_std]

extern crate alloc;

pub mod logging;
pub mod time;
pub mod memory;
pub mod arch;

/// A physical memory range with a kind.
#[derive(Debug, Clone, Copy)]
pub struct PhysRange {
    pub start: u64,
    pub end: u64,      // exclusive
    pub kind: PhysRangeKind,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysRangeKind {
    Usable,
    Reserved,
    Mmio,
    Firmware,
    KernelImage,
    BootModule,
    Framebuffer,
    Acpi,        // optional but useful on x86_64
    Other,
}

/// Description of a boot-loaded module.
#[derive(Clone, Copy)]
pub struct BootModuleDesc {
    /// Bootloader-provided module identifier (usually a path like "/boot/modules/sprout")
    pub name: &'static str,

    /// Module contents mapped read-only (ideally) into kernel address space.
    pub bytes: &'static [u8],

    /// Optional physical range (useful for diagnostics / later remapping)
    pub phys_start: u64,
    pub phys_end: u64, // exclusive

    /// Optional classification hint (kernel can ignore)
    pub kind: BootModuleKind,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BootModuleKind {
    Unknown,
    Elf,        // likely user program
    Wasm,       // if you go that route
    Data,       // fonts, images, etc.
}

pub struct FramebufferInfo {
    pub addr: u64,
    pub byte_len: usize,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,     // bytes per row
    pub bpp: u16,
    pub format: PixelFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Xrgb8888,
    Argb8888,
    Rgb565,
    // add as needed
    Unknown,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct IrqState(pub usize);

pub trait BootRuntime {
    // Output / halt
    fn putchar(&self, c: u8);
    fn halt(&self) -> !;

    // Time
    fn mono_ticks(&self) -> u64 { 0 }
    fn mono_freq_hz(&self) -> u64 { 0 }

    // SIMD
    fn simd_init_cpu(&self) {}
    fn simd_state_layout(&self) -> (usize, usize) { (0, 1) }
    
    /// Save current CPU SIMD state into `dst`.
    /// 
    /// # Safety
    /// `dst` must be valid for writes of size `layout.size` and aligned to `layout.align`.
    unsafe fn simd_save(&self, _dst: *mut u8) {}

    /// Restore CPU SIMD state from `src`.
    /// 
    /// # Safety
    /// `src` must be valid for reads of size `layout.size` and aligned to `layout.align`.
    unsafe fn simd_restore(&self, _src: *const u8) {}

    // Memory facts
    fn phys_memory_map(&self) -> &'static [PhysRange] { &[] }
    fn modules(&self) -> &'static [BootModuleDesc] { &[] }

    fn module_by_name(&self, name: &str) -> Option<&'static BootModuleDesc> {
        self.modules().iter().find(|m| m.name == name)
    }

    // Paging / Address Translation
    fn page_size(&self) -> usize { 4096 }
    fn kernel_virt_base(&self) -> u64; 
    fn phys_to_virt_offset(&self) -> u64;

    // Framebuffer
    fn framebuffer(&self) -> Option<FramebufferInfo> { None }

    // CPU / SMP
    fn cpu_count(&self) -> usize { 1 }
    fn boot_cpu_id(&self) -> usize { 0 }
    unsafe fn start_aps(
        &self,
        _ap_entry: extern "C" fn(cpu_id: usize) -> !,
        _stacks: &'static [u64],
    ) { }

    // Interrupt control
    fn irq_disable(&self) -> IrqState { IrqState(0) }
    fn irq_restore(&self, _state: IrqState) {}

    // Barriers (minimal)
    fn fence_full(&self) {}
    fn icache_invalidate(&self) {}
}

pub fn start(runtime: &'static dyn BootRuntime) -> ! {
    unsafe {
        logging::init(runtime);
    }

    kinfo!("System booted");
    
    // Initialize arch paging (HHDM offset)
    crate::arch::imp::paging::init(runtime.phys_to_virt_offset());

    let map = runtime.phys_memory_map();
    let modules = runtime.modules();
    
    kinfo!("boot: phys ranges={} modules={}", map.len(), modules.len());
    
    if map.is_empty() {
        // This might happen if Bran didn't provide a map or we failed to read it.
        // It's not necessarily fatal if we have defaults, but for this task we expect a map.
        kinfo!("Warning: No physical memory map provided!");
    }

    let frame_alloc = crate::memory::boot_frame_alloc::BootFrameAllocator::new(map);
    crate::memory::global_alloc::init(frame_alloc);

    // Sanity check
    {
        extern crate alloc;
        use alloc::vec::Vec;
        use alloc::boxed::Box;
        
        kinfo!("Allocating Box...");
        let b = Box::new(42);
        kinfo!("Boxed value: {}", *b);
        
        kinfo!("Allocating Vec...");
        let mut v = Vec::new();
        for i in 0..100 {
            v.push(i);
        }
        kinfo!("Vec length: {}", v.len());
    }
    
    unsafe {
         // Accessing global allocator via static for stats
         // We can't easily access GLOBAL directly if it's not pub.
         // But we added stats() to BootGlobalAlloc and GLOBAL is static.
         // We didn't make GLOBAL pub in the file though. 
         // Let's just trust the heap logs internally if we want, or make GLOBAL pub.
         // Actually global_alloc.rs didn't make GLOBAL pub.
         // But we can invoke a helper? No helper for stats.
         // Let's just skip explicit stats call for now, BootHeap logs stats on OOM.
         // Or I can add `pub fn print_stats()` to `global_alloc.rs`.
    }

    kinfo!("System halted");
    runtime.halt();
}
pub mod simd;
pub mod task;
