// Updated X86_64 runtime with user entry mapping and alignment check
use crate::runtime::ArchRuntime;
use core::arch::asm;
use core::sync::atomic::{AtomicU64, Ordering};
use kernel::time::MonotonicClamp;
use kernel::{CpuId, FrameAllocatorHook, IrqState, MapKind, MapPerms, UserEntry, UserTaskSpec};

pub mod acpi;
pub mod apic;
pub mod cmos;
pub mod gdt;
pub mod idt;
pub mod ioapic;
pub mod paging;
pub mod pci;
pub mod pic;
pub mod simd;
pub mod smp;
pub mod syscall;
pub mod task;
pub mod trap;

pub struct X86_64Runtime {
    clamp: MonotonicClamp,
    hhdm_offset: AtomicU64,
}

pub static mut CPU_IDS: [CpuId; acpi::MAX_CPUS] = [CpuId(0); acpi::MAX_CPUS];
pub static CPU_COUNT: AtomicU64 = AtomicU64::new(1); // Default to 1 (BSP)

impl X86_64Runtime {
    pub fn cpu_ids(&self) -> &'static [CpuId] {
        let count = CPU_COUNT.load(Ordering::Relaxed) as usize;
        unsafe { &CPU_IDS[..count] }
    }

    pub fn current_cpu_id(&self) -> CpuId {
        match self.lapic_id() {
             Ok(id) => CpuId(id),
             Err(_) => CpuId(0),
        }
    }
}

const BOOT_TEMP_MAP_BASE: u64 = 0xffffff10_00000000;

impl X86_64Runtime {
    pub const fn new() -> Self {
        Self {
            clamp: MonotonicClamp::new(),
            hhdm_offset: AtomicU64::new(0),
        }
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
                MapPerms {
                    user: true,
                    read: true,
                    write: false,
                    exec: true,
                },
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
                MapPerms {
                    user: true,
                    read: true,
                    write: true,
                    exec: false,
                },
                MapKind::Normal,
                &ProxyAllocator,
            )?;
        } else {
            return Err(());
        }

        Ok(())
    }
}

pub use paging::X86_64AddressSpace;
pub use task::X86_64Context;

impl ArchRuntime for X86_64Runtime {
    type Context = X86_64Context;
    type AddressSpace = X86_64AddressSpace;

    fn init(&self, hhdm_offset: u64) {
        self.hhdm_offset.store(hhdm_offset, Ordering::Relaxed);
        unsafe {
            gdt::init();

            // Allocate Double Fault Stack
            let phys = kernel::memory::alloc_frame().expect("No frames for DF stack");
            let virt = phys + hhdm_offset + 4096; // Top of stack
            gdt::set_ist1(virt);

            idt::init();
        }
        paging::init(hhdm_offset);
        unsafe {
            syscall::init();
        }

        // Initialize IOAPIC for interrupt routing (after IDT is set up)
        crate::arch::init_ioapic();
    }

    fn putchar(&self, c: u8) {
        unsafe {
            let port = 0x3f8;
            core::arch::asm!("out dx, al", in("dx") port, in("al") c);
        }
    }

    fn halt(&self) -> ! {
        hcf()
    }

    fn wait_for_interrupt(&self) {
        // Enable interrupts and halt until next IRQ
        unsafe {
            core::arch::asm!("sti", "hlt", options(nomem, nostack));
        }
    }

    fn mono_ticks(&self) -> u64 {
        let low: u32;
        let high: u32;
        unsafe {
            core::arch::asm!("rdtsc", out("eax") low, out("edx") high);
        }
        let raw = ((high as u64) << 32) | (low as u64);
        self.clamp.clamp(raw)
    }

    fn mono_freq_hz(&self) -> u64 {
        2_000_000_000
    }

    fn irq_disable(&self) -> IrqState {
        let rflags: u64;
        unsafe {
            core::arch::asm!("pushfq", "pop {}", out(reg) rflags);
            core::arch::asm!("cli");
        }
        IrqState(((rflags >> 9) & 1) as usize)
    }

    fn irq_restore(&self, state: IrqState) {
        if state.0 != 0 {
            unsafe {
                core::arch::asm!("sti");
            }
        }
    }

