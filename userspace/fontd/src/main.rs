#![no_std]
#![no_main]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use alloc::string::String;
use stem::syscall;
use stem::{info, warn, error};
use spin::Mutex;
use abi::font::{FontRequest, FontResponse, FontId, FaceId, TextBitmap, FontInfo, TextMetrics};
use abi::types::{WatchSpec, WatchEvent, WatchMode};

struct FontState {
    font: fontdue::Font,
    name: String,
}

struct ServiceState {
    fonts: BTreeMap<FontId, FontState>,
}

// Global state
static STATE: Mutex<ServiceState> = Mutex::new(ServiceState {
    fonts: BTreeMap::new(),
});

static PENDING_CONNS: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());

#[stem::main]
fn main(arg: usize) -> ! {
    let svc_font_id = arg;
    info!("[fontd] starting for svc.Font={}", svc_font_id);

    // Create Listener Port
    let (notif_w, notif_r) = syscall::port_create(4096).expect("port_create failed");
    
    if let Err(e) = syscall::stream::stream_listen(svc_font_id, notif_w) {
        error!("[fontd] listen failed: {:?}", e);
        loop { stem::sleep_ms(1000); }
    }
    
    info!("[fontd] listening on port {}...", notif_r);

    // Spawn Watch Thread
    if let Err(e) = stem::thread::spawn(watch_loop_entry) {
        error!("[fontd] failed to spawn watch loop: {:?}", e);
    }

    // Accept Loop
    let mut buf = [0u8; 128]; 
    loop {
        match syscall::port_recv(notif_r, &mut buf) {
            Ok(len) if len >= 32 => {
                 // Decode Accept Msg: [Magic(8), ServerRead(8), ServerWrite(8), pad(8)]
                 let magic = u64::from_le_bytes(buf[0..8].try_into().unwrap());
                 
                 // Magic: 0x53545245414D434E (STREAMCN)
                 if magic == 0x53545245414D434E { 
                     let s_read_64 = u64::from_le_bytes(buf[8..16].try_into().unwrap());
                     let s_write_64 = u64::from_le_bytes(buf[16..24].try_into().unwrap());
                     
                     let s_read = s_read_64 as u32;
                     let s_write = s_write_64 as u32;
                     
                     info!("[fontd] New connection! r={} w={}", s_read, s_write);
                     
                     // Push to pending and spawn handler
                     PENDING_CONNS.lock().push((s_read, s_write));
                     
                     if let Err(e) = stem::thread::spawn(client_handler_entry) {
                         warn!("[fontd] failed to spawn client handler: {:?}", e);
                         // If spawn failed, we should cleanup port or retry?
                         // For now, remove from pending to avoid leak? or keep it?
                         // If we failed to spawn, nobody will pop it. It stays in pending.
                         // But we won't retry spawning. It's lost in pending.
                         // Clean it up?
                         let mut pending = PENDING_CONNS.lock();
                         if let Some((r, w)) = pending.pop() {
                             let _ = syscall::port_close(r);
                             let _ = syscall::port_close(w);
                         }
                     }
                 } else {
                     warn!("[fontd] invalid magic: {:x}", magic);
                 }
            }
            Ok(_) => { stem::sleep_ms(10); } 
            Err(e) => {
                error!("[fontd] recv error: {:?}", e);
                stem::sleep_ms(100);
            }
        }
    }
}

extern "C" fn client_handler_entry() -> ! {
    // Pop one connection
    let (read_port, write_port) = {
        let mut pending = PENDING_CONNS.lock();
        pending.pop().expect("client_handler spawned but no pending connection")
    };

    
    let mut buf = [0u8; 8192];
    loop {
        match syscall::port_recv(read_port, &mut buf) {
             Ok(len) if len > 0 => {
                 if let Some(req) = FontRequest::decode(&buf[..len]) {
                     process_request(&req, write_port);
                 }
             }
             Err(_) => break, 
             Ok(_) => {
                 // CRITICAL: Yield when no data available to avoid busy-waiting
                 // Without this, the tight loop starves all other tasks
                 stem::thread::yield_now();
             }
         }
    }
     
    let _ = syscall::port_close(read_port);
    let _ = syscall::port_close(write_port);
    stem::syscall::exit(0);
}

