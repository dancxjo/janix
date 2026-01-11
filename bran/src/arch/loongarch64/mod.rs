use core::arch::asm;
use kernel::{IrqState, UserTaskSpec, UserEntry, FrameAllocatorHook, MapPerms, MapKind};
use kernel::time::MonotonicClamp;
use crate::runtime::ArchRuntime;

pub mod task;
pub mod trap;
pub mod paging;
pub mod vector;

pub struct LoongArch64Runtime {
    clamp: MonotonicClamp,
}

impl LoongArch64Runtime {
    pub const fn new() -> Self {
        Self {
            clamp: MonotonicClamp::new(),
        }
    }
}

pub use task::LoongArch64Context;
pub use paging::LoongArch64AddressSpace;

impl ArchRuntime for LoongArch64Runtime {
    type Context = LoongArch64Context;
    type AddressSpace = LoongArch64AddressSpace;

    fn init(&self, hhdm_offset: u64) { 
        paging::init(hhdm_offset); 
        unsafe { vector::init(); }
    }
    fn putchar(&self, c: u8) {
        unsafe {
             let uart = 0x1fe001e0 as *mut u8;
             core::ptr::write_volatile(uart, c);
        }
    }

    fn halt(&self) -> ! { hcf() }

    fn mono_ticks(&self) -> u64 {
        let val: u64;
        unsafe { asm!("rdtime.d {}, $r0", out(reg) val); }
        self.clamp.clamp(val)
    }

    fn mono_freq_hz(&self) -> u64 { 100_000_000 }

    fn irq_disable(&self) -> IrqState {
        let prmd: usize;
        unsafe {
            asm!("csrrd {}, 0x1", out(reg) prmd);
            asm!("csrwr $r0, 0x1");
        }
        IrqState((prmd >> 2) & 1) 
    }

    fn irq_restore(&self, state: IrqState) {
        if state.0 != 0 {
            unsafe { 
                asm!("csrrd $r9, 0x1");
                asm!("ori $r9, $r9, 0x4");
                asm!("csrwr $r9, 0x1");
            }
        } else {
            unsafe {
                asm!("csrrd $r9, 0x1");
                asm!("andi $r9, $r9, 0xFFB");
                asm!("csrwr $r9, 0x1");
            }
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
        // LoongArch64 user mode entry via ertn
        // PRMD (0x1): PPLv (bits 1:0) = 3 (User)
        //             PIE (bit 2) = 0 (Interrupts masked for safety, as per prompt)
        // ERA (0x6): entry_pc
        // $sp: user_sp
        // $a0: arg0
        
        let mut prmd: usize;
        unsafe { asm!("csrrd {}, 0x1", out(reg) prmd); }
        prmd |= 3; // Set PPLv to 3 (User - PLV3)
        prmd &= !(1 << 2); // Functionally Clear PIE (disable interrupts)
        
        asm!(
            "csrwr {prmd}, 0x1",
            "csrwr {pc}, 0x6",
            "move $sp, {sp}",
            "move $a0, {arg}",
            "ertn",
            prmd = in(reg) prmd,
            pc = in(reg) entry.entry_pc,
            sp = in(reg) entry.user_sp,
            arg = in(reg) entry.arg0,
            options(noreturn)
        );
    }

    // Paging
    fn make_user_address_space(&self) -> Self::AddressSpace {
        paging::make_user_address_space(self.active_address_space(), &DumbKernelAlloc)
    }

    fn active_address_space(&self) -> Self::AddressSpace {
        paging::active_address_space()
    }
    
    fn activate_address_space(&self, aspace: Self::AddressSpace) {
        unsafe { asm!("csrwr {}, 0x19", in(reg) aspace.0); }
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

pub fn hcf() -> ! {
    loop { unsafe { asm!("idle 0"); } }
}
