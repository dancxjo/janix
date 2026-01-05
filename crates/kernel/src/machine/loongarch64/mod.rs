pub mod abi;
pub mod context;
pub mod mmu;
mod serial;
pub mod timer;
pub mod trap;
pub use mmu::AddressSpace;

pub type TrapFrame = trap::TrapContext; // Added

use core::arch::global_asm;
global_asm!(include_str!("vectors.S"));

use crate::machine::{BootColor, Context, Machine, MmioFlags, MmioMapping, MmioRange};

use core::sync::atomic::{AtomicU64, Ordering};
pub(crate) static HHDM_OFFSET: AtomicU64 = AtomicU64::new(0);

struct LoongArchMachine {
    pub(crate) kernel_phys_base: AtomicU64,
    pub(crate) kernel_virt_base: AtomicU64,
}

static ARCH_MACHINE_IMPL: LoongArchMachine = LoongArchMachine {
    kernel_phys_base: AtomicU64::new(0),
    kernel_virt_base: AtomicU64::new(0),
};

pub static ARCH_MACHINE: &'static dyn Machine = &ARCH_MACHINE_IMPL;

/// Per-CPU kernel stack top for trap entry.
#[no_mangle]
pub static KERNEL_STACK_TOP: AtomicU64 = AtomicU64::new(0);

impl Machine for LoongArchMachine {
    fn init(&self, info: crate::machine::PreBootInfo) {
        HHDM_OFFSET.store(info.hhdm_offset, Ordering::Relaxed);
        self.kernel_phys_base
            .store(info.kernel_phys_base, Ordering::Relaxed);
        self.kernel_virt_base
            .store(info.kernel_virt_base, Ordering::Relaxed);

        // Capture kernel page tables before any address space switching
        mmu::init();

        // Install trap vector
        extern "C" {
            static loongarch64_trap_vector: u8; // Symbol
        }
        unsafe {
            let vector_addr = core::ptr::addr_of!(loongarch64_trap_vector) as u64;
            // Set exception entry base (EENTRY)
            core::arch::asm!("csrwr {}, 0xc", in(reg) vector_addr);

            // Initialize KS0 to 0 (indicates kernel mode)
            core::arch::asm!("csrwr $r0, 0x30");

            timer::init();
        }
    }

    fn console_write(&self, bytes: &[u8]) -> usize {
        serial::Serial::new().write(bytes);
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

        let hhdm = HHDM_OFFSET.load(Ordering::Relaxed);
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

    fn monotonic_now(&self) -> u64 {
        let time: u64;
        unsafe {
            core::arch::asm!("rdtime.d {}, $r0", out(reg) time);
        }
        // Assume 100MHz for now
        time * 10
    }

    fn irq_disable(&self) -> u64 {
        let crmd: u64;
        unsafe {
            // Read CRMD (Current Request Mode Definition) - CSR 0x0
            core::arch::asm!("csrrd {}, 0x0", out(reg) crmd);
            // Clear IE (Interrupt Enable) - bit 2
            let new_crmd = crmd & !0x4;
            core::arch::asm!("csrwr {}, 0x0", in(reg) new_crmd);
        }
        crmd
    }

    fn irq_restore(&self, token: u64) {
        let ie = token & 0x4;
        unsafe {
            let mut current: u64;
            core::arch::asm!("csrrd {}, 0x0", out(reg) current);
            if ie != 0 {
                current |= 0x4;
            } else {
                current &= !0x4;
            }
            core::arch::asm!("csrwr {}, 0x0", in(reg) current);
        }
    }

    fn irq_enable(&self) {
        unsafe {
            let mut val = 4u64;
            core::arch::asm!("csrxchg {}, {}, 0x0", inout(reg) val, in(reg) val);
            let _ = val;
        }
    }

    fn halt(&self) -> ! {
        loop {
            unsafe {
                core::arch::asm!("idle 0");
            }
        }
    }

    fn idle(&self) {
        unsafe {
            core::arch::asm!("idle 0");
        }
    }

    fn switch_to(&self, old_ctx: &mut Context, new_ctx: &Context) {
        extern "C" {
            fn loongarch64_switch_to(old_sp: &mut u64, new_sp: u64);
        }
        unsafe {
            loongarch64_switch_to(&mut old_ctx.sp, new_ctx.sp);
        }
    }

    fn task_entry_stub(&self) -> u64 {
        extern "C" {
            fn loongarch64_task_entry_stub();
        }
        loongarch64_task_entry_stub as *const () as usize as u64
    }

    fn virt_to_phys(&self, virt: u64) -> u64 {
        let hhdm = HHDM_OFFSET.load(Ordering::Relaxed);
        if virt >= hhdm {
            virt - hhdm
        } else {
            let kernel_virt = self.kernel_virt_base.load(Ordering::Relaxed);
            let kernel_phys = self.kernel_phys_base.load(Ordering::Relaxed);
            if virt >= kernel_virt {
                virt - kernel_virt + kernel_phys
            } else {
                virt
            }
        }
    }

    fn set_kernel_stack(&self, top: u64) {
        KERNEL_STACK_TOP.store(top, Ordering::Release);
    }
}