extern "C" fn watch_loop_entry() -> ! {
    info!("[fontd] watch_loop started");

    // NOTE: With query_ptr=0 and query_len=0, root_watch_open will return EFAULT
    // because the kernel validates query_ptr!=0. This is expected for now - 
    // the watch functionality requires a proper query to be useful for font discovery.
    // For v0, we just enter a maintenance sleep loop.
    let spec = WatchSpec {
        mode: lex_watch_mode(),
        query_ptr: 0,
        query_len: 0,
    };
    
    let watch_id = match syscall::root_watch_open(&spec) {
        Ok(id) => id,
        Err(e) => {
            // Expected: EFAULT because query_ptr=0
            // This is not a bug - fontd watch needs a proper query implementation
            warn!("[fontd] watch disabled (no query configured): {:?}", e);
            loop { stem::sleep_ms(10000); }
        }
    };

    let mut evt = WatchEvent::default();
    loop {
        match syscall::root_watch_next(watch_id, &mut evt) {
            Ok(1) => { 
                check_node(evt.node_id);
            }
             _ => { stem::sleep_ms(100); }
        }
    }
}


fn lex_watch_mode() -> u32 {
    WatchMode::QueryThenStream as u32
}

fn check_node(id: u64) {
    use stem::thing::sys::describe_thing;
    
    let mut buf = [0u8; 512];
    if let Ok(len) = describe_thing(stem::thing::ThingId(id), &mut buf) {
        let desc = core::str::from_utf8(&buf[..len]).unwrap_or("");
        if desc.contains("name: \"") {
             if desc.contains(".ttf\"") || desc.contains(".otf\"") || desc.contains(".ttc\"") {
                 let mut st = STATE.lock();
                 if !st.fonts.contains_key(&FontId(id)) {
                     info!("[fontd] Found font candidate: {}", desc);
                     // Drop lock while loading to avoid blocking?
                     // Loading takes time.
                     drop(st);
                     
                     // Helper: load_font needs to insert into state.
                     load_and_insert_font(id);
                 }
             }
        }
    }
}

fn load_and_insert_font(id: u64) {
    use stem::thing::sys::{prop_get, bytespace_map, bytespace_unmap, bytespace_info};
    
    let bs_id = match prop_get(stem::thing::ThingId(id), "bytespace") {
        Ok(val) => stem::thing::ThingId(val),
        Err(_) => return,
    };
    
    let ptr = match bytespace_map(bs_id) {
        Ok(p) => p,
        Err(_) => return,
    };
    
    let size = match bytespace_info(bs_id) {
        Ok(s) => s,
        Err(_) => {
            let _ = bytespace_unmap(bs_id, ptr);
            return;
        }
    };
    
    let slice = unsafe { core::slice::from_raw_parts(ptr, size) };
    let settings = fontdue::FontSettings::default();
    
    match fontdue::Font::from_bytes(slice, settings) {
        Ok(font) => {
            let name = alloc::format!("font_{}", id);
            info!("[fontd] Loaded font: {}", name);
            
            let mut st = STATE.lock();
            st.fonts.insert(FontId(id), FontState { font, name });
        }
        Err(e) => {
            warn!("[fontd] Failed to parse font {}: {}", id, e);
        }
    }
    
    let _ = bytespace_unmap(bs_id, ptr);
}

