//! PS/2 Mouse Driver
//! 
//! Reads mouse packets from i8042 controller aux port.
//! QEMU's Q35 machine has PS/2 mouse enabled by default.

#![no_std]
#![no_main]

use stem::info;
use stem::syscall::{ioport_read, ioport_write, port_send, PortHandle};

const PS2_DATA: usize = 0x60;
const PS2_STATUS: usize = 0x64;
const PS2_CMD: usize = 0x64;

const STATUS_OUTPUT_FULL: usize = 0x01;
const STATUS_AUX_DATA: usize = 0x20;

const CMD_ENABLE_AUX: u8 = 0xA8;
const CMD_WRITE_AUX: u8 = 0xD4;
const MOUSE_ENABLE: u8 = 0xF4;

fn wait_input_empty() {
    for _ in 0..10000 {
        if ioport_read(PS2_STATUS, 1) & 0x02 == 0 { return; }
    }
}

fn send_aux_byte(byte: u8) {
    wait_input_empty();
    ioport_write(PS2_CMD, 1, CMD_WRITE_AUX as usize);
    wait_input_empty();
    ioport_write(PS2_DATA, 1, byte as usize);
}

fn init_mouse() {
    info!("ps2_mouse: enabling aux port");
    
    // Enable aux port
    wait_input_empty();
    ioport_write(PS2_CMD, 1, CMD_ENABLE_AUX as usize);
    stem::sleep_ms(50);
    
    // Enable mouse data reporting
    info!("ps2_mouse: sending enable command");
    send_aux_byte(MOUSE_ENABLE);
    stem::sleep_ms(100);
    
    // Drain any response bytes
    for _ in 0..10 {
        if ioport_read(PS2_STATUS, 1) & STATUS_OUTPUT_FULL != 0 {
            let byte = ioport_read(PS2_DATA, 1) as u8;
            info!("ps2_mouse: drained 0x{:02x}", byte);
        }
        stem::sleep_ms(10);
    }
    
    info!("ps2_mouse: init done");
}

#[stem::main]
fn main(raw_write_handle: usize) -> ! {
    let handle = raw_write_handle as PortHandle;
    
    info!("ps2_mouse: online (handle={})", handle);
    
    init_mouse();
    
    let mut packet = [0u8; 3];
    let mut idx = 0usize;
    let mut total_bytes = 0u64;
    let mut packets_sent = 0u64;
    
    info!("ps2_mouse: polling for data...");
    
    loop {
        let status = ioport_read(PS2_STATUS, 1);
        
        if status & STATUS_OUTPUT_FULL != 0 {
            let byte = ioport_read(PS2_DATA, 1) as u8;
            total_bytes += 1;
            
            // Log first few bytes to debug
            if total_bytes <= 20 {
                info!("ps2_mouse: byte {} = 0x{:02x} (aux={})", 
                      total_bytes, byte, (status & STATUS_AUX_DATA) != 0);
            }
            
            // Only process if aux bit says it's mouse data
            if status & STATUS_AUX_DATA != 0 {
                // First byte must have bit 3 set
                if idx == 0 && (byte & 0x08) == 0 {
                    continue;
                }
                
                packet[idx] = byte;
                idx += 1;
                
                if idx == 3 {
                    packets_sent += 1;
                    let _ = port_send(handle, &packet);
                    
                    if packets_sent <= 10 || packets_sent % 100 == 0 {
                        info!("ps2_mouse: packet {} = [{:02x} {:02x} {:02x}]",
                              packets_sent, packet[0], packet[1], packet[2]);
                    }
                    idx = 0;
                }
            }
        } else {
            stem::yield_now();
        }
    }
}