    fn threads_supported(&self) -> bool {
        true
    }

    fn simd_init_cpu(&self) {
        simd::init_cpu();
    }
    fn simd_state_layout(&self) -> (usize, usize) {
        simd::STATE_LAYOUT
    }
    unsafe fn simd_save(&self, dst: *mut u8) {
        unsafe { simd::save(dst) }
    }
    unsafe fn simd_restore(&self, src: *const u8) {
        unsafe { simd::restore(src) }
    }

    fn fence_full(&self) {
        unsafe {
            core::arch::asm!("mfence", options(nostack, preserves_flags));
        }
    }

    // Tasking
    fn init_kernel_context(
        &self,
        entry: extern "C" fn(usize) -> !,
        stack_top: u64,
        arg: usize,
    ) -> Self::Context {
        task::init_kernel_context(entry, stack_top, arg)
    }
    fn init_user_context(
        &self,
        spec: UserTaskSpec<Self::AddressSpace>,
        kstack_top: u64,
    ) -> Self::Context {
        task::init_user_context(spec, kstack_top)
    }
    unsafe fn switch(&self, from: &mut Self::Context, to: &Self::Context) {
        unsafe { task::switch(from, to) }
    }

    unsafe fn enter_user(&self, entry: UserEntry) -> ! {
        // Ensure stack alignment
        if entry.user_sp & 0x7 != 0 {
            panic!("user_sp not 8-byte aligned");
        }
        // Map required pages before entering user mode
        self.map_user_entry(&entry)
            .expect("failed to map user entry pages");
        // x86_64 user mode entry via IRETQ
        let user_data_sel: u64 = gdt::USER_DATA_SEL as u64;
        let user_code_sel: u64 = gdt::USER_CODE_SEL as u64;
        let rflags: u64 = 0x202; // IF + reserved
        #[cfg(debug_assertions)]
        {
            let tid = unsafe { kernel::task::scheduler::current_tid_current() };
            let (cs, ss, rsp, rip, rflags_before, cr3, fs_base, gs_base) = capture_entry_state();
            let cpl = (cs & 0x3) as u64;
            kernel::log_event!(
                kernel::logging::LogLevel::Info,
                "task.user_enter",
                "Entering user mode",
                {
                    tid: tid,
                    target_pc: entry.entry_pc as u64,
                    target_sp: entry.user_sp as u64,
                    target_cs: user_code_sel,
                    target_ss: user_data_sel,
                    CS: cs as u64,
                    SS: ss as u64,
                    CPL_KERNEL_BEFORE: cpl,
                    RIP_BEFORE: rip,
                    RSP_BEFORE: rsp,
                    RFLAGS_BEFORE: rflags_before,
                    CR3_BEFORE: cr3,
                    fs_base: fs_base,
                    gs_base: gs_base
                },
                about=[]
            );
        }
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
    fn make_user_address_space(&self) -> Self::AddressSpace {
        paging::make_user_address_space(self.active_address_space(), &ProxyAllocator)
    }
    fn active_address_space(&self) -> Self::AddressSpace {
        paging::active_address_space()
    }
    fn activate_address_space(&self, aspace: Self::AddressSpace) {
        unsafe {
            core::arch::asm!("mov cr3, {}", in(reg) aspace.0);
        }
    }
    fn map_page(
        &self,
        aspace: Self::AddressSpace,
        virt: u64,
        phys: u64,
        perms: MapPerms,
        kind: MapKind,
        allocator: &dyn FrameAllocatorHook,
    ) -> Result<(), ()> {
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

    fn setup_preemption_timer(&self, hz: u32) {
        ioapic::setup_lapic_timer(idt::IRQ_TIMER_VECTOR, hz);
    }

    fn debug_active_aspace_root(&self) -> u64 {
        paging::active_address_space().0
    }

    fn pci_cfg_read32(
        &self,
        bus: u8,
        dev: u8,
        func: u8,
        offset: u8,
    ) -> Result<u32, abi::errors::Errno> {
        Ok(pci::read_config(bus, dev, func, offset))
    }
    fn pci_cfg_write32(
        &self,
        bus: u8,
        dev: u8,
        func: u8,
        offset: u8,
        value: u32,
    ) -> Result<(), abi::errors::Errno> {
        pci::write_config(bus, dev, func, offset, value);
        Ok(())
    }

    fn lapic_id(&self) -> Result<u32, abi::errors::Errno> {
        let base = apic::base_phys();
        let hhdm = self.hhdm_offset.load(Ordering::Relaxed);
        Ok(apic::id(base, hhdm))
    }
    fn lapic_base_phys(&self) -> Result<u64, abi::errors::Errno> {
        Ok(apic::base_phys())
    }

    fn map_phys_temp(&self, phys: u64, size: usize) -> Result<u64, abi::errors::Errno> {
        let aspace = self.active_address_space();
        let page_size = 4096u64;
        let phys_aligned = phys & !(page_size - 1);
        let offset = phys - phys_aligned;
        let size_to_map = (size as u64 + offset + (page_size - 1)) & !(page_size - 1);
        let pages = (size_to_map / page_size) as usize;

        for i in 0..pages {
            let p_addr = phys_aligned + (i as u64 * page_size);
            let v_addr = BOOT_TEMP_MAP_BASE + (i as u64 * page_size);
            self.map_page(
                aspace,
                v_addr,
                p_addr,
                MapPerms {
                    user: false,
                    read: true,
                    write: false,
                    exec: false,
                },
                MapKind::Normal,
                &ProxyAllocator,
            )
            .map_err(|_| abi::errors::Errno::ENOMEM)?;
            self.tlb_flush_page(v_addr);
        }

        Ok(BOOT_TEMP_MAP_BASE + offset)
    }

    fn start_secondary_cpus(
        &self,
        entry: extern "C" fn(usize) -> !,
    ) -> Result<(), abi::errors::Errno> {
        use kernel::{kinfo, kerror};
        
        let ids = self.cpu_ids();
        let bsp_id = self.current_cpu_id();
        let cpu_count = ids.len();
        
        if cpu_count <= 1 {
            return Ok(());
        }

        kinfo!("SMP: Starting {} secondary CPUs...", cpu_count - 1);

        // 1. Map trampoline page (0x8000)
        let trampoline_base: u64 = 0x8000;
        let trampoline_addr = match self.map_phys_temp(trampoline_base, 4096) {
            Ok(addr) => addr,
            Err(e) => {
                kerror!("SMP: Failed to map trampoline page: {:?}", e);
                return Err(e);
            }
        };

        // 2. Copy trampoline code
        unsafe {
            let start = &smp::trampoline_start as *const _ as *const u8;
            let end = &smp::trampoline_end as *const _ as *const u8;
            let len = end.offset_from(start) as usize;
            
            if len > 4096 {
                panic!("SMP: Trampoline too large!");
            }
            
            core::ptr::copy_nonoverlapping(start, trampoline_addr as *mut u8, len);
            kinfo!("SMP: Copied trampoline to 0x{:x}", trampoline_base);
        }

        // 3. Helper to write to trampoline data
        let write_trampoline_data = |offset: usize, val: u64| unsafe {
             core::ptr::write_volatile((trampoline_addr + offset as u64) as *mut u64, val);
        };
        
        // 4. Helper to send IPI
        let lapic_base = self.lapic_base_phys().unwrap(); // We know it works if we are here
        // Map LAPIC
        let lapic_virt = match self.map_phys_temp(lapic_base, 4096) {
             Ok(a) => a,
             Err(e) => return Err(e),
        };

        let write_icr = |high: u32, low: u32| unsafe {
            core::ptr::write_volatile((lapic_virt + 0x310) as *mut u32, high);
            core::ptr::write_volatile((lapic_virt + 0x300) as *mut u32, low);
        };

        // 5. Start each AP
        for (i, &cpu_id) in ids.iter().enumerate() {
            if cpu_id == bsp_id {
                continue;
            }
            
            let apic_id = cpu_id.0 as u32;
            kinfo!("SMP: Starting CPU {} (APIC {})", i, apic_id);

            // Allocate stack
            // 1 page (4KB) for bootstrap stack
            let stack_frames = 1;
            let stack_base_run = kernel::memory::alloc_frame()
                .expect("Failed to alloc AP stack");
            let stack_top = stack_base_run + 4096;

            // Setup trampoline data
            // We use the offsets defined in smp.rs comments
            // flag: 0x100
            // cr3: 0x108
            // stack_top: 0x110
            // entry_point: 0x118
            // cpu_index: 0x120
            
            write_trampoline_data(0x100, 0); // Clear flag
            write_trampoline_data(0x108, self.debug_active_aspace_root()); // CR3
            write_trampoline_data(0x110, stack_top);
            write_trampoline_data(0x118, entry as usize as u64);
            write_trampoline_data(0x120, i as u64);

            // INIT IPI
            // Dest Shorthand=0, TrigMode=Level, Level=Assert, DelivMode=INIT
            // 0x00004500
            write_icr(apic_id << 24, 0x00004500);
            
            // Wait 10ms (busy loop approximation - 2G Hz = 20M cycles)
            // We don't have good delay here, just loop a bit.
            for _ in 0..10_000_000 { core::hint::spin_loop(); }

            // SIPI
            // Vector = 0x08 (0x8000)
            // 0x00004608
            write_icr(apic_id << 24, 0x00004608);

            // Wait for come up
            let mut came_up = false;
            for _ in 0..10_000_000 {
                let flag = unsafe { core::ptr::read_volatile((trampoline_addr + 0x100) as *const u64) };
                if flag == 1 {
                    came_up = true;
                    break;
                }
                core::hint::spin_loop();
            }
            
            if came_up {
                kinfo!("SMP: CPU {} started", i);
            } else {
                kerror!("SMP: CPU {} timed out", i);
            }
        }
        
        // Unmap temporary pages
        self.unmap_phys_temp(trampoline_addr, 4096);
        self.unmap_phys_temp(lapic_virt, 4096);

        Ok(())
    }

    fn unmap_phys_temp(&self, virt: u64, size: usize) {
        let aspace = self.active_address_space();
        let page_size = 4096u64;
        let virt_aligned = virt & !(page_size - 1);
        let offset = virt - virt_aligned;
        let size_to_unmap = (size as u64 + offset + (page_size - 1)) & !(page_size - 1);
        let pages = (size_to_unmap / page_size) as usize;

        for i in 0..pages {
            let v_addr = virt_aligned + (i as u64 * page_size);
            let _ = self.unmap_page(aspace, v_addr);
            self.tlb_flush_page(v_addr);
        }
    }
}

struct ProxyAllocator;
impl FrameAllocatorHook for ProxyAllocator {
    fn alloc_frame(&self) -> Option<u64> {
        kernel::memory::alloc_frame()
    }
}

#[cfg(debug_assertions)]
fn capture_entry_state() -> (u16, u16, u64, u64, u64, u64, u64, u64) {
    let cs: u16;
    let ss: u16;
    let rsp: u64;
    let rip: u64;
    let rflags: u64;
    let cr3: u64;

    unsafe {
        core::arch::asm!(
            "mov {cs_out:x}, cs",
            "mov {ss_out:x}, ss",
            "mov {rsp_out}, rsp",
            "lea {rip_out}, [rip]",
            "pushfq",
            "pop {rflags_out}",
            cs_out = out(reg) cs,
            ss_out = out(reg) ss,
            rsp_out = out(reg) rsp,
            rip_out = out(reg) rip,
            rflags_out = out(reg) rflags,
        );
        core::arch::asm!("mov {cr3_out}, cr3", cr3_out = out(reg) cr3);
    }

    let fs_base = read_msr_debug(0xC000_0100);
    let gs_base = read_msr_debug(0xC000_0101);

    (cs, ss, rsp, rip, rflags, cr3, fs_base, gs_base)
}

#[cfg(debug_assertions)]
fn read_msr_debug(msr: u32) -> u64 {
    let low: u32;
    let high: u32;
    unsafe {
        core::arch::asm!(
            "rdmsr",
            in("ecx") msr,
            out("eax") low,
            out("edx") high,
            options(nostack, preserves_flags)
        );
    }
    ((high as u64) << 32) | (low as u64)
}

pub fn hcf() -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
