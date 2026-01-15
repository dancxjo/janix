use alloc::sync::Arc;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use stem::syscall;
use stem::{info, warn, error};
use abi::font::{FontRequest, FontResponse, FaceId, TextBitmap, FontId};

static REQ_PORT: AtomicU32 = AtomicU32::new(0);
static RESP_PORT: AtomicU32 = AtomicU32::new(0);
// Cached default face: 0 = not set, non-zero = font_id is valid
static DEFAULT_FACE_FONT_ID: AtomicU64 = AtomicU64::new(0);
static DEFAULT_FACE_INDEX: AtomicU32 = AtomicU32::new(0);
static DEFAULT_FACE_VALID: AtomicU32 = AtomicU32::new(0);
static LAST_FACE_CHECK: AtomicU64 = AtomicU64::new(0);



pub fn init(node_id: u64) {
    match stem::syscall::stream::stream_open(node_id as usize) {
        Ok((req, resp)) => {
            REQ_PORT.store(req, Ordering::SeqCst);
            RESP_PORT.store(resp, Ordering::SeqCst);
            info!("[font_client] connected to fontd via stream svc={} (req={} resp={})", node_id, req, resp);
        }
        Err(e) => {
            error!("[font_client] failed to open stream to svc={}: {:?}", node_id, e);
        }
    }
}

pub fn render_text(face: FaceId, size_px: u32, text: &str, color: u32) -> Option<TextBitmap> {
    let req_port = REQ_PORT.load(Ordering::Relaxed);
    let resp_port = RESP_PORT.load(Ordering::Relaxed);
    if req_port == 0 || resp_port == 0 { return None; }

    let req = FontRequest::RenderText {
        face,
        size_px,
        text: alloc::string::String::from(text),
        color,
    };
    
    let mut buf = [0u8; 4096];
    if let Some(len) = req.encode(&mut buf) {
        if let Err(e) = syscall::port_send(req_port, &buf[..len]) {
            warn!("[font_client] send failed: {:?}", e);
            return None;
        }

        // Blocking recv
        match syscall::port_recv(resp_port, &mut buf) {
            Ok(len) if len > 0 => {
                match FontResponse::decode(&buf[..len]) {
                    Some(FontResponse::Rendered(bmp)) => return Some(bmp),
                    Some(FontResponse::Error(e)) => {
                        warn!("[font_client] render error: {}", e);
                    }
                    _ => warn!("[font_client] unexpected response"),
                }
            }
            Ok(_) => warn!("[font_client] empty response"),
            Err(e) => warn!("[font_client] recv failed: {:?}", e),
        }
    }
    None
}

pub fn measure_text(face: FaceId, size_px: u32, text: &str) -> Option<abi::font::TextMetrics> {
    let req_port = REQ_PORT.load(Ordering::Relaxed);
    let resp_port = RESP_PORT.load(Ordering::Relaxed);
    if req_port == 0 || resp_port == 0 { return None; }

    let req = FontRequest::MeasureText {
        face,
        size_px,
        text: alloc::string::String::from(text),
    };

    let mut buf = [0u8; 4096];
    if let Some(len) = req.encode(&mut buf) {
        if let Err(e) = syscall::port_send(req_port, &buf[..len]) {
             warn!("[font_client] send failed: {:?}", e);
             return None;
        }

        match syscall::port_recv(resp_port, &mut buf) {
            Ok(len) if len > 0 => {
                match FontResponse::decode(&buf[..len]) {
                    Some(FontResponse::Measured(m)) => return Some(m),
                    Some(FontResponse::Error(e)) => {
                        warn!("[font_client] measure error: {}", e);
                    }
                    _ => warn!("[font_client] unexpected response"),
                }
            }
            Ok(_) => warn!("[font_client] empty response"),
            Err(e) => warn!("[font_client] recv failed: {:?}", e),
        }
    }
    None
}

pub fn list_fonts() -> Option<alloc::vec::Vec<abi::font::FontInfo>> {
    let req_port = REQ_PORT.load(Ordering::Relaxed);
    let resp_port = RESP_PORT.load(Ordering::Relaxed);
    if req_port == 0 || resp_port == 0 { return None; }

    let req = FontRequest::ListFonts;
    let mut buf = [0u8; 4096];
    
    if let Some(len) = req.encode(&mut buf) {
        if let Err(_) = syscall::port_send(req_port, &buf[..len]) { return None; }
        match syscall::port_recv(resp_port, &mut buf) {
            Ok(len) if len > 0 => {
                match FontResponse::decode(&buf[..len]) {
                    Some(FontResponse::FontList(list)) => return Some(list),
                    _ => {}
                }
            }
            _ => {}
        }
    }
    None
}

/// Check if font client is connected to fontd
pub fn is_ready() -> bool {
    REQ_PORT.load(Ordering::Relaxed) != 0 && RESP_PORT.load(Ordering::Relaxed) != 0
}

/// Get a cached default face, refreshing periodically
/// Returns None if no fonts are available yet
pub fn get_default_face(frame_id: u64) -> Option<FaceId> {
    // Check cache first
    if DEFAULT_FACE_VALID.load(Ordering::Acquire) != 0 {
        return Some(FaceId {
            font_id: FontId(DEFAULT_FACE_FONT_ID.load(Ordering::Relaxed)),
            index: DEFAULT_FACE_INDEX.load(Ordering::Relaxed),
        });
    }
    
    // Only check every ~60 frames to avoid spamming fontd
    let last_check = LAST_FACE_CHECK.load(Ordering::Relaxed);
    if frame_id < last_check + 60 {
        return None;
    }
    LAST_FACE_CHECK.store(frame_id, Ordering::Relaxed);
    
    // Query fontd for available fonts
    if let Some(list) = list_fonts() {
        if let Some(first) = list.first() {
            DEFAULT_FACE_FONT_ID.store(first.face_id.font_id.0, Ordering::Relaxed);
            DEFAULT_FACE_INDEX.store(first.face_id.index, Ordering::Relaxed);
            DEFAULT_FACE_VALID.store(1, Ordering::Release);
            return Some(first.face_id);
        }
    }
    
    None
}
