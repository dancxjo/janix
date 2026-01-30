//! smoltcp Device trait implementation for Thing-OS NIC

use smoltcp::phy::{self, Device, DeviceCapabilities, Medium};
use smoltcp::time::Instant;

pub struct ThingNicDevice {
    rx_buffer: [u8; 2048],
    rx_len: usize,
}

impl ThingNicDevice {
    pub fn new() -> Self {
        Self {
            rx_buffer: [0u8; 2048],
            rx_len: 0,
        }
    }
    
    pub fn now() -> Instant {
        Instant::from_millis(stem::time::now().as_millis() as i64)
    }
}

impl Device for ThingNicDevice {
    type RxToken<'a> = RxToken<'a>;
    type TxToken<'a> = TxToken;

    fn receive(&mut self, _timestamp: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        // Poll for RX frame
        match stem::pal::net::nic_poll_rx(&mut self.rx_buffer) {
            Ok(len) if len > 0 => {
                self.rx_len = len;
                Some((
                    RxToken {
                        buffer: &self.rx_buffer[..len],
                    },
                    TxToken,
                ))
            }
            _ => None,
        }
    }

    fn transmit(&mut self, _timestamp: Instant) -> Option<Self::TxToken<'_>> {
        Some(TxToken)
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.max_transmission_unit = 1500;
        caps.max_burst_size = Some(1);
        caps.medium = Medium::Ethernet;
        caps
    }
}

pub struct RxToken<'a> {
    buffer: &'a [u8],
}

impl<'a> phy::RxToken for RxToken<'a> {
    fn consume<R, F>(self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut buf = [0u8; 2048];
        let len = self.buffer.len();
        buf[..len].copy_from_slice(self.buffer);
        f(&mut buf[..len])
    }
}

pub struct TxToken;

impl phy::TxToken for TxToken {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut buffer = [0u8; 2048];
        let result = f(&mut buffer[..len]);
        let _ = stem::pal::net::nic_tx(&buffer[..len]);
        result
    }
}
