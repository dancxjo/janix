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
    info!("ps2_mouse: starting MINIMAL init");

    // 1. Enable aux port
    // This is required to route data from mouse to controller
    wait_input_empty();
    ioport_write(PS2_CMD, CMD_ENABLE_AUX as usize, 1);
    
    // 2. Controller Config - REQUIRED for IRQ12
    // Even if we entered via "Minimal Init", we MUST ensure Bit 1 is set
    // otherwise the controller won't assert the IRQ line to the IOAPIC.
    info!("ps2_mouse: configuring CCB for IRQs...");
    let mut ccb = read_controller_config();
    // Enable IRQ1 (0), IRQ12 (1), Translation (6)
    ccb |= 0x43; 
    // Clear Disable Mouse (5)
    ccb &= !0x20;
    write_controller_config(ccb);
    info!("ps2_mouse: CCB set to 0x{:02x}", ccb);

    // 3. Set Defaults (0xF6) - More stable than Reset in QEMU?
    info!("ps2_mouse: setting defaults (0xF6)");
    send_aux_byte(0xF6);
    if let Some(ack) = read_data_filtered(true, "defaults ACK") {
        info!("ps2_mouse: defaults ACK received (0x{:02x})", ack);
    }

    // 4. Status Request (Verify Liveness)
    // defaults (0xF6) seems to enable the mouse in QEMU (Status 0x00),
    // but sending 0xF4 afterwards DISABILITIES it (Status 0x20).
    // So we check status, and only send Enable if needed.
    info!("ps2_mouse: sending status request (0xE9)");
    send_aux_byte(0xE9);
    
    let mut needs_enable = true;

    if let Some(ack) = read_data_filtered(true, "status req ACK") {
        info!("ps2_mouse: status req ACK (0x{:02x})", ack);
        // Read 3 bytes of status
        let b1 = read_data_filtered(true, "status b1").unwrap_or(0xFF);
        let b2 = read_data_filtered(true, "status b2").unwrap_or(0);
        let b3 = read_data_filtered(true, "status b3").unwrap_or(0);
        info!("ps2_mouse: STATUS BYTES: {:02x} {:02x} {:02x}", b1, b2, b3);

        if (b1 & 0x20) == 0 {
             info!("ps2_mouse: Mouse already ENABLED (Bit 5 is 0). Skipping 0xF4.");
             needs_enable = false;
        } else {
             info!("ps2_mouse: Mouse DISABLED (Bit 5 is 1). Proceeding with 0xF4.");
        }
    }

    // 5. Enable data reporting (Conditional)
    if needs_enable {
        info!("ps2_mouse: sending enable (0xF4)");
        send_aux_byte(MOUSE_ENABLE);
        if let Some(ack) = read_data_filtered(true, "enable ACK") {
            info!("ps2_mouse: enable ACK received (0x{:02x})", ack);
        } else {
            info!("ps2_mouse: WARNING - enable ACK missed!");
        }
    }

    // 6. Final Status Check
    info!("ps2_mouse: sending final status request (0xE9)");
    send_aux_byte(0xE9);
    if let Some(ack) = read_data_filtered(true, "final status ACK") {
        info!("ps2_mouse: final status ACK (0x{:02x})", ack);
        let b1 = read_data_filtered(true, "status b1").unwrap_or(0);
        let b2 = read_data_filtered(true, "status b2").unwrap_or(0);
        let b3 = read_data_filtered(true, "status b3").unwrap_or(0);
        info!("ps2_mouse: FINAL STATUS: {:02x} {:02x} {:02x}", b1, b2, b3);
    }

    info!("ps2_mouse: init sequence done");
}

#[stem::main]
fn main(raw_write_handle: usize) -> ! {
    let handle = raw_write_handle as PortHandle;

    info!("ps2_mouse: online (handle={})", handle);

    init_mouse();

    // Subscribe to mouse interrupt

    // Force polling for debugging
    // info!("ps2_mouse: FORCING POLLING MODE WITH CCB");
    // polling_loop(handle);

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
    let mut packet_count: u64 = 0;

    loop {
        // info!("ps2_mouse: waiting for IRQ..."); // Debug
        if let Err(e) = irq_wait(MOUSE_VECTOR) {
            info!("ps2_mouse: irq_wait error: {:?}", e);
            stem::yield_now();
            continue;
        }
        info!("ps2_mouse: IRQ received!");

        // Drain data
        let mut loop_count = 0;
        loop {
              let status = ioport_read(PS2_STATUS, 1);
              if (status & 0x01) == 0 {
                  break;
              }
              if (status & 0x20) == 0 {
                  // Data but not for mouse (aux bit clear)
                  // STEAL IT to clear the jam.
                  let stolen = ioport_read(PS2_DATA, 1) as u8;
                  info!("ps2_mouse: STEALING KEYBOARD DATA 0x{:02x}", stolen);
                  // We cleared the buffer, so we can check for more or exit.
                  // Usually buffer is 1 byte deep, so we are done, but loop checks status.
                  continue;
              }
              
              let byte = ioport_read(PS2_DATA, 1) as u8;
              
              packet[idx] = byte;
              idx += 1;

              if idx == 3 {
                  packet_count += 1;
                  // Send packet
                  if (packet_count % 10) == 0 {
                       info!("ps2_mouse: packet sent (total={}) {:02x} {:02x} {:02x}", packet_count, packet[0], packet[1], packet[2]);
                  }
                  
                  let _ = port_send(handle, &packet);
                  idx = 0;
              }
              
              loop_count += 1;
              if loop_count > 64 { break; }
        }
    }

    // unreachable code removed
}

/// Drain all pending mouse data and assemble packets
fn drain_mouse_data(
    handle: PortHandle,
    packet: &mut [u8; 3],
    idx: &mut usize,
    packet_count: &mut u64,
) -> bool {
    let mut read_any = false;
    for _ in 0..16 {
        let status = ioport_read(PS2_STATUS, 1);

        if status & STATUS_OUTPUT_FULL == 0 {
            break;
        }

        if status & STATUS_AUX_DATA != 0 {
            let byte = ioport_read(PS2_DATA, 1) as u8;
            read_any = true;

            // First byte must have bit 3 set (sync)
            if *idx == 0 && (byte & 0x08) == 0 {
                continue;
            }

            packet[*idx] = byte;
            *idx += 1;

            if *idx == 3 {
                *packet_count += 1;
                if *packet_count <= 3 || (*packet_count % 128 == 0) {
                    info!(
                        "ps2_mouse: packet {} [{:02x} {:02x} {:02x}]",
                        packet_count, packet[0], packet[1], packet[2]
                    );
                }
                let _ = port_send(handle, packet);
                *idx = 0;
            }
        } else {
            // Not mouse data, stop draining - let ps2_kbd handle it
            break;
        }
    }
    read_any
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
