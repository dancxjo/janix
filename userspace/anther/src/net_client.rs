//! Network client for anther
//!
//! Provides simplified TCP socket operations by communicating with netd's socket API.

use alloc::vec::Vec;
use stem::syscall::port::{port_recv, port_send, PortHandle};
use stem::thing::sys as thingsys;
use stem::thing::ThingId;
use stem::{info, warn, trace};

// Socket API message types (must match netd's socket_api.rs)
const MSG_TCP_LISTEN: u16 = 0x0204;
const MSG_TCP_ACCEPT: u16 = 0x0205;
const MSG_TCP_SEND: u16 = 0x0201;
const MSG_TCP_RECV: u16 = 0x0202;
const MSG_TCP_CLOSE: u16 = 0x0203;

// Response types
const RESP_OK: u16 = 0x0000;
const RESP_ERROR: u16 = 0x0001;
const RESP_HANDLE: u16 = 0x0002;
const RESP_DATA: u16 = 0x0003;
const RESP_ACCEPT: u16 = 0x0004;
const RESP_EMPTY: u16 = 0x0005;

/// A connection to netd's socket API
pub struct NetClient {
    /// Port to send requests to netd (netd's read port)
    netd_write_port: PortHandle,
    /// Our write port (netd sends responses here)
    our_write_port: PortHandle,
    /// Our read port (we read responses from here)
    our_read_port: PortHandle,
}

/// Result of an accept operation
pub struct AcceptResult {
    pub conn_handle: u32,
    pub remote_ip: [u8; 4],
    pub remote_port: u16,
}

impl NetClient {
    /// Connect to netd's socket API
    pub fn connect() -> Option<Self> {
        // Find the network stack service
        let mut buf = [ThingId::default(); 1];
        let count = thingsys::find("svc.net.Stack", &mut buf).ok()?;
        
        if count == 0 {
            warn!("anther: Network stack not found");
            return None;
        }
        
        let net_id = buf[0];
        
        // Get netd's socket API write port (we send to this)
        let netd_write_port = thingsys::prop_get(net_id, "net.socket_api").ok()? as PortHandle;
        
        // Create our own port pair for receiving responses
        // We give netd our write port so it can send responses to us
        let (our_write, our_read) = stem::syscall::port_create(8192).ok()?;
        
        info!("anther: Connected to netd socket API (netd_port={}, our_write={}, our_read={})", 
              netd_write_port, our_write, our_read);
        
        Some(Self {
            netd_write_port,
            our_write_port: our_write,
            our_read_port: our_read,
        })
    }

    /// Build a message with our response port prepended
    fn build_msg(&self, msg_type: u16, payload: &[u8]) -> Vec<u8> {
        // Message format: [4: response_port][2: msg_type][payload...]
        let mut msg = Vec::with_capacity(6 + payload.len());
        msg.extend_from_slice(&(self.our_write_port as u32).to_le_bytes());
        msg.extend_from_slice(&msg_type.to_le_bytes());
        msg.extend_from_slice(payload);
        msg
    }

    /// Start listening on a TCP port
    pub fn tcp_listen(&self, port: u16) -> Option<u32> {
        let mut payload = Vec::with_capacity(4);
        payload.extend_from_slice(&port.to_le_bytes());
        payload.extend_from_slice(&64u16.to_le_bytes()); // backlog
        
        let msg = self.build_msg(MSG_TCP_LISTEN, &payload);

        if let Err(e) = port_send(self.netd_write_port, &msg) {
            warn!("anther: Failed to send TCP_LISTEN: {:?}", e);
            return None;
        }

        // Wait for response
        let mut resp_buf = [0u8; 64];
        for i in 0..100 {
            match port_recv(self.our_read_port, &mut resp_buf) {
                Ok(len) if len >= 6 => {
                    let resp_type = u16::from_le_bytes([resp_buf[0], resp_buf[1]]);
                    if resp_type == RESP_HANDLE {
                        let handle = u32::from_le_bytes([resp_buf[2], resp_buf[3], resp_buf[4], resp_buf[5]]);
                        return Some(handle);
                    } else if resp_type == RESP_ERROR {
                        warn!("anther: TCP_LISTEN returned error");
                        return None;
                    }
                }
                _ => {
                    if i == 0 {
                        trace!("anther: TCP_LISTEN waiting...");
                    }
                    stem::time::sleep_ms(10);
                }
            }
        }
        
        warn!("anther: TCP_LISTEN timeout");
        None
    }

