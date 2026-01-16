//! Presenter implementations for Bloom compositor
//!
//! Presenters handle the final step of getting rendered frames to the display.
//! They receive damage information to potentially optimize transfers.
//!
//! Frame Transaction API:
//! - `acquire_frame()`: Acquire a frame slot with asset generation snapshot
//! - `present_frame()`: Present a completed frame (consumes token)

use abi::display_driver_protocol as drvproto;
use abi::display_driver_protocol::{BindPayload, ErrResp, RegisterPayload};
use stem::info;
use stem::syscall::{port_recv, port_send, PortHandle};

use crate::damage::Damage;
use crate::frame::{AssetGeneration, FrameSpec, FrameToken, PresentStats};
use crate::log;
use crate::reclaimer;

/// Presenter trait with transactional frame API
pub trait Presenter {
    /// Acquire a frame slot, snapshotting current asset generation.
    /// Returns a token that must be consumed by present_frame().
    fn acquire_frame(&mut self, spec: FrameSpec, asset_gen: AssetGeneration) -> FrameToken;
    
    /// Present a completed frame (consumes token).
    /// Returns statistics about the presentation.
    fn present_frame(&mut self, token: FrameToken) -> PresentStats;
    
    /// Legacy present method (deprecated, use present_frame)
    fn present(&mut self, damage: &Damage);
    
    /// Pump the message queue for driver communication.
    fn pump(&mut self);
}

pub struct NullPresenter;

impl Presenter for NullPresenter {
    fn acquire_frame(&mut self, spec: FrameSpec, asset_gen: AssetGeneration) -> FrameToken {
        static FRAME_COUNTER: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);
        let frame_id = FRAME_COUNTER.fetch_add(1, core::sync::atomic::Ordering::Relaxed) + 1;
        
        // Register in-flight frame for safe eviction
        reclaimer::register_in_flight(frame_id, asset_gen);
        
        FrameToken::new(frame_id, asset_gen, spec)
    }
    
    fn present_frame(&mut self, token: FrameToken) -> PresentStats {
        let ops_count = token.ops.iter().count();
        let damage_rect_count = token.damage.rect_count();
        let frame_id = token.frame_id;
        let asset_gen = token.asset_gen;
        
        // Complete in-flight frame
        reclaimer::complete_in_flight(frame_id);
        
        PresentStats {
            frame_id,
            asset_gen,
            ops_count,
            damage_rect_count,
            fast_path_taken: token.damage.is_empty(),
        }
    }
    
    fn present(&mut self, _damage: &Damage) {}
    fn pump(&mut self) {}
}

