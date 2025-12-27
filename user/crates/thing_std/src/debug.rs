use core::fmt;

pub struct PortWrites;

impl fmt::Write for PortWrites {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            unsafe {
                core::arch::asm!(
                    "out dx, al",
                    in("dx") 0x3F8u16,
                    in("al") b,
                    options(nomem, nostack, preserves_flags)
                );
            }
        }
        Ok(())
    }
}

pub fn log(s: &str) {
    use core::fmt::Write;
    let _ = PortWrites.write_str(s);
}
