#![no_std]
#![allow(clippy::missing_safety_doc)]
#![feature(abi_x86_interrupt)]

extern crate alloc;

pub mod gdt;
pub mod interrupts;
pub mod user;

#[cfg(target_arch = "x86_64")]
use core::arch::asm;
use kernel::bridge::HardwareBridge;

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct ArchContext(pub [u64; 34]);

impl Default for ArchContext {
    fn default() -> Self {
        Self([0; 34])
    }
}

pub struct Bridge;

// Hook for scheduler. Only set by kernel binary.
pub static mut TICK_HOOK: Option<fn(&mut interrupts::trap::TrapFrame)> = None;

pub fn set_tick_hook(hook: fn(&mut interrupts::trap::TrapFrame)) {
    unsafe {
        TICK_HOOK = Some(hook);
    }
}

// Hook for page faults. Returns true if handled.
// args: (stack_frame, fault_addr, error_code)
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

use core::sync::atomic::{AtomicU64, Ordering};

pub static HHDM_OFFSET: AtomicU64 = AtomicU64::new(0);

#[cfg(target_arch = "x86_64")]
impl Bridge {
    pub unsafe fn init(_rsdp_addr: Option<u64>, hhdm: u64) {
        HHDM_OFFSET.store(hhdm, Ordering::Relaxed);
        use kernel::bridge::HardwareBridge;
        let b = Bridge;

        // Initialize Serial first to prevent logging hangs
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

        // Serial was already initialized above

        // ACPI init moved to explicit call
    }

    pub unsafe fn init_acpi(rsdp_addr: u64, hhdm: u64) {
        use kernel::bridge::HardwareBridge;
        let b = Bridge;
        b.log("BRIDGE: acpi::init\n");
        kernel::platform::acpi::init(&b, rsdp_addr, hhdm);
    }
}

pub fn print_u64(val: u64) {
    let bridge = Bridge;
    use kernel::bridge::HardwareBridge;

    if val == 0 {
        bridge.log("0");
        return;
    }

    let mut buffer = [0u8; 20];
    let mut i = 0;
    let mut n = val;

    while n > 0 {
        buffer[i] = (n % 10) as u8 + b'0';
        n /= 10;
        i += 1;
    }

    while i > 0 {
        i -= 1;
        bridge.log(core::str::from_utf8(&[buffer[i]]).unwrap());
    }
}

pub fn print_hex(val: u64) {
    let bridge = Bridge;
    use kernel::bridge::HardwareBridge;
    bridge.log("0x");
    let mut printed = false;
    for i in (0..16).rev() {
        let digit = (val >> (i * 4)) & 0xF;
        if digit != 0 || printed || i == 0 {
            let c = if digit < 10 {
                digit as u8 + b'0'
            } else {
                digit as u8 - 10 + b'a'
            };
            bridge.log(core::str::from_utf8(&[c]).unwrap());
            printed = true;
        }
    }
}

#[cfg(target_arch = "x86_64")]
impl HardwareBridge for Bridge {
    type Context = ArchContext;

