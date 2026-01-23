
use crate::io;

pub struct AnonPipe;

impl AnonPipe {
    pub fn read(&self, _buf: &mut [u8]) -> io::Result<usize> { Ok(0) }
    pub fn write(&self, _buf: &[u8]) -> io::Result<usize> { Ok(0) }
    pub fn diverge(&self) -> ! { panic!("diverge") }
}

pub fn read2(_p1: AnonPipe, _v1: &mut Vec<u8>, _p2: AnonPipe, _v2: &mut Vec<u8>) -> io::Result<()> {
    Ok(())
}
