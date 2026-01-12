//! PS/2 Mouse Driver
//! 
//! Reads mouse packets from i8042 controller and sends them to a port.
//! This is a "leaf" driver - it handles hardware and emits raw packets to Bristle.

#![no_std]
#![no_main]

use stem::info;
use stem::syscall::{ioport_read, ioport_write, port_send, PortHandle};

/// PS/2 controller status register (read-only)
const PS2_STATUS: usize = 0x64;
/// PS/2 controller data register (read/write)  
const PS2_DATA: usize = 0x60;
/// PS/2 controller command register (write-only)
const PS2_CMD: usize = 0x64;

/// Status bit: output buffer full (data ready to read)
const STATUS_OUTPUT_FULL: usize = 0x01;
/// Status bit: indicates data is from mouse (aux port)
const STATUS_MOUSE_DATA: usize = 0x20;

/// Command: write to second PS/2 port (mouse)
const CMD_WRITE_AUX: u8 = 0xD4;
/// Mouse command: enable data reporting
const MOUSE_ENABLE: u8 = 0xF4;
/// Mouse command: set defaults
const MOUSE_SET_DEFAULTS: u8 = 0xF6;

/// PS/2 Mouse packet (3 bytes for standard mouse)
#[repr(C, packed)]
#[derive(Clone, Copy)]
struct MousePacket {
    /// Byte 0: buttons and overflow flags
    /// Bit 0: Left button
    /// Bit 1: Right button  
    /// Bit 2: Middle button
    /// Bit 3: Always 1
    /// Bit 4: X sign bit
    /// Bit 5: Y sign bit
    /// Bit 6: X overflow
    /// Bit 7: Y overflow
    buttons_flags: u8,
    /// Byte 1: X movement (signed, 9-bit with sign from byte 0)
    x_movement: u8,
    /// Byte 2: Y movement (signed, 9-bit with sign from byte 0)
    y_movement: u8,
}

/// Send a command to the mouse (via aux port)
fn send_mouse_cmd(cmd: u8) {
    // Wait for input buffer to be empty
    for _ in 0..10000 {
        if ioport_read(PS2_STATUS, 1) & 0x02 == 0 {
            break;
        }
    }
    
    // Tell controller we're writing to aux port
    ioport_write(PS2_CMD, 1, CMD_WRITE_AUX as usize);
    
    // Wait for input buffer again
    for _ in 0..10000 {
        if ioport_read(PS2_STATUS, 1) & 0x02 == 0 {
            break;
        }
    }
    
    // Send the actual command
    ioport_write(PS2_DATA, 1, cmd as usize);
    
    // Wait for and discard ACK
    for _ in 0..10000 {
        let status = ioport_read(PS2_STATUS, 1);
        if status & STATUS_OUTPUT_FULL != 0 {
            let _ = ioport_read(PS2_DATA, 1);
            break;
        }
    }
}

/// Initialize the PS/2 mouse
fn init_mouse() {
    info!("ps2_mouse: initializing...");
    
    // Set defaults
    send_mouse_cmd(MOUSE_SET_DEFAULTS);
    stem::sleep_ms(10);
    
    // Enable data reporting
    send_mouse_cmd(MOUSE_ENABLE);
    stem::sleep_ms(10);
    
    info!("ps2_mouse: enabled data reporting");
}

#[stem::main]
fn main(raw_write_handle: usize) -> ! {
    let handle = raw_write_handle as PortHandle;
    
    info!("ps2_mouse: online (handle={})", handle);
    info!("ps2_mouse: using ports 0x60/0x64 (aux)");
    
    // Initialize the mouse
    init_mouse();
    
    let mut packet_buf = [0u8; 3];
    let mut byte_index = 0;
    
    loop {
        // Poll status register
        let status = ioport_read(PS2_STATUS, 1);
        
        if status & STATUS_OUTPUT_FULL != 0 {
            // Check if data is from mouse (aux port)
            if status & STATUS_MOUSE_DATA != 0 {
                let byte = ioport_read(PS2_DATA, 1) as u8;
                
                // Byte 0 must have bit 3 set (always 1 in valid mouse packets)
                if byte_index == 0 && (byte & 0x08) == 0 {
                    // Re-sync: discard and wait for valid start byte
                    continue;
                }
                
                packet_buf[byte_index] = byte;
                byte_index += 1;
                
                if byte_index == 3 {
                    // Complete packet - send to Bristle
                    let _ = port_send(handle, &packet_buf);
                    byte_index = 0;
                }
            } else {
                // Keyboard data - discard (handled by ps2_kbd)
                let _ = ioport_read(PS2_DATA, 1);
            }
        } else {
            // No data available, yield to avoid burning CPU
            stem::yield_now();
        }
    }
}
