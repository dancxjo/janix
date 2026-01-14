use alloc::sync::Arc;
use core::sync::atomic::{AtomicU32, Ordering};
use stem::syscall;
use stem::{info, warn, error};
use abi::font::{FontRequest, FontResponse, FaceId, TextBitmap};

static REQ_PORT: AtomicU32 = AtomicU32::new(0);
static RESP_PORT: AtomicU32 = AtomicU32::new(0);

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