    /// Accept a connection on a listening socket (non-blocking)
    pub fn tcp_accept(&self, listen_handle: u32) -> Option<AcceptResult> {
        let msg = self.build_msg(MSG_TCP_ACCEPT, &listen_handle.to_le_bytes());

        if let Err(e) = port_send(self.netd_write_port, &msg) {
            warn!("anther: Failed to send TCP_ACCEPT: {:?}", e);
            return None;
        }

        // Wait for response
        let mut resp_buf = [0u8; 64];
        for _ in 0..10 {
            match port_recv(self.our_read_port, &mut resp_buf) {
                Ok(len) if len >= 2 => {
                    let resp_type = u16::from_le_bytes([resp_buf[0], resp_buf[1]]);
                    if resp_type == RESP_ACCEPT && len >= 12 {
                        let conn_handle = u32::from_le_bytes([resp_buf[2], resp_buf[3], resp_buf[4], resp_buf[5]]);
                        let remote_ip = [resp_buf[6], resp_buf[7], resp_buf[8], resp_buf[9]];
                        let remote_port = u16::from_le_bytes([resp_buf[10], resp_buf[11]]);
                        return Some(AcceptResult {
                            conn_handle,
                            remote_ip,
                            remote_port,
                        });
                    } else if resp_type == RESP_EMPTY {
                        // No connection ready
                        return None;
                    } else if resp_type == RESP_ERROR {
                        return None;
                    }
                }
                _ => {
                    stem::time::sleep_ms(5);
                }
            }
        }
        
        trace!("anther: TCP_ACCEPT no response yet");
        None
    }

    /// Receive data from a socket (non-blocking)
    pub fn tcp_recv(&self, handle: u32, max_len: u16) -> Option<Vec<u8>> {
        let mut payload = Vec::with_capacity(6);
        payload.extend_from_slice(&handle.to_le_bytes());
        payload.extend_from_slice(&max_len.to_le_bytes());
        
        let msg = self.build_msg(MSG_TCP_RECV, &payload);

        if let Err(_) = port_send(self.netd_write_port, &msg) {
            return None;
        }

        // Wait for response (non-blocking)
        let mut resp_buf = [0u8; 4096];
        for _ in 0..5 {
            match port_recv(self.our_read_port, &mut resp_buf) {
                Ok(len) if len >= 2 => {
                    let resp_type = u16::from_le_bytes([resp_buf[0], resp_buf[1]]);
                    if resp_type == RESP_DATA && len > 2 {
                        let data = resp_buf[2..len].to_vec();
                        return Some(data);
                    } else if resp_type == RESP_EMPTY || resp_type == RESP_ERROR || (resp_type == RESP_DATA && len == 2) {
                        return None;
                    } else {
                        warn!("anther: tcp_recv got unexpected resp_type 0x{:04x} len={}", resp_type, len);
                    }
                }
                _ => {
                    stem::time::sleep_ms(1);
                }
            }
        }
        
        // Timeout or unexpected response would end up here if we didn't return above.
        // But wait, the loop above only handles a few iterations. 
        // If we drained some other message, we might want to log it.
        None
    }

    /// Send data on a socket
    pub fn tcp_send(&self, handle: u32, data: &[u8]) -> usize {
        let mut payload = Vec::with_capacity(4 + data.len());
        payload.extend_from_slice(&handle.to_le_bytes());
        payload.extend_from_slice(data);
        
        let msg = self.build_msg(MSG_TCP_SEND, &payload);

        if let Err(_) = port_send(self.netd_write_port, &msg) {
            return 0;
        }

        // Wait for response
        let mut resp_buf = [0u8; 64];
        for _ in 0..50 {
            match port_recv(self.our_read_port, &mut resp_buf) {
                Ok(len) if len >= 4 => {
                    let resp_type = u16::from_le_bytes([resp_buf[0], resp_buf[1]]);
                    if resp_type == RESP_OK {
                        let sent = u16::from_le_bytes([resp_buf[2], resp_buf[3]]);
                        return sent as usize;
                    } else {
                        warn!("anther: tcp_send got unexpected resp_type 0x{:04x} len={}", resp_type, len);
                    }
                }
                _ => {
                    stem::time::sleep_ms(5);
                }
            }
        }
        
        0
    }

    /// Close a socket
    pub fn tcp_close(&self, handle: u32) {
        let msg = self.build_msg(MSG_TCP_CLOSE, &handle.to_le_bytes());

        let _ = port_send(self.netd_write_port, &msg);
        
        // Drain any response
        let mut resp_buf = [0u8; 64];
        for _ in 0..10 {
            if port_recv(self.our_read_port, &mut resp_buf).is_ok() {
                break;
            }
            stem::time::sleep_ms(5);
        }
    }
}
