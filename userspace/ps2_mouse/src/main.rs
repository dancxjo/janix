//! PS/2 Mouse Driver (Interrupt-driven)
//!
//! Subscribes to IRQ12 via IOAPIC, reads mouse packets on interrupt, sends to Bristle.

#![no_std]
#![no_main]

use stem::info;
use stem::syscall::{PortHandle, ioport_read, ioport_write, irq_subscribe, irq_wait, port_send};

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
        if ioport_read(PS2_STATUS, 1) & 0x02 == 0 {
            return;
        }
        stem::yield_now();
    }
}

fn flush_output_buffer() {
    // Drain up to 16 bytes of garbage
    for _ in 0..16 {
        if ioport_read(PS2_STATUS, 1) & STATUS_OUTPUT_FULL != 0 {
            let b = ioport_read(PS2_DATA, 1);
            info!("ps2_mouse: flushed garbage byte: 0x{:02x}", b);
        } else {
            break;
        }
        stem::yield_now();
    }
}

fn read_data_filtered(expect_aux: bool, label: &str) -> Option<u8> {
    let discarded_aux: u32 = 0;
    let discarded_non_aux: u32 = 0;
    for _ in 0..20000 {
        let status = ioport_read(PS2_STATUS, 1);
        if status & STATUS_OUTPUT_FULL == 0 {
            stem::yield_now();
            continue;
        }
        let is_aux = (status & STATUS_AUX_DATA) != 0;
        if is_aux != expect_aux {
            // It's not the type of data we're waiting for.
            // DO NOT read it, or we'll steal it from the other driver!
            stem::yield_now();
            continue;
        }
        let byte = ioport_read(PS2_DATA, 1) as u8;
        return Some(byte);
    }
    info!(
        "ps2_mouse: timed out waiting for {} (discarded_aux={}, discarded_non_aux={})",
        label, discarded_aux, discarded_non_aux
    );
    None
}

fn read_controller_config() -> u8 {
    wait_input_empty();
    // Flush any pending data (e.g. key scancodes) before asking for config
    flush_output_buffer();
    ioport_write(PS2_CMD, CMD_READ_CFG as usize, 1);
    if let Some(val) = read_data_filtered(false, "controller cfg") {
        return val;
    }
    // Fallback if we keep getting garbage
    info!("ps2_mouse: read_cfg failed, assuming default safe config (0x47)");
    0x47 // IRQ1, IRQ12, SysFlag, Translation
}

fn write_controller_config(cfg: u8) {
    wait_input_empty();
    ioport_write(PS2_CMD, CMD_WRITE_CFG as usize, 1);
    wait_input_empty();
    ioport_write(PS2_DATA, cfg as usize, 1);
}

fn send_aux_byte(byte: u8) {
    wait_input_empty();
    ioport_write(PS2_CMD, CMD_WRITE_AUX as usize, 1);
    wait_input_empty();
    ioport_write(PS2_DATA, byte as usize, 1);
}

