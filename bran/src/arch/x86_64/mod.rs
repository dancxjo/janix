// Updated X86_64 runtime with user entry mapping and alignment check
use core::arch::asm;
use kernel::{IrqState, UserTaskSpec, UserEntry, FrameAllocatorHook, MapPerms, MapKind};
use kernel::time::MonotonicClamp;
use crate::runtime::ArchRuntime;

pub mod task;
pub mod trap;
pub mod paging;
pub mod simd;
pub mod syscall;
pub mod gdt;
pub mod idt;
pub mod cmos;

pub struct X86_64Runtime {
    clamp: MonotonicClamp,
}

impl X86_64Runtime {
    pub const fn new() -> Self {
        Self { clamp: MonotonicClamp::new() }
    }

    // Helper to map user entry pages
    fn map_user_entry(&self, entry: &UserEntry) -> Result<(), ()> {
        let page_mask = !(0xFFF_u64);
        let code_base = (entry.entry_pc as u64) & page_mask;
        // Stack grows down, so mapped page is below SP
        let stack_base = ((entry.user_sp as u64).saturating_sub(8)) & page_mask;
        
        let aspace = self.active_address_space();

        // 1. Map Code Page
        // Need to translate first to preserve physical address
        if let Some(code_phys) = self.translate(aspace, code_base) {
            self.map_page(
                aspace,
                code_base,
                code_phys,
                MapPerms { user: true, read: true, write: false, exec: true },
                MapKind::Normal,
                &ProxyAllocator,
            )?;
        } else {
            // If code is not mapped, we can't run!
            // But maybe we should return Err?
            // The caller unwraps.
            return Err(());
        }

        // 2. Map Stack Page
        if let Some(stack_phys) = self.translate(aspace, stack_base) {
            self.map_page(
                aspace,
                stack_base,
                stack_phys,
                MapPerms { user: true, read: true, write: true, exec: false },
                MapKind::Normal,
                &ProxyAllocator,
            )?;
        } else {
            return Err(());
        }
        
        Ok(())
    }
}

pub use task::X86_64Context;
pub use paging::X86_64AddressSpace;

impl ArchRuntime for X86_64Runtime {
    type Context = X86_64Context;
    type AddressSpace = X86_64AddressSpace;

    fn init(&self, hhdm_offset: u64) {
        unsafe {
            gdt::init();
            
            // Allocate Double Fault Stack
            let phys = kernel::memory::alloc_frame().expect("No frames for DF stack");
            let virt = phys + hhdm_offset + 4096; // Top of stack
            gdt::set_ist1(virt);
            
            idt::init();
        }
        paging::init(hhdm_offset);
        unsafe { syscall::init(); }
    }

    fn putchar(&self, c: u8) {
        unsafe {
            let port = 0x3f8;
            core::arch::asm!("out dx, al", in("dx") port, in("al") c);
        }
    }

    fn halt(&self) -> ! { hcf() }

    fn mono_ticks(&self) -> u64 {
        let low: u32; let high: u32;
        unsafe { core::arch::asm!("rdtsc", out("eax") low, out("edx") high); }
        let raw = ((high as u64) << 32) | (low as u64);
        self.clamp.clamp(raw)
    }

    fn mono_freq_hz(&self) -> u64 { 2_000_000_000 }

    fn irq_disable(&self) -> IrqState {
        let rflags: u64;
        unsafe { core::arch::asm!("pushfq", "pop {}", out(reg) rflags); core::arch::asm!("cli"); }
        IrqState(((rflags >> 9) & 1) as usize)
    }

    fn irq_restore(&self, state: IrqState) {
        if state.0 != 0 { unsafe { core::arch::asm!("sti"); } }
    }

    fn threads_supported(&self) -> bool { true }

    fn simd_init_cpu(&self) { simd::init_cpu(); }
    fn simd_state_layout(&self) -> (usize, usize) { simd::STATE_LAYOUT }
    unsafe fn simd_save(&self, dst: *mut u8) { unsafe { simd::save(dst) } }
    unsafe fn simd_restore(&self, src: *const u8) { unsafe { simd::restore(src) } }

    fn fence_full(&self) { unsafe { core::arch::asm!("mfence", options(nostack, preserves_flags)); } }

    // Tasking
    fn init_kernel_context(&self, entry: extern "C" fn(usize) -> !, stack_top: u64, arg: usize) -> Self::Context {
        task::init_kernel_context(entry, stack_top, arg)
    }
    fn init_user_context(&self, spec: UserTaskSpec<Self::AddressSpace>, kstack_top: u64) -> Self::Context {
        task::init_user_context(spec, kstack_top)
    }
    unsafe fn switch(&self, from: &mut Self::Context, to: &Self::Context) { unsafe { task::switch(from, to) } }

    unsafe fn enter_user(&self, entry: UserEntry) -> ! {
        // Ensure stack alignment
        if entry.user_sp & 0xF != 0 { panic!("user_sp not 16-byte aligned"); }
        // Map required pages before entering user mode
        self.map_user_entry(&entry).expect("failed to map user entry pages");
        // x86_64 user mode entry via IRETQ
        let user_data_sel: u64 = gdt::USER_DATA_SEL as u64;
        let user_code_sel: u64 = gdt::USER_CODE_SEL as u64;
        let rflags: u64 = 0x202; // IF + reserved
        unsafe {
            asm!(
                "cli",
                "swapgs",
                "push {ss}",
                "push {rsp}",
                "push {rflags}",
                "push {cs}",
                "push {rip}",
                "iretq",
                ss = in(reg) user_data_sel,
                rsp = in(reg) entry.user_sp,
                rflags = in(reg) rflags,
                cs = in(reg) user_code_sel,
                rip = in(reg) entry.entry_pc,
                in("rdi") entry.arg0,
                options(noreturn)
            );
        }
    }

    // Paging
    fn make_user_address_space(&self) -> Self::AddressSpace { paging::make_user_address_space(self.active_address_space(), &ProxyAllocator) }
    fn active_address_space(&self) -> Self::AddressSpace { paging::active_address_space() }
    fn activate_address_space(&self, aspace: Self::AddressSpace) { unsafe { core::arch::asm!("mov cr3, {}", in(reg) aspace.0); } }
    fn map_page(&self, aspace: Self::AddressSpace, virt: u64, phys: u64, perms: MapPerms, kind: MapKind, allocator: &dyn FrameAllocatorHook) -> Result<(), ()> { paging::map_page(aspace, virt, phys, perms, kind, allocator) }
    fn unmap_page(&self, aspace: Self::AddressSpace, virt: u64) -> Result<Option<u64>, ()> { paging::unmap_page(aspace, virt) }
    fn translate(&self, aspace: Self::AddressSpace, virt: u64) -> Option<u64> { paging::translate(aspace, virt) }
    fn tlb_flush_page(&self, virt: u64) { paging::tlb_flush_page(virt) }

    fn read_rtc(&self) -> Option<abi::device::RtcTime> {
        unsafe { Some(cmos::read_rtc()) }
    }
}

struct ProxyAllocator;
impl FrameAllocatorHook for ProxyAllocator { fn alloc_frame(&self) -> Option<u64> { kernel::memory::alloc_frame() } }

pub fn hcf() -> ! { loop { unsafe { core::arch::asm!("hlt"); } } }
