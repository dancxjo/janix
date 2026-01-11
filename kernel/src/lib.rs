#![no_std]

extern crate alloc;

pub mod logging;
pub mod memory;
pub mod time;
pub mod task;
pub mod simd;
pub mod syscall;
pub mod root;
pub mod tests;

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

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UserEntry {
    pub entry_pc: usize,
    pub user_sp: usize,
    pub arg0: usize,
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
    unsafe fn enter_user(&self, entry: UserEntry) -> !;
    
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
    fn mono_freq_hz(&self) -> u64 { 10_000_000 }
    fn read_rtc(&self) -> Option<abi::device::RtcTime> { None }
}

pub trait BootRuntime: BootRuntimeBase + Sized + 'static {
    type Tasking: BootTasking<Runtime = Self>;
    fn tasking(&self) -> &Self::Tasking;

    fn halt(&self) -> !;


    fn threads_supported(&self) -> bool { false }
    fn simd_init_cpu(&self) {}
    fn simd_state_layout(&self) -> (usize, usize) { (0, 1) }
    unsafe fn simd_save(&self, _dst: *mut u8) {}
    unsafe fn simd_restore(&self, _src: *const u8) {}
    
    /// Very early architecture initialization, called before any significant stack usage.
    /// Used for critical setup like switching stack modes on AArch64.
    /// Default implementation does nothing.
    unsafe fn early_init(&self) {}

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

    fn acpi_rsdp(&self) -> Option<u64> { None }
    fn dtb_ptr(&self) -> Option<u64> { None }
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

struct GlobalAllocHook;
impl FrameAllocatorHook for GlobalAllocHook {
    fn alloc_frame(&self) -> Option<u64> {
        crate::memory::alloc_frame()
    }
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

    // Store global boot info for syscalls
    crate::boot_info::set(crate::boot_info::BootSyscallInfo {
        memory_map: runtime.phys_memory_map(),
        modules: runtime.modules(),
        framebuffer: runtime.framebuffer(),
        hhdm_offset: runtime.phys_to_virt_offset(),
        acpi_rsdp: runtime.acpi_rsdp(),
        dtb_ptr: runtime.dtb_ptr(),
    });

    crate::root::init_root_service::<R>();
    // Root Boot Registration

    // Check for sprout module
    // Root Boot Registration
    let boot_info = crate::root::boot_register::BootInfo {
        cpu_count: runtime.cpu_count(),
        memory_map: runtime.phys_memory_map(),
        modules: runtime.modules(),
        framebuffer: runtime.framebuffer(),
        hhdm_offset: runtime.phys_to_virt_offset(),
        acpi_rsdp: runtime.acpi_rsdp(),
        dtb_ptr: runtime.dtb_ptr(),
        arch: if cfg!(target_arch = "x86_64") { "x86_64" } 
              else if cfg!(target_arch = "aarch64") { "aarch64" }
              else if cfg!(target_arch = "riscv64") { "riscv64" }
              else if cfg!(target_arch = "loongarch64") { "loongarch64" }
              else { "unknown" },
        platform_profile: "unknown", // todo: ask runtime
    };
    let inventory = crate::root::boot_register::register_all(&boot_info);
    crate::root::debug_dump::dump_all_to_console();
    crate::kinfo!("KERNEL: root census complete: host=t{:x} kernel=t{:x} root=t{:x}", inventory.host, inventory.kernel, inventory.root);
    let modules = runtime.modules();
    let sprout = modules.iter().find(|m| m.name.contains("sprout"));

