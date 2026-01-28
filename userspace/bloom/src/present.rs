// Presenter implementations for Bloom compositor
//
// Presenters handle the final step of getting rendered frames to the display.
// They receive damage information to potentially optimize transfers.
//
// Frame Transaction API:
// - `acquire_frame()`: Acquire a frame slot with asset generation snapshot
// - `present_frame()`: Present a completed frame (consumes token)

use abi::display_driver_protocol as drvproto;
use abi::display_driver_protocol::BindPayload;
use abi::driver_frame::FrameReader;
use stem::info;
use stem::syscall::{port_recv, port_send, PortHandle};

use crate::damage::Damage;
use crate::frame::{AssetGeneration, FrameSpec, FrameToken, PresentStats};

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
    #[allow(dead_code)]
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
    frames: FrameReader<4096>,
    registered: bool,
    awaiting_bind_ack: bool,
    frame_count: u64,
}

impl DriverPresenter {
    pub fn new(req_write: PortHandle, resp_read: PortHandle) -> Self {
        Self {
            req_write,
            resp_read,
            frames: FrameReader::new(),
            registered: false,
            awaiting_bind_ack: false,
            frame_count: 0,
        }
    }


    pub fn send_bind(&mut self, payload: &BindPayload) {
        let mut bytes = [0u8; drvproto::BIND_PAYLOAD_WIRE_SIZE];
        if drvproto::encode_bind_payload_le(payload, &mut bytes).is_none() {
            return;
        }

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

        // Always send explicit damage rectangles
        // (Even for Damage::full, which contains a single rect covering the bounds)
        let rect_count = damage.rect_count() as u32;

        // Send message
        // Encode message buffer needs to be large enough for header + payload
        // DriverHeader (12) + Payload (136) = 148
        let mut buf = [0u8; 256];
        let payload_len = drvproto::encode_present_payload_le(
            rect_count,
            damage.iter().map(|r| abi::display_driver_protocol::Rect {
                x: r.x.max(0) as u32,
                y: r.y.max(0) as u32,
                w: r.w.max(0) as u32,
                h: r.h.max(0) as u32,
            }),
            &mut payload,
        );

        if let Some(payload_len) = payload_len {
            if let Some(len) =
                drvproto::encode_message(&mut buf, drvproto::MSG_PRESENT, &payload[..payload_len])
            {
                let _ = port_send(self.req_write, &buf[..len]);
            }
        }
    }

    fn pump_port(&mut self) {
        let mut temp = [0u8; 256];
        loop {
            let n = match port_recv(self.resp_read, &mut temp) {
                Ok(n) => n,
                Err(_) => break,
            };
            if n == 0 {
                break;
            }
            self.frames.push(&temp[..n]);
        }
    }

    fn handle_message(
        registered: &mut bool,
        awaiting_bind_ack: &mut bool,
        msg_type: u16,
        payload: &[u8],
    ) {
        match msg_type {
            drvproto::MSG_REGISTER => {
                if let Some(reg) = drvproto::decode_register_payload_le(payload) {
                    let driver_kind = reg.driver_kind;
                    let caps = reg.caps;
                    info!(
                        "bloom: driver REGISTER (kind={} caps=0x{:x})",
                        driver_kind, caps
                    );
                } else {
                    info!("bloom: driver REGISTER (payload too small)");
                }
                *registered = true;
            }
            drvproto::MSG_ACK => {
                if *awaiting_bind_ack {
                    info!("bloom: driver BIND ACK");
                    *awaiting_bind_ack = false;
                }
                // Silently accept PRESENT ACKs (high frequency)
            }
            drvproto::MSG_ERR => {
                let code = drvproto::decode_err_resp_le(payload)
                    .map(|err| err.code)
                    .unwrap_or(0);
                if *awaiting_bind_ack {
                    info!("bloom: driver BIND ERR code={}", code);
                    *awaiting_bind_ack = false;
                } else {
                    info!("bloom: driver PRESENT ERR code={}", code);
                }
            }
            _ => {}
        }
    }

    fn process_rx(&mut self) {
        let frames = &mut self.frames;
        let registered = &mut self.registered;
        let awaiting_bind_ack = &mut self.awaiting_bind_ack;
        while let Some((header, payload)) = frames.next_message() {
            Self::handle_message(registered, awaiting_bind_ack, header.msg_type, payload);
        }
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
            let _mem_used = reclaimer::decoded_bytes();
            let _mem_budget = reclaimer::memory_budget();
            let _evictions = reclaimer::eviction_count();
            let _in_flight = reclaimer::in_flight_count();
            let _min_gen = reclaimer::min_live_gen();
            
            /*
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
            */
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

        /*
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
        */

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

    #[allow(dead_code)]
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
