#![no_std]

extern crate alloc;

pub mod logging;
pub mod memory;
pub mod time;
pub mod task;
pub mod simd;

#[derive(Debug, Clone, Copy)]
pub struct PhysRange {
    pub start: u64,
    pub end: u64,
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
    Acpi,
    Other,
}

#[derive(Clone, Copy)]
pub struct BootModuleDesc {
    pub name: &'static str,
    pub bytes: &'static [u8],
    pub phys_start: u64,
    pub phys_end: u64,
    pub kind: BootModuleKind,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BootModuleKind {
    Unknown,
    Elf,
    Wasm,
    Data,
}

pub struct FramebufferInfo {
    pub addr: u64,
    pub byte_len: usize,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bpp: u16,
    pub format: PixelFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Xrgb8888,
    Argb8888,
    Rgb565,
    Unknown,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct IrqState(pub usize);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct MapPerms {
    pub user: bool,
    pub read: bool,
    pub write: bool,
    pub exec: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MapKind {
    Normal,
    Device,
    Framebuffer,
}

pub struct UserTaskSpec<AS> {
    pub entry: u64,
    pub stack_top: u64,
    pub aspace: AS,
    pub arg: usize,
}

pub trait FrameAllocatorHook {
    fn alloc_frame(&self) -> Option<u64>;
}

pub trait BootTasking {
    type Runtime: BootRuntime<Tasking = Self>;
    type Context: Copy + Default;
    type AddressSpace: Copy + Default;

    fn init(&self, hhdm_offset: u64);
    fn init_kernel_context(&self, entry: extern "C" fn(usize) -> !, stack_top: u64, arg: usize) -> Self::Context;
    fn init_user_context(&self, spec: UserTaskSpec<Self::AddressSpace>, kstack_top: u64) -> Self::Context;
    
    unsafe fn switch(&self, from: &mut Self::Context, to: &Self::Context);
    
    fn make_user_address_space(&self) -> Self::AddressSpace;
    fn active_address_space(&self) -> Self::AddressSpace;
    fn activate_address_space(&self, aspace: Self::AddressSpace);
    
    fn map_page(
        &self, 
        aspace: Self::AddressSpace, 
        virt: u64, 
        phys: u64, 
        perms: MapPerms, 
        kind: MapKind, 
        allocator: &dyn FrameAllocatorHook
    ) -> Result<(), ()>;
    
    fn unmap_page(&self, aspace: Self::AddressSpace, virt: u64) -> Result<Option<u64>, ()>;
    fn translate(&self, aspace: Self::AddressSpace, virt: u64) -> Option<u64>;
    fn tlb_flush_page(&self, virt: u64);
}

pub trait BootRuntimeBase: 'static {
    fn putchar(&self, c: u8);
    fn mono_ticks(&self) -> u64;
}

pub trait BootRuntime: BootRuntimeBase + Sized + 'static {
    type Tasking: BootTasking<Runtime = Self>;
    fn tasking(&self) -> &Self::Tasking;

    fn halt(&self) -> !;

    fn mono_freq_hz(&self) -> u64 { 10_000_000 }

    fn threads_supported(&self) -> bool { false }
    fn simd_init_cpu(&self) {}
    fn simd_state_layout(&self) -> (usize, usize) { (0, 1) }
    unsafe fn simd_save(&self, _dst: *mut u8) {}
    unsafe fn simd_restore(&self, _src: *const u8) {}

    fn fence_full(&self) {}
    fn icache_invalidate(&self) {}

    fn phys_memory_map(&self) -> &'static [PhysRange];
    fn phys_to_virt_offset(&self) -> u64;
    fn modules(&self) -> &'static [BootModuleDesc];
    fn framebuffer(&self) -> Option<FramebufferInfo>;

    fn page_size(&self) -> usize { 4096 }
    fn kernel_virt_base(&self) -> u64 { 0xffffffff80000000 }
    fn cpu_count(&self) -> usize { 1 }
    fn boot_cpu_id(&self) -> usize { 0 }
    
    fn irq_disable(&self) -> IrqState;
    fn irq_restore(&self, state: IrqState);
}

static mut RUNTIME: Option<&'static dyn core::any::Any> = None;
static mut RUNTIME_BASE: Option<&'static dyn BootRuntimeBase> = None;

pub unsafe fn init_runtime<R: BootRuntime>(runtime: &'static R) {
    unsafe { RUNTIME = Some(runtime) };
    unsafe { RUNTIME_BASE = Some(runtime as &'static dyn BootRuntimeBase) };
}

pub fn runtime<R: BootRuntime>() -> &'static R {
    unsafe { 
        RUNTIME.expect("Runtime not initialized")
            .downcast_ref::<R>()
            .expect("Runtime type mismatch") 
    }
}

pub fn runtime_base() -> &'static dyn BootRuntimeBase {
    unsafe { RUNTIME_BASE.expect("Runtime not initialized") }
}

pub fn start<R: BootRuntime>(runtime: &'static R) -> ! {
    unsafe { init_runtime(runtime) };
    unsafe { crate::logging::init(runtime) };
    
    kinfo!("thing-os kernel v0.1.0 starting...");
    kinfo!("Intent-Mechanism paging split active");
    kinfo!("System booted");

    memory::init(runtime);
    kinfo!("Initializing global allocator...");
    memory::global_alloc::init(runtime);

    kinfo!("Initializing tasking...");
    crate::task::init::<R>();

    kinfo!("Checking threads_supported...");
    if runtime.threads_supported() {
        kinfo!("Spawning Thread A...");
        crate::task::spawn::<R>(thread_a, 1);
        kinfo!("Spawning Thread B...");
        crate::task::spawn::<R>(thread_b, 2);
    }

    kinfo!("System initialized. Entering scheduler loop.");
    loop {
        crate::task::yield_now::<R>();
        core::hint::spin_loop();
    }
}

extern "C" fn thread_a(arg: usize) -> ! {
    loop {
        let ticks = runtime_base().mono_ticks();
        crate::kinfo!("Thread A (arg={}) ticks={}", arg, ticks);
        for _ in 0..1000000 { core::hint::black_box(()); }
        unsafe { crate::task::scheduler::yield_now_current(); }
    }
}

extern "C" fn thread_b(arg: usize) -> ! {
    loop {
        let ticks = runtime_base().mono_ticks();
        crate::kinfo!("Thread B (arg={}) ticks={}", arg, ticks);
        for _ in 0..1000000 { core::hint::black_box(()); }
        unsafe { crate::task::scheduler::yield_now_current(); }
    }
}
