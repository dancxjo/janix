pub trait ConsoleSink: Sync + Send {
    fn write_str(&self, s: &str);
}

struct NullSink;
impl ConsoleSink for NullSink {
    fn write_str(&self, _s: &str) {}
}

use spin::Mutex;

const MAX_SINKS: usize = 4;
static SINKS: Mutex<[Option<&'static dyn ConsoleSink>; MAX_SINKS]> = Mutex::new([None; MAX_SINKS]);

pub fn register_sink(sink: &'static dyn ConsoleSink) {
    let mut sinks = SINKS.lock();
    for i in 0..MAX_SINKS {
        if sinks[i].is_none() {
            sinks[i] = Some(sink);
            return;
        }
    }
}

pub fn print(s: &str) {
    // Disable interrupts to prevent deadlock if an ISR prints
    x86_64::instructions::interrupts::without_interrupts(|| {
        let sinks = SINKS.lock();
        for sink in sinks.iter().flatten() {
            sink.write_str(s);
        }
    });
}

use core::fmt;

pub struct ConsoleWriter {
    buf: [u8; 128],
    len: usize,
}

impl ConsoleWriter {
    fn new() -> Self {
        Self {
            buf: [0; 128],
            len: 0,
        }
    }

    fn flush(&mut self) {
        if self.len > 0 {
            // Attempt to convert the buffer to a string.
            match core::str::from_utf8(&self.buf[..self.len]) {
                Ok(s) => {
                    print(s);
                    self.len = 0;
                }
                Err(e) => {
                    // If the buffer ends with an incomplete UTF-8 sequence, print the valid part.
                    let valid_len = e.valid_up_to();
                    if valid_len > 0 {
                        // SAFETY: valid_up_to guarantees this range is valid UTF-8.
                        let s = unsafe { core::str::from_utf8_unchecked(&self.buf[..valid_len]) };
                        print(s);
                    }

                    // Move the remaining bytes (incomplete sequence) to the beginning.
                    let remaining = self.len - valid_len;
                    // We must use a loop or copy_within since we can't use memmove easily without unsafe,
                    // but copy_within is available on slices in core.
                    self.buf.copy_within(valid_len..self.len, 0);
                    self.len = remaining;
                }
            }
        }
    }
}

impl Drop for ConsoleWriter {
    fn drop(&mut self) {
        // Force flush everything on drop. If there are incomplete bytes at the end,
        // we can't do much but print them as replacement chars or just print the valid part.
        // But since we are likely finishing a line, it's rare to end in partial char unless
        // the input str was bad or we had a bug.
        // However, standard `flush` retains incomplete bytes.
        // We want to clear the buffer.

        if self.len > 0 {
             match core::str::from_utf8(&self.buf[..self.len]) {
                Ok(s) => print(s),
                Err(e) => {
                    let valid_len = e.valid_up_to();
                    if valid_len > 0 {
                         let s = unsafe { core::str::from_utf8_unchecked(&self.buf[..valid_len]) };
                         print(s);
                    }
                    // For the invalid tail, we could print a replacement char, but for now just drop it
                    // as we can't emit partial UTF-8 to the console sink which expects &str.
                }
            }
            self.len = 0;
        }
    }
}

impl fmt::Write for ConsoleWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let mut idx = 0;

        while idx < bytes.len() {
            let space = self.buf.len() - self.len;
            let chunk_len = space.min(bytes.len() - idx);

            self.buf[self.len..self.len + chunk_len].copy_from_slice(&bytes[idx..idx + chunk_len]);
            self.len += chunk_len;
            idx += chunk_len;

            if self.len == self.buf.len() {
                self.flush();
            }
        }
        Ok(())
    }
}

pub fn print_fmt(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = ConsoleWriter::new();
    let _ = writer.write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::print_fmt(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
