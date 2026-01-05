//! AArch64 SIMD (NEON/AdvSIMD) support.
//!
//! Enables NEON/floating-point by setting CPACR_EL1.FPEN bits.
//! Provides save/restore for FPCR/FPSR and Q0-Q31 registers.

use crate::log::{klog, Level};
use crate::machine::Simd;
use abi::cpu::{SimdFamily, SimdSavePolicy};
use alloc::vec::Vec;
use core::arch::asm;
use spin::Mutex;

/// NEON/FP state size: 32 x 128-bit registers + FPCR + FPSR
/// Q0-Q31 = 512 bytes, FPCR = 4 bytes, FPSR = 4 bytes, aligned to 16
const SIMD_STATE_SIZE: usize = 512 + 16;

struct SimdState {
    available: Vec<SimdFamily>,
    enabled: Vec<SimdFamily>,
    policy: SimdSavePolicy,
}

impl SimdState {
    const fn new() -> Self {
        Self {
            available: Vec::new(),
            enabled: Vec::new(),
            policy: SimdSavePolicy::None,
        }
    }
}

static STATE: Mutex<SimdState> = Mutex::new(SimdState::new());

pub struct AArch64Simd;

pub static AARCH64_SIMD: AArch64Simd = AArch64Simd;

impl Simd for AArch64Simd {
    fn enable(&self) -> bool {
        let mut state = STATE.lock();
        if state.policy != SimdSavePolicy::None {
            return true; // Already enabled
        }

        klog(Level::Info, "SIMD", "Enabling AArch64 NEON...");

        // NEON/AdvSIMD is mandatory on AArch64
        state.available.push(SimdFamily::ARM_NEON);

        // Enable NEON/FP by setting CPACR_EL1.FPEN = 0b11
        // Bits [21:20] = 0b11 means no traps for SIMD/FP at EL0 and EL1
        unsafe {
            let mut cpacr: u64;
            asm!("mrs {}, cpacr_el1", out(reg) cpacr, options(nomem, preserves_flags));
            cpacr |= 0b11 << 20; // FPEN = 0b11
            asm!("msr cpacr_el1, {}", in(reg) cpacr, options(nomem, preserves_flags));
            asm!("isb", options(nomem, preserves_flags));

            // Initialize FPCR and FPSR to sane defaults
            asm!("msr fpcr, xzr", options(nomem, preserves_flags));
            asm!("msr fpsr, xzr", options(nomem, preserves_flags));
        }

        state.enabled.push(SimdFamily::ARM_NEON);
        state.policy = SimdSavePolicy::Eager;
        
        klog(Level::Info, "SIMD", "NEON Enabled (Eager)");
        true
    }

    fn save(&self, buffer: &mut [u8]) {
        let state = STATE.lock();
        if state.policy == SimdSavePolicy::None {
            return;
        }

        if buffer.len() < SIMD_STATE_SIZE {
            return;
        }

        // Align buffer to 16 bytes
        let addr = buffer.as_mut_ptr() as usize;
        let align_offset = (16 - (addr % 16)) % 16;
        if buffer.len() < align_offset + SIMD_STATE_SIZE {
            return;
        }
        let ptr = unsafe { buffer.as_mut_ptr().add(align_offset) };

        unsafe {
            // Save Q0-Q31 (32 x 128-bit registers = 512 bytes)
            asm!(
                "stp q0, q1, [{ptr}]",
                "stp q2, q3, [{ptr}, #32]",
                "stp q4, q5, [{ptr}, #64]",
                "stp q6, q7, [{ptr}, #96]",
                "stp q8, q9, [{ptr}, #128]",
                "stp q10, q11, [{ptr}, #160]",
                "stp q12, q13, [{ptr}, #192]",
                "stp q14, q15, [{ptr}, #224]",
                "stp q16, q17, [{ptr}, #256]",
                "stp q18, q19, [{ptr}, #288]",
                "stp q20, q21, [{ptr}, #320]",
                "stp q22, q23, [{ptr}, #352]",
                "stp q24, q25, [{ptr}, #384]",
                "stp q26, q27, [{ptr}, #416]",
                "stp q28, q29, [{ptr}, #448]",
                "stp q30, q31, [{ptr}, #480]",
                ptr = in(reg) ptr,
                options(nostack, preserves_flags)
            );

            // Save FPCR and FPSR at offset 512
            let fpcr: u64;
            let fpsr: u64;
            asm!("mrs {}, fpcr", out(reg) fpcr, options(nomem, preserves_flags));
            asm!("mrs {}, fpsr", out(reg) fpsr, options(nomem, preserves_flags));
            let ctrl_ptr = ptr.add(512) as *mut u64;
            ctrl_ptr.write(fpcr);
            ctrl_ptr.add(1).write(fpsr);
        }
    }

    fn restore(&self, buffer: &[u8]) {
        let state = STATE.lock();
        if state.policy == SimdSavePolicy::None {
            return;
        }

        if buffer.len() < SIMD_STATE_SIZE {
            return;
        }

        // Align buffer to 16 bytes
        let addr = buffer.as_ptr() as usize;
        let align_offset = (16 - (addr % 16)) % 16;
        if buffer.len() < align_offset + SIMD_STATE_SIZE {
            return;
        }
        let ptr = unsafe { buffer.as_ptr().add(align_offset) };

        unsafe {
            // Restore FPCR and FPSR first
            let ctrl_ptr = ptr.add(512) as *const u64;
            let fpcr = ctrl_ptr.read();
            let fpsr = ctrl_ptr.add(1).read();
            asm!("msr fpcr, {}", in(reg) fpcr, options(nomem, preserves_flags));
            asm!("msr fpsr, {}", in(reg) fpsr, options(nomem, preserves_flags));

            // Restore Q0-Q31
            asm!(
                "ldp q0, q1, [{ptr}]",
                "ldp q2, q3, [{ptr}, #32]",
                "ldp q4, q5, [{ptr}, #64]",
                "ldp q6, q7, [{ptr}, #96]",
                "ldp q8, q9, [{ptr}, #128]",
                "ldp q10, q11, [{ptr}, #160]",
                "ldp q12, q13, [{ptr}, #192]",
                "ldp q14, q15, [{ptr}, #224]",
                "ldp q16, q17, [{ptr}, #256]",
                "ldp q18, q19, [{ptr}, #288]",
                "ldp q20, q21, [{ptr}, #320]",
                "ldp q22, q23, [{ptr}, #352]",
                "ldp q24, q25, [{ptr}, #384]",
                "ldp q26, q27, [{ptr}, #416]",
                "ldp q28, q29, [{ptr}, #448]",
                "ldp q30, q31, [{ptr}, #480]",
                ptr = in(reg) ptr,
                options(nostack, preserves_flags)
            );
        }
    }

    fn required_size(&self) -> usize {
        let state = STATE.lock();
        if state.policy == SimdSavePolicy::None {
            0
        } else {
            SIMD_STATE_SIZE + 16 // + alignment
        }
    }

    fn save_policy(&self) -> SimdSavePolicy {
        STATE.lock().policy
    }

    fn available_families(&self) -> Vec<SimdFamily> {
        STATE.lock().available.clone()
    }

    fn enabled_families(&self) -> Vec<SimdFamily> {
        STATE.lock().enabled.clone()
    }
}
