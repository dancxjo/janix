#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod device_registry;
pub mod entropy;
pub mod ipc;
pub mod irq;
pub mod logging;
pub mod memory;
pub mod net;
pub mod once_cell;
pub mod petals_session;
pub mod root;
pub mod sched;
pub mod simd;
pub mod syscall;
pub mod task;

pub mod time;
pub mod trace;
pub mod virtio;

use crate::task::StartupArg;
use abi::errors::Errno;
use abi::vm::{VmBackingKind, VmMapFlags, VmProt, VmRegionInfo};

#[unsafe(no_mangle)]
pub extern "C" fn kernel_handle_page_fault(rip: u64, addr: u64, err: u64) {
    // Decode x86_64 page fault error code bits
    let present = (err & 0x1) != 0;
    let write = (err & 0x2) != 0;
    let user = (err & 0x4) != 0;
    let instr_fetch = (err & 0x10) != 0;

    let stack_result = if user {
        unsafe { crate::sched::handle_user_stack_fault_current(addr) }
    } else {
        crate::sched::StackFaultResult::NotStack
    };
    if stack_result == crate::sched::StackFaultResult::Grew {
        return;
    }

    // Structured page fault logging with decoded error bits
    crate::log_event!(
        crate::logging::LogLevel::Error,
        "kernel::trap",
        "user_page_fault va=0x{:016x} rip=0x{:016x} err=0x{:04x} present={} user={} write={} instr_fetch={}",
        addr,
        rip,
        err,
        present as u8,
        user as u8,
        write as u8,
        instr_fetch as u8
    );

    if stack_result == crate::sched::StackFaultResult::Overflow {
        crate::kprintln!("STACK: overflow at va=0x{:x}", addr);
    }

    unsafe {
        crate::sched::exit_current(-1);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_handle_exception(rip: u64, error_code: u64, rsp: u64, cs: u64, kind: u64) {
    let name = match kind {
        0 => "user_divide_by_zero",
        6 => "user_invalid_opcode",
        13 => "user_gpf",
        _ => "user_exception",
    };

    crate::log_event!(
        crate::logging::LogLevel::Error,
        "kernel::trap",
        "{} rip=0x{:016x} err=0x{:04x} rsp=0x{:016x} cs=0x{:x} kind={}",
        name,
        rip,
        error_code,
        rsp,
        cs,
        kind
    );

    unsafe {
        crate::sched::exit_current(-1);
    }
}

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
    pub cmdline: &'static str,
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

/// Pixel format for framebuffer surfaces.
///
/// Values match `abi::schema::pixel_format` constants for wire compatibility.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    /// Unknown or unsupported format.
    Unknown = 0,
    /// 32-bit BGRA: Memory [B, G, R, A] -> u32 0xAARRGGBB.
    Bgra8888 = 1,
    /// 32-bit BGRX: Memory [B, G, R, X] -> u32 0xXXRRGGBB (alpha ignored).
    Bgrx8888 = 2,
    /// 16-bit RGB565.
    Rgb565 = 3,
}

impl PixelFormat {
    /// Convert to wire-compatible u64 for graph properties.
    #[inline]
    pub const fn to_wire(self) -> u64 {
        self as u64
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct IrqState(pub usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CpuId(pub u32);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct MapPerms {
    pub user: bool,
    pub read: bool,
    pub write: bool,
    pub exec: bool,
    pub kind: MapKind,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum MapKind {
    #[default]
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
    fn init_kernel_context(
        &self,
        entry: extern "C" fn(usize) -> !,
        stack_top: u64,
        arg: usize,
    ) -> Self::Context;
    fn init_user_context(
        &self,
        spec: UserTaskSpec<Self::AddressSpace>,
        kstack_top: u64,
    ) -> Self::Context;

    unsafe fn switch(&self, from: &mut Self::Context, to: &Self::Context, to_tid: u64);
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
        allocator: &dyn FrameAllocatorHook,
    ) -> Result<(), ()>;

    fn unmap_page(&self, aspace: Self::AddressSpace, virt: u64) -> Result<Option<u64>, ()>;
    fn translate(&self, aspace: Self::AddressSpace, virt: u64) -> Option<u64>;
    fn tlb_flush_page(&self, virt: u64);
}

pub trait BootRuntimeBase: 'static {
    fn putchar(&self, c: u8);
    /// Non-blocking serial read. Returns `Some(byte)` if data is available.
    fn getchar(&self) -> Option<u8> {
        None
    }
    fn mono_ticks(&self) -> u64;
    fn mono_freq_hz(&self) -> u64 {
        10_000_000
    }

    fn pci_cfg_read32(&self, _bus: u8, _dev: u8, _func: u8, _offset: u8) -> Result<u32, Errno> {
        Err(Errno::NotSupported)
    }
    fn pci_cfg_write32(
        &self,
        _bus: u8,
        _dev: u8,
        _func: u8,
        _offset: u8,
        _value: u32,
    ) -> Result<(), Errno> {
        Err(Errno::NotSupported)
    }

    fn lapic_id(&self) -> Result<u32, Errno> {
        Err(Errno::NotSupported)
    }
    fn lapic_base_phys(&self) -> Result<u64, Errno> {
        Err(Errno::NotSupported)
    }

    fn simd_init_cpu(&self) {}

    /// Wait for interrupt - low-power idle until next IRQ
    fn wait_for_interrupt(&self) {}

    /// Reboot the system. This should never return.
    fn reboot(&self) -> ! {
        loop {
            core::hint::spin_loop();
        }
    }

    /// Send an Inter-Processor Interrupt (IPI) to a specific CPU.
    fn send_ipi(&self, _cpu_index: usize, _vector: u8) {}

    fn current_cpu_id(&self) -> CpuId {
        CpuId(0)
    }

    fn current_cpu_index(&self) -> usize {
        0
    }

    fn current_tid(&self) -> u64 {
        0
    }

    fn set_current_tid(&self, _tid: u64) {}

    /// Broadcast a TLB shootdown IPI to all other CPUs.
    fn tlb_shootdown_broadcast(&self) {}

    /// Per-CPU initialization for secondary CPUs.
    /// Initialize a secondary CPU after it has entered the kernel.
    fn init_secondary_cpu(&self, cpu_index: usize);

    /// Total CPUs discovered on this platform.
    fn cpu_total_count(&self) -> usize {
        1
    }

    /// Returns the next offline CPU id.
    fn next_offline_cpu(&self) -> Option<CpuId> {
        None
    }

    /// Request that one CPU be started.
    unsafe fn start_cpu(
        &self,
        _cpu: CpuId,
        _entry: extern "C" fn(usize) -> !,
        _arg: usize,
    ) -> Result<(), Errno> {
        Err(Errno::NotSupported)
    }

    /// Fill buffer with hardware entropy bytes.
    /// Returns the number of bytes actually filled (0 = no HW RNG available).
    fn fill_entropy(&self, _dst: &mut [u8]) -> usize {
        0
    }
}

pub trait BootRuntime: BootRuntimeBase + Sized + 'static {
    type Tasking: BootTasking<Runtime = Self>;
    fn tasking(&self) -> &Self::Tasking;

    fn halt(&self) -> !;

    fn threads_supported(&self) -> bool {
        false
    }
    // simd_init_cpu moved to BootRuntimeBase
    fn simd_state_layout(&self) -> (usize, usize) {
        (0, 1)
    }
    unsafe fn simd_save(&self, _dst: *mut u8) {}
    unsafe fn simd_restore(&self, _src: *const u8) {}

    /// Very early architecture initialization, called before any significant stack usage.
    /// Used for critical setup like switching stack modes on AArch64.
    /// Default implementation does nothing.
    unsafe fn early_init(&self) {}

    fn fence_full(&self) {}
    fn icache_invalidate(&self) {}

    // wait_for_interrupt moved to BootRuntimeBase

    fn phys_memory_map(&self) -> &'static [PhysRange];
    fn phys_to_virt_offset(&self) -> u64;
    fn modules(&self) -> &'static [BootModuleDesc];
    fn framebuffer(&self) -> Option<FramebufferInfo>;

    fn page_size(&self) -> usize {
        4096
    }
    fn kernel_virt_base(&self) -> u64 {
        0xffffffff80000000
    }
    fn boot_cpu_id(&self) -> usize {
        0
    }
    fn cpu_ids(&self) -> &'static [CpuId] {
        const ONE: [CpuId; 1] = [CpuId(0)];
        &ONE
    }

    /// Start all non-boot CPUs and run `entry` on each of them.
    /// Deprecated: use start_cpu for lazy bring-up.
    fn start_secondary_cpus(&self, _entry: extern "C" fn(usize) -> !) -> Result<(), Errno> {
        Err(Errno::NotSupported)
    }

    fn irq_disable(&self) -> IrqState;
    fn irq_restore(&self, state: IrqState);

    /// Setup periodic preemption timer (e.g. 100Hz heartbeat)
    fn setup_preemption_timer(&self, _hz: u32) {}

    fn acpi_rsdp(&self) -> Option<u64> {
        None
    }
    fn dtb_ptr(&self) -> Option<u64> {
        None
    }

    // IO Port primitives (x86-only, stubs for other archs)
    fn ioport_read_u8(&self, _port: u16) -> u8 {
        0
    }
    fn ioport_read_u16(&self, _port: u16) -> u16 {
        0
    }
    fn ioport_read_u32(&self, _port: u16) -> u32 {
        0
    }
    fn ioport_write_u8(&self, _port: u16, _value: u8) {}
    fn ioport_write_u16(&self, _port: u16, _value: u16) {}
    fn ioport_write_u32(&self, _port: u16, _value: u32) {}

    fn debug_active_aspace_root(&self) -> u64 {
        0
    }

    /// Map a physical range into a temporary virtual address for boot-time copies.
    /// This is used for reading firmware tables that might not be in the HHDM.
    /// Returns the virtual address of the start of the range.
    fn map_phys_temp(&self, _phys: u64, _size: usize) -> Result<u64, Errno> {
        Err(Errno::NotSupported)
    }

    /// Unmap a previously mapped temporary physical range.
    fn unmap_phys_temp(&self, _virt: u64, _size: usize) {}
}

// Per-CPU generic tracking
// In a full implementation, this should be a per-cpu structure or array.
// For now, we only trust this for the boot CPU or rely on atomic updates.
static CPU_ONLINE: once_cell::OnceCell<&'static core::sync::atomic::AtomicUsize> =
    once_cell::OnceCell::new();

static RUNTIME: once_cell::OnceCell<&'static dyn core::any::Any> = once_cell::OnceCell::new();
static RUNTIME_BASE: once_cell::OnceCell<&'static dyn BootRuntimeBase> = once_cell::OnceCell::new();
static mut RAW_RUNTIME_BASE: Option<&'static dyn BootRuntimeBase> = None;

/// Initialize the runtime. Panics if called more than once.
pub fn init_runtime<R: BootRuntime>(runtime: &'static R) {
    let name = core::any::type_name::<R>();
    crate::contract!("INIT_RUNTIME: type={}", name);
    RUNTIME.set(runtime as &'static dyn core::any::Any);
    RUNTIME_BASE.set(runtime as &'static dyn BootRuntimeBase);
    unsafe {
        RAW_RUNTIME_BASE = Some(runtime as &'static dyn BootRuntimeBase);
    }
}

pub fn runtime<R: BootRuntime>() -> &'static R {
    let any_ref: &'static dyn core::any::Any = *RUNTIME.get();
    if let Some(rt) = any_ref.downcast_ref::<R>() {
        rt
    } else {
        panic!(
            "Runtime type mismatch: expected {}",
            core::any::type_name::<R>()
        );
    }
}

pub fn runtime_base() -> &'static dyn BootRuntimeBase {
    *RUNTIME_BASE.get()
}

// Global IO port accessor functions
// On x86, these use inline asm. On other archs, they are no-ops.
#[inline]
pub fn ioport_read_u8(_port: u16) -> u8 {
    #[cfg(target_arch = "x86_64")]
    {
        let val: u8;
        unsafe {
            core::arch::asm!("in al, dx", out("al") val, in("dx") _port, options(nostack, preserves_flags))
        };
        val
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        0
    }
}

#[inline]
pub fn ioport_read_u16(_port: u16) -> u16 {
    #[cfg(target_arch = "x86_64")]
    {
        let val: u16;
        unsafe {
            core::arch::asm!("in ax, dx", out("ax") val, in("dx") _port, options(nostack, preserves_flags))
        };
        val
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        0
    }
}

#[inline]
pub fn ioport_read_u32(_port: u16) -> u32 {
    #[cfg(target_arch = "x86_64")]
    {
        let val: u32;
        unsafe {
            core::arch::asm!("in eax, dx", out("eax") val, in("dx") _port, options(nostack, preserves_flags))
        };
        val
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        0
    }
}

#[inline]
pub fn ioport_write_u8(_port: u16, _val: u8) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("out dx, al", in("dx") _port, in("al") _val, options(nostack, preserves_flags))
    };
}

