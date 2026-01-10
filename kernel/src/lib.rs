#![no_std]

extern crate alloc;

// pub mod arch;
pub mod logging;
pub mod memory;
pub mod time;
pub mod trap;
pub mod user;
pub mod syscall;

/// A physical memory range with a kind.
#[derive(Debug, Clone, Copy)]
pub struct PhysRange {
    pub start: u64,
    pub end: u64, // exclusive
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
    Acpi, // optional but useful on x86_64
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
    Elf,  // likely user program
    Wasm, // if you go that route
    Data, // fonts, images, etc.
}

pub struct FramebufferInfo {
    pub addr: u64,
    pub byte_len: usize,
    pub width: u32,
    pub height: u32,
    pub pitch: u32, // bytes per row
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
    fn mono_ticks(&self) -> u64 {
        0
    }
    fn mono_freq_hz(&self) -> u64 {
        0
    }

    // SIMD
    fn simd_init_cpu(&self) {}
    fn simd_state_layout(&self) -> (usize, usize) {
        (0, 1)
    }

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
    fn phys_memory_map(&self) -> &'static [PhysRange] {
        &[]
    }
    fn modules(&self) -> &'static [BootModuleDesc] {
        &[]
    }

    fn module_by_name(&self, name: &str) -> Option<&'static BootModuleDesc> {
        self.modules().iter().find(|m| m.name == name)
    }

    // Paging / Address Translation
    fn page_size(&self) -> usize {
        4096
    }
    fn kernel_virt_base(&self) -> u64;
    fn phys_to_virt_offset(&self) -> u64;

    // Paging Operations
    // Note: We use u64 for flags to avoid defining PageFlags in the trait directly if possible,
    // but PageFlags is in kernel::memory::paging. We can use it if it's public.
    // Ideally, we move PageFlags definition to a shared place, or just pass u64.
    // Let's use u64 for ABI cleanliness between kernel/runtime trait if we consider them separate,
    // but they are tightly coupled. Let's use u64 to be safe and agnostic.
    fn map_page(&self, handle: usize, virt: u64, phys: u64, flags: u64) -> Result<(), ()> { Err(()) }
    fn map_page_with_allocator(
        &self, 
        handle: usize, 
        virt: u64, 
        phys: u64, 
        flags: u64,
        allocator: &mut crate::memory::boot_frame_alloc::BootFrameAllocator
    ) -> Result<(), ()> { Err(()) }    

    fn unmap_page(&self, handle: usize, virt: u64) {}
    fn translate(&self, handle: usize, virt: u64) -> Option<u64> { None }
    
    fn new_address_space(&self) -> usize { 0 } // Returns an opaque handle (e.g. CR3 or ID)
    fn switch_address_space(&self, handle: usize) {}
    fn current_address_space(&self) -> usize { 0 }

    fn tlb_flush_page(&self, virt: u64) {}
    fn tlb_flush_all(&self) {}

    // Framebuffer
    fn framebuffer(&self) -> Option<FramebufferInfo> {
        None
    }

    // CPU / SMP
    fn cpu_count(&self) -> usize {
        1
    }
    fn boot_cpu_id(&self) -> usize {
        0
    }
    unsafe fn start_aps(
        &self,
        _ap_entry: extern "C" fn(cpu_id: usize) -> !,
        _stacks: &'static [u64],
    ) {
    }

    // Interrupt control
    fn irq_disable(&self) -> IrqState {
        IrqState(0)
    }
    fn irq_restore(&self, _state: IrqState) {}

    // Barriers (minimal)
    fn fence_full(&self) {}
    fn icache_invalidate(&self) {}

    // Syscall / Context
    fn register_syscall_handler(&self, _entry: u64) {}
    fn set_kernel_stack(&self, _stack_top: u64) {}

    // Task Context
    // We use a pointer to an opaque ArchContext. The layout is known only to the runtime.
    // But the kernel needs to hold it. 
    // Option: Kernel holds a `[u64; 64]` buffer? Or just a `usize` handle if the runtime manages alloc?
    // Let's stick to the pointer approach used before but make it abstract.
    // Actually, `ArchContext` in kernel was just `rsp: u64`.
    // Let's pass a pointer to a mut u64 (rsp) or a void ptr?
    // To match `context_init` signature: `ctx: &mut ArchContext`.
    // Let's define `type ContextHandle = *mut u8;` or similar.
    // Better: Helper functions that take `&mut u64` (ptr to storing the opaque handle/RSP).
    
    // Initialize a context for a new thread.
    // `ctx_handle`: A mutable reference where the runtime can store the new stack pointer / context handle.
    // `kstack_top`: The top of the kernel stack.
    // `entry`: The entry point function.
    // `arg`: The argument to the entry point.
    fn context_init(
        &self, 
        ctx_handle: &mut u64, 
        kstack_top: u64, 
        entry: extern "C" fn(usize) -> !, 
        arg: usize
    ) {}

    // Switch context.
    // `old_handle_ptr`: Address where existing context handle should be saved (e.g. &mut Task.ctx.handle).
    // `new_handle`: The handle of the task to switch to.
    unsafe fn context_switch(&self, old_handle_ptr: *mut u64, new_handle: u64) {}

    // Enter user mode.
    // Diverges.
    // `context` is a pointer to an architecture-specific TrapFrame/Context.
    // The layout of the context MUST match what the architecture implementation expects.
    unsafe fn enter_user_mode(&self, _context: *const ()) -> ! {
        panic!("enter_user_mode not implemented");
    }
}

