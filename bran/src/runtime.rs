use kernel::{BootRuntime, PhysRange, PhysRangeKind, BootModuleDesc, BootModuleKind, FramebufferInfo, PixelFormat, IrqState};
use crate::requests::{MEMORY_MAP_REQUEST, HHDM_REQUEST, MODULE_REQUEST, FRAMEBUFFER_REQUEST};

// --- Shared Limine Data ---

static mut MEMORY_MAP_CACHE: [PhysRange; 128] = [PhysRange { start: 0, end: 0, kind: PhysRangeKind::Other }; 128];
static mut MEMORY_MAP_LEN: usize = 0;
static mut MEMORY_MAP_INIT: bool = false;

static mut MODULES_CACHE: [BootModuleDesc; 32] = [BootModuleDesc {
    name: "",
    bytes: &[],
    phys_start: 0,
    phys_end: 0,
    kind: BootModuleKind::Unknown,
}; 32];
static mut MODULES_LEN: usize = 0;
static mut MODULES_INIT: bool = false;

pub struct LimineRuntimeData;

impl LimineRuntimeData {
    pub const fn new() -> Self { Self }

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

    pub fn phys_memory_map(&self) -> &'static [PhysRange] {
        self.init_memory_map();
        unsafe {
            let ptr = core::ptr::addr_of!(MEMORY_MAP_CACHE);
            let len = *core::ptr::addr_of!(MEMORY_MAP_LEN);
            let slice_ptr = ptr as *const PhysRange;
            core::slice::from_raw_parts(slice_ptr, len)
        }
    }

    fn init_modules(&self) {
        unsafe {
             if *core::ptr::addr_of!(MODULES_INIT) { return; }

             let hhdm_offset = self.phys_to_virt_offset();

             if let Some(resp) = MODULE_REQUEST.get_response() {
                 let mut idx = 0;
                 let cache_ptr = core::ptr::addr_of_mut!(MODULES_CACHE);
                 let capacity = 32;

                 for m in resp.modules() {
                     if idx >= capacity { break; }

                     // Convert name
                     // Limine provides &CStr
                     let name = m.path().to_str().unwrap_or("unknown");

                     let vaddr = m.addr();
                     let size = m.size();
                     
                     // Convert to slice
                     let bytes = core::slice::from_raw_parts(vaddr as *const u8, size as usize);
                     
                     // Calculate physical address
                     // vaddr = paddr + hhdm_offset  =>  paddr = vaddr - hhdm_offset
                     let vaddr_u64 = vaddr as u64;
                     let paddr = if vaddr_u64 >= hhdm_offset {
                         vaddr_u64 - hhdm_offset
                     } else {
                         0 
                     };

                     (*cache_ptr)[idx] = BootModuleDesc {
                        name,
                        bytes,
                        phys_start: paddr,
                        phys_end: paddr + size,
                        kind: BootModuleKind::Unknown, 
                     };

                     idx += 1;
                 }
                 *core::ptr::addr_of_mut!(MODULES_LEN) = idx;
             }
             *core::ptr::addr_of_mut!(MODULES_INIT) = true;
        }
    }

    pub fn modules(&self) -> &'static [BootModuleDesc] {
        self.init_modules();
        unsafe {
            let ptr = core::ptr::addr_of!(MODULES_CACHE);
            let len = *core::ptr::addr_of!(MODULES_LEN);
            let slice_ptr = ptr as *const BootModuleDesc;
            core::slice::from_raw_parts(slice_ptr, len)
        }
    }

    pub fn phys_to_virt_offset(&self) -> u64 {
        HHDM_REQUEST.get_response()
            .map(|r| r.offset())
            .unwrap_or(0)
    }

    pub fn framebuffer(&self) -> Option<FramebufferInfo> {
        let resp = FRAMEBUFFER_REQUEST.get_response()?;
        let fb = resp.framebuffers().next()?;

        // Convert format
        let format = match (fb.red_mask_size(), fb.green_mask_size(), fb.blue_mask_size()) {
            (8, 8, 8) => PixelFormat::Xrgb8888, // Simplified assumption
            (5, 6, 5) => PixelFormat::Rgb565,
            _ => PixelFormat::Unknown,
        };

        Some(FramebufferInfo {
            addr: fb.addr() as u64,
            byte_len: fb.height() as usize * fb.pitch() as usize, // approximate safe bound
            width: fb.width() as u32,
            height: fb.height() as u32,
            pitch: fb.pitch() as u32,
            bpp: fb.bpp() as u16,
            format,
        })
    }
}

// --- Architecture Contract ---

pub trait ArchRuntime {
    fn putchar(&self, c: u8);
    fn halt(&self) -> !;
    fn mono_ticks(&self) -> u64;
    fn mono_freq_hz(&self) -> u64;
    fn irq_disable(&self) -> IrqState;
    fn irq_restore(&self, state: IrqState);

