use crate::log::{klog, Level};
use crate::machine::Simd;
use abi::cpu::{SimdFamily, SimdSavePolicy};
use alloc::vec::Vec;
use core::arch::asm;
use core::arch::x86_64::__cpuid;
use spin::Mutex;
use x86_64::registers::control::{Cr0, Cr0Flags, Cr4, Cr4Flags};
use x86_64::registers::xcontrol::{XCr0, XCr0Flags};

struct SimdState {
    available: Vec<SimdFamily>,
    enabled: Vec<SimdFamily>,
    policy: SimdSavePolicy,
    use_xsave: bool,
    save_size: usize,
}

impl SimdState {
    const fn new() -> Self {
        Self {
            available: Vec::new(),
            enabled: Vec::new(),
            policy: SimdSavePolicy::None,
            use_xsave: false,
            save_size: 0,
        }
    }
}

static STATE: Mutex<SimdState> = Mutex::new(SimdState::new());

pub struct X86Simd;

pub static X86_SIMD: X86Simd = X86Simd;

impl Simd for X86Simd {
    fn enable(&self) -> bool {
        let mut state = STATE.lock();
        if state.policy != SimdSavePolicy::None {
            return true; // Already enabled
        }

        // 1. Detect
        klog(Level::Info, "SIMD", "Detecting x86 features...");
        let cpuid1 = unsafe { __cpuid(1) };
        let has_sse = (cpuid1.edx & (1 << 25)) != 0;
        let _has_sse2 = (cpuid1.edx & (1 << 26)) != 0;
        let has_xsave = (cpuid1.ecx & (1 << 26)) != 0;
        let has_avx = (cpuid1.ecx & (1 << 28)) != 0;

        state.available.clear();
        if has_sse {
            state.available.push(SimdFamily::X86_SSE);
        }
        if has_avx {
            state.available.push(SimdFamily::X86_AVX);
        }

        if !has_sse {
            return false;
        }

        // 2. Enable SSE (CR0/CR4)
        klog(Level::Info, "SIMD", "Enabling SSE...");
        unsafe {
            let mut cr0 = Cr0::read();
            // EM (Emulation) bit 2
            let em = Cr0Flags::from_bits_truncate(1 << 2);
            cr0.remove(em);
            cr0.insert(Cr0Flags::MONITOR_COPROCESSOR);
            Cr0::write(cr0);

            let mut cr4 = Cr4::read();
            cr4.insert(Cr4Flags::OSFXSR);
            cr4.insert(Cr4Flags::OSXMMEXCPT_ENABLE);

            if has_xsave && has_avx {
                cr4.insert(Cr4Flags::OSXSAVE);
            }
            Cr4::write(cr4);

            // 3. Enable AVX (XCR0)
            if has_xsave && has_avx {
                // XCR0 defaults to 1 (x87). We add SSE (bit 1) and AVX (bit 2).
                let mut new_xcr0 = XCr0::read();
                new_xcr0.insert(XCr0Flags::SSE);
                new_xcr0.insert(XCr0Flags::AVX);
                XCr0::write(new_xcr0);

                state.use_xsave = true;

                // Get XSAVE size
                let cpuid_d = __cpuid(0xD);
                state.save_size = cpuid_d.ebx as usize;

                state.enabled.push(SimdFamily::X86_SSE);
                state.enabled.push(SimdFamily::X86_AVX);
            } else {
                state.save_size = 512; // FXSAVE size
                state.enabled.push(SimdFamily::X86_SSE);
            }
        }

        state.policy = SimdSavePolicy::Eager;
        klog(Level::Info, "SIMD", "SIMD Enabled (Eager)");
        true
    }

    fn save(&self, buffer: &mut [u8]) {
        let state = STATE.lock();
        if state.policy == SimdSavePolicy::None {
            return;
        }

        let addr = buffer.as_mut_ptr() as usize;
        let align_offset = if state.use_xsave {
            (64 - (addr % 64)) % 64
        } else {
            (16 - (addr % 16)) % 16
        };

        if buffer.len() < align_offset + state.save_size {
            return;
        }
        let ptr = unsafe { buffer.as_mut_ptr().add(align_offset) };

        unsafe {
            if state.use_xsave {
                let mask_lo = 0xFFFFFFFFu32;
                let mask_hi = 0xFFFFFFFFu32;
                asm!("xsave64 [{}]", in(reg) ptr, in("eax") mask_lo, in("edx") mask_hi, options(nostack, preserves_flags));
            } else {
                asm!("fxsave64 [{}]", in(reg) ptr, options(nostack, preserves_flags));
            }
        }
    }

    fn restore(&self, buffer: &[u8]) {
        let state = STATE.lock();
        if state.policy == SimdSavePolicy::None {
            return;
        }

        let addr = buffer.as_ptr() as usize;
        let align_offset = if state.use_xsave {
            (64 - (addr % 64)) % 64
        } else {
            (16 - (addr % 16)) % 16
        };

        if buffer.len() < align_offset + state.save_size {
            return;
        }
        let ptr = unsafe { buffer.as_ptr().add(align_offset) };

        unsafe {
            if state.use_xsave {
                let mask_lo = 0xFFFFFFFFu32;
                let mask_hi = 0xFFFFFFFFu32;
                asm!("xrstor64 [{}]", in(reg) ptr, in("eax") mask_lo, in("edx") mask_hi, options(nostack, preserves_flags));
            } else {
                asm!("fxrstor64 [{}]", in(reg) ptr, options(nostack, preserves_flags));
            }
        }
    }

    fn required_size(&self) -> usize {
        let state = STATE.lock();
        if state.save_size == 0 {
            0
        } else {
            state.save_size + 64
        } // + alignment
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
