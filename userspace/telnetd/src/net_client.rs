use abi::schema::keys;
use alloc::vec::Vec;
use stem::syscall::port::{port_recv, port_send_all, port_wait, PortHandle};
use stem::thing::sys as thingsys;
use stem::thing::ThingId;
use stem::{info, warn};

const MSG_TCP_LISTEN: u16 = 0x0204;
const MSG_TCP_ACCEPT: u16 = 0x0205;
const MSG_TCP_SEND: u16 = 0x0201;
const MSG_TCP_RECV: u16 = 0x0202;
const MSG_TCP_CLOSE: u16 = 0x0203;

const RESP_OK: u16 = 0x0000;
const RESP_ERROR: u16 = 0x0001;
const RESP_HANDLE: u16 = 0x0002;
const RESP_DATA: u16 = 0x0003;
const RESP_ACCEPT: u16 = 0x0004;
const RESP_EMPTY: u16 = 0x0005;
const RESP_CLOSED: u16 = 0x0006;

pub struct NetClient {
    netd_write_port: PortHandle,
    our_write_port: PortHandle,
    our_read_port: PortHandle,
}

pub struct AcceptResult {
    pub conn_handle: u32,
}

pub enum TcpRecvResult {
    Data(Vec<u8>),
    Empty,
    Closed,
}

impl NetClient {
    pub const MAX_RECV_LEN: u16 = 4000;

    pub fn connect() -> Option<Self> {
        let mut buf = [ThingId::default(); 1];
        let count = thingsys::find("svc.net.Stack", &mut buf).ok()?;
        if count == 0 {
            return None;
        }

        let net_id = buf[0];
        let netd_write_port =
            thingsys::prop_get(net_id, keys::WRITE_PORT_HANDLE).ok()? as PortHandle;
        let (our_write, our_read) = stem::syscall::port_create(16384).ok()?;
        info!(
            "telnetd: connected to socket API (netd={}, write={}, read={})",
            netd_write_port, our_write, our_read
        );
        Some(Self {
            netd_write_port,
            our_write_port: our_write,
            our_read_port: our_read,
        })
    }

    fn build_msg(&self, msg_type: u16, payload: &[u8]) -> Vec<u8> {
        let mut msg = Vec::with_capacity(16 + payload.len());
        msg.extend_from_slice(&(self.our_write_port as u32).to_le_bytes());
        let caller_tid = stem::syscall::get_tid().unwrap_or(0);
        msg.extend_from_slice(&caller_tid.to_le_bytes());
        msg.extend_from_slice(&msg_type.to_le_bytes());
        msg.extend_from_slice(&(payload.len() as u16).to_le_bytes());
        msg.extend_from_slice(payload);
        msg
    }

    fn send_api_msg(&self, msg: &[u8]) -> bool {
        let deadline_ms = (stem::time::now().as_millis() as u64).saturating_add(5_000);
        loop {
            match port_send_all(self.netd_write_port, msg) {
                Ok(n) if n == msg.len() => return true,
                Ok(_) => return false,
                Err(abi::errors::Errno::EAGAIN) => {
                    if (stem::time::now().as_millis() as u64) >= deadline_ms {
                        break;
                    }
                    let _ = port_wait(&[self.netd_write_port], abi::syscall::port_wait::WRITABLE);
                }
                Err(e) => {
                    warn!("telnetd: socket API send failed: {:?}", e);
                    return false;
                }
            }
        }
        false
    }

    fn wait_for_response<F, T>(&self, mut check: F) -> Option<T>
    where
        F: FnMut(&[u8]) -> Option<T>,
    {
        let deadline = (stem::time::now().as_millis() as u64).saturating_add(5_000);
        let mut resp_buf = [0u8; 4096 + 128];
        loop {
            match port_recv(self.our_read_port, &mut resp_buf) {
                Ok(len) => {
                    if let Some(res) = check(&resp_buf[..len]) {
                        return Some(res);
                    }
                }
                Err(abi::errors::Errno::EAGAIN) => {
                    if (stem::time::now().as_millis() as u64) > deadline {
                        return None;
                    }
                    let _ = port_wait(&[self.our_read_port], abi::syscall::port_wait::READABLE);
                }
                _ => {
                    stem::syscall::yield_now();
                }
            }
        }
    }

    pub fn drain_stale_responses(&self) {
        let mut buf = [0u8; 4096 + 128];
        for _ in 0..32 {
            if port_recv(self.our_read_port, &mut buf).is_err() {
                break;
            }
        }
    }

