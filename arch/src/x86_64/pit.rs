use x86_64::instructions::port::Port;

pub const PIT_FREQUENCY: u32 = 1193182;
pub const TARGET_FREQUENCY: u32 = 1000;

pub fn init() {
    // 1000 Hz
    let divisor = 1193182 / 1000;
    let mut command_port = Port::<u8>::new(0x43);
    let mut data_port = Port::<u8>::new(0x40);

    unsafe {
        command_port.write(0x36); // Channel 0, lobyte/hibyte, Mode 3
        data_port.write((divisor & 0xFF) as u8);
        data_port.write((divisor >> 8) as u8);
    }
}
