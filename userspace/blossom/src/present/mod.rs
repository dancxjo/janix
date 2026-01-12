extern crate alloc;

use alloc::vec::Vec;
use core::mem;

use abi::display_driver_protocol as drvproto;
use stem::syscall::{port_recv, port_send, PortHandle};

use crate::surface::{BackBuffer, PixelFormat, Surface};

#[derive(Clone, Copy, Debug)]
pub struct DisplaySurface {
    pub bytespace_id: u64,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
}

pub trait Presenter {
    fn surface_mut(&mut self) -> Surface<'_>;
    fn present(&mut self);
    fn poll_driver(&mut self);
    fn vsync_hint(&self) -> Option<u32> {
        None
    }
}

pub struct DriverPresenter {
    surface: DisplaySurface,
    backbuffer: BackBuffer,
    bytespace_ptr: *mut u8,
    drv_req_write: PortHandle,
    drv_resp_read: PortHandle,
    driver_ready: bool,
    driver_bound: bool,
    pending_present: bool,
    rx_buf: Vec<u8>,
    rx_len: usize,
}

impl DriverPresenter {
    pub fn new(
        surface: DisplaySurface,
        bytespace_ptr: *mut u8,
        drv_req_write: PortHandle,
        drv_resp_read: PortHandle,
    ) -> Self {
        let format = match surface.format {
            abi::display_protocol::FORMAT_XRGB8888 => PixelFormat::Xrgb8888,
            _ => PixelFormat::Xrgb8888,
        };
        let backbuffer = BackBuffer::new(
            surface.width as usize,
            surface.height as usize,
            surface.stride as usize,
            format,
        );
        if !backbuffer.has_buffer() {
            stem::info!("blossom: backbuffer alloc failed, using bytespace directly");
        }
        Self {
            surface,
            backbuffer,
            bytespace_ptr,
            drv_req_write,
            drv_resp_read,
            driver_ready: false,
            driver_bound: false,
            pending_present: false,
            rx_buf: alloc::vec![0u8; 1024],
            rx_len: 0,
        }
    }

    pub fn driver_ready(&self) -> bool {
        self.driver_ready && self.driver_bound
    }

    fn send_msg(&self, msg_type: u16, payload: &[u8]) {
        let mut buf = [0u8; 256];
        if let Some(len) = drvproto::encode_message(&mut buf, msg_type, payload) {
            let _ = port_send(self.drv_req_write, &buf[..len]);
        }
    }

    fn handle_driver_message(&mut self, header: drvproto::DriverHeader, payload: &[u8]) {
        match header.msg_type {
            drvproto::MSG_REGISTER => {
                if payload.len() >= mem::size_of::<drvproto::RegisterPayload>() {
                    self.driver_ready = true;
                    self.send_msg(drvproto::MSG_ACK, &[]);

                    let bind = drvproto::BindPayload {
                        bytespace_id: self.surface.bytespace_id,
                        width: self.surface.width,
                        height: self.surface.height,
                        stride: self.surface.stride,
                        format: self.surface.format,
                    };
                    let bind_bytes = unsafe {
                        core::slice::from_raw_parts(
                            &bind as *const _ as *const u8,
                            mem::size_of::<drvproto::BindPayload>(),
                        )
                    };
                    self.send_msg(drvproto::MSG_BIND, bind_bytes);
                }
            }
            drvproto::MSG_ACK => {
                if !self.driver_bound {
                    self.driver_bound = true;
                }
                self.pending_present = false;
            }
            drvproto::MSG_ERR => {
                self.pending_present = false;
            }
            _ => {}
        }
    }

    fn copy_to_bytespace(&mut self) {
        let src = match self.backbuffer.buf() {
            Some(buf) => buf,
            None => return,
        };
        let dst_len = (self.surface.height as usize) * (self.surface.stride as usize);
        if dst_len == 0 || src.len() < dst_len {
            return;
        }
        unsafe {
            core::ptr::copy_nonoverlapping(src.as_ptr(), self.bytespace_ptr, dst_len);
        }
    }
}

impl Presenter for DriverPresenter {
    fn surface_mut(&mut self) -> Surface<'_> {
        self.backbuffer.surface_mut_with_fallback(self.bytespace_ptr)
    }

    fn present(&mut self) {
        if self.backbuffer.has_buffer() {
            self.copy_to_bytespace();
        }
        if !self.driver_ready() || self.pending_present {
            return;
        }
        self.pending_present = true;
        self.send_msg(drvproto::MSG_PRESENT, &[]);
    }

    fn poll_driver(&mut self) {
        let mut buf = [0u8; 512];
        if let Ok(n) = port_recv(self.drv_resp_read, &mut buf) {
            if n > 0 {
                if self.rx_len + n > self.rx_buf.len() {
                    self.rx_len = 0;
                }
                self.rx_buf[self.rx_len..self.rx_len + n].copy_from_slice(&buf[..n]);
                self.rx_len += n;
            }
        }

        while self.rx_len >= drvproto::HEADER_SIZE {
            let header = match parse_drv_header(&self.rx_buf[..self.rx_len]) {
                Some(header) => header,
                None => {
                    self.rx_len = 0;
                    break;
                }
            };

            if header.magic != drvproto::DRIVER_MAGIC || header.version != drvproto::DRIVER_VERSION {
                self.rx_buf.copy_within(1..self.rx_len, 0);
                self.rx_len -= 1;
                continue;
            }

            let total = drvproto::HEADER_SIZE + (header.payload_len as usize);
            if self.rx_len < total {
                break;
            }

            let payload = self.rx_buf[drvproto::HEADER_SIZE..total].to_vec();
            self.handle_driver_message(header, &payload);

            if total < self.rx_len {
                self.rx_buf.copy_within(total..self.rx_len, 0);
            }
            self.rx_len -= total;
        }
    }
}

fn parse_drv_header(buf: &[u8]) -> Option<drvproto::DriverHeader> {
    if buf.len() < drvproto::HEADER_SIZE {
        return None;
    }

    let magic = u32::from_le_bytes(buf[0..4].try_into().ok()?);
    let version = u16::from_le_bytes(buf[4..6].try_into().ok()?);
    let msg_type = u16::from_le_bytes(buf[6..8].try_into().ok()?);
    let payload_len = u32::from_le_bytes(buf[8..12].try_into().ok()?);

    Some(drvproto::DriverHeader {
        magic,
        version,
        msg_type,
        payload_len,
    })
}
