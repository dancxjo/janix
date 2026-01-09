use core::arch::asm;

// Legacy SBI Console Putchar Extension ID
const SBI_EID_console_putchar: usize = 1;

pub struct SerialPort;

impl SerialPort {
    pub const fn new() -> Self {
        Self
    }

    pub fn putchar(&self, c: u8) {
        unsafe {
            // Legacy SBI: EID=1, a0=char
            asm!(
                "ecall",
                in("a7") SBI_EID_console_putchar,
                in("a0") c as usize,
                options(nostack, preserves_flags)
            );
        }
    }
}
