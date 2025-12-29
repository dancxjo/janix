use core::fmt;

pub struct PortWrites;

impl fmt::Write for PortWrites {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
                #[cfg(target_arch = "x86_64")]
                unsafe {
                    let mut status: u8;
                    loop {
                         core::arch::asm!(
                            "in al, dx",
                            out("al") status,
                            in("dx") 0x3FDu16,
                            options(nomem, nostack, preserves_flags)
                         );
                         if status & 0x20 != 0 { break; }
                    }
                    
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
