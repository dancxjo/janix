use core::arch::asm;
use core::arch::x86_64::__cpuid;

static mut USE_XSAVE: bool = false;
static mut SAVE_SIZE: usize = 512;

/// Initialize SIMD (SSE/SSE2/AVX) on this CPU.
pub fn init_cpu() {
    unsafe {
        // 1. Detect Features
        let cpuid1 = __cpuid(1);
        let has_sse = (cpuid1.edx & (1 << 25)) != 0;
        let has_xsave = (cpuid1.ecx & (1 << 26)) != 0;
        let has_avx = (cpuid1.ecx & (1 << 28)) != 0;

        if !has_sse {
            return; // Should panic? For now simple return (system requirements usually enforce SSE2)
        }

        // 2. Enable SSE (CR0)
        let mut cr0: u64;
        asm!("mov {}, cr0", out(reg) cr0, options(nomem, nostack, preserves_flags));
        cr0 &= !(1 << 2); // Clear EM
        cr0 |= 1 << 1;    // Set MP
        asm!("mov cr0, {}", in(reg) cr0, options(nomem, nostack, preserves_flags));

        // 3. Enable SSE/XSAVE (CR4)
        let mut cr4: u64;
        asm!("mov {}, cr4", out(reg) cr4, options(nomem, nostack, preserves_flags));
        cr4 |= (1 << 9) | (1 << 10); // OSFXSR | OSXMMEXCPT
        
        if has_xsave && has_avx {
            cr4 |= 1 << 18; // OSXSAVE
        }
        asm!("mov cr4, {}", in(reg) cr4, options(nomem, nostack, preserves_flags));

        // 4. Enable AVX (XCR0)
        if has_xsave && has_avx {
            // XCR0 = SSE (bit 1) | AVX (bit 2) | x87 (bit 0 default)
            let xcr0_val: u64 = 1 | 2 | 4; 
            let xcr0_reg: u32 = 0;
            let eax = xcr0_val as u32;
            let edx = (xcr0_val >> 32) as u32;
            asm!("xsetbv", in("ecx") xcr0_reg, in("eax") eax, in("edx") edx, options(nomem, nostack, preserves_flags));
            
            USE_XSAVE = true;
            
            // Get Save Size
            let cpuid_d = __cpuid(0xD);
            SAVE_SIZE = cpuid_d.ebx as usize;
        }
    }
}

pub const STATE_LAYOUT: (usize, usize) = (1024, 64); // Oversized safe defaults

#[inline]
pub unsafe fn save(dst: *mut u8) {
    // Both are unsafe ops
    unsafe {
        if USE_XSAVE {
            let mask = 0xFFFFFFFFu32;
            asm!("xsave64 [{}]", in(reg) dst, in("eax") mask, in("edx") mask, options(nostack, preserves_flags));
        } else {
            asm!("fxsave64 [{}]", in(reg) dst, options(nostack, preserves_flags));
        }
    }
}

#[inline]
pub unsafe fn restore(src: *const u8) {
    unsafe {
        if USE_XSAVE {
            let mask = 0xFFFFFFFFu32;
            asm!("xrstor64 [{}]", in(reg) src, in("eax") mask, in("edx") mask, options(nostack, preserves_flags));
        } else {
            asm!("fxrstor64 [{}]", in(reg) src, options(nostack, preserves_flags));
        }
    }
}