fn process_request(req: &FontRequest, reply_port: u32) {
    let mut buf = [0u8; 8192];
    let resp = match req {
        FontRequest::ListFonts => {
            let st = STATE.lock();
            let mut list = Vec::new();
            for (id, font) in &st.fonts {
                list.push(FontInfo {
                    face_id: FaceId { font_id: *id, index: 0 },
                    family: font.name.clone(),
                    style: "Regular".into(),
                });
            }
            FontResponse::FontList(list)
        }
        FontRequest::RenderText { face, size_px, text, color } => {
            let st = STATE.lock();
             if let Some(fs) = st.fonts.get(&face.font_id) {
                 render_text(&fs.font, text, *size_px as f32, *color)
             } else {
                 FontResponse::Error("Font not found".into())
             }
        }
        FontRequest::MeasureText { face, size_px, text } => {
             let st = STATE.lock();
             if let Some(fs) = st.fonts.get(&face.font_id) {
                 measure_text(&fs.font, text, *size_px as f32)
             } else {
                 FontResponse::Error("Font not found".into())
             }
        }
    };
    
    if let Some(len) = resp.encode(&mut buf) {
        let _ = syscall::port_send(reply_port, &buf[..len]);
    } else {
        warn!("[fontd] response too large");
        let err = FontResponse::Error("Response too large".into());
        let _ = err.encode(&mut buf).map(|l| syscall::port_send(reply_port, &buf[..l]));
    }
}

fn render_text(font: &fontdue::Font, text: &str, size: f32, color: u32) -> FontResponse {
    use fontdue::layout::{Layout, CoordinateSystem, TextStyle};
    
    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    layout.append(&[font], &TextStyle::new(text, size, 0));
    let glyphs = layout.glyphs();
    
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    
    for g in glyphs {
        let (metrics, _) = font.rasterize_config(g.key);
        let x = g.x as i32;
        let y = g.y as i32;
        let w = metrics.width as i32;
        let h = metrics.height as i32;
        
        if x < min_x { min_x = x; }
        if y < min_y { min_y = y; }
        if x + w > max_x { max_x = x + w; }
        if y + h > max_y { max_y = y + h; }
    }
    
    let width = (max_x - min_x) as usize;
    let height = (max_y - min_y) as usize;
    if width == 0 || height == 0 {
        return FontResponse::Error("Empty text".into());
    }
    
    let buf_size = width * height * 4;
    let bs_id = match syscall::root_bytespace_create(buf_size, 0, 4) {
        Ok(id) => id,
        Err(_) => return FontResponse::Error("Failed to allocate bitmap".into()),
    };
    
    // Map and Draw
    use stem::thing::sys::{bytespace_map, bytespace_unmap};

    if let Ok(ptr) = bytespace_map(stem::thing::ThingId(bs_id as u64)) {
        let buf = unsafe { core::slice::from_raw_parts_mut(ptr as *mut u32, width * height) };
        for p in buf.iter_mut() { *p = 0; }
        
        let a = ((color >> 24) & 0xFF) as u32;
        let r = ((color >> 16) & 0xFF) as u32;
        let g = ((color >> 8) & 0xFF) as u32;
        let b = (color & 0xFF) as u32;
        
        for glyph in glyphs {
             let (metrics, bitmap) = font.rasterize_config(glyph.key);
             for (i, v) in bitmap.into_iter().enumerate() {
                 let density = v as u32; 
                 if density == 0 { continue; }
                 
                 let gx = (i % metrics.width) as i32;
                 let gy = (i / metrics.width) as i32;
                 
                 let x = (glyph.x as i32 + gx) - min_x;
                 let y = (glyph.y as i32 + gy) - min_y;
                 
                 if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                     let idx = (y as usize) * width + (x as usize);
                     let final_a = (a * density) / 255;
                     buf[idx] = (final_a << 24) | (r << 16) | (g << 8) | b;
                 }
             }
        }
        let _ = bytespace_unmap(stem::thing::ThingId(bs_id as u64), ptr);
    }
    
    FontResponse::Rendered(TextBitmap {
        width: width as u32,
        height: height as u32,
        baseline_y: -min_y, 
        buffer_id: bs_id as u64,
        buffer_size: buf_size,
        format_a8: false,
    })
}

fn measure_text(font: &fontdue::Font, text: &str, size: f32) -> FontResponse {
     use fontdue::layout::{Layout, CoordinateSystem, TextStyle};
    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    layout.append(&[font], &TextStyle::new(text, size, 0));
    
    let mut max_x = 0.0;
    if let Some(g) = layout.glyphs().last() {
        max_x = g.x + g.width as f32;
    }
    
    FontResponse::Measured(TextMetrics {
        width: max_x as u32,
        height: size as u32,
        baseline_y: size as i32, 
        advance_x: max_x as i32,
    })
}