    fn log(&self, msg: &str) {
        unsafe {
            for b in msg.bytes() {
                // 0xE9 Debugcon (Fire and Forget)
                asm!("out dx, al", in("dx") 0xE9u16, in("al") b);

                // Wait for Transmit Holding Register Empty (Bit 5)
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

    fn hhdm_offset(&self) -> u64 {
        HHDM_OFFSET.load(Ordering::Relaxed)
    }

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

    fn ticks(&self) -> u64 {
        let eax: u32;
        let edx: u32;
        unsafe {
            asm!("rdtsc", out("eax") eax, out("edx") edx, options(nomem, nostack));
        }
        ((edx as u64) << 32) | (eax as u64)
    }

    fn system_now(&self) -> u64 {
        0
    }

    fn monotonic_now(&self) -> u64 {
        kernel::drivers::hpet::read_ns()
    }

    fn idle(&self) {
        unsafe {
            asm!("hlt");
        }
    }

    fn shutdown(&self) -> ! {
        unsafe {
            // QEMU ISA debug exit
            asm!("out dx, ax", in("dx") 0x604u16, in("ax") 0x2000u16);
            loop {
                asm!("hlt");
            }
        }
    }

    fn irq_disable(&self) {
        unsafe {
            asm!("cli");
        }
    }

    fn irq_enable(&self) {
        unsafe {
            asm!("sti");
        }
    }

    fn init_thread_context(&self, entry: u64, stack: u64, arg: u64) -> Self::Context {
        // [r15...rax, rip, cs, rflags, rsp, ss]
        // ...
        let mut ctx = [0u64; 34];
        // RDI = arg
        ctx[13] = arg;

        // RIP
        ctx[15] = entry;

        // If entry is in higher half, use Kernel Segments. Else User.
        let is_kernel = entry >= 0xFFFF_8000_0000_0000;

        if is_kernel {
            // CS: Kernel Code
            ctx[16] = unsafe { gdt::KERNEL_CODE_SELECTOR.0 as u64 };
            // RFLAGS: Interrupts enabled (0x200). IOPL 0.
            ctx[17] = 0x202;
            // RSP
            ctx[18] = stack;
            // SS: Kernel Data
            ctx[19] = unsafe { gdt::KERNEL_DATA_SELECTOR.0 as u64 };
        } else {
            // CS: User Code (RPL 3)
            ctx[16] = unsafe { gdt::USER_CODE_SELECTOR.0 as u64 | 3 };
            // RFLAGS: Interrupts enabled (0x200). IOPL 3 (0x3000) -> 0x3202
            ctx[17] = 0x3202;
            // RSP
            ctx[18] = stack;
            // SS: User Data (RPL 3)
            ctx[19] = unsafe { gdt::USER_DATA_SELECTOR.0 as u64 | 3 };
        }

        ArchContext(ctx)
    }

    fn resume_user_mode(&self, context: &Self::Context) -> ! {
        crate::user::enter::resume_user_mode(&context.0, &kernel::sched::fpu::FpuContext::default())
    }

    fn set_kernel_stack(&self, stack_top: u64) {
        unsafe {
            gdt::set_kernel_stack(stack_top);
            interrupts::syscall::set_kernel_stack(stack_top);
        }
    }

    fn rtc_read(&self, out: &mut abi::wire::time::RtcSample) {
        unsafe {
            // Helper to read CMOS register
            let read_reg = |reg: u8| -> u8 {
                asm!("out dx, al", in("dx") 0x70u16, in("al") reg);
                let val: u8;
                asm!("in al, dx", out("al") val, in("dx") 0x71u16);
                val
            };

            // Wait for update in progress (Register A, bit 7)
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

            // BCD conversion (if Bit 2 of Reg B is 0)
            if (reg_b & 0x04) == 0 {
                sec = (sec & 0x0F) + ((sec / 16) * 10);
                min = (min & 0x0F) + ((min / 16) * 10);
                hour = ((hour & 0x0F) + ((hour & 0x70) / 16 * 10)) | (hour & 0x80);
                day = (day & 0x0F) + ((day / 16) * 10);
                mon = (mon & 0x0F) + ((mon / 16) * 10);
                year = (year & 0x0F) as u16 + ((year / 16) as u16 * 10);
            }

            // 12-hour format (if Bit 1 of Reg B is 0)
            if (reg_b & 0x02) == 0 && (hour & 0x80) != 0 {
                hour = ((hour & 0x7F) + 12) % 24;
            }

            // Century guessing (2000-2099)
            // Real OS reads ACPI FADT century byte, we'll just assume 20xx
            out.year = 2000 + year;
            out.mon = mon;
            out.day = day;
            out.hour = hour;
            out.min = min;
            out.sec = sec;
        }
    }

    fn map_new_user_page(&self, virt_addr: u64, flags: u64) -> Result<(), ()> {
        use core::alloc::Layout;
        use x86_64::structures::paging::{
            mapper::Mapper, FrameAllocator, OffsetPageTable, Page, PageTableFlags, PhysFrame,
            Size4KiB, Translate,
        };
        use x86_64::VirtAddr;

        // 1. Allocate a page from Kernel Heap (physically backed)
        let layout = unsafe { Layout::from_size_align_unchecked(4096, 4096) };
        let ptr = unsafe { alloc::alloc::alloc_zeroed(layout) };
        if ptr.is_null() {
            return Err(());
        }

        let kernel_virt = VirtAddr::new(ptr as u64);

        // 2. Get Physical Address of this kernel page
        // We need an active mapper.
        unsafe {
            let hhdm = HHDM_OFFSET.load(Ordering::Relaxed);
            let (l4_frame, _) = x86_64::registers::control::Cr3::read();
            let phys_l4 = l4_frame.start_address();
            let virt_l4 = VirtAddr::new(hhdm + phys_l4.as_u64());
            let page_table_ptr = virt_l4.as_mut_ptr();
            let mut mapper = OffsetPageTable::new(&mut *page_table_ptr, VirtAddr::new(hhdm));

            // Translate kernel virt to phys
            let phys_frame = match mapper.translate_addr(kernel_virt) {
                Some(p) => PhysFrame::<Size4KiB>::containing_address(p),
                None => {
                    alloc::alloc::dealloc(ptr, layout);
                    return Err(());
                }
            };

            // 3. Map to User Virtual Address
            let user_page = Page::<Size4KiB>::containing_address(VirtAddr::new(virt_addr));
            let map_flags = PageTableFlags::from_bits_truncate(flags)
                | PageTableFlags::PRESENT
                | PageTableFlags::USER_ACCESSIBLE;

            // We need a FrameAllocator for page tables.
            // We can reuse the kernel heap allocator logic ad-hoc?
            // "OffsetPageTable requires a FrameAllocator to map new pages (create tables)".
            // Struct HeapAllocator wrapper.
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
                        // Translate:
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

    fn save_fpu(&self, area: &mut [u8; 512]) {
        unsafe {
            // fxsave [rax]
            let ptr = area.as_mut_ptr();
            asm!("fxsave [{}]", in(reg) ptr);
        }
    }

    fn restore_fpu(&self, area: &[u8; 512]) {
        unsafe {
            // fxrstor [rax]
            let ptr = area.as_ptr();
            asm!("fxrstor [{}]", in(reg) ptr);
        }
    }
}

#[cfg(not(target_arch = "x86_64"))]
impl HardwareBridge for Bridge {
    type Context = ArchContext;

    fn log(&self, _msg: &str) {}
    fn hhdm_offset(&self) -> u64 {
        0
    }
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

    fn ticks(&self) -> u64 {
        0
    }
    fn monotonic_now(&self) -> u64 {
        0
    }
    fn system_now(&self) -> u64 {
        0
    }
    fn idle(&self) {}
    fn shutdown(&self) -> ! {
        loop {}
    }
    fn irq_disable(&self) {}
    fn irq_enable(&self) {}
    fn init_thread_context(&self, _entry: u64, _stack: u64, _arg: u64) -> Self::Context {
        ArchContext([0; 34])
    }
    fn resume_user_mode(&self, _context: &Self::Context) -> ! {
        loop {}
    }
    fn set_kernel_stack(&self, _: u64) {}
    fn rtc_read(&self, _out: &mut abi::wire::time::RtcSample) {}
    fn save_fpu(&self, _area: &mut [u8; 512]) {}
    fn restore_fpu(&self, _area: &[u8; 512]) {}

    fn map_new_user_page(&self, _virt_addr: u64, _flags: u64) -> Result<(), ()> {
        Err(())
    }

    fn map_user_mmio(&self, _virt_addr: u64, _phys_addr: u64, _flags: u64) -> Result<(), ()> {
        Err(())
    }
}
