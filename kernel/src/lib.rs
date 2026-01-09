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

static mut RUNTIME: Option<&'static dyn BootRuntime> = None;

pub fn runtime() -> &'static dyn BootRuntime {
    unsafe { RUNTIME.expect("Kernel runtime not initialized") }
}

pub fn start(runtime: &'static dyn BootRuntime) -> ! {
    unsafe {
        RUNTIME = Some(runtime);
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
    crate::memory::global_alloc::init_boot(frame_alloc_boot);

    // 2. Real Frame Allocator Init
    //    We need to allocate backing memory for the bitmap *using* the BootHeap.
    extern crate alloc;
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
    kinfo!("Allocating bitmap of {} words...", words);
    // Use manual alloc to debug
    // let bitmap_slice = vec![0u64; words as usize].leak();
    let layout = alloc::alloc::Layout::from_size_align(words as usize * 8, 8).unwrap();
    let ptr = unsafe { alloc::alloc::alloc(layout) } as *mut u64;
    kinfo!("Bitmap allocated at {:p}", ptr);
    
    if ptr.is_null() {
        panic!("Bitmap alloc failed");
    }
    
    // Zero it manually to see if write faults
    kinfo!("Zeroing bitmap...");
    unsafe { core::ptr::write_bytes(ptr, 0, words as usize); }
    kinfo!("Bitmap zeroed.");
    
    let bitmap_slice = unsafe { core::slice::from_raw_parts_mut(ptr, words as usize) };
    
    let mut local_alloc = FrameAllocator::new_from_boot(map, modules, bitmap_slice);
    
    // Sync state: Mark frames consumed by BootHeap as used
    unsafe {
         crate::memory::global_alloc::transfer_boot_frames(&mut local_alloc);
    }
    
    let stats = local_alloc.stats();
    kinfo!("frame_alloc: total={} free={} used={}", stats.total_frames, stats.free_frames, stats.used_frames);

    // Initialize global allocator
    unsafe {
        crate::memory::frame_alloc::FRAME_ALLOCATOR.init(local_alloc);
    }

    // 3. Sanity Check
    //    Alloc N frames, check overlap, free all
    {
        kinfo!("Running frame_alloc sanity check...");
        const N: usize = 32;
        let mut allocated = [crate::memory::frame_alloc::PhysFrame(0); N];
        
        crate::memory::frame_alloc::FRAME_ALLOCATOR.with_lock(|alloc| {
            for i in 0..N {
                allocated[i] = alloc.alloc().expect("Sanity alloc failed");
                // Check exclusion
                if i > 0 && allocated[i] == allocated[i-1] {
                     panic!("Allocator returned duplicate frame!");
                }
            }
            
            for i in 0..N {
                alloc.free(allocated[i]);
            }
        });
        
        // no easy access to stats via with_lock wrapper yet without returning it, but that's fine.
        kinfo!("frame_alloc: sanity: single ok"); 

        // 4. Contiguous Sanity Check
        {
            const N_CONTIG: u64 = 8;
            let range = crate::memory::frame_alloc::FRAME_ALLOCATOR.with_lock(|alloc| {
                alloc.alloc_contiguous(N_CONTIG).expect("Contig sanity alloc failed")
            });
            
            // Verify addresses (optional deeper check could verify they were actually free before, but stats help)
            if range.count != N_CONTIG { panic!("Contig alloc returned wrong count"); }
            if range.base.0 % FRAME_SIZE != 0 { panic!("Contig alloc returned unaligned base"); }
            
            crate::memory::frame_alloc::FRAME_ALLOCATOR.with_lock(|alloc| {
                alloc.free_contiguous(range.base, N_CONTIG);
            });
             
            kinfo!("frame_alloc: sanity: contig({}) ok", N_CONTIG);
        }
    }
    
    crate::arch::imp::paging::test_paging();
    kinfo!("Paging subsystem test passed");

    // 5. Initialize Kernel Heap
    kinfo!("Initializing Kernel Heap...");
    // 64 pages = 256 KiB initial commit
    crate::memory::global_alloc::kernel_heap().init(64);
    
    // 6. Switch Allocator
        crate::memory::global_alloc::switch_to_kernel_heap();
    
    // 7. Heap Sanity Demo
    {
        kinfo!("Running heap sanity check...");
        use alloc::vec::Vec;
        use alloc::boxed::Box;
        
        let mut v = Vec::new();
        for i in 0..1000 {
            v.push(i as u64);
        }
        
        // Verify
        for i in 0..1000 {
            if v[i] != i as u64 { panic!("Heap sanity: Vec data corruption at {}", i); }
        }
        
        let b = Box::new(42);
        if *b != 42 { panic!("Heap sanity: Box corrupted"); }
        
        kinfo!("kheap: sanity ok"); 
        
        // Force growth
        kinfo!("kheap: forcing growth...");
        let big_vec: Vec<u8> = alloc::vec![0u8; 300 * 1024]; // 300 KiB > 256 KiB
        kinfo!("kheap: big allocation ok (len={})", big_vec.len());
    }

    // Diagnostics
    crate::memory::global_alloc::kernel_heap().stats();

    kinfo!("System halted");
    runtime.halt();
}

pub mod simd;
pub mod task;
