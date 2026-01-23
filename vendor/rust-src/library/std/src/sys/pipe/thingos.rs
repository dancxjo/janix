
use crate::io::{self, Read, Write};
use crate::fs::File;

#[derive(Copy, Clone, Debug)]
pub struct AnonPipe(());
pub type Pipe = AnonPipe;

impl AnonPipe {
    pub fn read(&self, _buf: &mut [u8]) -> io::Result<usize> { Ok(0) }
    pub fn read_vectored(&self, _bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> { Ok(0) }
    pub fn is_read_vectored(&self) -> bool { false }
    pub fn read_to_end(&self, _buf: &mut Vec<u8>) -> io::Result<usize> { Ok(0) }
    pub fn read_buf(&self, _cursor: crate::io::BorrowedCursor<'_>) -> io::Result<()> { Ok(()) }

    pub fn write(&self, _buf: &[u8]) -> io::Result<usize> { Ok(0) }
    pub fn write_vectored(&self, _bufs: &[io::IoSlice<'_>]) -> io::Result<usize> { Ok(0) }
    pub fn is_write_vectored(&self) -> bool { false }
    pub fn try_clone(&self) -> io::Result<AnonPipe> { Ok(AnonPipe(())) }
}

impl Read for AnonPipe {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.read(buf)
    }
    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        self.read_vectored(bufs)
    }
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.read_to_end(buf)
    }
}

impl<'a> Read for &'a AnonPipe {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (*self).read(buf)
    }
    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        (*self).read_vectored(bufs)
    }
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        (*self).read_to_end(buf)
    }
}

impl Write for AnonPipe {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        self.write_vectored(bufs)
    }
}

impl<'a> Write for &'a AnonPipe {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (*self).write(buf)
    }
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        (*self).write_vectored(bufs)
    }
}

pub fn pipe() -> io::Result<(AnonPipe, AnonPipe)> {
    Err(io::Error::UNSUPPORTED_PLATFORM)
}

impl Into<File> for AnonPipe {
    fn into(self) -> File {
        panic!("unsupported")
    }
}
