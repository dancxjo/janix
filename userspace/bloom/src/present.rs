// Presenter implementations for Bloom compositor
//
// Presenters handle the final step of getting rendered frames to the display.
// They receive damage information to potentially optimize transfers.
//
// Frame Transaction API:
// - `acquire_frame()`: Acquire a frame slot with asset generation snapshot
// - `present_frame()`: Present a completed frame (consumes token)

use abi::display_driver_protocol::{self as drvproto, BindPayload, OfferFramebufferPayload};
use abi::ThingId;
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

    /// Acquire a buffer for the next frame.
    /// Returns (bytespace_id, width, height, stride, format, buffer_age).
    fn acquire_buffer(&mut self) -> (ThingId, u32, u32, u32, u32, u32);

    /// Get negotiated display capabilities (if available).
    fn negotiation_info(&self) -> Option<DisplayNegotiation>;
}

pub struct NullPresenter;

impl Presenter for NullPresenter {
    fn acquire_frame(&mut self, _spec: FrameSpec, _asset_gen: AssetGeneration) -> FrameToken {
        unimplemented!("NullPresenter doesn't support transactional frames yet")
    }

    fn present_frame(&mut self, _token: FrameToken) -> PresentStats {
        unimplemented!("NullPresenter doesn't support transactional frames yet")
    }

    fn present(&mut self, _damage: &Damage) {}
    fn pump(&mut self) {}

    fn acquire_buffer(&mut self) -> (ThingId, u32, u32, u32, u32, u32) {
        (ThingId::default(), 0, 0, 0, 0, 0)
    }

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
    /// Damage history for buffer age expansion (last 4 frames)
    damage_history: Vec<Vec<crate::geometry::Rect>>,
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
            damage_history: Vec::new(),
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