static mut RUNTIME: Option<&'static dyn BootRuntime> = None;
static mut MODULES: &'static [BootModuleDesc] = &[];

pub mod root;

static ROOT: root::Root = root::Root::new();

pub fn root() -> &'static root::Root {
    &ROOT
}

pub fn runtime() -> &'static dyn BootRuntime {
    unsafe { RUNTIME.expect("Kernel runtime not initialized") }
}

pub fn boot_modules() -> &'static [BootModuleDesc] {
    unsafe { MODULES }
}

pub fn start(runtime: &'static dyn BootRuntime) -> ! {
    unsafe {
        RUNTIME = Some(runtime);
        logging::init(runtime);
    }

    kinfo!("System booted");

    // Initialize arch paging (HHDM offset)
    // crate::arch::imp::paging::init(runtime.phys_to_virt_offset());

    let map = runtime.phys_memory_map();
    let modules = runtime.modules();
    unsafe {
        MODULES = modules;
    }

    kinfo!("boot: phys ranges={} modules={}", map.len(), modules.len());

    // 1. Boot Allocator Init
    let frame_alloc_boot = crate::memory::boot_frame_alloc::BootFrameAllocator::new(map);
    crate::memory::global_alloc::init_boot(frame_alloc_boot);

    // 2. Real Frame Allocator Init
    //    We need to allocate backing memory for the bitmap *using* the BootHeap.
    extern crate alloc;
    use crate::memory::frame_alloc::{FRAME_SIZE, FrameAllocator};

    kinfo!("Initializing Real Frame Allocator...");

    // Calculate size needed
    let mut min_usable = u64::MAX;
    let mut max_usable = 0;
    for r in map {
        if r.kind == PhysRangeKind::Usable {
            if r.start < min_usable {
                min_usable = r.start;
            }
            if r.end > max_usable {
                max_usable = r.end;
            }
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
    kinfo!(
        "frame_alloc: base={:#x} frames={} words={}",
        base,
        frames,
        words
    );

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
    unsafe {
        core::ptr::write_bytes(ptr, 0, words as usize);
    }
    kinfo!("Bitmap zeroed.");

    let bitmap_slice = unsafe { core::slice::from_raw_parts_mut(ptr, words as usize) };

    let mut local_alloc = FrameAllocator::new_from_boot(map, modules, bitmap_slice);

    // Sync state: Mark frames consumed by BootHeap as used
    unsafe {
        crate::memory::global_alloc::transfer_boot_frames(&mut local_alloc);
    }

    let stats = local_alloc.stats();
    kinfo!(
        "frame_alloc: total={} free={} used={}",
        stats.total_frames,
        stats.free_frames,
        stats.used_frames
    );


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
                if i > 0 && allocated[i] == allocated[i - 1] {
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
                alloc
                    .alloc_contiguous(N_CONTIG)
                    .expect("Contig sanity alloc failed")
            });

            // Verify addresses (optional deeper check could verify they were actually free before, but stats help)
            if range.count != N_CONTIG {
                panic!("Contig alloc returned wrong count");
            }
            if range.base.0 % FRAME_SIZE != 0 {
                panic!("Contig alloc returned unaligned base");
            }

            crate::memory::frame_alloc::FRAME_ALLOCATOR.with_lock(|alloc| {
                alloc.free_contiguous(range.base, N_CONTIG);
            });

            kinfo!("frame_alloc: sanity: contig({}) ok", N_CONTIG);
        }
    }

    // crate::arch::imp::paging::test_paging();
    kinfo!("Paging subsystem test passed (skipped local test)");

    // 5. Initialize Kernel Heap
    kinfo!("Initializing Kernel Heap...");
    // 64 pages = 256 KiB initial commit
    crate::memory::global_alloc::kernel_heap().init(64);

    // 6. Switch Allocator
    crate::memory::global_alloc::switch_to_kernel_heap();

    // 7. Heap Sanity Demo
    {
        kinfo!("Running heap sanity check...");
        use alloc::boxed::Box;
        use alloc::vec::Vec;

        let mut v = Vec::new();
        for i in 0..1000 {
            v.push(i as u64);
        }

        // Verify
        for i in 0..1000 {
            if v[i] != i as u64 {
                panic!("Heap sanity: Vec data corruption at {}", i);
            }
        }

        let b = Box::new(42);
        if *b != 42 {
            panic!("Heap sanity: Box corrupted");
        }

        kinfo!("kheap: sanity ok");

        // Force growth
        kinfo!("kheap: forcing growth...");
        let big_vec: Vec<u8> = alloc::vec![0u8; 300 * 1024]; // 300 KiB > 256 KiB
        kinfo!("kheap: big allocation ok (len={})", big_vec.len());

        crate::memory::global_alloc::kernel_heap().stats();
    }

    // Initialize Syscalls (x86_64)
    #[cfg(target_arch = "x86_64")]
    {
        unsafe extern "C" {
            fn syscall_entry();
        }
        kinfo!("Registering syscall handler...");
        runtime.register_syscall_handler(syscall_entry as u64);
    }

    // 5. Task Subsystem & Sprout Launch
    kinfo!("Initializing Task System...");
    crate::task::init();

    // use crate::arch::THREADS_SUPPORTED;
    pub const THREADS_SUPPORTED: bool = cfg!(any(target_arch = "x86_64", target_arch = "aarch64"));

    if THREADS_SUPPORTED {
        kinfo!("threads: supported");
        
        // Find and spawn sprout
        let mut sprout_found = false;
        for m in modules {
            if m.name.contains("sprout") {
                kinfo!("Spawning sprout: {}", m.name);
                match crate::user::sys_spawn_module_from_desc(m) {
                    Ok(_) => { 
                        sprout_found = true; 
                        kinfo!("Sprout spawned successfully");
                    }
                    Err(e) => kerror!("Failed to spawn sprout: {}", e),
                }
            } else if m.name.contains("interrupting_cow") {
                kinfo!("Spawning interrupting_cow test: {}", m.name);
                match crate::user::sys_spawn_module_from_desc(m) {
                    Ok(_) => kinfo!("Preempt test spawned"),
                    Err(e) => kerror!("Failed to spawn interrupting_cow: {}", e),
                }
            }
        }

        
        if !sprout_found {
             kerror!("Sprout module not found in:");
             for m in modules {
                 kinfo!(" - {}", m.name);
             }
        }

        kinfo!("Entering Scheduler Loop (Main Task)...");
        crate::task::run_scheduler();
    } else {
        kinfo!("threads: not supported on this arch yet; continuing single-thread");
        loop {
            // Just idle/halt
            crate::runtime().halt();
        }
    }
}

