use x86_64::instructions::port::Port;

pub unsafe fn init() {
    let bridge = crate::Bridge;
    use kernel_core::bridge::HardwareBridge;

    bridge.log("PS2: Initializing...\n");

    let mut cmd_port = Port::<u8>::new(0x64);
    let mut data_port = Port::<u8>::new(0x60);

    // Enable Aux Device (Mouse)
    wait_write(&mut cmd_port);
    cmd_port.write(0xA8);

    // Read Controller Configuration Byte (CCB)
    wait_write(&mut cmd_port);
    cmd_port.write(0x20); // Command: Read Byte 0
    wait_read(&mut cmd_port);
    let mut status = data_port.read();

    // Enable IRQ12 (Bit 1)
    status |= 0x02;
    // Clear Disable Mouse (Bit 5) - just in case
    status &= !0x20;

    // Write CCB back
    wait_write(&mut cmd_port);
    cmd_port.write(0x60); // Command: Write Byte 0
    wait_write(&mut cmd_port);
    data_port.write(status);

    // Reset/Enable Mouse Packet Streaming
    // Send 0xD4 to 0x64 (Write to Aux Device)
    wait_write(&mut cmd_port);
    cmd_port.write(0xD4);
    wait_write(&mut cmd_port);
    // Send 0xF4 to 0x60 (Enable Data Reporting)
    data_port.write(0xF4);

    // Wait for ACK (0xFA)
    wait_read(&mut cmd_port);
    let ack = data_port.read();

    bridge.log("PS2: Mouse init ACK=");
    crate::print_hex(ack as u64);
    bridge.log("\n");
}

unsafe fn wait_write(port: &mut Port<u8>) {
    // Wait for bit 1 (Input Buffer Full) of Status Register (0x64) to be 0
    // Arg is `cmd_port` (0x64), so reading it returns Status.
    let mut count = 0;
    while port.read() & 2 != 0 {
        count += 1;
        if count > 100000 {
            break;
        }
        core::hint::spin_loop();
    }
}

unsafe fn wait_read(port: &mut Port<u8>) {
    // Wait for bit 0 (Output Buffer Full) of Status Register (0x64) to be 1
    // Arg is `cmd_port` (0x64).
    let mut count = 0;
    while port.read() & 1 == 0 {
        count += 1;
        if count > 100000 {
            break;
        }
        core::hint::spin_loop();
    }
}
