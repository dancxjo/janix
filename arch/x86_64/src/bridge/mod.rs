use crate::bringup::{self, gdt, interrupts};
use core::arch::asm;
use core::sync::atomic::{AtomicU64, Ordering};
use kernel::bridge::{CpuBridge, MachineBridge, PortIo, Power, Rtc, VmMapper};

pub mod ports;
pub mod rtc;
pub mod timer;
pub mod uart;
pub mod user;

pub struct Bridge;

/// Interrupt mask token captures the IF flag state.
#[derive(Copy, Clone, Debug)]
pub struct IrqState(pub bool);

impl Default for IrqState {
    fn default() -> Self {
        Self(true)
    }
}

/// Saved FPU/SIMD state; fxsave/fxrstor expects 16-byte alignment.
#[repr(C, align(16))]
#[derive(Copy, Clone, Debug)]
pub struct FpuState(pub [u8; 512]);

impl Default for FpuState {
    fn default() -> Self {
        Self([0u8; 512])
    }
}

// Hook for scheduler. Only set by kernel binary.
pub static mut TICK_HOOK: Option<fn(&mut interrupts::trap::TrapFrame)> = None;

pub fn set_tick_hook(hook: fn(&mut interrupts::trap::TrapFrame)) {
    unsafe {
        TICK_HOOK = Some(hook);
    }
}

// Hook for page faults.
pub static mut PAGE_FAULT_HOOK: Option<
    fn(
        &x86_64::structures::idt::InterruptStackFrame,
        u64,
        x86_64::structures::idt::PageFaultErrorCode,
    ) -> bool,
> = None;

pub fn set_page_fault_hook(
    hook: fn(
        &x86_64::structures::idt::InterruptStackFrame,
        u64,
        x86_64::structures::idt::PageFaultErrorCode,
    ) -> bool,
) {
    unsafe {
        PAGE_FAULT_HOOK = Some(hook);
    }
}

pub static HHDM_OFFSET: AtomicU64 = AtomicU64::new(0);

#[cfg(target_arch = "x86_64")]
impl Bridge {
    pub unsafe fn init(_rsdp_addr: Option<u64>, hhdm: u64) {
        // Debug: very first thing - output to debugcon
        unsafe {
            core::arch::asm!(
                "out dx, al",
                in("dx") 0xe9u16,
                in("al") b'B',
                options(nomem, nostack, preserves_flags)
            );
        }

        HHDM_OFFSET.store(hhdm, Ordering::Relaxed);
        let b = Bridge;

        // Initialize Serial first
        kernel::drivers::serial::init(&b);

        b.log("BRIDGE: gdt::init\n");
        gdt::init();
        b.log("BRIDGE: idt::init\n");
        interrupts::idt::init();
        b.log("BRIDGE: pic::init\n");
        interrupts::pic::init();
        b.log("BRIDGE: syscall::init\n");
        interrupts::syscall::init();

        b.log("BRIDGE: ps2::init\n");
        kernel::drivers::ps2::init(&b);
    }

    pub unsafe fn init_acpi(rsdp_addr: u64, hhdm: u64) {
        let b = Bridge;
        b.log("BRIDGE: acpi::init\n");
        kernel::platform::acpi::init(&b, rsdp_addr, hhdm);
    }
}

#[cfg(target_arch = "x86_64")]
impl CpuBridge for Bridge {
    type IrqState = IrqState;
    type Context = bringup::ArchContext;
    type FpuState = FpuState;
    const CONTEXT_WORDS: usize = 20;

    fn log(&self, msg: &str) {
        unsafe {
            for b in msg.bytes() {
                // 0xE9 Debugcon
                asm!("out dx, al", in("dx") 0xE9u16, in("al") b);

                // UART
                let mut status: u8;
                loop {
                    asm!("in al, dx", out("al") status, in("dx") 0x3F8u16 + 5);
                    if status & 0x20 != 0 {
                        break;
                    }
                    core::hint::spin_loop();
                }
                asm!("out dx, al", in("dx") 0x3F8u16, in("al") b);
            }
        }
    }

    fn irq_disable(&self) -> Self::IrqState {
        let flags: u64;
        unsafe {
            asm!(
                "pushfq",
                "pop {}",
                "cli",
                out(reg) flags,
                options(nomem, preserves_flags)
            );
        }
        IrqState(flags & (1 << 9) != 0)
    }

    fn irq_restore(&self, state: Self::IrqState) {
        unsafe {
            if state.0 {
                asm!("sti", options(nomem, nostack, preserves_flags));
            } else {
                asm!("cli", options(nomem, nostack, preserves_flags));
            }
        }
    }