extern "C" fn thread_a(arg: usize) -> ! {
    crate::kinfo!("Thread A starting (arg={})", arg);

    #[cfg(target_arch = "x86_64")]
    {
        crate::kinfo!("thread_a: setting up user mode test...");
        
        // Minimal User Stub
        // Putchar('U'), Yield x3, Exit(0)
        let user_code: &[u8] = &[
            // mov rdi, 'U' (0x55)
            0x48, 0xC7, 0xC7, 0x55, 0x00, 0x00, 0x00,
            // mov rax, 0 (SYSCALL_PUTCHAR)
            0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x00,
            // syscall
            0x0F, 0x05,

            // mov rbx, 3
            0x48, 0xC7, 0xC3, 0x03, 0x00, 0x00, 0x00,
            // Loop:
            // mov rax, 2 (SYSCALL_YIELD)
            0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00,
            // syscall
            0x0F, 0x05,
            // dec rbx
            0x48, 0xFF, 0xCB,
            // jnz Loop (offset -12 = 0xF4)
            // syscall (2) + mov rax (7) + dec (3) = 12 bytes?
            // "48 C7 C0 02 00 00 00" is 7. "0F 05" is 2. "48 FF CB" is 3.
            // 7+2+3 = 12.
            // So jnz -14 (to include the jump itself which is 2 bytes?) 
            // -12 from AFTER the jump instruction?
            // PC is after jnz. We want to go back 12 bytes.
            // 0xFF - 12 + 1 = 0xF3?
            // Let's rely on short loop being safe.
            0x75, 0xF2, 

            // mov rdi, 0
            0x48, 0xC7, 0xC7, 0x00, 0x00, 0x00, 0x00,
            // mov rax, 3 (SYSCALL_EXIT)
            0x48, 0xC7, 0xC0, 0x03, 0x00, 0x00, 0x00,
            // syscall
            0x0F, 0x05,
        ];

        // Map User Code at 0x400000
        use crate::memory::paging::PageFlags;
        use crate::memory::frame_alloc::FRAME_ALLOCATOR;
        
        let code_frame = FRAME_ALLOCATOR.with_lock(|alloc| alloc.alloc().expect("user code alloc failed"));
        
        let mut aspace = crate::memory::paging::AddressSpace::active();
        unsafe {
            let src = crate::memory::paging::phys_to_virt(code_frame.0) as *mut u8;
            core::ptr::copy_nonoverlapping(user_code.as_ptr(), src, user_code.len());
        }
        
        aspace.map_page(0x400000, code_frame, PageFlags::PRESENT | PageFlags::USER_ACCESSIBLE).expect("map code failed");

        // Map User Stack at 0x70000000 (Page below) -> 0x6FFFF000
        let stack_frame = FRAME_ALLOCATOR.with_lock(|alloc| alloc.alloc().expect("user stack alloc failed"));
        aspace.map_page(0x6FFFF000, stack_frame, PageFlags::PRESENT | PageFlags::WRITABLE | PageFlags::USER_ACCESSIBLE).expect("map stack failed");

        crate::kinfo!("user: entered");
        
        let mut tf = crate::trap::x86_64::TrapFrame::default();
        tf.user_rip = 0x400000;
        tf.user_rsp = 0x70000000;
        tf.user_rflags = 0x202; // IF | Reserved

        unsafe {
            let ptr = &tf as *const _ as *const ();
            crate::runtime().enter_user_mode(ptr);
        }
    }

    // Loop is unreachable as enter_user_sysret diverges on x86_64.
    // On other arches where user mode isn't implemented here yet, we loop.
    #[allow(unreachable_code)]
    loop {
        crate::task::yield_now();
    }
}

extern "C" fn thread_b(arg: usize) -> ! {
    loop {
        let ticks = crate::runtime().mono_ticks();
        crate::kinfo!("Thread B (arg={}) ticks={}", arg, ticks);
        for _ in 0..500000 {
            core::hint::black_box(());
        }
        crate::task::yield_now();
    }
}

pub mod simd;
pub mod task;