    // SIMD
    fn simd_init_cpu(&self) {}
    fn simd_state_layout(&self) -> (usize, usize) { (0, 1) }
    unsafe fn simd_save(&self, _dst: *mut u8) {}
    unsafe fn simd_restore(&self, _src: *const u8) {}

    // Barriers - defaults
    fn fence_full(&self) {}
    fn icache_invalidate(&self) {}

    // Paging
    fn page_size(&self) -> usize { 4096 }
    fn kernel_virt_base(&self) -> u64 { 0xFFFF_8000_0000_0000 }
    fn phys_to_virt_offset(&self) -> u64 { 0 }
    
    fn map_page(&self, _handle: usize, _virt: u64, _phys: u64, _flags: u64) -> Result<(), ()> { Err(()) }
    fn map_page_with_allocator(
        &self, 
        _handle: usize, 
        _virt: u64, 
        _phys: u64, 
        _flags: u64,
        _alloc: &mut kernel::memory::boot_frame_alloc::BootFrameAllocator
    ) -> Result<(), ()> { Err(()) }

    fn unmap_page(&self, _handle: usize, _virt: u64) {}
    fn translate(&self, _handle: usize, _virt: u64) -> Option<u64> { None }

    fn new_address_space(&self) -> usize { 0 }
    fn switch_address_space(&self, _handle: usize) {}
    fn current_address_space(&self) -> usize { 0 }
    
    fn tlb_flush_page(&self, _virt: u64) {}
    fn tlb_flush_all(&self) {}
    
    // Task Context
    fn context_init(
        &self, 
        _kstack_top: u64, 
        _entry: extern "C" fn(usize) -> !, 
        _arg: usize
    ) -> usize { 0 }

    unsafe fn context_switch(&self, _old_handle_ptr: *mut usize, _new_handle: usize) {}

    // Syscall / Context (Restored)
    fn register_syscall_handler(&self, _entry: u64) {}
    fn set_kernel_stack(&self, _stack_top: u64) {}

    unsafe fn enter_user_mode(&self, _context: &kernel::arch::TrapFrame) -> ! { loop {} }
}

// --- Generic Runtime ---

pub struct Runtime<A: ArchRuntime> {
    pub arch: A,
    pub limine: LimineRuntimeData,
}

impl<A: ArchRuntime> Runtime<A> {
    pub const fn new(arch: A) -> Self {
        Self {
            arch,
            limine: LimineRuntimeData::new(),
        }
    }
}

impl<A: ArchRuntime> BootRuntime for Runtime<A> {
    fn putchar(&self, c: u8) { self.arch.putchar(c) }
    fn halt(&self) -> ! { self.arch.halt() }

    fn mono_ticks(&self) -> u64 { self.arch.mono_ticks() }
    fn mono_freq_hz(&self) -> u64 { self.arch.mono_freq_hz() }

    fn irq_disable(&self) -> IrqState { self.arch.irq_disable() }
    fn irq_restore(&self, s: IrqState) { self.arch.irq_restore(s) }
    
    // SIMD
    fn simd_init_cpu(&self) { self.arch.simd_init_cpu() }
    fn simd_state_layout(&self) -> (usize, usize) { self.arch.simd_state_layout() }
    unsafe fn simd_save(&self, dst: *mut u8) { unsafe { self.arch.simd_save(dst) } }
    unsafe fn simd_restore(&self, src: *const u8) { unsafe { self.arch.simd_restore(src) } }

    // Barriers
    fn fence_full(&self) { self.arch.fence_full() }
    fn icache_invalidate(&self) { self.arch.icache_invalidate() }

    fn register_syscall_handler(&self, entry: u64) { self.arch.register_syscall_handler(entry) }
    fn set_kernel_stack(&self, stack_top: u64) { self.arch.set_kernel_stack(stack_top) }
    unsafe fn enter_user_mode(&self, context: &kernel::arch::TrapFrame) -> ! { unsafe { self.arch.enter_user_mode(context) } }

    fn phys_memory_map(&self) -> &'static [PhysRange] { self.limine.phys_memory_map() }
    fn phys_to_virt_offset(&self) -> u64 { self.limine.phys_to_virt_offset() } // Use Limine default if arch calls it
    fn modules(&self) -> &'static [BootModuleDesc] { self.limine.modules() }
    fn framebuffer(&self) -> Option<FramebufferInfo> { self.limine.framebuffer() }

    // Defaults
    fn cpu_count(&self) -> usize { 1 }
    fn boot_cpu_id(&self) -> usize { 0 }
    
    // Paging Delegates
    fn page_size(&self) -> usize { self.arch.page_size() }
    fn kernel_virt_base(&self) -> u64 { self.arch.kernel_virt_base() }
    
