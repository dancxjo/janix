use kernel_core::console::{ConsoleSink, register_sink};
use spin::Mutex;

#[cfg(target_arch = "x86_64")]
pub mod arch {
    use super::*;
    use x86_64::instructions::port::Port;

    struct SerialPort {
        data: Port<u8>,
        int_en: Port<u8>,
        fifo_ctrl: Port<u8>,
        line_ctrl: Port<u8>,
        modem_ctrl: Port<u8>,
        line_sts: Port<u8>,
    }

    impl SerialPort {
        const unsafe fn new(base: u16) -> Self {
            Self {
                data: Port::new(base),
                int_en: Port::new(base + 1),
                fifo_ctrl: Port::new(base + 2),
                line_ctrl: Port::new(base + 3),
                modem_ctrl: Port::new(base + 4),
                line_sts: Port::new(base + 5),
            }
        }

        fn init(&mut self) {
            unsafe {
                self.int_en.write(0x00);
                self.line_ctrl.write(0x80);
                self.data.write(0x03);
                self.int_en.write(0x00);
                self.line_ctrl.write(0x03);
                self.fifo_ctrl.write(0xC7);
                self.modem_ctrl.write(0x0B);
            }
        }

        fn send(&mut self, byte: u8) {
            unsafe {
                while (self.line_sts.read() & 0x20) == 0 {}
                self.data.write(byte);
            }
        }
    }

    struct SerialSink(Mutex<SerialPort>);
    unsafe impl Sync for SerialSink {}
    unsafe impl Send for SerialSink {}

    impl ConsoleSink for SerialSink {
        fn write_str(&self, s: &str) {
            let mut port = self.0.lock();
            for byte in s.bytes() {
                port.send(byte);
            }
        }
    }

    static SERIAL: SerialSink = SerialSink(Mutex::new(unsafe { SerialPort::new(0x3F8) }));

    pub fn init_serial() {
        SERIAL.0.lock().init();
        register_sink(&SERIAL);
    }
}

#[cfg(target_arch = "aarch64")]
pub mod arch {
    use super::*;
    
    struct Pl011 {
        base: *mut u32,
    }
    
    impl Pl011 {
        const unsafe fn new(base: *mut u8) -> Self {
            Self { base: base as *mut u32 }
        }
        
        fn send(&mut self, byte: u8) {
             unsafe {
                // Wait for TXFF (Transmit FIFO Full) to be clear. FR is at offset 0x18 (6 words)
                while (self.base.add(6).read_volatile() & 0x20) != 0 {}
                self.base.write_volatile(byte as u32);
             }
        }
    }
    
    struct SerialSink(Mutex<Pl011>);
    unsafe impl Sync for SerialSink {}
    unsafe impl Send for SerialSink {}

    impl ConsoleSink for SerialSink {
        fn write_str(&self, s: &str) {
            let mut port = self.0.lock();
            for byte in s.bytes() {
                port.send(byte);
            }
        }
    }

    static SERIAL: SerialSink = SerialSink(Mutex::new(unsafe { Pl011::new(0x09000000 as *mut u8) }));

    pub fn init_serial() {
        register_sink(&SERIAL);
    }
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
pub mod arch {
    pub fn init_serial() {}
}