    fn ticks(&self) -> u64 {
        kernel::drivers::hpet::read_ticks()
    }

    fn ticks_per_second(&self) -> u64 {
        kernel::drivers::hpet::ticks_per_second()
    }

    fn idle(&self) {
        unsafe {
            asm!("hlt");
        }
    }

    fn init_thread_context(&self, entry: u64, stack: u64, arg: u64) -> Self::Context {
        let mut ctx = [0u64; 20];
        ctx[13] = arg;
        ctx[15] = entry;

        let is_kernel = entry >= 0xFFFF_8000_0000_0000;

        if is_kernel {
            ctx[16] = unsafe { gdt::KERNEL_CODE_SELECTOR.0 as u64 };
            ctx[17] = 0x202;
            ctx[18] = stack;
            ctx[19] = unsafe { gdt::KERNEL_DATA_SELECTOR.0 as u64 };
        } else {
            ctx[16] = unsafe { gdt::USER_CODE_SELECTOR.0 as u64 | 3 };
            ctx[17] = 0x3202;
            ctx[18] = stack;
            ctx[19] = unsafe { gdt::USER_DATA_SELECTOR.0 as u64 | 3 };
        }

        bringup::ArchContext(ctx)
    }

    fn switch(&self, from: &mut Self::Context, to: &Self::Context) {
        *from = *to;
        crate::bringup::user::enter::resume_user_mode(&to.0);
    }

    fn set_kernel_stack(&self, stack_top: u64) {
        unsafe {
            gdt::set_kernel_stack(stack_top);
            interrupts::syscall::set_kernel_stack(stack_top);
        }
    }

    fn save_fpu(&self, area: &mut Self::FpuState) {
        unsafe {
            let ptr = area.0.as_mut_ptr();
            asm!("fxsave [{}]", in(reg) ptr);
        }
    }

    fn restore_fpu(&self, area: &Self::FpuState) {
        unsafe {
            let ptr = area.0.as_ptr();
            asm!("fxrstor [{}]", in(reg) ptr);
        }
    }
}

#[cfg(target_arch = "x86_64")]
impl MachineBridge for Bridge {
    fn hhdm_offset(&self) -> u64 {
        HHDM_OFFSET.load(Ordering::Relaxed)
    }
}

#[cfg(target_arch = "x86_64")]
impl PortIo for Bridge {
    fn port_outb(&self, port: u16, val: u8) {
        unsafe {
            asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack, preserves_flags));
        }
    }

    fn port_inb(&self, port: u16) -> u8 {
        let val: u8;
        unsafe {
            asm!("in al, dx", out("al") val, in("dx") port, options(nomem, nostack, preserves_flags));
        }
        val
    }

    fn port_outw(&self, port: u16, val: u16) {
        unsafe {
            asm!("out dx, ax", in("dx") port, in("ax") val, options(nomem, nostack, preserves_flags));
        }
    }

    fn port_inw(&self, port: u16) -> u16 {
        let val: u16;
        unsafe {
            asm!("in ax, dx", out("ax") val, in("dx") port, options(nomem, nostack, preserves_flags));
        }
        val
    }

    fn port_outd(&self, port: u16, val: u32) {
        unsafe {
            asm!("out dx, eax", in("dx") port, in("eax") val, options(nomem, nostack, preserves_flags));
        }
    }

    fn port_ind(&self, port: u16) -> u32 {
        let val: u32;
        unsafe {
            asm!("in eax, dx", out("eax") val, in("dx") port, options(nomem, nostack, preserves_flags));
        }
        val
    }
}

#[cfg(target_arch = "x86_64")]
impl Rtc for Bridge {
    fn rtc_read(&self, out: &mut abi::wire::time::RtcSample) {
        unsafe {
            let read_reg = |reg: u8| -> u8 {
                asm!("out dx, al", in("dx") 0x70u16, in("al") reg);
                let val: u8;
                asm!("in al, dx", out("al") val, in("dx") 0x71u16);
                val
            };

            while (read_reg(0x0A) & 0x80) != 0 {
                core::hint::spin_loop();
            }

            let mut sec = read_reg(0x00);
            let mut min = read_reg(0x02);
            let mut hour = read_reg(0x04);
            let mut day = read_reg(0x07);
            let mut mon = read_reg(0x08);
            let mut year = read_reg(0x09) as u16;

            let reg_b = read_reg(0x0B);

            if (reg_b & 0x04) == 0 {
                sec = (sec & 0x0F) + ((sec / 16) * 10);
                min = (min & 0x0F) + ((min / 16) * 10);
                hour = ((hour & 0x0F) + ((hour & 0x70) / 16 * 10)) | (hour & 0x80);
                day = (day & 0x0F) + ((day / 16) * 10);
                mon = (mon & 0x0F) + ((mon / 16) * 10);
                year = (year & 0x0F) as u16 + ((year / 16) as u16 * 10);
            }

            if (reg_b & 0x02) == 0 && (hour & 0x80) != 0 {
                hour = ((hour & 0x7F) + 12) % 24;
            }

            out.year = 2000 + year;
            out.mon = mon;
            out.day = day;
            out.hour = hour;
            out.min = min;
            out.sec = sec;
        }
    }
}