    if let Some(mod_desc) = sprout {
        kinfo!("Found sprout module, loading...");
        
        let aspace = runtime.tasking().active_address_space();
        let hook = GlobalAllocHook;
        
        // Load Sprout
        let user_entry = crate::task::loader::load_module(runtime, aspace, mod_desc)
            .expect("Failed to load sprout");

        // Prepare Module Registry Page
        let reg_phys = crate::memory::alloc_frame().expect("OOM Registry");
        let reg_virt = reg_phys + runtime.phys_to_virt_offset();
        
        // Fill registry
        unsafe {
            let count = modules.len();
            let ptr = reg_virt as *mut usize;
            *ptr = count; // First word = count
            
            // Array of ModuleEntry { ptr: usize, len: usize } starts at offset 8 (64-bit)
            // Strings start after array. Max 16 modules typical, but let's calculate.
            // entry_size = 16 bytes.
            let array_start = ptr.add(1) as *mut usize;
            let mut string_offset_bytes = 8 + (count * 16);
            
            for (i, m) in modules.iter().enumerate() {
                let name_bytes = m.name.as_bytes();
                let name_len = name_bytes.len();
                
                // Safety check for page overflow
                if string_offset_bytes + name_len > 4096 {
                    kinfo!("Warning: Module registry page overflow, truncating list.");
                    *ptr = i; // Update count
                    break;
                }
                
                // Copy string
                let string_dst = (reg_virt as *mut u8).add(string_offset_bytes);
                core::ptr::copy_nonoverlapping(name_bytes.as_ptr(), string_dst, name_len);
                
                // Write Entry (ptr, len)
                let entry_slot = array_start.add(i * 2);
                *entry_slot = 0x600000 + string_offset_bytes; // User virtual address
                *entry_slot.add(1) = name_len;
                
                string_offset_bytes += name_len;
            }
        }
        
        // Map Registry to fixed user address 0x600000
        // We map it read-only for user
        runtime.tasking().map_page(aspace, 0x600000, reg_phys, 
             MapPerms { user: true, read: true, write: false, exec: false }, 
             MapKind::Normal, &hook).unwrap();
             
        // Flush TLB by reloading CR3
        runtime.tasking().activate_address_space(aspace);
             
        kinfo!("Spawning sprout with registry at 0x600000...");
        unsafe {
             crate::task::scheduler::spawn_user_thread::<R>(
                 user_entry.entry_pc, 
                 user_entry.user_sp, 
                 0x600000 // arg0 = registry ptr
             );
        }
    } else {
        kinfo!("Sprout not found. Checking fallback...");
        
        let threads_demo = modules.iter().find(|m| m.name.contains("threads_demo"));
        if let Some(mod_desc) = threads_demo {
             kinfo!("Found threads_demo fallback...");
             let aspace = runtime.tasking().active_address_space();
             let hook = GlobalAllocHook;
             let user_entry = crate::task::loader::load_module(runtime, aspace, mod_desc)
                .expect("Failed to load threads_demo");
             unsafe {
                 crate::task::scheduler::spawn_user_thread::<R>(
                     user_entry.entry_pc, 
                     user_entry.user_sp, 
                     0
                 );
             }
        } else {
            kinfo!("No modules found. Checking threads_supported...");
            if runtime.threads_supported() {
                kinfo!("Spawning Thread A...");
                crate::task::spawn::<R>(thread_a, 1);
                kinfo!("Spawning Thread B...");
                crate::task::spawn::<R>(thread_b, 2);
            }
        }
    }

    crate::tests::root_test::run_selftest();
    kinfo!("System initialized. Entering scheduler loop.");
    loop {
        crate::task::yield_now::<R>();
        core::hint::spin_loop();
    }
}

extern "C" fn thread_a(arg: usize) -> ! {
    let mut count: usize = 0;
    loop {
        // Only log the first few iterations to avoid flooding serial output
        if count < 5 {
            let ticks = runtime_base().mono_ticks();
            crate::kinfo!("Thread A (arg={}) ticks={}", arg, ticks);
        }
        count = count.wrapping_add(1);
        for _ in 0..1000000 { core::hint::black_box(()); }
        unsafe { crate::task::scheduler::yield_now_current(); }
    }
}

extern "C" fn thread_b(arg: usize) -> ! {
    let mut count: usize = 0;
    loop {
        // Only log the first few iterations to avoid flooding serial output
        if count < 5 {
            let ticks = runtime_base().mono_ticks();
            crate::kinfo!("Thread B (arg={}) ticks={}", arg, ticks);
        }
        count = count.wrapping_add(1);
        for _ in 0..1000000 { core::hint::black_box(()); }
        unsafe { crate::task::scheduler::yield_now_current(); }
    }
}
pub fn run_fairness_test<R: BootRuntime>() { tests::fairness::run::<R>(); }
pub mod boot_info;