    pub fn tcp_listen(&self, port: u16) -> Option<u32> {
        self.drain_stale_responses();
        let mut payload = Vec::with_capacity(4);
        payload.extend_from_slice(&port.to_le_bytes());
        payload.extend_from_slice(&64u16.to_le_bytes());
        if !self.send_api_msg(&self.build_msg(MSG_TCP_LISTEN, &payload)) {
            return None;
        }

        self.wait_for_response(|resp_buf| {
            if resp_buf.len() >= 6 {
                let resp_type = u16::from_le_bytes([resp_buf[0], resp_buf[1]]);
                if resp_type == RESP_HANDLE {
                    return Some(Some(u32::from_le_bytes([
                        resp_buf[2],
                        resp_buf[3],
                        resp_buf[4],
                        resp_buf[5],
                    ])));
                }
                if resp_type == RESP_ERROR {
                    return Some(None);
                }
            }
            None
        }).flatten()
    }

    pub fn tcp_accept(&self, listen_handle: u32) -> Option<AcceptResult> {
        self.drain_stale_responses();
        if !self.send_api_msg(&self.build_msg(MSG_TCP_ACCEPT, &listen_handle.to_le_bytes())) {
            return None;
        }

        self.wait_for_response(|resp_buf| {
            if resp_buf.len() >= 2 {
                let resp_type = u16::from_le_bytes([resp_buf[0], resp_buf[1]]);
                if resp_type == RESP_ACCEPT && resp_buf.len() >= 12 {
                    return Some(Some(AcceptResult {
                        conn_handle: u32::from_le_bytes([
                            resp_buf[2],
                            resp_buf[3],
                            resp_buf[4],
                            resp_buf[5],
                        ]),
                    }));
                }
                if resp_type == RESP_EMPTY || resp_type == RESP_ERROR {
                    return Some(None);
                }
            }
            None
        }).flatten()
    }

    pub fn tcp_recv(&self, handle: u32, max_len: u16) -> TcpRecvResult {
        self.drain_stale_responses();
        let mut payload = Vec::with_capacity(6);
        payload.extend_from_slice(&handle.to_le_bytes());
        payload.extend_from_slice(&max_len.to_le_bytes());
        if !self.send_api_msg(&self.build_msg(MSG_TCP_RECV, &payload)) {
            return TcpRecvResult::Empty;
        }

        self.wait_for_response(|resp_buf| {
            if resp_buf.len() >= 2 {
                let resp_type = u16::from_le_bytes([resp_buf[0], resp_buf[1]]);
                if resp_type == RESP_DATA && resp_buf.len() > 2 {
                    return Some(TcpRecvResult::Data(resp_buf[2..].to_vec()));
                }
                if resp_type == RESP_EMPTY
                    || resp_type == RESP_ERROR
                    || (resp_type == RESP_DATA && resp_buf.len() == 2)
                {
                    return Some(TcpRecvResult::Empty);
                }
                if resp_type == RESP_CLOSED {
                    return Some(TcpRecvResult::Closed);
                }
            }
            None
        }).unwrap_or(TcpRecvResult::Empty)
    }

    pub fn tcp_send(&self, handle: u32, data: &[u8]) -> usize {
        let mut total_sent = 0usize;

        self.drain_stale_responses();
        for chunk in data.chunks(4000) {
            let mut payload = Vec::with_capacity(4 + chunk.len());
            payload.extend_from_slice(&handle.to_le_bytes());
            payload.extend_from_slice(chunk);
            if !self.send_api_msg(&self.build_msg(MSG_TCP_SEND, &payload)) {
                break;
            }

            let sent_this_chunk = self.wait_for_response(|resp_buf| {
                if resp_buf.len() >= 4 {
                    let resp_type = u16::from_le_bytes([resp_buf[0], resp_buf[1]]);
                    if resp_type == RESP_OK {
                        return Some(Some(u16::from_le_bytes([resp_buf[2], resp_buf[3]]) as usize));
                    }
                    if resp_type == RESP_ERROR {
                        return Some(None);
                    }
                }
                None
            }).flatten();

            match sent_this_chunk {
                Some(sent) => {
                    total_sent += sent;
                    if sent < chunk.len() {
                        break;
                    }
                }
                None => break,
            }
        }

        total_sent
    }

    pub fn tcp_close(&self, handle: u32) {
        let _ = self.send_api_msg(&self.build_msg(MSG_TCP_CLOSE, &handle.to_le_bytes()));
    }
}
