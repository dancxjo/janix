pub mod apic;
pub mod enter;
pub mod fpu;
pub mod paging;
pub mod pic;
pub mod pit;
pub mod rtc;
pub mod syscall;
pub mod time;
pub mod trap;

use super::{Arch, UserEntryRegs};

pub struct X86Arch;

impl Arch for X86Arch {
    fn enter_user_mode(regs: &UserEntryRegs) -> ! {
        enter::enter_user_mode(regs)
    }

    fn resume_user_mode(context: &[u64], fpu_context: &kernel::sched::FpuContext) -> ! {
        enter::resume_user_mode(context, fpu_context)
    }

    fn install_syscall_handler() {
        unsafe {
            use x86_64::registers::control::{Cr0, Cr0Flags, Cr4, Cr4Flags};
            let mut cr0 = Cr0::read();
            cr0.remove(Cr0Flags::from_bits_truncate(1 << 2));
            cr0.insert(Cr0Flags::MONITOR_COPROCESSOR);
            Cr0::write(cr0);

            let mut cr4 = Cr4::read();
            cr4.insert(Cr4Flags::OSFXSR);
            cr4.insert(Cr4Flags::OSXMMEXCPT_ENABLE);
            cr4.insert(Cr4Flags::PAGE_GLOBAL); 
            Cr4::write(cr4);

            // Configure PAT:
            // Index 0: WB (default)
            // Index 1: WT (default)
            // Index 2: UC- (default)
            // Index 3: UC (default)
            // Index 4: WC (We set this)
            // Index 5: WT (default)
            // Index 6: UC- (default)
            // Index 7: UC (default)
            // MSR 0x277 (IA32_PAT)
            // Encodings:
            // 0: UC, 1: WC, 4: WT, 5: WP, 6: WB, 7: UC-
            // We want indices:
            // 0 (P0=0,P1=0): WB (6)
            // 1 (P0=1,P1=0): WT (4)
            // 2 (P0=0,P1=1): UC- (7)
            // 3 (P0=1,P1=1): UC (0)
            // 4 (PAT=1,P0=0,P1=0): WC (1)
            // 5..7: Defaults (WT, UC-, UC)
            
            const PAT_UC: u64 = 0x00;
            const PAT_WC: u64 = 0x01;
            const PAT_WT: u64 = 0x04;
            const PAT_WB: u64 = 0x06;
            const PAT_UCM: u64 = 0x07;

            let pat_val = 
                (PAT_WB << 0) |
                (PAT_WC << 8) | // Index 1: Re-purposed to WC (was WT)
                (PAT_UCM<< 16)|
                (PAT_UC << 24)|
                (PAT_WB << 32)| // Index 4: WB
                (PAT_WC << 40)| // Index 5: WC (was WT)
                (PAT_UCM<< 48)|
                (PAT_UC << 56);
            
            use x86_64::registers::model_specific::Msr;
            let mut pat_msr = Msr::new(0x277);
            unsafe { pat_msr.write(pat_val); }
        }
        syscall::install_handler();
        time::init_arch_timer();
        rtc::init_arch_rtc();
    }

    fn activate_user_address_space(token: Option<u64>) {
        enter::activate_address_space(token);
    }
}

pub use enter::{alloc_user_stack, init_user_stack, resume_user_mode};
