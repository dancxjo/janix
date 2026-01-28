#[cfg(all(not(test), any(target_os = "none", target_os = "thingos")))]
use crate::syscall::{debug_write, exit};
#[cfg(all(not(test), any(target_os = "none", target_os = "thingos")))]
use core::panic::PanicInfo;

#[cfg(all(not(test), any(target_os = "none", target_os = "thingos")))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    struct BufWriter {
        buf: [u8; 256],
        len: usize,
    }

    impl core::fmt::Write for BufWriter {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            let bytes = s.as_bytes();
            let remaining = self.buf.len().saturating_sub(self.len);
            let to_copy = bytes.len().min(remaining);
            if to_copy == 0 {
                return Ok(());
            }
            self.buf[self.len..self.len + to_copy].copy_from_slice(&bytes[..to_copy]);
            self.len += to_copy;
            Ok(())
        }
    }

    let mut writer = BufWriter {
        buf: [0u8; 256],
        len: 0,
    };
    let _ = core::fmt::write(&mut writer, format_args!("STEM PANIC: {}\n", info));
    let _ = debug_write(
        core::str::from_utf8(&writer.buf[..writer.len]).unwrap_or("STEM PANIC\n"),
        writer.len,
    );
    exit(101)
}
