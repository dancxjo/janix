//! x86_64 architecture-specific implementation.

use core::arch::asm;
use kernel::BootRuntime;
use kernel::time::MonotonicClamp;
use core::sync::atomic::{AtomicU64, Ordering};

/// Serial port implementation for x86_64 using I/O port 0x3F8 (COM1).
pub struct SerialPort {
    clamp: MonotonicClamp,
    freq_hz: AtomicU64,
}

impl SerialPort {
    pub const fn new() -> Self {
        Self {
            clamp: MonotonicClamp::new(),
            freq_hz: AtomicU64::new(0),
        }
    }

    /// Calibrate TSC using the PIT (Programmable Interval Timer).
    /// This is a simplified calibration that runs once on the first call to mono_freq_hz.
    fn calibrate(&self) -> u64 {
        // Check if we already calibrated
        let cached = self.freq_hz.load(Ordering::Relaxed);
        if cached != 0 {
            return cached;
        }

        // basic calibration using PIT channel 2 (speaker) or channel 0 (system timer).
        // Let's use Channel 2 gate if possible, but actually we can just check 
        // if we can use Channel 0 for a simple wait.
        // Or simpler: We are in bootloader/kernel init. We can just spin-wait on a PIT counter.
        
        let freq = unsafe { calibrate_tsc_pit() };
        self.freq_hz.store(freq, Ordering::Relaxed);
        freq
    }
}

impl BootRuntime for SerialPort {
    fn putchar(&self, c: u8) {
        unsafe {
            // Wait for transmit empty
            while (inb(0x3F8 + 5) & 0x20) == 0 {}
            outb(0x3F8, c);
        }
    }

    fn halt(&self) -> ! {
        hcf()
    }

    fn mono_ticks(&self) -> u64 {
        let raw = unsafe { rdtsc() };
        self.clamp.clamp(raw)
    }

    fn mono_freq_hz(&self) -> u64 {
        self.calibrate()
    }
}

#[inline]
unsafe fn outb(port: u16, val: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack, preserves_flags));
    }
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    unsafe {
        asm!("in al, dx", out("al") ret, in("dx") port, options(nomem, nostack, preserves_flags));
    }
    ret
}

/// Halt and catch fire - enters an infinite halt loop.
pub fn hcf() -> ! {
    loop {
        unsafe { asm!("hlt") };
    }
}

#[inline]
unsafe fn rdtsc() -> u64 {
    let low: u32;
    let high: u32;
    unsafe {
        asm!("rdtsc", out("eax") low, out("edx") high, options(nomem, nostack));
    }
    ((high as u64) << 32) | (low as u64)
}

/// Calibrate TSC against PIT.
/// Returns frequency in Hz, or 0 if failed.
unsafe fn calibrate_tsc_pit() -> u64 {
    // 1. Setup PIT Channel 2 (0x42) to one-shot mode (Mode 0)
    // We want to wait for a known duration.
    // The PIT runs at 1.193182 MHz.
    // Let's measure for ~10ms.
    // 10ms = 11932 ticks.
    
    // Using Channel 0 might interfere with system timer if used by legacy OS,
    // but here we are bare metal boot.
    // Channel 2 is often used for PC Speaker, safe to mess with.
    
    // Control Word: Channel 2, Access Lo/Hi, Mode 0 (Interrupt on Terminal Count), Binary
    // 0b10_11_000_0 = 0xB0
    outb(0x43, 0xB0);
    
    // Reload value = 11932 (0x2E9C) for ~10ms
    let count = 11932u16;
    outb(0x42, (count & 0xFF) as u8);
    outb(0x42, (count >> 8) as u8);
    
    // Enable Channel 2 Gate (bit 0 of Port 0x61)
    let port61 = inb(0x61);
    outb(0x61, port61 | 0x01);
    
    let start_tsc = rdtsc();
    
    // Wait for output bit to go high (bit 5 of Port 0x61 checks Tmr 2 Out)
    // Or check if count reached 0? Mode 0 sets OUT high when count reaches 0.
    
    // WAIT a bit for the counter to actually load and start?
    // The count loads when written.
    
    // Spin until bit 5 of 0x61 becomes 1.
    // Timeout safety?
    let mut timeout = 100_000_000;
    while (inb(0x61) & 0x20) == 0 {
        core::hint::spin_loop();
        timeout -= 1;
        if timeout == 0 {
            return 0; // Failed
        }
    }
    
    let end_tsc = rdtsc();
    
    // Disable Gate just in case
    outb(0x61, port61 & !0x01);
    
    let delta = end_tsc.saturating_sub(start_tsc);
    
    // freq = delta * (1.193182 MHz / 11932) * 11932 / 10ms wait?
    // We waited for 11932 PIT ticks.
    // PIT freq = 1,193,182 Hz.
    // Time elapsed = 11932 / 1,193,182 = 0.01 seconds.
    // TSC frequency = delta / 0.01 = delta * 100.
    
    delta.saturating_mul(100)
}
