use alloc::vec::Vec;
use thing_std::event::EventStreamReader;

pub trait InterruptInSource {
    fn poll(&mut self, buf: &mut [u8], ts: &mut u64) -> Option<usize>;
}

pub struct SyntheticMouseSource {
    phase: u64,
}

impl SyntheticMouseSource {
    pub fn new() -> Self {
        Self { phase: 0 }
    }
}

impl InterruptInSource for SyntheticMouseSource {
    fn poll(&mut self, buf: &mut [u8], ts: &mut u64) -> Option<usize> {
        self.phase = self.phase.wrapping_add(1);
        *ts = 0;
        if self.phase % 100 == 0 {
            // Emulate movement
            if buf.len() >= 3 {
                buf[0] = 0;
                buf[1] = 1;
                buf[2] = 0;
                return Some(3);
            }
        }
        None
    }
}

pub struct UsbInterruptStreamSource {
    reader: EventStreamReader,
}

impl UsbInterruptStreamSource {
    pub fn new(ptr: *const u8) -> Option<Self> {
        EventStreamReader::new(ptr).map(|reader| Self { reader })
    }
}

impl InterruptInSource for UsbInterruptStreamSource {
    fn poll(&mut self, buf: &mut [u8], ts: &mut u64) -> Option<usize> {
        if let Some(record) = self.reader.poll() {
            let len = record.payload.len().min(buf.len());
            buf[..len].copy_from_slice(&record.payload[..len]);
            *ts = record.ts_mono;
            return Some(len);
        }
        None
    }
}