    pub fn is_zero_copy(&self) -> bool {
        true // Now always zero-copy via swapchain
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
            drvproto::MSG_ACQUIRED => {
                // Handled in acquire_buffer sync loop
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

    /// Submit virgl 3D commands to the GPU driver.
    /// The command buffer is a byte slice containing virgl command stream.
    #[cfg(feature = "gpu")]
    pub fn send_submit_3d(&mut self, ctx_id: u32, cmd_buf: &[u8]) {
        // Allocate buffer for header + command data
        let total_payload = drvproto::SUBMIT_3D_HEADER_WIRE_SIZE + cmd_buf.len();
        let total_msg = drvproto::HEADER_SIZE + total_payload;
        
        // Use Vec for variable-size buffer
        let mut buf = alloc::vec![0u8; total_msg];
        let mut payload = alloc::vec![0u8; total_payload];
        
        // Encode Submit3d header
        let header = drvproto::Submit3dHeader {
            ctx_id,
            cmd_len: cmd_buf.len() as u32,
        };
        if drvproto::encode_submit_3d_header_le(&header, &mut payload).is_none() {
            return;
        }
        
        // Copy command buffer after header
        payload[drvproto::SUBMIT_3D_HEADER_WIRE_SIZE..].copy_from_slice(cmd_buf);
        
        // Encode and send message
        if let Some(len) = drvproto::encode_message(&mut buf, drvproto::MSG_SUBMIT_3D, &payload) {
            let _ = port_send(self.req_write, &buf[..len]);
        }
    }

    /// Check if 3D commands are supported by the driver.
    #[cfg(feature = "gpu")]
    pub fn has_3d_cap(&self) -> bool {
        self.negotiation.map(|n| n.caps & drvproto::CAP_3D != 0).unwrap_or(false)
    }

    /// Request the driver to create a GPU texture.
    /// Returns the resource_id from MSG_TEXTURE_CREATED response synchronously.
    #[cfg(feature = "gpu")]
    pub fn send_create_texture_3d(&mut self, client_id: u64, width: u32, height: u32, format: u32) -> Option<u32> {
        let header = drvproto::CreateTexture3dHeader {
            client_id,
            width,
            height,
            format,
            _pad: 0,
        };
        
        let mut payload = [0u8; drvproto::CREATE_TEXTURE_3D_HEADER_WIRE_SIZE];
        if drvproto::encode_create_texture_3d_header_le(&header, &mut payload).is_none() {
            return None;
        }
        
        let total_msg = drvproto::HEADER_SIZE + payload.len();
        let mut buf = alloc::vec![0u8; total_msg];
        if let Some(len) = drvproto::encode_message(&mut buf, drvproto::MSG_CREATE_TEXTURE_3D, &payload) {
            let _ = port_send(self.req_write, &buf[..len]);
        }
        
        // Wait for MSG_TEXTURE_CREATED response
        loop {
            self.pump_port();
            while let Some((hdr, payload_data)) = self.frames.next_message() {
                if hdr.msg_type == drvproto::MSG_TEXTURE_CREATED {
                    if let Some(resp) = drvproto::decode_texture_created_response_le(payload_data) {
                        if resp.client_id == client_id && resp.status == 0 {
                            return Some(resp.resource_id);
                        } else {
                            return None; // Error
                        }
                    }
                } else {
                    let payload_copy = payload_data.to_vec();
                    self.handle_message(hdr.msg_type, &payload_copy);
                }
            }
            stem::yield_now();
        }
    }

    /// Upload pixel data to an existing GPU texture.
    #[cfg(feature = "gpu")]
    pub fn send_upload_texture_3d(&mut self, resource_id: u32, width: u32, height: u32, stride: u32, data: &[u8]) {
        let header = drvproto::UploadTexture3dHeader {
            resource_id,
            width,
            height,
            stride,
            x: 0,
            y: 0,
            data_len: data.len() as u32,
            _pad: 0,
        };
        
        let total_payload = drvproto::UPLOAD_TEXTURE_3D_HEADER_WIRE_SIZE + data.len();
        let total_msg = drvproto::HEADER_SIZE + total_payload;
        
        let mut payload = alloc::vec![0u8; total_payload];
        if drvproto::encode_upload_texture_3d_header_le(&header, &mut payload).is_none() {
            return;
        }
        payload[drvproto::UPLOAD_TEXTURE_3D_HEADER_WIRE_SIZE..].copy_from_slice(data);
        
        let mut buf = alloc::vec![0u8; total_msg];
        if let Some(len) = drvproto::encode_message(&mut buf, drvproto::MSG_UPLOAD_TEXTURE_3D, &payload) {
            let _ = port_send(self.req_write, &buf[..len]);
        }
        
        // Don't wait for ACK to avoid latency - texture upload is fire-and-forget
    }
}

impl Presenter for DriverPresenter {
    fn acquire_frame(&mut self, spec: FrameSpec, asset_gen: AssetGeneration) -> FrameToken {
        self.frame_count += 1;
        reclaimer::register_in_flight(self.frame_count, asset_gen);
        FrameToken::new(self.frame_count, asset_gen, spec)
    }

    fn present_frame(&mut self, token: FrameToken) -> PresentStats {
        let ops_count = token.ops.iter().count();
        let frame_id = token.frame_id;
        let asset_gen = token.asset_gen;

        // Record damage in history for future buffer expansion
        let current_damage: Vec<crate::geometry::Rect> = token.present_damage.rects().to_vec();
        self.damage_history.insert(0, current_damage);
        if self.damage_history.len() > 4 {
            self.damage_history.pop();
        }

        reclaimer::complete_in_flight(frame_id);

        let damage_rect_count = self.send_present(&token.present_damage);

        PresentStats {
            frame_id,
            asset_gen,
            ops_count,
            damage_rect_count,
            fast_path_taken: token.damage.is_empty(),
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

    fn acquire_buffer(&mut self) -> (ThingId, u32, u32, u32, u32, u32) {
        let mut buf = [0u8; 128];
        if let Some(total) = drvproto::encode_message(&mut buf, drvproto::MSG_ACQUIRE, &[]) {
            let _ = port_send(self.req_write, &buf[..total]);
        }

        // Synchronous wait for ACQUIRED
        loop {
            self.pump_port();
            while let Some((header, payload)) = self.frames.next_message() {
                let msg_type = header.msg_type;
                if msg_type == drvproto::MSG_ACQUIRED {
                    if let Some(acq) = drvproto::decode_acquired_payload_le(payload) {
                        return (
                            ThingId({
                                let mut b = [0u8; 16];
                                b[0..8].copy_from_slice(&acq.bytespace_id.to_le_bytes());
                                b
                            }),
                            acq.width,
                            acq.height,
                            acq.stride,
                            acq.format,
                            acq.buffer_age,
                        );
                    }
                } else {
                    // Copy payload to break borrow from self.frames
                    let payload_vec = payload.to_vec();
                    self.handle_message(msg_type, &payload_vec);
                }
            }
            stem::yield_now();
        }
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

    pub fn acquire_buffer(&mut self) -> (ThingId, u32, u32, u32, u32, u32) {
        match self {
            PresenterImpl::Null(inner) => inner.acquire_buffer(),
            PresenterImpl::Driver(inner) => inner.acquire_buffer(),
        }
    }

    pub fn negotiation_info(&self) -> Option<DisplayNegotiation> {
        match self {
            PresenterImpl::Null(inner) => inner.negotiation_info(),
            PresenterImpl::Driver(inner) => inner.negotiation_info(),
        }
    }
}

impl DriverPresenter {
    /// Expand damage based on buffer age
    pub fn expand_damage(&self, damage: &mut Damage, age: u32) {
        if age <= 1 {
            return;
        }

        // age=2 means we need to union with damage from 1 frame ago
        // age=3 means we need to union with damage from 1 and 2 frames ago
        // etc.
        let to_union = (age as usize).saturating_sub(1);
        for i in 0..to_union.min(self.damage_history.len()) {
            for &rect in &self.damage_history[i] {
                damage.add_rect(rect);
            }
        }
    }
}
