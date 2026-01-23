use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

use stem::syscall;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    NotFound,
    Unsupported,
    InvalidInput,
    Other,
}

#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    message: Option<String>,
}

impl Error {
    pub fn new(kind: ErrorKind, message: &str) -> Self {
        Self {
            kind,
            message: Some(message.to_string()),
        }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(msg) => write!(f, "{:?}: {}", self.kind, msg),
            None => write!(f, "{:?}", self.kind),
        }
    }
}

impl core::error::Error for Error {}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        let mut remaining = buf;
        while !remaining.is_empty() {
            let written = self.write(remaining)?;
            if written == 0 {
                return Err(Error::new(ErrorKind::Other, "write returned 0"));
            }
            remaining = &remaining[written..];
        }
        Ok(())
    }

    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> Result<()> {
        struct Adapter<'a, W: ?Sized> {
            inner: &'a mut W,
            error: Option<Error>,
        }

        impl<'a, W: ?Sized + Write> fmt::Write for Adapter<'a, W> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                match self.inner.write_all(s.as_bytes()) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.error = Some(e);
                        Err(fmt::Error)
                    }
                }
            }
        }

        let mut adapter = Adapter { inner: self, error: None };
        match fmt::write(&mut adapter, args) {
            Ok(()) => Ok(()),
            Err(_) => Err(adapter.error.unwrap_or_else(|| Error::new(ErrorKind::Other, "format error"))),
        }
    }
}

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()> {
        while !buf.is_empty() {
            let n = self.read(buf)?;
            if n == 0 {
                return Err(Error::new(ErrorKind::Other, "unexpected EOF"));
            }
            let tmp = buf;
            buf = &mut tmp[n..];
        }
        Ok(())
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        let start_len = buf.len();
        let mut temp = [0u8; 256];
        loop {
            let n = self.read(&mut temp)?;
            if n == 0 {
                break;
            }
            buf.extend_from_slice(&temp[..n]);
        }
        Ok(buf.len() - start_len)
    }
}

pub struct Stdout;
pub struct Stderr;
pub struct Stdin;

fn write_bytes(level: usize, buf: &[u8]) -> Result<usize> {
    const CHUNK: usize = 1024;
    let mut written = 0;
    for chunk in buf.chunks(CHUNK) {
        let text = match core::str::from_utf8(chunk) {
            Ok(s) => Cow::Borrowed(s),
            Err(_) => Cow::Owned(String::from_utf8_lossy(chunk).into_owned()),
        };
        syscall::log_write(&text, level)
            .map_err(|_| Error::new(ErrorKind::Other, "log_write failed"))?;
        written += chunk.len();
    }
    Ok(written)
}

impl Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        write_bytes(3, buf)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Write for Stderr {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        write_bytes(1, buf)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes()).map_err(|_| fmt::Error)
    }
}

impl fmt::Write for Stderr {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes()).map_err(|_| fmt::Error)
    }
}

impl Read for Stdin {
    fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
        panic!("stdin is not implemented on ThingOS yet");
    }
}

pub fn stdout() -> Stdout {
    Stdout
}

pub fn stderr() -> Stderr {
    Stderr
}

pub fn stdin() -> Stdin {
    Stdin
}

pub fn _print(args: fmt::Arguments<'_>) {
    let mut out = Stdout;
    let _ = out.write_fmt(args);
}

pub fn _eprint(args: fmt::Arguments<'_>) {
    let mut out = Stderr;
    let _ = out.write_fmt(args);
}
