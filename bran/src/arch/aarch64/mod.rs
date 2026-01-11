use core::arch::asm;
use kernel::{IrqState, UserTaskSpec, UserEntry, FrameAllocatorHook, MapPerms, MapKind};
use kernel::time::MonotonicClamp;
use crate::runtime::ArchRuntime;

pub mod simd;
pub mod task;
pub mod trap;
pub mod paging;
pub mod syscall;
pub mod vector;

pub struct AArch64Runtime {
    serial: SerialPort,
}

impl AArch64Runtime {
    pub const fn new() -> Self {
        Self {
            serial: SerialPort::new(),
        }
    }
}

pub use task::AArch64Context;
pub use paging::AArch64AddressSpace;

impl ArchRuntime for AArch64Runtime {
    type Context = AArch64Context;
    type AddressSpace = AArch64AddressSpace;

    fn init(&self, hhdm_offset: u64) { 
        paging::init(hhdm_offset); 
        unsafe { vector::init(); }
    }
    fn putchar(&self, c: u8) {
        self.serial.putchar(c);
    }

    fn halt(&self) -> ! {
        hcf()
    }

    fn mono_ticks(&self) -> u64 {
        self.serial.clamp.clamp(read_cntvct_el0())
    }
    
    fn mono_freq_hz(&self) -> u64 {
        read_cntfrq_el0()
    }

    fn irq_disable(&self) -> IrqState {
        let daif: u64;
        unsafe {
            asm!("mrs {}, daif", out(reg) daif, options(nomem, nostack));
            asm!("msr daifset, #2", options(nomem, nostack));
        }
        IrqState((daif >> 7) as usize & 1)
    }

    fn irq_restore(&self, state: IrqState) {
        if state.0 == 0 {
             unsafe { asm!("msr daifclr, #2", options(nomem, nostack)); }
        } else {
             unsafe { asm!("msr daifset, #2", options(nomem, nostack)); }
        }
    }
    
    fn simd_init_cpu(&self) { simd::init_cpu(); }
    fn simd_state_layout(&self) -> (usize, usize) { simd::STATE_LAYOUT }
    unsafe fn simd_save(&self, dst: *mut u8) { unsafe { simd::save(dst) } }
    unsafe fn simd_restore(&self, src: *const u8) { unsafe { simd::restore(src) } }
    
    fn fence_full(&self) {
         unsafe { asm!("dmb sy", options(nostack, preserves_flags)); }
    }
    
    fn icache_invalidate(&self) {
         unsafe { 
             asm!("ic ialluis", options(nostack, preserves_flags));
             asm!("dsb ish", options(nostack, preserves_flags));
             asm!("isb", options(nostack, preserves_flags));
         }
    }

    fn threads_supported(&self) -> bool { true }

    // Tasking
    fn init_kernel_context(&self, entry: extern "C" fn(usize) -> !, stack_top: u64, arg: usize) -> Self::Context {
        task::init_kernel_context(entry, stack_top, arg)
    }

    fn init_user_context(&self, spec: UserTaskSpec<Self::AddressSpace>, kstack_top: u64) -> Self::Context {
        task::init_user_context(spec, kstack_top)
    }

    unsafe fn switch(&self, from: &mut Self::Context, to: &Self::Context) {
        unsafe { task::switch(from, to) }
    }

    unsafe fn enter_user(&self, entry: UserEntry) -> ! {
        // AArch64 user mode entry via ERET
        // SP_EL0 = user_sp
        // ELR_EL1 = entry_pc
        // SPSR_EL1 = EL0t (0) with interrupts masked (DAIF set) initially
        // SPSR: M[3:0]=0 (EL0t), F=1, I=1, A=1, D=1 => 0x3C0
        // Or unmasked? "if unstable, start with interrupts masked".
        // Let's use 0x3C0 for now (all masked, EL0t).
        
        let spsr: u64 = 0x3C0; 

        unsafe { asm!(
            "msr sp_el0, {sp}",
            "msr elr_el1, {pc}",
            "msr spsr_el1, {spsr}",
            "mov x0, {arg}",
            "eret",
            sp = in(reg) entry.user_sp,
            pc = in(reg) entry.entry_pc,
            spsr = in(reg) spsr,
            arg = in(reg) entry.arg0,
            options(noreturn)
        ); }
    }

    // Paging
    fn make_user_address_space(&self) -> Self::AddressSpace {
        paging::make_user_address_space(self.active_address_space(), &DumbKernelAlloc)
    }

    fn active_address_space(&self) -> Self::AddressSpace {
        paging::active_address_space()
    }
    
    fn activate_address_space(&self, aspace: Self::AddressSpace) {
        unsafe { asm!("msr ttbr0_el1, {}", in(reg) aspace.0); }
    }

    fn map_page(&self, aspace: Self::AddressSpace, virt: u64, phys: u64, perms: MapPerms, kind: MapKind, allocator: &dyn FrameAllocatorHook) -> Result<(), ()> {
        paging::map_page(aspace, virt, phys, perms, kind, allocator)
    }

    fn unmap_page(&self, aspace: Self::AddressSpace, virt: u64) -> Result<Option<u64>, ()> {
        paging::unmap_page(aspace, virt)
    }

    fn translate(&self, aspace: Self::AddressSpace, virt: u64) -> Option<u64> {
        paging::translate(aspace, virt)
    }

    fn tlb_flush_page(&self, virt: u64) {
        paging::tlb_flush_page(virt)
    }
}

struct DumbKernelAlloc;
impl FrameAllocatorHook for DumbKernelAlloc {
    fn alloc_frame(&self) -> Option<u64> { None }
}

pub struct SerialPort {
    pub clamp: MonotonicClamp,
}

impl SerialPort {
    pub const fn new() -> Self {
        Self {
            clamp: MonotonicClamp::new(),
        }
    }

    fn putchar(&self, c: u8) {
        // Semihosting SYS_WRITEC operation
        let ch = c;
        unsafe {
            asm!(
                "hlt #0xF000",
                in("w0") 0x03,
                in("x1") &ch,
                options(nostack, preserves_flags)
            );
        }
    }
}

pub fn hcf() -> ! {
    loop {
        unsafe { asm!("wfi", options(nomem, nostack)); }
    }
}

#[inline]
fn read_cntfrq_el0() -> u64 {
    let val: u64;
    unsafe { asm!("mrs {}, cntfrq_el0", out(reg) val, options(nomem, nostack)); }
    val
}

#[inline]
fn read_cntvct_el0() -> u64 {
    let val: u64;
    unsafe { asm!("mrs {}, cntvct_el0", out(reg) val, options(nomem, nostack)); }
    val
}
