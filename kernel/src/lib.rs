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
    
    // 1. Boot Allocator Init
    let frame_alloc_boot = crate::memory::boot_frame_alloc::BootFrameAllocator::new(map);
    crate::memory::global_alloc::init(frame_alloc_boot);

    // 2. Real Frame Allocator Init
    //    We need to allocate backing memory for the bitmap *using* the BootHeap.
    extern crate alloc;
    use alloc::vec;
    use crate::memory::frame_alloc::{FrameAllocator, FRAME_SIZE};

    kinfo!("Initializing Real Frame Allocator...");
    
    // Calculate size needed
    let mut min_usable = u64::MAX;
    let mut max_usable = 0;
    for r in map {
        if r.kind == PhysRangeKind::Usable {
            if r.start < min_usable { min_usable = r.start; }
            if r.end > max_usable { max_usable = r.end; }
        }
    }
    
    // If no memory, we panic or skip
    if min_usable == u64::MAX {
        kinfo!("No usable memory found!");
        runtime.halt();
    }

    let base = (min_usable + FRAME_SIZE - 1) & !(FRAME_SIZE - 1);
    let len_bytes = max_usable.saturating_sub(base);
    let frames = len_bytes / FRAME_SIZE;
    let words = (frames + 63) / 64;
    kinfo!("frame_alloc: base={:#x} frames={} words={}", base, frames, words);

    // Leak the bitmap slice so it lives forever
    let bitmap_slice = vec![0u64; words as usize].leak();
    
    let mut frame_alloc = FrameAllocator::new_from_boot(map, modules, bitmap_slice);
    
    // Sync state: Mark frames consumed by BootHeap as used
    // Access global safely (single threaded boot)
    unsafe {
         crate::memory::global_alloc::get_global().transfer_boot_frames(&mut frame_alloc);
    }
    
    let stats = frame_alloc.stats();
    kinfo!("frame_alloc: total={} free={} used={}", stats.total_frames, stats.free_frames, stats.used_frames);

    // 3. Sanity Check
    //    Alloc N frames, check overlap, free all
    {
        kinfo!("Running frame_alloc sanity check...");
        const N: usize = 32;
        let mut allocated = [crate::memory::frame_alloc::PhysFrame(0); N];
        for i in 0..N {
            allocated[i] = frame_alloc.alloc().expect("Sanity alloc failed");
            // Check exclusion
            if i > 0 && allocated[i] == allocated[i-1] {
                panic!("Allocator returned duplicate frame!");
            }
        }
        
        for i in 0..N {
            frame_alloc.free(allocated[i]);
        }
        
        let stats_after = frame_alloc.stats();
        // Should be same as before?
        // BootHeap might have allocated more during vec![] calls? 
        // No, vec![] happened before frame_alloc init scan. 
        // But vec![] allocation used BootHeap which used BootFrameAllocator.
        // FrameAllocator::new_from_boot synced that state *after* vec allocated.
        // So free/used should match initial stats.
        if stats_after.used_frames != stats.used_frames {
             kinfo!("Warning: Stats mismatch after sanity? used {} vs {}", stats_after.used_frames, stats.used_frames);
             // It's possible if we didn't perfectly reclaim logic, but bitmap allocator should be exact.
        } else {
             kinfo!("frame_alloc: sanity ok"); 
        }
    }

    kinfo!("System halted");
    runtime.halt();
}

pub mod simd;
pub mod task;
