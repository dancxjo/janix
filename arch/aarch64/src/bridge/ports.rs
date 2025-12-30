// Port IO is not available on AArch64; provide harmless stubs so shared drivers
// can run without panicking on architectures that lack IO ports.
pub fn outb(_port: u16, _val: u8) {}

pub fn inb(_port: u16) -> u8 {
    0
}

pub fn outw(_port: u16, _val: u16) {}

pub fn inw(_port: u16) -> u16 {
    0
}

pub fn outd(_port: u16, _val: u32) {}

pub fn ind(_port: u16) -> u32 {
    0
}
