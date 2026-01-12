//! Echo: Keyboard Event Display
//!
//! Reads KeyEventV0 records and prints them to the console.
//! This is the simplest possible keyboard consumer.

#![no_std]
#![no_main]

use stem::info;
use stem::syscall::{port_recv, PortHandle};

/// Simple scancode name lookup for common keys
fn scancode_name(sc: u8, extended: bool) -> &'static str {
    if extended {
        // Extended keys (after E0 prefix)
        match sc {
            0x48 => "Up",
            0x50 => "Down",
            0x4B => "Left",
            0x4D => "Right",
            0x47 => "Home",
            0x4F => "End",
            0x49 => "PgUp",
            0x51 => "PgDn",
            0x52 => "Ins",
            0x53 => "Del",
            _ => "Ext",
        }
    } else {
        // Non-extended scancodes (Set 1, US layout approximation)
        match sc {
            0x01 => "Esc",
            0x02 => "1", 0x03 => "2", 0x04 => "3", 0x05 => "4",
            0x06 => "5", 0x07 => "6", 0x08 => "7", 0x09 => "8",
            0x0A => "9", 0x0B => "0",
            0x0C => "-", 0x0D => "=",
            0x0E => "Bksp",
            0x0F => "Tab",
            0x10 => "Q", 0x11 => "W", 0x12 => "E", 0x13 => "R", 0x14 => "T",
            0x15 => "Y", 0x16 => "U", 0x17 => "I", 0x18 => "O", 0x19 => "P",
            0x1A => "[", 0x1B => "]",
            0x1C => "Enter",
            0x1D => "Ctrl",
            0x1E => "A", 0x1F => "S", 0x20 => "D", 0x21 => "F", 0x22 => "G",
            0x23 => "H", 0x24 => "J", 0x25 => "K", 0x26 => "L",
            0x27 => ";", 0x28 => "'",
            0x29 => "`",
            0x2A => "LShft",
            0x2B => "\\",
            0x2C => "Z", 0x2D => "X", 0x2E => "C", 0x2F => "V", 0x30 => "B",
            0x31 => "N", 0x32 => "M",
            0x33 => ",", 0x34 => ".", 0x35 => "/",
            0x36 => "RShft",
            0x37 => "*",
            0x38 => "Alt",
            0x39 => "Space",
            0x3A => "Caps",
            0x3B..=0x44 => "F?", // F1-F10
            _ => "?",
        }
    }
}

/// Format modifier bits
fn format_mods(mods: u8) -> &'static str {
    match mods {
        0 => "",
        1 => "+Sh",
        2 => "+Ct",
        3 => "+Sh+Ct",
        4 => "+Al",
        5 => "+Sh+Al",
        6 => "+Ct+Al",
        7 => "+Sh+Ct+Al",
        _ => "+???",
    }
}

#[stem::main]
fn main(evt_read_handle: usize) -> ! {
    let handle = evt_read_handle as PortHandle;
    
    info!("echo: online (handle={})", handle);

    let mut buf = [0u8; 64];

    loop {
        match port_recv(handle, &mut buf) {
            Ok(n) if n >= 4 => {
                // Process 4-byte event records
                for chunk in buf[..n].chunks_exact(4) {
                    let kind = chunk[0];
                    let mods = chunk[1];
                    let scancode = chunk[2];
                    let flags = chunk[3];
                    
                    let extended = flags & 1 != 0;
                    let kind_str = if kind == 1 { "down" } else { "up" };
                    let key_name = scancode_name(scancode, extended);
                    let mod_str = format_mods(mods);
                    
                    info!("[echo] {} {}{} (sc=0x{:02x})", kind_str, key_name, mod_str, scancode);
                }
            }
            _ => {
                // No data, yield to avoid busy-spin
                stem::yield_now();
            }
        }
    }
}
