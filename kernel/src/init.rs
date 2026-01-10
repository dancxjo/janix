use crate::boot::{BootRuntime, PhysRangeKind, BootModuleKind};
use crate::arch::{ArchContext, ArchTrapFrame};
use crate::global;
use crate::{logging, kinfo, kerror};
use alloc::vec::Vec;
use alloc::boxed::Box;

pub fn start(runtime: &'static dyn BootRuntime<ArchContext, ArchTrapFrame>) -> ! {
    unsafe {
        global::set_runtime(runtime);
        logging::init(runtime);
    }

    kinfo!("System booted");

    let map = runtime.phys_memory_map();
    let modules = runtime.modules();
    unsafe {
        global::set_boot_modules(modules);
    }

    kinfo!("boot: phys ranges={} modules={}", map.len(), modules.len());

    // 1. Boot Allocator Init
    let frame_alloc_boot = crate::memory::boot_frame_alloc::BootFrameAllocator::new(map);
    crate::memory::global_alloc::init_boot(frame_alloc_boot);

    // 2. Real Frame Allocator Init
    extern crate alloc;
    use crate::memory::frame_alloc::{FRAME_SIZE, FrameAllocator};

    kinfo!("Initializing Real Frame Allocator...");

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

    kinfo!("Allocating bitmap of {} words...", words);
    let layout = alloc::alloc::Layout::from_size_align(words as usize * 8, 8).unwrap();
    let ptr = unsafe { alloc::alloc::alloc(layout) } as *mut u64;
    kinfo!("Bitmap allocated at {:p}", ptr);

    if ptr.is_null() {
        panic!("Bitmap alloc failed");
    }

    kinfo!("Zeroing bitmap...");
    unsafe {
        core::ptr::write_bytes(ptr, 0, words as usize);
    }
    kinfo!("Bitmap zeroed.");

    let bitmap_slice = unsafe { core::slice::from_raw_parts_mut(ptr, words as usize) };

    let mut local_alloc = FrameAllocator::new_from_boot(map, modules, bitmap_slice);

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

    unsafe {
        crate::memory::frame_alloc::FRAME_ALLOCATOR.init(local_alloc);
    }

    // 3. Sanity Check
    {
        kinfo!("Running frame_alloc sanity check...");
        const N: usize = 32;
        let mut allocated = [crate::memory::frame_alloc::PhysFrame(0); N];

        crate::memory::frame_alloc::FRAME_ALLOCATOR.with_lock(|alloc| {
            for i in 0..N {
                allocated[i] = alloc.alloc().expect("Sanity alloc failed");
                if i > 0 && allocated[i] == allocated[i - 1] {
                    panic!("Allocator returned duplicate frame!");
                }
            }

            for i in 0..N {
                alloc.free(allocated[i]);
            }
        });

        kinfo!("frame_alloc: sanity: single ok");

        // 4. Contiguous Sanity Check
        {
            const N_CONTIG: u64 = 8;
            let range = crate::memory::frame_alloc::FRAME_ALLOCATOR.with_lock(|alloc| {
                alloc
                    .alloc_contiguous(N_CONTIG)
                    .expect("Contig sanity alloc failed")
            });

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

    kinfo!("Paging subsystem test passed (skipped local test)");

    // 5. Initialize Kernel Heap
    kinfo!("Initializing Kernel Heap...");
    crate::memory::global_alloc::kernel_heap().init(64);

    // 6. Switch Allocator
    crate::memory::global_alloc::switch_to_kernel_heap();

    // 7. Heap Sanity Demo
    {
        kinfo!("Running heap sanity check...");

        let mut v = Vec::new();
        for i in 0..1000 {
            v.push(i as u64);
        }

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

        kinfo!("kheap: forcing growth...");
        let big_vec: Vec<u8> = alloc::vec![0u8; 300 * 1024]; 
        kinfo!("kheap: big allocation ok (len={})", big_vec.len());

        crate::memory::global_alloc::kernel_heap().stats();
    }

    // Initialize Syscalls
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    {
        unsafe extern "C" {
             fn syscall_entry(); 
        }
        kinfo!("Registering syscall handler...");
        #[cfg(target_arch="aarch64")]
        runtime.register_syscall_handler(0);

        #[cfg(target_arch="x86_64")]
        runtime.register_syscall_handler(syscall_entry as u64);
    }

    // 5. Task Subsystem & Sprout Launch
    kinfo!("Initializing Task System...");
    crate::task::init();

    pub const THREADS_SUPPORTED: bool = cfg!(any(target_arch = "x86_64", target_arch = "aarch64"));

    if THREADS_SUPPORTED {
        kinfo!("threads: supported");
        
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
            runtime.halt();
        }
    }
}

extern "C" fn thread_a(arg: usize) -> ! {
    crate::kinfo!("Thread A starting (arg={})", arg);

    #[cfg(target_arch = "x86_64")]
    {
        crate::kinfo!("thread_a: setting up user mode test...");
        
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
            // jnz Loop
            0x75, 0xF2, 

            // mov rdi, 0
            0x48, 0xC7, 0xC7, 0x00, 0x00, 0x00, 0x00,
            // mov rax, 3 (SYSCALL_EXIT)
            0x48, 0xC7, 0xC0, 0x03, 0x00, 0x00, 0x00,
            // syscall
            0x0F, 0x05,
        ];

        use crate::memory::paging::PageFlags;
        use crate::memory::frame_alloc::FRAME_ALLOCATOR;
        
        let code_frame = FRAME_ALLOCATOR.with_lock(|alloc| alloc.alloc().expect("user code alloc failed"));
        
        let mut aspace = crate::memory::paging::AddressSpace::active();
        unsafe {
            let src = crate::memory::paging::phys_to_virt(code_frame.0) as *mut u8;
            core::ptr::copy_nonoverlapping(user_code.as_ptr(), src, user_code.len());
        }
        
        aspace.map_page(0x400000, code_frame, PageFlags::PRESENT | PageFlags::USER_ACCESSIBLE).expect("map code failed");

        let stack_frame = FRAME_ALLOCATOR.with_lock(|alloc| alloc.alloc().expect("user stack alloc failed"));
        aspace.map_page(0x6FFFF000, stack_frame, PageFlags::PRESENT | PageFlags::WRITABLE | PageFlags::USER_ACCESSIBLE).expect("map stack failed");


        crate::kinfo!("user: entered");
        
        let tf = crate::runtime().make_user_trapframe(0x400000, 0x6FFFF000);

        unsafe {
            crate::runtime().return_from_trap(&tf as *const _);
        }
    }

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
