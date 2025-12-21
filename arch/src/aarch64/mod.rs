pub mod dtb;
pub mod enter;
pub mod paging;
pub mod rtc;
pub mod time;
pub mod trap;

use super::{Arch, UserEntryRegs};

pub struct AArch64Arch;

impl Arch for AArch64Arch {
    fn enter_user_mode(regs: &UserEntryRegs) -> ! {
        enter::enter_user_mode(regs)
    }

    fn resume_user_mode(context: &[u64], fpu_context: &kernel::sched::FpuContext) -> ! {
        enter::resume_user_mode(context, fpu_context)
    }

    fn install_syscall_handler() {
        trap::init();
        time::init_arch_timer();
        rtc::init_arch_rtc();
    }

    fn activate_user_address_space(token: Option<u64>) {
        enter::activate_address_space(token);
    }
}

pub use enter::{alloc_user_stack, init_user_stack};

pub fn map_boot_device_regions() -> bool {
    let hhdm = kernel::memory::get_hhdm_offset();
    unsafe {
        // Generic PCI/MMIO? Copied from legacy platform.rs
        paging::map_device_region(0x3f000000, 0x01000000, hhdm);
        paging::map_device_region(0x10000000, 0x2effffff, hhdm);
        // PL011 UART
        paging::map_device_region(0x09000000, 0x1000, hhdm);
    }
    true
}