#[cfg(target_arch = "x86_64")]
impl Power for Bridge {
    fn shutdown(&self) -> ! {
        loop {
            unsafe {
                asm!("out dx, ax", in("dx") 0x604u16, in("ax") 0x2000u16);
                asm!("hlt");
            }
        }
    }
}

#[cfg(target_arch = "x86_64")]
impl VmMapper for Bridge {
    fn map_new_user_page(&self, virt_addr: u64, flags: u64) -> Result<(), ()> {
        use core::alloc::Layout;
        use x86_64::structures::paging::{
            mapper::Mapper, FrameAllocator, OffsetPageTable, Page, PageTableFlags, PhysFrame,
            Size4KiB, Translate,
        };
        use x86_64::VirtAddr;

        let layout = unsafe { Layout::from_size_align_unchecked(4096, 4096) };
        let ptr = unsafe { alloc::alloc::alloc_zeroed(layout) };
        if ptr.is_null() {
            return Err(());
        }

        let kernel_virt = VirtAddr::new(ptr as u64);

        unsafe {
            let hhdm = HHDM_OFFSET.load(Ordering::Relaxed);
            let (l4_frame, _) = x86_64::registers::control::Cr3::read();
            let phys_l4 = l4_frame.start_address();
            let virt_l4 = VirtAddr::new(hhdm + phys_l4.as_u64());
            let page_table_ptr = virt_l4.as_mut_ptr();
            let mut mapper = OffsetPageTable::new(&mut *page_table_ptr, VirtAddr::new(hhdm));

            let phys_frame = match mapper.translate_addr(kernel_virt) {
                Some(p) => PhysFrame::<Size4KiB>::containing_address(p),
                None => {
                    alloc::alloc::dealloc(ptr, layout);
                    return Err(());
                }
            };

            let user_page = Page::<Size4KiB>::containing_address(VirtAddr::new(virt_addr));
            let map_flags = PageTableFlags::from_bits_truncate(flags)
                | PageTableFlags::PRESENT
                | PageTableFlags::USER_ACCESSIBLE;

            struct HeapFrameAllocator;
            unsafe impl FrameAllocator<Size4KiB> for HeapFrameAllocator {
                fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
                    let layout = unsafe { Layout::from_size_align_unchecked(4096, 4096) };
                    let ptr = unsafe { alloc::alloc::alloc_zeroed(layout) };
                    if ptr.is_null() {
                        return None;
                    }

                    let hhdm = HHDM_OFFSET.load(Ordering::Relaxed);
                    let virt = ptr as u64;

                    unsafe {
                        let (l4_frame, _) = x86_64::registers::control::Cr3::read();
                        let phys_l4 = l4_frame.start_address();
                        let virt_l4 = VirtAddr::new(hhdm + phys_l4.as_u64());
                        let page_table_ptr = virt_l4.as_mut_ptr();
                        let mapper =
                            OffsetPageTable::new(&mut *page_table_ptr, VirtAddr::new(hhdm));
                        mapper
                            .translate_addr(VirtAddr::new(virt))
                            .map(|p| PhysFrame::containing_address(p))
                    }
                }
            }

            let mut allocator = HeapFrameAllocator;

            match mapper.map_to(user_page, phys_frame, map_flags, &mut allocator) {
                Ok(flush) => {
                    flush.flush();
                    Ok(())
                }
                Err(_) => {
                    alloc::alloc::dealloc(ptr, layout);
                    Err(())
                }
            }
        }
    }

    fn map_user_mmio(&self, virt_addr: u64, phys_addr: u64, flags: u64) -> Result<(), ()> {
        use core::alloc::Layout;
        use x86_64::structures::paging::{
            mapper::Mapper, FrameAllocator, OffsetPageTable, Page, PageTableFlags, PhysFrame,
            Size4KiB, Translate,
        };
        use x86_64::VirtAddr;

        unsafe {
            let hhdm = HHDM_OFFSET.load(Ordering::Relaxed);
            let (l4_frame, _) = x86_64::registers::control::Cr3::read();
            let phys_l4 = l4_frame.start_address();
            let virt_l4 = VirtAddr::new(hhdm + phys_l4.as_u64());
            let page_table_ptr = virt_l4.as_mut_ptr();
            let mut mapper = OffsetPageTable::new(&mut *page_table_ptr, VirtAddr::new(hhdm));

            let user_page = Page::<Size4KiB>::containing_address(VirtAddr::new(virt_addr));
            let phys_frame =
                PhysFrame::<Size4KiB>::containing_address(x86_64::PhysAddr::new(phys_addr));
            let map_flags = PageTableFlags::from_bits_truncate(flags)
                | PageTableFlags::PRESENT
                | PageTableFlags::USER_ACCESSIBLE
                | PageTableFlags::NO_CACHE
                | PageTableFlags::WRITE_THROUGH;

            struct HeapFrameAllocator;
            unsafe impl FrameAllocator<Size4KiB> for HeapFrameAllocator {
                fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
                    let layout = unsafe { Layout::from_size_align_unchecked(4096, 4096) };
                    let ptr = unsafe { alloc::alloc::alloc_zeroed(layout) };
                    if ptr.is_null() {
                        return None;
                    }
                    let hhdm = HHDM_OFFSET.load(Ordering::Relaxed);
                    let virt = ptr as u64;
                    unsafe {
                        let (l4_frame, _) = x86_64::registers::control::Cr3::read();
                        let phys_l4 = l4_frame.start_address();
                        let virt_l4 = VirtAddr::new(hhdm + phys_l4.as_u64());
                        let page_table_ptr = virt_l4.as_mut_ptr();
                        let mapper =
                            OffsetPageTable::new(&mut *page_table_ptr, VirtAddr::new(hhdm));
                        mapper
                            .translate_addr(VirtAddr::new(virt))
                            .map(|p| PhysFrame::containing_address(p))
                    }
                }
            }
            let mut allocator = HeapFrameAllocator;
            match mapper.map_to(user_page, phys_frame, map_flags, &mut allocator) {
                Ok(flush) => {
                    flush.flush();
                    Ok(())
                }
                Err(_) => Err(()),
            }
        }
    }
}

