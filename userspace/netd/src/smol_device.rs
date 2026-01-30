//! smoltcp Device trait implementation for VirtIO-NET
//!
//! Uses the userspace VirtioNetDriver for packet TX/RX

use smoltcp::phy::{self, Device, DeviceCapabilities, Medium};
use smoltcp::time::Instant;
use crate::virtio_net::VirtioNetDriver;

pub struct VirtioNicDevice<'a> {
    driver: &'a mut VirtioNetDriver,
    rx_buffer: [u8; 2048],
    rx_len: usize,
}

impl<'a> VirtioNicDevice<'a> {
    pub fn new(driver: &'a mut VirtioNetDriver) -> Self {
        Self {
            driver,
            rx_buffer: [0u8; 2048],
            rx_len: 0,
        }
    }
    
    pub fn now() -> Instant {
        Instant::from_millis(stem::time::now().as_millis() as i64)
    }
}

impl<'a> Device for VirtioNicDevice<'a> {
    type RxToken<'b> = RxToken<'b> where Self: 'b;
    type TxToken<'b> = TxToken<'b> where Self: 'b;

    fn receive(&mut self, _timestamp: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        // Poll for RX frame from VirtIO driver
        if let Some(data) = self.driver.poll_rx() {
            // Copy to local buffer
            let len = data.len().min(self.rx_buffer.len());
            self.rx_buffer[..len].copy_from_slice(&data[..len]);
            self.rx_len = len;
            
            Some((
                RxToken {
                    buffer: &self.rx_buffer[..self.rx_len],
                },
                TxToken {
                    driver: self.driver,
                },
            ))
        } else {
            None
        }
    }

    fn transmit(&mut self, _timestamp: Instant) -> Option<Self::TxToken<'_>> {
        Some(TxToken {
            driver: self.driver,
        })
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

pub struct TxToken<'a> {
    driver: &'a mut VirtioNetDriver,
}

impl<'a> phy::TxToken for TxToken<'a> {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut buffer = [0u8; 2048];
        let result = f(&mut buffer[..len]);
        let _ = self.driver.tx(&buffer[..len]);
        result
    }
}