fn init_mouse() {
    info!("ps2_mouse: starting robust init");

    // 1. Clear any initial garbage
    flush_output_buffer();

    // 2. Enable aux port
    wait_input_empty();
    ioport_write(PS2_CMD, CMD_ENABLE_AUX as usize, 1);
    stem::sleep_ms(50);

    // 3. Reset mouse
    info!("ps2_mouse: sending reset (0xFF)");
    send_aux_byte(0xFF);
    
    // Wait for ACK (0xFA)
    if let Some(ack) = read_data_filtered(true, "reset ACK") {
        info!("ps2_mouse: reset ACK received (0x{:02x})", ack);
    }
    
    // Wait for BAT completion (0xAA)
    if let Some(bat) = read_data_filtered(true, "BAT result") {
        info!("ps2_mouse: BAT result received (0x{:02x})", bat);
    }
    
    // Wait for Device ID (0x00)
    if let Some(id) = read_data_filtered(true, "device ID") {
        info!("ps2_mouse: device ID received (0x{:02x})", id);
    }

    // 4. Set Defaults
    info!("ps2_mouse: sending set defaults (0xF6)");
    send_aux_byte(0xF6);
    let _ = read_data_filtered(true, "set defaults ACK");

    // 5. Set Sample Rate (100)
    info!("ps2_mouse: setting sample rate (100)");
    send_aux_byte(0xF3);
    let _ = read_data_filtered(true, "sample rate cmd ACK");
    send_aux_byte(100);
    let _ = read_data_filtered(true, "sample rate val ACK");

    // 6. Set Resolution (3)
    info!("ps2_mouse: setting resolution (3)");
    send_aux_byte(0xE8);
    let _ = read_data_filtered(true, "resolution cmd ACK");
    send_aux_byte(3);
    let _ = read_data_filtered(true, "resolution val ACK");

    // 7. Controller Config
    let cfg = read_controller_config();
    let new_cfg = (cfg | 0x02) & !0x20; // Enable IRQ12, Enable Mouse
    if new_cfg != cfg {
        write_controller_config(new_cfg);
        info!("ps2_mouse: updated controller cfg 0x{:02x} -> 0x{:02x}", cfg, new_cfg);
    }

    // 8. Enable data reporting
    info!("ps2_mouse: sending enable (0xF4)");
    send_aux_byte(MOUSE_ENABLE);
    if let Some(ack) = read_data_filtered(true, "enable ACK") {
        info!("ps2_mouse: enable ACK received (0x{:02x})", ack);
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
        Ok(()) => info!(
            "ps2_mouse: subscribed to IRQ12 (vector 0x{:02x})",
            MOUSE_VECTOR
        ),
        Err(e) => {
            info!(
                "ps2_mouse: IRQ subscribe failed ({:?}), falling back to polling",
                e
            );
            polling_loop(handle);
        }
    }

    info!("ps2_mouse: entering interrupt-driven loop");

    let mut packet = [0u8; 3];
    let mut idx = 0usize;

    loop {
        // Wait for mouse interrupt
        match irq_wait(MOUSE_VECTOR) {
            Ok(_count) => {
                // Drain all available mouse data
                drain_mouse_data(handle, &mut packet, &mut idx);
            }
            Err(_) => {
                // Fallback: yield and retry
                stem::yield_now();
            }
        }
    }
}

/// Drain all pending mouse data and assemble packets
fn drain_mouse_data(handle: PortHandle, packet: &mut [u8; 3], idx: &mut usize) {
    for _ in 0..16 {
        let status = ioport_read(PS2_STATUS, 1);

        if status & STATUS_OUTPUT_FULL == 0 {
            break;
        }

        if status & STATUS_AUX_DATA != 0 {
            let byte = ioport_read(PS2_DATA, 1) as u8;

            // First byte must have bit 3 set (sync)
            if *idx == 0 && (byte & 0x08) == 0 {
                continue;
            }

            packet[*idx] = byte;
            *idx += 1;

            if *idx == 3 {
                let _ = port_send(handle, packet);
                *idx = 0;
            }
        } else {
            // Not mouse data, stop draining - let ps2_kbd handle it
            break;
        }
    }
}

/// Fallback polling loop
fn polling_loop(handle: PortHandle) -> ! {
    info!("ps2_mouse: using polling mode");

    let mut packet = [0u8; 3];
    let mut idx = 0usize;

    loop {
        let status = ioport_read(PS2_STATUS, 1);

        if status & STATUS_OUTPUT_FULL != 0 {
            let is_aux = status & STATUS_AUX_DATA != 0;
            let byte = ioport_read(PS2_DATA, 1) as u8;
            stem::info!("ps2_mouse: (POLL) byte=0x{:02x} aux={}", byte, is_aux);

            if is_aux {
                if idx == 0 && (byte & 0x08) == 0 {
                    continue;
                }

                packet[idx] = byte;
                idx += 1;

                if idx == 3 {
                    if let Err(e) = port_send(handle, &packet) {
                        stem::error!("ps2_mouse: (POLL) port_send FAILED: {:?}", e);
                    } else {
                        stem::info!("ps2_mouse: (POLL) packet sent: [{:02x} {:02x} {:02x}]", packet[0], packet[1], packet[2]);
                    }
                    idx = 0;
                }
            }
        } else {
            stem::yield_now();
        }
    }
}
