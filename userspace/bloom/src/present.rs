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
use alloc::string::String;
use alloc::vec::Vec;
use stem::info;
use stem::syscall::{port_recv, port_send, PortHandle};

use crate::damage::Damage;
use crate::frame::{AssetGeneration, FrameSpec, FrameToken, PresentDamageSnapshot, PresentStats};
use crate::reclaimer;
use crate::state::OverlayMode;

#[derive(Clone, Copy, Debug)]
struct DriverNegotiation {
    proto_major: u16,
    proto_minor: u16,
    caps: u32,
    max_rects: u16,
}

#[derive(Clone, Copy, Debug)]
pub struct DisplayNegotiation {
    pub caps: u32,
    pub max_rects: u16,
}

pub(crate) struct PresentStrategy {
    pub use_rects: bool,
    pub flags: u32,
    pub reason: &'static str,
    pub mode: OverlayMode,
}

pub(crate) fn evaluate_present_strategy(
    negotiation: Option<DisplayNegotiation>,
    snapshot: &PresentDamageSnapshot,
) -> PresentStrategy {
    if snapshot.is_full() {
        return PresentStrategy {
            use_rects: false,
            flags: drvproto::PRESENT_FLAG_FULLFRAME,
            reason: "full-frame damage",
            mode: OverlayMode::Fullframe,
        };
    }

    if let Some(neg) = negotiation {
        let rect_count = snapshot.len();
        if (neg.caps & drvproto::CAP_DIRTY_RECTS != 0)
            && rect_count <= neg.max_rects as usize
            && !snapshot.overflowed()
        {
            PresentStrategy {
                use_rects: true,
                flags: 0,
                reason: "DIRTY_RECTS",
                mode: OverlayMode::DirtyRects,
            }
        } else {
            let reason = if snapshot.overflowed() {
                "damage overflow"
            } else if rect_count > neg.max_rects as usize {
                "rects exceed max_rects"
            } else {
                "driver lacks DIRTY_RECTS"
            };
            PresentStrategy {
                use_rects: false,
                flags: drvproto::PRESENT_FLAG_FULLFRAME,
                reason,
                mode: OverlayMode::Fullframe,
            }
        }
    } else {
        PresentStrategy {
            use_rects: false,
            flags: drvproto::PRESENT_FLAG_FULLFRAME,
            reason: "no negotiation",
            mode: OverlayMode::Fullframe,
        }
    }
}

fn caps_to_string(caps: u32) -> String {
    let mut out = String::new();
    let mut first = true;
    let mut push = |name: &str, out: &mut String, first: &mut bool| {
        if !*first {
            out.push('|');
        }
        out.push_str(name);
        *first = false;
    };

    if caps & drvproto::CAP_DIRTY_RECTS != 0 {
        push("DIRTY_RECTS", &mut out, &mut first);
    }
    if caps & drvproto::CAP_FULLFRAME != 0 {
        push("FULLFRAME", &mut out, &mut first);
    }
    if caps & drvproto::CAP_MULTI_DISPLAY != 0 {
        push("MULTI_DISPLAY", &mut out, &mut first);
    }
    if caps & drvproto::CAP_FENCE != 0 {
        push("FENCE", &mut out, &mut first);
    }

    if out.is_empty() {
        out.push_str("NONE");
    }

    out
}

const MAX_LOCAL_DAMAGE_RECTS: usize = 256;
const MAX_PRESENT_PAYLOAD_BYTES: usize =
    drvproto::PRESENT_HEADER_WIRE_SIZE + drvproto::RECT_WIRE_SIZE * MAX_LOCAL_DAMAGE_RECTS;
const MAX_PRESENT_MESSAGE_BYTES: usize = drvproto::HEADER_SIZE + MAX_PRESENT_PAYLOAD_BYTES;

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

    /// Get negotiated display capabilities (if available).
    fn negotiation_info(&self) -> Option<DisplayNegotiation>;
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

    fn negotiation_info(&self) -> Option<DisplayNegotiation> {
        None
    }
}

pub struct DriverPresenter {
    req_write: PortHandle,
    resp_read: PortHandle,
    frames: FrameReader<4096>,
    awaiting_bind_ack: bool,
    negotiation: Option<DriverNegotiation>,
    pending_bind: Option<BindPayload>,
    fallback_warned: bool,
    unknown_msg_logged: bool,
    frame_count: u64,
}

impl DriverPresenter {
    pub fn new(req_write: PortHandle, resp_read: PortHandle) -> Self {
        Self {
            req_write,
            resp_read,
            frames: FrameReader::new(),
            awaiting_bind_ack: false,
            negotiation: None,
            pending_bind: None,
            fallback_warned: false,
            unknown_msg_logged: false,
            frame_count: 0,
        }
    }

    pub fn start_handshake(&mut self) {
        let hello = drvproto::HelloPayload {
            proto_major: drvproto::PROTO_MAJOR,
            proto_minor: drvproto::PROTO_MINOR,
            want_caps: drvproto::CAP_DIRTY_RECTS | drvproto::CAP_FULLFRAME,
        };
        let mut hello_bytes = [0u8; drvproto::HELLO_PAYLOAD_WIRE_SIZE];
        if let Some(len) = drvproto::encode_hello_payload_le(&hello, &mut hello_bytes) {
            let mut buf = [0u8; 128];
            if let Some(total) =
                drvproto::encode_message(&mut buf, drvproto::MSG_HELLO, &hello_bytes[..len])
            {
                let _ = port_send(self.req_write, &buf[..total]);
            }
        }
    }

