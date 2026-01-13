//! PS/2 Mouse Driver (Interrupt-driven)
//! 
//! Subscribes to IRQ12 via IOAPIC, reads mouse packets on interrupt, sends to Bristle.

#![no_std]
#![no_main]

use stem::info;
use stem::syscall::{ioport_read, ioport_write, irq_subscribe, irq_wait, port_send, PortHandle};

const PS2_DATA: usize = 0x60;
const PS2_STATUS: usize = 0x64;
const PS2_CMD: usize = 0x64;

const STATUS_OUTPUT_FULL: usize = 0x01;
const STATUS_AUX_DATA: usize = 0x20;

const CMD_ENABLE_AUX: u8 = 0xA8;
const CMD_READ_CFG: u8 = 0x20;
const CMD_WRITE_CFG: u8 = 0x60;
const CMD_WRITE_AUX: u8 = 0xD4;
const MOUSE_ENABLE: u8 = 0xF4;

/// IRQ12 vector (mouse) - legacy IRQ12 maps to vector 0x2C after IOAPIC remap
const MOUSE_VECTOR: u8 = 0x2C;

fn wait_input_empty() {
    for _ in 0..10000 {
        if ioport_read(PS2_STATUS, 1) & 0x02 == 0 { return; }
        stem::yield_now();
    }
}

fn wait_output_full() {
    for _ in 0..10000 {
        if ioport_read(PS2_STATUS, 1) & STATUS_OUTPUT_FULL != 0 { return; }
        stem::yield_now();
    }
}

fn flush_output_buffer() {
    // Drain up to 16 bytes of garbage
    for i in 0..16 {
        if ioport_read(PS2_STATUS, 1) & STATUS_OUTPUT_FULL != 0 {
            let b = ioport_read(PS2_DATA, 1);
            info!("ps2_mouse: flushed garbage byte: 0x{:02x}", b);
        } else {
            break;
        }
        stem::yield_now();
    }
}

fn read_controller_config() -> u8 {
    for _ in 0..5 {
        wait_input_empty();
        // Flush any pending data (e.g. key scancodes) before asking for config
        flush_output_buffer();
        ioport_write(PS2_CMD, 1, CMD_READ_CFG as usize);
        wait_output_full();
        let val = ioport_read(PS2_DATA, 1) as u8;
        
        // If we got an ACK (0xFA) or Resend (0xFE), it's likely a stale response
        // to a previous command, not the config byte. Retry.
        if val == 0xFA || val == 0xFE {
            info!("ps2_mouse: read_cfg got {:02x}, retrying...", val);
            continue;
        }
        return val;
    }
    // Fallback if we keep getting garbage
    info!("ps2_mouse: read_cfg failed, assuming default safe config (0x47)");
    0x47 // IRQ1, IRQ12, SysFlag, Translation
}

fn write_controller_config(cfg: u8) {
    wait_input_empty();
    ioport_write(PS2_CMD, 1, CMD_WRITE_CFG as usize);
    wait_input_empty();
    ioport_write(PS2_DATA, 1, cfg as usize);
}

fn send_aux_byte(byte: u8) {
    wait_input_empty();
    ioport_write(PS2_CMD, 1, CMD_WRITE_AUX as usize);
    wait_input_empty();
    ioport_write(PS2_DATA, 1, byte as usize);
}