#[inline]
pub fn ioport_write_u16(_port: u16, _val: u16) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("out dx, ax", in("dx") _port, in("ax") _val, options(nostack, preserves_flags))
    };
}

#[inline]
pub fn ioport_write_u32(_port: u16, _val: u32) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("out dx, eax", in("dx") _port, in("eax") _val, options(nostack, preserves_flags))
    };
}

struct GlobalAllocHook;
impl FrameAllocatorHook for GlobalAllocHook {
    fn alloc_frame(&self) -> Option<u64> {
        crate::memory::alloc_frame()
    }
}

pub fn start<R: BootRuntime>(runtime: &'static R) -> ! {
    init_runtime(runtime);
    unsafe { crate::logging::init(runtime) };

    if let Some(fb) = runtime.framebuffer() {
        crate::kinfo!(
            "BOOTFB: width={} height={} pitch={} bpp={} format={:?}",
            fb.width,
            fb.height,
            fb.pitch,
            fb.bpp,
            fb.format
        );
    }

    contract!("thing-os kernel starting...");

    memory::init(runtime);
    contract!("Initializing global allocator...");
    memory::global_alloc::init(runtime);

    contract!("Seeding entropy pool...");
    crate::entropy::seed_from_hardware();

    contract!("Initializing SIMD...");
    runtime.simd_init_cpu();

    contract!("Initializing tasking...");
    crate::task::init::<R>();

    // CRITICAL: Calibrate the BSP preemption timer BEFORE starting secondary CPUs.
    // Secondary CPUs read timer_vector/timer_init_cnt in init_secondary_cpu().
    // If these aren't set yet, secondary CPUs get no LAPIC timer, meaning
    // wake_sleepers() (called only from on_tick → PreemptTick) never fires
    // on those CPUs, and any task that calls sleep_ms() is stuck forever.
    kinfo!("System initialized. Setting up preemption timer (100Hz)...");
    runtime.setup_preemption_timer(100);

    // Bring up all secondary CPUs during early boot.
    let cpu_total = runtime.cpu_total_count();
    if cpu_total > 1 {
        crate::kinfo!(
            "Kernel: Detected {} CPUs. Starting {} secondaries...",
            cpu_total,
            cpu_total - 1
        );
        match runtime.start_secondary_cpus(kernel_secondary_entry::<R>) {
            Ok(()) => crate::kinfo!("Kernel: Secondary CPU bring-up complete."),
            Err(err) => crate::kerror!("Kernel: Secondary CPU bring-up failed: {:?}", err),
        }
    } else {
        crate::kinfo!("Kernel: Detected {} CPU.", cpu_total);
    }

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

    let boot_info = crate::root::boot_register::BootInfo {
        cpu_count: runtime.cpu_total_count(),
        memory_map: runtime.phys_memory_map(),
        modules: runtime.modules(),
        framebuffer: runtime.framebuffer(),
        hhdm_offset: runtime.phys_to_virt_offset(),
        acpi_rsdp: runtime.acpi_rsdp(),
        dtb_ptr: runtime.dtb_ptr(),
        arch: if cfg!(target_arch = "x86_64") {
            "x86_64"
        } else if cfg!(target_arch = "aarch64") {
            "aarch64"
        } else if cfg!(target_arch = "riscv64") {
            "riscv64"
        } else if cfg!(target_arch = "loongarch64") {
            "loongarch64"
        } else {
            "unknown"
        },
        platform_profile: "unknown", // todo: ask runtime
    };
    let inventory = crate::root::boot_register::register_all(runtime, &boot_info);
    #[cfg(feature = "diagnostic-apps")]
    crate::root::debug_dump::dump_all_to_console();
    contract!(
        "KERNEL: root census complete: host=t{:x} kernel=t{:x} root=t{:x}",
        inventory.host,
        inventory.kernel,
        inventory.root
    );
    let modules = runtime.modules();
    contract!("Kernel: Enumerating {} boot modules...", modules.len());
    for (i, m) in modules.iter().enumerate() {
        contract!(
            "  [{}] name='{}' cmdline='{}' size={}",
            i,
            m.name,
            m.cmdline,
            m.bytes.len()
        );
    }

    // Look for module with "init" in cmdline, otherwise fallback to "sprout" by name
    let init_module = modules
        .iter()
        .find(|m| m.cmdline.contains("init"))
        .or_else(|| modules.iter().find(|m| m.name.contains("sprout")));

    if let Some(mod_desc) = init_module {
        kinfo!(
            "Found init module: {} (cmdline: '{}'), loading...",
            mod_desc.name,
            mod_desc.cmdline
        );

        let aspace = runtime.tasking().make_user_address_space();
        let _hook = GlobalAllocHook;

        // Load Sprout
        let (user_entry, stack_info, mut regions) =
            crate::task::loader::load_module(runtime, aspace, mod_desc)
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
        runtime
            .tasking()
            .map_page(
                aspace,
                0x600000,
                reg_phys,
                MapPerms {
                    user: true,
                    read: true,
                    write: false,
                    exec: false,
                    kind: MapKind::Normal,
                },
                MapKind::Normal,
                &GlobalAllocHook,
            )
            .unwrap();

        regions.push(VmRegionInfo {
            start: 0x600000,
            end: 0x601000,
            prot: VmProt::USER | VmProt::READ,
            flags: VmMapFlags::empty(),
            backing_kind: VmBackingKind::Unknown,
            _reserved: [0; 7],
        });

        // Flush TLB by reloading CR3
        runtime.tasking().activate_address_space(aspace);

        kinfo!("Spawning sprout with registry at 0x600000...");
        unsafe {
            contract!("Spawning init process...");
            let mut entry = user_entry;
            entry.arg0 = StartupArg::BootRegistry.to_raw(); // arg0 = registry ptr
                                                            // Spawn at Normal priority - all tasks share the same priority for fair scheduling
            crate::sched::spawn_user_task_full::<R>(
                entry,
                aspace,
                stack_info,
                regions,
                crate::task::TaskPriority::Normal,
            );
        }
    } else {
        kinfo!("Sprout not found. Checking fallback...");

        let spawned_fallback = false;
        #[cfg(feature = "diagnostic-apps")]
        {
            if let Some(mod_desc) = modules.iter().find(|m| m.name.contains("threads_demo")) {
                kinfo!("Found threads_demo fallback...");
                let aspace = runtime.tasking().make_user_address_space();
                let (user_entry, stack_info, regions) =
                    crate::task::loader::load_module(runtime, aspace, mod_desc)
                        .expect("Failed to load threads_demo");
                unsafe {
                    crate::sched::spawn_user_task_full::<R>(
                        user_entry,
                        aspace,
                        stack_info,
                        regions,
                        crate::task::TaskPriority::Normal,
                    );
                }
                spawned_fallback = true;
            }
        }

        if !spawned_fallback {
            kinfo!("No modules found. Checking threads_supported...");
            if runtime.threads_supported() {
                kinfo!("Spawning initial threads...");
                kinfo!("Spawning Thread A...");
                crate::task::spawn::<R>(
                    thread_a,
                    StartupArg::Raw(1),
                    crate::task::TaskPriority::Normal,
                    crate::task::Affinity::Any,
                );
                kinfo!("Spawning Thread B...");
                crate::task::spawn::<R>(
                    thread_b,
                    StartupArg::Raw(2),
                    crate::task::TaskPriority::Normal,
                    crate::task::Affinity::Any,
                );
            }
        }
    }

    contract!("Entering scheduler loop.");
    crate::petals_session::init();
    loop {
        crate::petals_session::poll();
        crate::task::yield_now::<R>();
        // runtime.wait_for_interrupt(); // TODO: Only call when runqueue is empty
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
        for _ in 0..1000000 {
            core::hint::black_box(());
        }
        unsafe {
            crate::sched::yield_now_current();
        }
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
        for _ in 0..1000000 {
            core::hint::black_box(());
        }
        unsafe {
            crate::sched::yield_now_current();
        }
    }
}

pub mod boot_info;

extern "C" fn kernel_secondary_entry<R: BootRuntime>(cpu_index: usize) -> ! {
    // CRITICAL: First, load the kernel's GDT/IDT and set GS_BASE on this secondary CPU
    // This must happen before ANY kernel code that might fault or use logging (which uses GS).
    let base = unsafe { RAW_RUNTIME_BASE.expect("RAW_RUNTIME_BASE not initialized") };
    base.init_secondary_cpu(cpu_index);
    // Verification done via base properties later if needed

    crate::kinfo!("SMP: Entering kernel_secondary_entry for CPU {}", cpu_index);

    // Per-CPU init
    base.mono_ticks(); // ok for logging
                       // IMPORTANT: per-CPU SIMD init
    base.simd_init_cpu();

    // Then:
    unsafe {
        crate::sched::cpu_online::<R>(cpu_index);
        crate::sched::enter_secondary(cpu_index);
    }
}