    fn send_bind_now(&mut self, payload: &BindPayload) {
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

    pub fn send_bind(&mut self, payload: &BindPayload) {
        if self.negotiation.is_some() {
            self.send_bind_now(payload);
        } else {
            self.pending_bind = Some(*payload);
        }
    }

    fn send_present(&mut self, snapshot: &PresentDamageSnapshot) -> usize {
        let mut payload = [0u8; MAX_PRESENT_PAYLOAD_BYTES];
        let mut buf = [0u8; MAX_PRESENT_MESSAGE_BYTES];
        let strategy = evaluate_present_strategy(self.negotiation_info(), snapshot);

        if !strategy.use_rects {
            self.log_fullframe_fallback(strategy.reason);
            self.encode_and_send(
                0,
                strategy.flags,
                core::iter::empty::<abi::display_driver_protocol::Rect>(),
                &mut payload,
                &mut buf,
            );
            return 0;
        }

        let rects = snapshot.rects();
        let rect_count = rects.len() as u32;
        self.encode_and_send(
            rect_count,
            strategy.flags,
            rects.iter().map(|r| abi::display_driver_protocol::Rect {
                x: r.x().max(0) as u32,
                y: r.y().max(0) as u32,
                w: r.width().max(0) as u32,
                h: r.height().max(0) as u32,
            }),
            &mut payload,
            &mut buf,
        );
        rect_count as usize
    }

    fn encode_and_send<I>(
        &mut self,
        rect_count: u32,
        flags: u32,
        rects: I,
        payload: &mut [u8],
        buf: &mut [u8],
    ) where
        I: IntoIterator<Item = abi::display_driver_protocol::Rect>,
    {
        if let Some(payload_len) =
            drvproto::encode_present_payload_with_flags_le(rect_count, flags, rects, payload)
        {
            if let Some(len) =
                drvproto::encode_message(buf, drvproto::MSG_PRESENT, &payload[..payload_len])
            {
                let _ = port_send(self.req_write, &buf[..len]);
            }
        }
    }

    fn log_fullframe_fallback(&mut self, reason: &str) {
        if !self.fallback_warned {
            info!("display: {}, using full-frame present", reason);
            self.fallback_warned = true;
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

    fn apply_negotiation(&mut self, negotiation: DriverNegotiation) {
        if self.negotiation.is_some() {
            return;
        }
        let caps_string = caps_to_string(negotiation.caps);
        info!(
            "display: negotiated proto {}.{} caps={} max_rects={}",
            negotiation.proto_major, negotiation.proto_minor, caps_string, negotiation.max_rects
        );
        self.negotiation = Some(negotiation);
        if let Some(bind) = self.pending_bind.take() {
            self.send_bind_now(&bind);
        }
    }

    fn handle_message(&mut self, msg_type: u16, payload: &[u8]) {
        match msg_type {
            drvproto::MSG_WELCOME => {
                if let Some(welcome) = drvproto::decode_welcome_payload_le(payload) {
                    self.apply_negotiation(DriverNegotiation {
                        proto_major: welcome.proto_major,
                        proto_minor: welcome.proto_minor,
                        caps: welcome.have_caps,
                        max_rects: welcome.max_rects,
                    });
                } else {
                    info!("display: WELCOME payload too small");
                }
            }
            drvproto::MSG_REGISTER => {
                if let Some(reg) = drvproto::decode_register_payload_le(payload) {
                    let driver_kind = reg.driver_kind;
                    let caps = reg.caps;
                    info!(
                        "bloom: driver REGISTER (kind={} caps=0x{:x})",
                        driver_kind, caps
                    );
                    self.apply_negotiation(DriverNegotiation {
                        proto_major: drvproto::PROTO_MAJOR,
                        proto_minor: drvproto::PROTO_MINOR,
                        caps,
                        max_rects: crate::damage::MAX_RECTS as u16,
                    });
                } else {
                    info!("bloom: driver REGISTER (payload too small)");
                }
            }
            drvproto::MSG_ACK => {
                if self.awaiting_bind_ack {
                    info!("bloom: driver BIND ACK");
                    self.awaiting_bind_ack = false;
                }
                // Silently accept PRESENT ACKs (high frequency)
            }
            drvproto::MSG_ERR => {
                let code = drvproto::decode_err_resp_le(payload)
                    .map(|err| err.code)
                    .unwrap_or(0);
                if self.awaiting_bind_ack {
                    info!("bloom: driver BIND ERR code={}", code);
                    self.awaiting_bind_ack = false;
                } else {
                    info!("bloom: driver PRESENT ERR code={}", code);
                }
            }
            _ => {
                if !self.unknown_msg_logged {
                    info!("display: ignoring unknown driver msg {}", msg_type);
                    self.unknown_msg_logged = true;
                }
            }
        }
    }

    fn process_rx(&mut self) {
        while let Some((header, payload)) = self.frames.next_message() {
            let msg_type = header.msg_type;
            let payload_copy: Vec<u8> = payload.to_vec();
            self.handle_message(msg_type, &payload_copy);
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
        let damage_rect_count = if fast_path {
            0
        } else {
            self.send_present(&token.present_damage)
        };

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

        let mut snapshot = PresentDamageSnapshot::new();
        snapshot.update_from_damage(damage);
        self.send_present(&snapshot);
    }

    fn pump(&mut self) {
        self.pump_port();
        self.process_rx();
    }

    fn negotiation_info(&self) -> Option<DisplayNegotiation> {
        self.negotiation.as_ref().map(|n| DisplayNegotiation {
            caps: n.caps,
            max_rects: n.max_rects,
        })
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

    pub fn negotiation_info(&self) -> Option<DisplayNegotiation> {
        match self {
            PresenterImpl::Null(inner) => inner.negotiation_info(),
            PresenterImpl::Driver(inner) => inner.negotiation_info(),
        }
    }
}
