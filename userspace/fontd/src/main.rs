#![no_std]
#![no_main]

extern crate alloc;

use stem::{info, error};
use stem::syscall::{channel_recv_handle, channel_send_handle};
use abi::font_protocol::{
    AtlasFormat, FaceMetrics, FontRequestTag, FontResponseTag,
    GlyphPlacement, EnsureGlyphs, EnsureGlyphsResp, GetFaceMetrics,
    decode_request_tag, encode_error, FontError,
};
use petals::font::TextRenderer;
use abi::ids::HandleId;
use abi::wire::ThingId;
use alloc::vec::Vec;
use alloc::vec;
use alloc::collections::BTreeMap;

use petals::Atlas;

struct FontService {
    renderer: TextRenderer,
    // (face_id, px_size) -> Atlas
    atlases: BTreeMap<(u64, u16), Atlas>,
    // (face_id, px_size, glyph_id) -> GlyphPlacement
    cache: BTreeMap<(u64, u16, u32), GlyphPlacement>,
}

#[stem::main]
fn main(arg0: usize) -> ! {
    info!("fontd: starting up...");
    
    let listen_port = arg0 as u32;
    if listen_port == 0 {
        error!("fontd: No listen port provided!");
        loop { stem::yield_now(); }
    }

    let mut service = FontService {
        renderer: TextRenderer::load_from_boot("/assets/fonts/NotoSans-Regular.ttf")
            .expect("Failed to load default font"),
        atlases: BTreeMap::new(),
        cache: BTreeMap::new(),
    };

    info!("fontd: Entering IPC loop on handle {}", listen_port);

    let mut buf = [0u8; 4096 * 4];
    loop {
        match stem::syscall::channel_recv(listen_port, &mut buf) {
            Ok(n) => {
                let tag = match decode_request_tag(&buf[..n]) {
                    Some(tag) => tag,
                    None => {
                        error!("fontd: Received invalid request tag");
                        continue;
                    }
                };
                
                match tag {
                    FontRequestTag::Ping => {
                        let mut resp = [0u8; 1];
                        resp[0] = FontResponseTag::Pong as u8;
                        let _ = stem::syscall::channel_send(listen_port, &resp);
                    }
                    FontRequestTag::GetFaceMetrics => {
                        if let Some(req) = GetFaceMetrics::decode(&buf[1..n]) {
                            let font = &service.renderer.font;
                            let metrics = font.horizontal_line_metrics(req.px_size as f32)
                                .unwrap_or_else(|| font.horizontal_line_metrics(16.0).unwrap());
                            
                            let resp = FaceMetrics {
                                ascent: metrics.ascent as i16,
                                descent: metrics.descent as i16,
                                line_gap: metrics.line_gap as i16,
                                units_per_em: font.units_per_em() as u16,
                            };
                            let mut out_buf = [0u8; 10];
                            if let Some(len) = resp.encode(&mut out_buf) {
                                let _ = stem::syscall::channel_send(listen_port, &out_buf[..len]);
                            }
                        }
                    }
                    FontRequestTag::EnsureGlyphs => {
                        if let Some(req) = EnsureGlyphs::decode(&buf[1..n]) {
                            handle_ensure_glyphs(&mut service, listen_port, req);
                        }
                    }
                }
            }
            Err(_) => {
                stem::yield_now();
            }
        }
    }
}

fn handle_ensure_glyphs(service: &mut FontService, port: u32, req: EnsureGlyphs) {
    let face_id_u64 = req.face_id.to_u64_lossy();
    let px_size = req.px_size;
    
    // Get or create atlas
    if !service.atlases.contains_key(&(face_id_u64, px_size)) {
        let atlas = Atlas::new("font_atlas", 1024, 1024, 1).expect("Failed to create atlas");
        service.atlases.insert((face_id_u64, px_size), atlas);
    }
    let atlas = service.atlases.get_mut(&(face_id_u64, px_size)).unwrap();
    
    let mut placements = Vec::new();
    let mut missing = Vec::new();
    
    for &gid in &req.glyph_ids {
        if let Some(p) = service.cache.get(&(face_id_u64, px_size, gid)) {
            placements.push(*p);
            continue;
        }
        
        // Rasterize
        let (metrics, bitmap) = service.renderer.font.rasterize(char::from_u32(gid).unwrap_or(' '), px_size as f32);
        
        if metrics.width == 0 || metrics.height == 0 {
            let p = GlyphPlacement {
                glyph_id: gid,
                x: 0, y: 0, w: 0, h: 0,
                bearing_x: metrics.xmin as i16,
                bearing_y: metrics.ymin as i16,
                advance: metrics.advance_width as i16,
            };
            service.cache.insert((face_id_u64, px_size, gid), p);
            placements.push(p);
            continue;
        }

        // Pack
        if let Some((x, y)) = atlas.pack(metrics.width as u32, metrics.height as u32, &bitmap) {
            let p = GlyphPlacement {
                glyph_id: gid,
                x: x as u16,
                y: y as u16,
                w: metrics.width as u16,
                h: metrics.height as u16,
                bearing_x: metrics.xmin as i16,
                bearing_y: metrics.ymin as i16,
                advance: metrics.advance_width as i16,
            };
            service.cache.insert((face_id_u64, px_size, gid), p);
            placements.push(p);
        } else {
            missing.push(gid);
        }
    }
    
    let resp = EnsureGlyphsResp {
        req_face_id: req.face_id,
        req_px_size: px_size,
        atlas_fd: atlas.texture.fd,
        atlas_width: atlas.texture.width,
        atlas_height: atlas.texture.height,
        atlas_format: abi::font_protocol::AtlasFormat::A8,
        atlas_version: 1, // Need to increment this if we invalidate
        placements,
        missing,
    };
    
    let mut resp_buf = vec![0u8; 4096 * 4];
    if let Some(len) = resp.encode(&mut resp_buf) {
        let _ = stem::syscall::channel_send(port, &resp_buf[..len]);
        // Also send the FD if the protocol expects it via channel_send_handle
        let _ = stem::syscall::channel_send_handle(port, atlas.texture.fd);
    }
}
