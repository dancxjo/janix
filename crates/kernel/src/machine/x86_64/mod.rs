//! x86_64 architecture implementation
global_asm!(include_str!("interrupts.S"));

use self::serial::Serial;
use crate::machine::{BootColor, Context, Machine, MmioFlags, MmioMapping, MmioRange, PreBootInfo};
use crate::sched;
use core::arch::asm;
use core::arch::global_asm;

pub mod abi;
pub mod apic;
pub mod context;
pub mod gdt;
pub mod idt;
pub mod mmu;
pub mod pci;
pub mod percpu;
pub mod ps2_keyboard;
pub mod ps2_mouse;
pub mod serial;
pub mod simd;
pub mod timer;
pub use mmu::AddressSpace;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TrapFrame {
    // Pushed by us
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,

    // Pushed by CPU
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

#[no_mangle]
pub extern "C" fn sched_tick_asm_helper(sp: u64) -> u64 {
    sched::tick(sp)
}

#[no_mangle]
pub extern "C" fn timer_ack_asm_helper() {
    unsafe {
        timer::ack();
    }
}

#[no_mangle]
pub extern "C" fn task_dispatch(dispatch_ptr: u64, entry: extern "C" fn(u64) -> !) {
    entry(dispatch_ptr);
}

#[no_mangle]
pub extern "C" fn keyboard_handler_asm_helper() {
    unsafe {
        ps2_keyboard::irq_handler();
        timer::ack(); // Send EOI (now goes to LAPIC)
    }
}

#[no_mangle]
pub extern "C" fn mouse_handler_asm_helper() {
    unsafe {
        ps2_mouse::irq_handler();
        ps2_mouse::process_packets(); // Decode mouse bytes into pointer movement
        timer::ack(); // Send EOI to LAPIC (via IO-APIC routing)
    }
}

// -----------------------------------------------------------------------------
// Machine Implementation
// -----------------------------------------------------------------------------

pub struct ArchMachine {
    serial: Serial,
    hhdm_offset: core::sync::atomic::AtomicU64,
    kernel_phys_base: core::sync::atomic::AtomicU64,
    kernel_virt_base: core::sync::atomic::AtomicU64,
}

static ARCH_MACHINE_IMPL: ArchMachine = ArchMachine::new();
pub static ARCH_MACHINE: &'static dyn Machine = &ARCH_MACHINE_IMPL;

impl ArchMachine {
    pub const fn new() -> Self {
        Self {
            serial: Serial::new(),
            hhdm_offset: core::sync::atomic::AtomicU64::new(0),
            kernel_phys_base: core::sync::atomic::AtomicU64::new(0),
            kernel_virt_base: core::sync::atomic::AtomicU64::new(0),
        }
    }
}

// BSP PerCore structures
pub static mut PERCPU_BSP: self::percpu::x86PerCpu = self::percpu::x86PerCpu::new(0, 0);
pub static mut BSP_GDT: self::gdt::GdtTss = self::gdt::GdtTss::new();

/// Early init (GDT, IDT, etc) - called before LAPIC
fn init_early() {
    unsafe {
        use percpu::init_gs_base;

        // 1. GDT/TSS (Reloads Segments, clearing GS Base)
        gdt::init(&mut *(&raw mut BSP_GDT));

        // 2. PerCpu (Sets GS Base)
        init_gs_base(&mut *(&raw mut PERCPU_BSP));

        // 3. IDT
        idt::init();

        // 4. Timer (disables legacy PIC)
        timer::init();

        // 5. Syscall
        syscall_init();

        // 6. Keyboard (Arch specific init)
        crate::machine::input::init();
    }
}

/// Late init (LAPIC) - requires HHDM to be known
fn init_lapic(hhdm_offset: u64) {
    unsafe {
        timer::init_lapic(hhdm_offset);
    }
}

extern "C" {
    fn x86_switch_context(old_ctx: *mut Context, new_ctx: *const Context);
    fn task_entry();
    pub fn user_mode_trampoline();
}

core::arch::global_asm!(
    ".global x86_switch_context",
    "x86_switch_context:",
    "push rbx",
    "push rbp",
    "push r12",
    "push r13",
    "push r14",
    "push r15",
    "mov [rdi], rsp",
    "mov rsp, [rsi]",
    "pop r15",
    "pop r14",
    "pop r13",
    "pop r12",
    "pop rbp",
    "pop rbx",
    "ret",
    ".global task_entry",
    "task_entry:",
    "pop rdi", // dispatch_ptr
    "pop rsi", // entry_point
    "call task_dispatch",
    "1: hlt",
    "jmp 1b"
);

impl Machine for ArchMachine {
    fn init(&self, info: PreBootInfo) {
        self.hhdm_offset
            .store(info.hhdm_offset, core::sync::atomic::Ordering::Relaxed);
        self.kernel_phys_base
            .store(info.kernel_phys_base, core::sync::atomic::Ordering::Relaxed);
        self.kernel_virt_base
            .store(info.kernel_virt_base, core::sync::atomic::Ordering::Relaxed);

        // Phase 1: Early init (GDT, IDT, disable PIC)
        init_early();
    }

    fn console_write(&self, bytes: &[u8]) -> usize {
        self.serial.write(bytes);
        bytes.len()
    }

    fn set_boot_color(&self, fb: &crate::boot::FramebufferInfo, color: BootColor) {
        let bytes_per_pixel = (fb.bpp / 8) as usize;
        if bytes_per_pixel < 4 || fb.width == 0 || fb.height == 0 || fb.pitch == 0 {
            return;
        }
        let min_pitch = fb.width as usize * bytes_per_pixel;
        if (fb.pitch as usize) < min_pitch {
            return;
        }

        let pack = |component: u8, size: u8, shift: u8| -> u32 {
            if size == 0 || size > 24 {
                return 0;
            }
            let mask = (1u32 << size) - 1;
            let scaled = (component as u32 * mask + 127) / 255;
            scaled << shift
        };

        let pixel = pack(color.red, fb.red_mask_size, fb.red_mask_shift)
            | pack(color.green, fb.green_mask_size, fb.green_mask_shift)
            | pack(color.blue, fb.blue_mask_size, fb.blue_mask_shift);

        let hhdm = self
            .hhdm_offset
            .load(core::sync::atomic::Ordering::Relaxed);
        let fb_base = fb.addr + hhdm;

        let width = fb.width as usize;
        let height = fb.height as usize;
        let pitch = fb.pitch as usize;
        let row_len_bytes = width * bytes_per_pixel;

        unsafe {
            if pitch == row_len_bytes {
                let buf = core::slice::from_raw_parts_mut(fb_base as *mut u32, width * height);
                buf.fill(pixel);
            } else {
                for row in 0..height {
                    let row_ptr = (fb_base + (row * pitch) as u64) as *mut u32;
                    let row_buf = core::slice::from_raw_parts_mut(row_ptr, width);
                    row_buf.fill(pixel);
                }
            }
        }
    }

    fn mmio_map(&self, _range: MmioRange, _flags: MmioFlags) -> Option<MmioMapping> {
        None
    }

    fn port_read(&self, port: u16, size: u8) -> u32 {
        use x86_64::instructions::port::Port;
        unsafe {
            match size {
                1 => u32::from(Port::<u8>::new(port).read()),
                2 => u32::from(Port::<u16>::new(port).read()),
                4 => Port::<u32>::new(port).read(),
                _ => 0,
            }
        }
    }

    fn port_write(&self, port: u16, val: u32, size: u8) {
        use x86_64::instructions::port::Port;
        unsafe {
            match size {
                1 => Port::<u8>::new(port).write(val as u8),
                2 => Port::<u16>::new(port).write(val as u16),
                4 => Port::<u32>::new(port).write(val),
                _ => {}
            }
        }
    }

    fn monotonic_now(&self) -> u64 {
        let tsc = unsafe { core::arch::x86_64::_rdtsc() };
        // Assume 2GHz for now (1 cycle = 0.5ns)
        tsc / 2
    }

    fn irq_disable(&self) -> u64 {
        let flags: u64;
        unsafe {
            asm!("pushfq; pop {}; cli", out(reg) flags, options(nomem, preserves_flags));
        }
        flags
    }

    fn irq_restore(&self, token: u64) {
        unsafe {
            if token & 0x200 != 0 {
                asm!("sti", options(nomem, preserves_flags));
            }
        }
    }

    fn halt(&self) -> ! {
        loop {
            unsafe {
                asm!("cli; hlt");
            }
        }
    }

    fn idle(&self) {
        unsafe {
            core::arch::asm!("sti; hlt");
        }
    }

    fn switch_to(&self, old_ctx: &mut Context, new_ctx: &Context) {
        unsafe {
            x86_switch_context(old_ctx, new_ctx);
        }
    }

    fn task_entry_stub(&self) -> u64 {
        task_entry as *const () as u64
    }

    fn set_kernel_stack(&self, top: u64) {
        // Update per-cpu exception_stack_ptr (offset 72)
        // This is used by syscall_entry in interrupts.S
        unsafe {
            core::arch::asm!("mov gs:[72], {}", in(reg) top);
            gdt::set_tss_rsp0(top);
        }
    }

    fn simd(&self) -> &'static dyn crate::machine::Simd {
        &simd::X86_SIMD
    }

    fn virt_to_phys(&self, virt: u64) -> u64 {
        let hhdm = self.hhdm_offset.load(core::sync::atomic::Ordering::Relaxed);
        let k_virt = self
            .kernel_virt_base
            .load(core::sync::atomic::Ordering::Relaxed);
        let k_phys = self
            .kernel_phys_base
            .load(core::sync::atomic::Ordering::Relaxed);

        if virt >= k_virt && k_virt != 0 {
            virt - k_virt + k_phys
        } else if virt >= hhdm && hhdm != 0 {
            virt - hhdm
        } else {
            virt
        }
    }

    // Timer/CPU info for platform layer
    fn timer_frequency_hz(&self) -> u32 {
        timer::timer_frequency_hz()
    }

    fn local_cpu_id(&self) -> u32 {
        timer::lapic_id()
    }
}

pub fn syscall_init() {
    use x86_64::registers::model_specific::{Efer, EferFlags, LStar, SFMask};
    use x86_64::registers::rflags::RFlags;

    extern "C" {
        fn syscall_entry();
    }

    // Enable syscall extension
    unsafe {
        let mut efer = Efer::read();
        efer |= EferFlags::SYSTEM_CALL_EXTENSIONS;
        Efer::write(efer);

        let handler_addr = syscall_entry as *const () as u64;
        LStar::write(x86_64::VirtAddr::new(handler_addr));

        let kernel_base = 0x0008u64;
        let user_base = 0x0018u64;
        let star_val = (user_base << 48) | (kernel_base << 32);

        core::arch::asm!(
            "wrmsr",
            in("ecx") 0xC0000081u32,
            in("eax") (star_val & 0xFFFFFFFF) as u32,
            in("edx") (star_val >> 32) as u32,
            options(nostack)
        );

        SFMask::write(RFlags::INTERRUPT_FLAG | RFlags::TRAP_FLAG | RFlags::DIRECTION_FLAG);
    }
}
