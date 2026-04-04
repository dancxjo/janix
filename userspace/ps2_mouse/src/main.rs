//! PS/2 Mouse Driver (Interrupt-driven)
//!
//! Subscribes to IRQ12 via IOAPIC, reads mouse packets on interrupt, sends to Bristle.

#![feature(restricted_std)]
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
    info!("ps2_mouse: enabling aux port");

    // Clear any initial garbage
    flush_output_buffer();

    // Enable aux port
    wait_input_empty();
    ioport_write(PS2_CMD, CMD_ENABLE_AUX as usize, 1);
    stem::sleep_ms(50);

    // Ensure IRQ12 is enabled (Bit 1) and Mouse Disabled (Bit 5) is CLEARED.
    // Bit 5: 1 = Mouse Disabled, 0 = Mouse Enabled.
    let cfg = read_controller_config();

    // Force: Set Bit 1 (IRQ12), Clear Bit 5 (Mouse Disable)
    let new_cfg = (cfg | 0x02) & !0x20;

    if new_cfg != cfg {
        write_controller_config(new_cfg);
        info!(
            "ps2_mouse: updated controller cfg 0x{:02x} -> 0x{:02x}",
            cfg, new_cfg
        );
    } else {
        info!("ps2_mouse: controller cfg already correct (0x{:02x})", cfg);
    }

    // Reset mouse (0xFF)
    info!("ps2_mouse: sending RESET (0xFF)");
    send_aux_byte(0xFF);
    let ack = read_data_filtered(true, "reset ACK (0xfa)").unwrap_or(0);
    if ack == 0xFA {
        info!("ps2_mouse: reset ACK received (0xfa)");
        let bat = read_data_filtered(true, "BAT byte (0xAA)").unwrap_or(0);
        let id = read_data_filtered(true, "Device ID (0x00)").unwrap_or(1);
        info!("ps2_mouse: BAT passed (0x{:02x}), ID 0x{:02x} confirmed", bat, id);
    }

    info!("ps2_mouse: setting sample rate (100)");
    send_aux_byte(0xF3);
    read_data_filtered(true, "sample rate ACK");
    send_aux_byte(100);
    read_data_filtered(true, "sample rate set ACK");

    info!("ps2_mouse: setting resolution (3)");
    send_aux_byte(0xE8);
    read_data_filtered(true, "resolution ACK");
    send_aux_byte(3);
    read_data_filtered(true, "resolution set ACK");

    send_aux_byte(0xE9);
    let _s_ack = read_data_filtered(true, "status request ACK");
    let b1 = read_data_filtered(true, "status byte 1").unwrap_or(0);
    let b2 = read_data_filtered(true, "status byte 2").unwrap_or(0);
    let b3 = read_data_filtered(true, "status byte 3").unwrap_or(0);
    info!("ps2_mouse: status result = Some({}) Some({}) Some({})", b1, b2, b3);

    // If bit 5 is 0, it means it's already enabled in streaming mode.
    if b1 & 0x20 != 0 {
        // Enable mouse data reporting (0xF4)
        info!("ps2_mouse: sending enable command (0xF4)");
        send_aux_byte(MOUSE_ENABLE);
        let e_ack = read_data_filtered(true, "enable ACK (0xFA)").unwrap_or(0);
        info!("ps2_mouse: enable ACK received (0x{:02x})", e_ack);
    } else {
        info!("ps2_mouse: already enabled, skipping 0xF4 command");
    }

    stem::sleep_ms(100);

    // Drain any lingering response bytes.
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

    // Force polling diagnostic
    info!("ps2_mouse: FORCING POLLING LOOP FOR DIAGNOSTIC");
    polling_loop(handle);

    /*
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
                stem::yield_now();
            }
        }
    }
    */
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
            // Not mouse data; steal it to clear the jam!
            let stolen = ioport_read(PS2_DATA, 1) as u8;
            info!("ps2_mouse: STEALING keyboard byte 0x{:02x} to clear jam", stolen);
            continue;
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
            let byte = ioport_read(PS2_DATA, 1) as u8;
            
            if status & STATUS_AUX_DATA != 0 {
                info!("ps2_mouse: POLL got mouse byte 0x{:02x}", byte);
                
                if idx == 0 && (byte & 0x08) == 0 {
                    continue;
                }

                packet[idx] = byte;
                idx += 1;

                if idx == 3 {
                    let _ = port_send(handle, &packet);
                    idx = 0;
                }
            } else {
                info!("ps2_mouse: POLL stealing keyboard byte 0x{:02x}", byte);
            }
        } else {
            stem::sleep_ms(10);
        }
    }
}
