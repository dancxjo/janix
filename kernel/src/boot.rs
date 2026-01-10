

extern crate alloc;

use alloc::boxed::Box;
use core::any::Any;
use core::fmt::Debug;

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

// Architecture Context Traits

pub trait ArchContext: Any + Debug + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait ArchTrapFrame: Any + Debug + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    
    // Syscall accessors
    fn syscall_arg(&self, idx: usize) -> u64;
    fn syscall_ret(&mut self, val: u64);
    fn syscall_num(&self) -> u64;
    
    // User state accessors
    fn set_user_stack(&mut self, stack: u64);
    fn user_stack(&self) -> u64;
    fn set_user_ip(&mut self, ip: u64);
    fn user_ip(&self) -> u64;
    
    // Exception info
    fn trap_num(&self) -> usize { 0 }
    fn error_code(&self) -> usize { 0 }
}

pub trait BootRuntime: Sync + Send {
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
    unsafe fn simd_save(&self, _dst: *mut u8) {}

    /// Restore CPU SIMD state from `src`.
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
    fn map_page(&self, _handle: usize, _virt: u64, _phys: u64, _flags: u64) -> Result<(), ()> { Err(()) }
    fn map_page_with_allocator(
        &self, 
        _handle: usize, 
        _virt: u64, 
        _phys: u64, 
        _flags: u64,
        _allocator: &mut crate::memory::boot_frame_alloc::BootFrameAllocator
    ) -> Result<(), ()> { Err(()) }    

    fn unmap_page(&self, _handle: usize, _virt: u64) {}
    fn translate(&self, _handle: usize, _virt: u64) -> Option<u64> { None }
    
    fn new_address_space(&self) -> usize { 0 }
    fn switch_address_space(&self, _handle: usize) {}
    fn current_address_space(&self) -> usize { 0 }

    fn tlb_flush_page(&self, _virt: u64) {}
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

    // Task Context Factory
    fn new_context(&self) -> Box<dyn ArchContext>;
    fn new_trapframe(&self) -> Box<dyn ArchTrapFrame>;

    // Initialize a context for a new thread.
    fn init_task_context(
        &self, 
        _out: &mut dyn ArchContext,
        _kstack_top: u64, 
        _entry: extern "C" fn(usize) -> !, 
        _arg: usize
    ) -> usize { 0 }

    // Trap <-> Context bridge
    fn save_from_trap(&self, _tf: &dyn ArchTrapFrame, _out: &mut dyn ArchContext) {}
    fn load_into_trap(&self, _ctx: &dyn ArchContext, _tf: &mut dyn ArchTrapFrame) {}

    // Cooperative Context Switch
    unsafe fn switch_tasks(&self, _old: &mut dyn ArchContext, _new: &dyn ArchContext) {
        panic!("switch_tasks not implemented");
    }

    // Trap return (diverges)
    unsafe fn return_from_trap(&self, _tf: &dyn ArchTrapFrame) -> ! {
        loop {}
    }

    // User entry
    fn make_user_trapframe(&self, _rip: u64, _rsp: u64) -> Box<dyn ArchTrapFrame> {
         panic!("make_user_trapframe not implemented")
    }
}