fn init_mouse() {
    info!("ps2_mouse: enabling aux port");
    
    // Clear any initial garbage
    flush_output_buffer();
    
    // Enable aux port
    wait_input_empty();
    ioport_write(PS2_CMD, 1, CMD_ENABLE_AUX as usize);
    stem::sleep_ms(50);

    // Ensure IRQ12 is enabled (Bit 1) and Mouse Disabled (Bit 5) is CLEARED.
    // Bit 5: 1 = Mouse Disabled, 0 = Mouse Enabled.
    let cfg = read_controller_config();
    
    // Force: Set Bit 1 (IRQ12), Clear Bit 5 (Mouse Disable)
    let new_cfg = (cfg | 0x02) & !0x20;
    
    if new_cfg != cfg {
        write_controller_config(new_cfg);
        info!("ps2_mouse: updated controller cfg 0x{:02x} -> 0x{:02x}", cfg, new_cfg);
    } else {
        info!("ps2_mouse: controller cfg already correct (0x{:02x})", cfg);
    }
    
    // Enable mouse data reporting (0xF4)
    info!("ps2_mouse: sending enable command (0xF4)");
    send_aux_byte(MOUSE_ENABLE);
    
    // Wait for ACK (0xFA)
    wait_output_full();
    let ack = ioport_read(PS2_DATA, 1) as u8;
    if ack == 0xFA {
        info!("ps2_mouse: enable ACK received (0xFA)");
    } else {
        info!("ps2_mouse: enable failed? received 0x{:02x} instead of ACK", ack);
    }
    
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
    
    // Subscribe to mouse interrupt
    match irq_subscribe(MOUSE_VECTOR) {
        Ok(()) => info!("ps2_mouse: subscribed to IRQ12 (vector 0x{:02x})", MOUSE_VECTOR),
        Err(e) => {
            info!("ps2_mouse: IRQ subscribe failed ({:?}), falling back to polling", e);
            polling_loop(handle);
        }
    }
    
    info!("ps2_mouse: entering interrupt-driven loop");
    
    let mut packet = [0u8; 3];
    let mut idx = 0usize;
    let mut packets_sent = 0u64;
    
    loop {
        // Wait for mouse interrupt
        match irq_wait(MOUSE_VECTOR) {
            Ok(_count) => {
                // Drain all available mouse data
                drain_mouse_data(handle, &mut packet, &mut idx, &mut packets_sent);
            }
            Err(_) => {
                stem::yield_now();
            }
        }
    }
}

/// Drain all pending mouse data and assemble packets
fn drain_mouse_data(handle: PortHandle, packet: &mut [u8; 3], idx: &mut usize, packets_sent: &mut u64) {
    for _ in 0..16 {
        let status = ioport_read(PS2_STATUS, 1);
        
        if status & STATUS_OUTPUT_FULL == 0 {
            break;
        }
        
        // Only process aux data (mouse)
        if status & STATUS_AUX_DATA != 0 {
            let byte = ioport_read(PS2_DATA, 1) as u8;
            
            // First byte must have bit 3 set (sync)
            if *idx == 0 && (byte & 0x08) == 0 {
                continue;
            }
            
            packet[*idx] = byte;
            *idx += 1;
            
            if *idx == 3 {
                *packets_sent += 1;
                let _ = port_send(handle, packet);
                
                if *packets_sent <= 10 || *packets_sent % 100 == 0 {
                    info!("ps2_mouse: packet {} = [{:02x} {:02x} {:02x}]",
                          packets_sent, packet[0], packet[1], packet[2]);
                }
                *idx = 0;
            }
        }
    }
}

/// Fallback polling loop
fn polling_loop(handle: PortHandle) -> ! {
    info!("ps2_mouse: using polling mode");
    
    let mut packet = [0u8; 3];
    let mut idx = 0usize;
    let mut packets_sent = 0u64;
    
    loop {
        let status = ioport_read(PS2_STATUS, 1);
        
        if status & STATUS_OUTPUT_FULL != 0 {
            if status & STATUS_AUX_DATA != 0 {
                let byte = ioport_read(PS2_DATA, 1) as u8;
                
                if idx == 0 && (byte & 0x08) == 0 {
                    continue;
                }
                
                packet[idx] = byte;
                idx += 1;
                
                if idx == 3 {
                    packets_sent += 1;
                    let _ = port_send(handle, &packet);
                    idx = 0;
                }
            }
        } else {
            stem::yield_now();
        }
    }
}