    // Note: limine.phys_to_virt_offset() uses HHDM request.
    // If arch overrides it (like x86_64 might want to?), we should trust arch?
    // But Limine is the source of truth for HHDM.
    // However, x86_64 might store it differently.
    // x86_64 `phys_to_virt_offset` impl called `paging::phys_to_virt_offset()`.
    // My previous `x86_64/runtime.rs` implemented it.
    // If I want `BootRuntime` to call `self.arch`, I need to expose it in `ArchRuntime`.
    // Currently `BootRuntime` impl here calling `self.limine.phys_to_virt_offset()`.
    // I should change it to `self.arch.phys_to_virt_offset()` AND provide default in `ArchRuntime` that calls limine? No, circular.
    // Best: `ArchRuntime` can provide it.
    // x86_64 provides it. Other arches use default?
    // I added `phys_to_virt_offset` to `ArchRuntime` above (start of replacement).
    // So here I should use `self.arch.phys_to_virt_offset()`.
    // Wait, the `x86_64` implementation I wrote in `paging.rs` returns `HHDM_OFFSET`.
    // `HHDM_OFFSET` is set by `paging::init`.
    // Who calls `paging::init`?
    // Originally `kernel`. Now `kernel` does NOT call it.
    // So `HHDM_OFFSET` in `bran/src/arch/x86_64/paging.rs` is 0 by default.
    // WE HAVE A PROBLEM.
    // `kernel` used to call `paging::init(offset)`.
    // Now usage is: `runtime().phys_to_virt_offset()`.
    // `LimineRuntimeData` gets it from `HHDM_REQUEST`.
    // So `bran` knows it.
    // `x86_64` paging implementation uses `HHDM_OFFSET` static.
    // Using `phys_to_virt` helper inside `paging.rs`.
    // So `paging.rs` needs `init` to be called with the offset.
    // `X86_64Runtime::new()` should verify/init it?
    // `X86_64Runtime::new()` is const.
    // But `HHDM_REQUEST` is available.
    // Maybe `X86_64Runtime` (or `bran/src/main.rs`) should call `paging::init` early?
    // `bran/src/main.rs` calls `kernel::start(&RUNTIME)`.
    // Before that, we should ensure `paging` is initialized.
    // But `Runtime` construction is const static. `MODULES` etc are lazy or static.
    // `phys_to_virt_offset` in `limine` data is also lazy (calls `get_response`).
    // So I can make `paging::init` part of `X86_64Runtime` laziness?
    // Or just make `x86_64::paging` use `HHDM_REQUEST` directly instead of a static `HHDM_OFFSET`.
    // Yes! `bran` has access to requests.
    // `bran/src/requests.rs` has `HHDM_REQUEST`.
    // So `bran/src/arch/x86_64/paging.rs` should use `crate::requests::HHDM_REQUEST.get_response()...` directly.
    // I need to update `paging.rs` to do that. And then I don't need `init` and don't need `HHDM_OFFSET` static.
    
    // So back to `bran/src/runtime.rs`:
    // `fn phys_to_virt_offset(&self) -> u64 { self.arch.phys_to_virt_offset() }`
    // `ArchRuntime` default: `0`.
    // `x86_64` impl: `paging::phys_to_virt_offset()`.
    // `paging::phys_to_virt_offset` currently reads static. I will update it.
    
    // Delegation:
    fn map_page(&self, h: usize, v: u64, p: u64, f: u64) -> Result<(), ()> { self.arch.map_page(h, v, p, f) }
    fn map_page_with_allocator(
        &self, 
        h: usize, 
        v: u64, 
        p: u64, 
        f: u64,
        a: &mut kernel::memory::boot_frame_alloc::BootFrameAllocator
    ) -> Result<(), ()> {
        self.arch.map_page_with_allocator(h, v, p, f, a)
    }
    fn unmap_page(&self, h: usize, v: u64) { self.arch.unmap_page(h, v) }
    fn translate(&self, h: usize, v: u64) -> Option<u64> { self.arch.translate(h, v) }
    fn new_address_space(&self) -> usize { self.arch.new_address_space() }
    fn switch_address_space(&self, h: usize) { self.arch.switch_address_space(h) }
    fn current_address_space(&self) -> usize { self.arch.current_address_space() }
    fn tlb_flush_page(&self, v: u64) { self.arch.tlb_flush_page(v) }
    fn tlb_flush_all(&self) { self.arch.tlb_flush_all() }
    
    fn context_init(&self, k: u64, e: extern "C" fn(usize)->!, a: usize) -> usize {
        self.arch.context_init(k, e, a)
    }
    unsafe fn context_switch(&self, o: *mut usize, n: usize) {
        unsafe { self.arch.context_switch(o, n) }
    }
}
