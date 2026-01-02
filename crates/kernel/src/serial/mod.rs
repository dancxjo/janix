//! Kernel serial output routed through the machine interface.

use crate::machine;

pub fn putc(c: u8) {
    machine::machine().console_write(&[c]);
}

pub fn init() {
    // Machine backend performs its own hardware setup on first use.
}

pub fn write(bytes: &[u8]) {
    machine::machine().console_write(bytes);
}

pub fn write_hex(val: u64) {
    const HEX: &[u8] = b"0123456789abcdef";
    let mut buf = [b'0'; 18];
    buf[0] = b'0';
    buf[1] = b'x';
    for i in 0..16 {
        buf[17 - i] = HEX[((val >> (i * 4)) & 0xf) as usize];
    }
    write(&buf);
}

pub fn write_num(val: u64) {
    if val == 0 {
        write(b"0");
        return;
    }
    let mut buf = [0u8; 20];
    let mut n = val;
    let mut i = 19;
    while n > 0 {
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
        i -= 1;
    }
    write(&buf[i+1..]);
}
