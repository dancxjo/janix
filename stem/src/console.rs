use core::fmt;
use crate::syscall::log_write;

struct BufConsole {
    buf: [u8; 256],
    len: usize,
    level: usize,
}

impl BufConsole {
    fn new(level: usize) -> Self {
        BufConsole {
            buf: [0u8; 256],
            len: 0,
            level,
        }
    }

    fn flush(&mut self) {
        if self.len > 0 {
            if let Ok(s) = core::str::from_utf8(&self.buf[..self.len]) {
                let _ = log_write(s, self.level);
            }
            self.len = 0;
        }
    }
}

impl fmt::Write for BufConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let mut remaining = bytes;

        while !remaining.is_empty() {
            let space = self.buf.len() - self.len;
            if space == 0 {
                self.flush();
                continue;
            }
            
            let chunk_len = core::cmp::min(space, remaining.len());
            self.buf[self.len..self.len + chunk_len].copy_from_slice(&remaining[..chunk_len]);
            self.len += chunk_len;
            remaining = &remaining[chunk_len..];
        }
        Ok(())
    }
}

pub fn log(level: usize, args: fmt::Arguments) {
    use core::fmt::Write;
    let mut cons = BufConsole::new(level);
    let _ = cons.write_fmt(args);
    cons.flush();
}

pub fn print(args: fmt::Arguments) {
    log(3, args); // Default to Info
}
