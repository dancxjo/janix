use abi::display_driver_protocol as drvproto;
use abi::display_driver_protocol::{BindPayload, ErrResp, RegisterPayload};
use stem::info;
use stem::syscall::{port_recv, port_send, PortHandle};

pub trait Presenter {
    fn present(&mut self);
    fn pump(&mut self);
}

pub struct NullPresenter;

impl Presenter for NullPresenter {
    fn present(&mut self) {}
    fn pump(&mut self) {}
}

pub struct DriverPresenter {
    req_write: PortHandle,
    resp_read: PortHandle,
    rx_buf: [u8; 1024],
    rx_len: usize,
    registered: bool,
    awaiting_bind_ack: bool,
}

impl DriverPresenter {
    pub fn new(req_write: PortHandle, resp_read: PortHandle) -> Self {
        Self {
            req_write,
            resp_read,
            rx_buf: [0u8; 1024],
            rx_len: 0,
            registered: false,
            awaiting_bind_ack: false,
        }
    }

    pub fn wait_for_register(&mut self) {
        loop {
            self.pump();
            if self.registered {
                break;
            }
            stem::yield_now();
        }
    }

    pub fn send_bind(&mut self, payload: &BindPayload) {
        let mut bytes = [0u8; core::mem::size_of::<BindPayload>()];
        bytes[0..8].copy_from_slice(&payload.bytespace_id.to_le_bytes());
        bytes[8..12].copy_from_slice(&payload.width.to_le_bytes());
        bytes[12..16].copy_from_slice(&payload.height.to_le_bytes());
        bytes[16..20].copy_from_slice(&payload.stride.to_le_bytes());
        bytes[20..24].copy_from_slice(&payload.format.to_le_bytes());

        let mut buf = [0u8; 128];
        if let Some(len) = drvproto::encode_message(&mut buf, drvproto::MSG_BIND, &bytes) {
            let _ = port_send(self.req_write, &buf[..len]);
            self.awaiting_bind_ack = true;
        }
    }

    fn send_present(&mut self) {
        let mut buf = [0u8; 64];
        if let Some(len) = drvproto::encode_message(&mut buf, drvproto::MSG_PRESENT, &[]) {
            let _ = port_send(self.req_write, &buf[..len]);
        }
    }

    fn pump_port(&mut self) {
        let mut temp = [0u8; 256];
        loop {
            let n = match port_recv(self.resp_read, &mut temp) {
                Ok(n) => n,
                Err(_) => break,
            };
            let remaining = self.rx_buf.len().saturating_sub(self.rx_len);
            let to_copy = n.min(remaining);
            if to_copy > 0 {
                self.rx_buf[self.rx_len..self.rx_len + to_copy]
                    .copy_from_slice(&temp[..to_copy]);
                self.rx_len += to_copy;
            } else {
                self.rx_len = 0;
                break;
            }
        }
    }

    fn handle_message(&mut self, msg_type: u16, payload_ptr: *const u8, payload_len: usize) {
        match msg_type {
            drvproto::MSG_REGISTER => {
                if payload_len >= core::mem::size_of::<RegisterPayload>() {
                    let reg: RegisterPayload = unsafe {
                        core::ptr::read_unaligned(payload_ptr as *const RegisterPayload)
                    };
                    let driver_kind =
                        unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(reg.driver_kind)) };
                    let caps =
                        unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(reg.caps)) };
                    info!(
                        "bloom: driver REGISTER (kind={} caps=0x{:x})",
                        driver_kind, caps
                    );
                } else {
                    info!("bloom: driver REGISTER (payload too small)");
                }
                self.registered = true;
            }
            drvproto::MSG_ACK => {
                if self.awaiting_bind_ack {
                    info!("bloom: driver BIND ACK");
                    self.awaiting_bind_ack = false;
                } else {
                    info!("bloom: driver PRESENT ACK");
                }
            }
            drvproto::MSG_ERR => {
                let code = if payload_len >= core::mem::size_of::<ErrResp>() {
                    let err: ErrResp = unsafe {
                        core::ptr::read_unaligned(payload_ptr as *const ErrResp)
                    };
                    unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(err.code)) }
                } else {
                    0
                };
                if self.awaiting_bind_ack {
                    info!("bloom: driver BIND ERR code={}", code);
                    self.awaiting_bind_ack = false;
                } else {
                    info!("bloom: driver PRESENT ERR code={}", code);
                }
            }
            _ => {}
        }
    }

    fn process_rx(&mut self) {
        loop {
            if self.rx_len < drvproto::HEADER_SIZE {
                break;
            }

            let magic = u32::from_le_bytes(self.rx_buf[0..4].try_into().unwrap());
            let version = u16::from_le_bytes(self.rx_buf[4..6].try_into().unwrap());
            if magic != drvproto::DRIVER_MAGIC || version != drvproto::DRIVER_VERSION {
                self.shift_rx(1);
                continue;
            }
            let msg_type = u16::from_le_bytes(self.rx_buf[6..8].try_into().unwrap());
            let payload_len = u32::from_le_bytes(self.rx_buf[8..12].try_into().unwrap()) as usize;
            let total = drvproto::HEADER_SIZE + payload_len;
            if total > self.rx_buf.len() {
                self.rx_len = 0;
                break;
            }
            if self.rx_len < total {
                break;
            }

            let payload_ptr = unsafe { self.rx_buf.as_ptr().add(drvproto::HEADER_SIZE) };
            self.handle_message(msg_type, payload_ptr, payload_len);
            self.shift_rx(total);
        }
    }

    fn shift_rx(&mut self, count: usize) {
        if count >= self.rx_len {
            self.rx_len = 0;
            return;
        }
        let remaining = self.rx_len - count;
        for i in 0..remaining {
            self.rx_buf[i] = self.rx_buf[count + i];
        }
        self.rx_len = remaining;
    }
}

impl Presenter for DriverPresenter {
    fn present(&mut self) {
        self.send_present();
    }

    fn pump(&mut self) {
        self.pump_port();
        self.process_rx();
    }
}

pub enum PresenterImpl {
    Null(NullPresenter),
    Driver(DriverPresenter),
}

impl PresenterImpl {
    pub fn present(&mut self) {
        match self {
            PresenterImpl::Null(inner) => inner.present(),
            PresenterImpl::Driver(inner) => inner.present(),
        }
    }

    pub fn pump(&mut self) {
        match self {
            PresenterImpl::Null(inner) => inner.pump(),
            PresenterImpl::Driver(inner) => inner.pump(),
        }
    }
}