pub struct DriverPresenter {
    req_write: PortHandle,
    resp_read: PortHandle,
    rx_buf: [u8; 1024],
    rx_len: usize,
    registered: bool,
    awaiting_bind_ack: bool,
    frame_count: u64,
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
            frame_count: 0,
        }
    }

    pub fn wait_for_register(&mut self) {
        let mut loops = 0u32;
        loop {
            log!("[present] wait_for_register: loop={} registered={}", loops, self.registered);
            self.pump();
            if self.registered {
                log!("[present] registered SUCCESS");
                break;
            }
            if loops >= 200 {
                info!("bloom: driver REGISTER timeout; continuing");
                break;
            }
            loops += 1;
            stem::yield_now();
            stem::sleep_ms(10);
        }
    }

    pub fn wait_for_bind(&mut self) {
        let mut loops = 0u32;
        loop {
            self.pump();
            if !self.awaiting_bind_ack {
                break;
            }
            if loops >= 200 {
                info!("bloom: driver BIND timeout; continuing");
                break;
            }
            loops += 1;
            stem::yield_now();
            stem::sleep_ms(10);
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

    fn send_present(&mut self, damage: &Damage) {
        // Calculate payload size
        // Header: 8 bytes
        // Rects: 16 bytes each
        // Max 8 rects => 128 bytes
        // Total payload max: 136 bytes
        let mut payload = [0u8; 136];
        
        let mut rect_count = 0;
        let mut offset = 8; // Skip header for now

        // Always send explicit damage rectangles
        // (Even for Damage::full, which contains a single rect covering the bounds)
        rect_count = damage.rect_count() as u32;

        for r in damage.iter() {
            let abi_rect = abi::display_driver_protocol::Rect {
                x: r.x.max(0) as u32,
                y: r.y.max(0) as u32,
                w: r.w.max(0) as u32,
                h: r.h.max(0) as u32,
            };
            let r_bytes: [u8; 16] = unsafe { core::mem::transmute(abi_rect) };
            payload[offset..offset+16].copy_from_slice(&r_bytes);
            offset += 16;
        }

        // Write header
        let header = abi::display_driver_protocol::PresentHeader {
            rect_count,
            _pad: 0,
        };
        let h_bytes: [u8; 8] = unsafe { core::mem::transmute(header) };
        payload[0..8].copy_from_slice(&h_bytes);

        // Send message
        // Encode message buffer needs to be large enough for header + payload
        // DriverHeader (12) + Payload (136) = 148
        let mut buf = [0u8; 256];
        let payload_len = 8 + (rect_count as usize * 16);
        
        if let Some(len) = drvproto::encode_message(&mut buf, drvproto::MSG_PRESENT, &payload[..payload_len]) {
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
                }
                // Silently accept PRESENT ACKs (high frequency)
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
    fn acquire_frame(&mut self, spec: FrameSpec, asset_gen: AssetGeneration) -> FrameToken {
        self.frame_count += 1;
        
        // Register in-flight frame for safe eviction
        reclaimer::register_in_flight(self.frame_count, asset_gen);
        
        FrameToken::new(self.frame_count, asset_gen, spec)
    }
    
    fn present_frame(&mut self, token: FrameToken) -> PresentStats {
        let ops_count = token.ops.iter().count();
        let damage_rect_count = token.damage.rect_count();
        let fast_path = token.damage.is_empty();
        let frame_id = token.frame_id;
        let asset_gen = token.asset_gen;

        // Log damage stats periodically (every 120 frames = ~2 seconds at 60fps)
        if frame_id % 120 == 0 {
            let mem_used = reclaimer::decoded_bytes();
            let mem_budget = reclaimer::memory_budget();
            let evictions = reclaimer::eviction_count();
            let in_flight = reclaimer::in_flight_count();
            let min_gen = reclaimer::min_live_gen();
            
            if token.damage.is_full {
                info!("bloom: frame {} gen={} (full redraw) mem={}/{}b evictions={} in_flight={} min_gen={}", 
                    frame_id, asset_gen.0, mem_used, mem_budget, evictions, in_flight, min_gen.0);
            } else if damage_rect_count == 0 {
                info!("bloom: frame {} gen={} (no damage - idle) mem={}/{}b", 
                    frame_id, asset_gen.0, mem_used, mem_budget);
            } else {
                info!("bloom: frame {} gen={} ({} damage rects) mem={}/{}b", 
                    frame_id, asset_gen.0, damage_rect_count, mem_used, mem_budget);
            }
        }

        // Complete in-flight frame before present
        reclaimer::complete_in_flight(frame_id);

        // Fast-path: skip present if no damage
        if !fast_path {
            self.send_present(&token.damage);
        }

        PresentStats {
            frame_id,
            asset_gen,
            ops_count,
            damage_rect_count,
            fast_path_taken: fast_path,
        }
    }

    fn present(&mut self, damage: &Damage) {
        // Legacy path - kept for compatibility during transition
        self.frame_count += 1;

        if self.frame_count % 120 == 0 {
            let rect_count = damage.rect_count();
            if damage.is_full {
                info!("bloom: presenter frame {} (full redraw)", self.frame_count);
            } else if rect_count == 0 {
                info!("bloom: presenter frame {} (no damage - idle)", self.frame_count);
            } else {
                info!("bloom: presenter frame {} ({} damage rects)", self.frame_count, rect_count);
            }
        }

        self.send_present(damage);
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
    pub fn acquire_frame(&mut self, spec: FrameSpec, asset_gen: AssetGeneration) -> FrameToken {
        match self {
            PresenterImpl::Null(inner) => inner.acquire_frame(spec, asset_gen),
            PresenterImpl::Driver(inner) => inner.acquire_frame(spec, asset_gen),
        }
    }
    
    pub fn present_frame(&mut self, token: FrameToken) -> PresentStats {
        match self {
            PresenterImpl::Null(inner) => inner.present_frame(token),
            PresenterImpl::Driver(inner) => inner.present_frame(token),
        }
    }

    pub fn present(&mut self, damage: &Damage) {
        match self {
            PresenterImpl::Null(inner) => inner.present(damage),
            PresenterImpl::Driver(inner) => inner.present(damage),
        }
    }

    pub fn pump(&mut self) {
        match self {
            PresenterImpl::Null(inner) => inner.pump(),
            PresenterImpl::Driver(inner) => inner.pump(),
        }
    }
}