#[cfg(not(target_arch = "x86_64"))]
impl CpuBridge for Bridge {
    type IrqState = IrqState;
    type Context = bringup::ArchContext;
    type FpuState = FpuState;
    const CONTEXT_WORDS: usize = 20;
    fn log(&self, _msg: &str) {}
    fn ticks(&self) -> u64 {
        0
    }
    fn ticks_per_second(&self) -> u64 {
        0
    }
    fn idle(&self) {}
    fn shutdown(&self) -> ! {
        loop {}
    }
    fn irq_disable(&self) -> Self::IrqState {
        IrqState::default()
    }
    fn irq_restore(&self, _state: Self::IrqState) {}
    fn init_thread_context(&self, _entry: u64, _stack: u64, _arg: u64) -> Self::Context {
        bringup::ArchContext([0; 20])
    }
    fn switch(&self, _from: &mut Self::Context, _to: &Self::Context) {}
    fn set_kernel_stack(&self, _: u64) {}
    fn save_fpu(&self, _area: &mut Self::FpuState) {}
    fn restore_fpu(&self, _area: &Self::FpuState) {}
}

#[cfg(not(target_arch = "x86_64"))]
impl MachineBridge for Bridge {
    fn hhdm_offset(&self) -> u64 {
        0
    }
}

#[cfg(not(target_arch = "x86_64"))]
impl PortIo for Bridge {
    fn port_outb(&self, _port: u16, _val: u8) {}
    fn port_inb(&self, _port: u16) -> u8 {
        0
    }
    fn port_outw(&self, _port: u16, _val: u16) {}
    fn port_inw(&self, _port: u16) -> u16 {
        0
    }
    fn port_outd(&self, _port: u16, _val: u32) {}
    fn port_ind(&self, _port: u16) -> u32 {
        0
    }
}

#[cfg(not(target_arch = "x86_64"))]
impl Rtc for Bridge {
    fn rtc_read(&self, _out: &mut abi::wire::time::RtcSample) {}
}

#[cfg(not(target_arch = "x86_64"))]
impl Power for Bridge {
    fn shutdown(&self) -> ! {
        loop {}
    }
}

#[cfg(not(target_arch = "x86_64"))]
impl VmMapper for Bridge {
    fn map_new_user_page(&self, _virt_addr: u64, _flags: u64) -> Result<(), ()> {
        Err(())
    }
    fn map_user_mmio(&self, _virt_addr: u64, _phys_addr: u64, _flags: u64) -> Result<(), ()> {
        Err(())
    }
}
