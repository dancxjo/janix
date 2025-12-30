use crate::bridge::PortIo;

pub fn init(bridge: &impl PortIo) {
    bridge.log("PS2: Initializing...\n");

    let cmd_port = 0x64;
    let data_port = 0x60;

    // Enable Aux Device (Mouse)
    wait_write(bridge, cmd_port);
    bridge.port_outb(cmd_port, 0xA8);

    // Read Controller Configuration Byte (CCB)
    wait_write(bridge, cmd_port);
    bridge.port_outb(cmd_port, 0x20); // Command: Read Byte 0
    wait_read(bridge, cmd_port);
    let mut status = bridge.port_inb(data_port);

    // Enable IRQ12 (Bit 1)
    status |= 0x02;
    // Clear Disable Mouse (Bit 5) - just in case
    status &= !0x20;

    // Write CCB back
    wait_write(bridge, cmd_port);
    bridge.port_outb(cmd_port, 0x60); // Command: Write Byte 0
    wait_write(bridge, cmd_port);
    bridge.port_outb(data_port, status);

    // Reset/Enable Mouse Packet Streaming
    // Send 0xD4 to 0x64 (Write to Aux Device)
    wait_write(bridge, cmd_port);
    bridge.port_outb(cmd_port, 0xD4);
    wait_write(bridge, cmd_port);
    // Send 0xF4 to 0x60 (Enable Data Reporting)
    bridge.port_outb(data_port, 0xF4);

    // Wait for ACK (0xFA)
    wait_read(bridge, cmd_port);
    let ack = bridge.port_inb(data_port);

    bridge.log("PS2: Mouse init ACK=");
    // crate::print_hex(ack as u64);
    if ack == 0xFA {
        bridge.log("OK");
    } else {
        bridge.log("FAIL");
    }
    bridge.log("\n");
}

fn wait_write(bridge: &impl PortIo, port: u16) {
    // Wait for bit 1 (Input Buffer Full) of Status Register (0x64) to be 0
    let mut count = 0;
    while bridge.port_inb(port) & 2 != 0 {
        count += 1;
        if count > 100000 {
            break;
        }
        core::hint::spin_loop();
    }
}

fn wait_read(bridge: &impl PortIo, port: u16) {
    // Wait for bit 0 (Output Buffer Full) of Status Register (0x64) to be 1
    let mut count = 0;
    while bridge.port_inb(port) & 1 == 0 {
        count += 1;
        if count > 100000 {
            break;
        }
        core::hint::spin_loop();
    }
}
