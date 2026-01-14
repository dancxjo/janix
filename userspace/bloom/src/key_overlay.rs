use alloc::collections::BTreeSet;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use crate::asset::Image;
use crate::frame::AssetGeneration;
use crate::drawlist::{DrawList, DrawCmd};
use crate::damage::Rect;
use crate::geometry::{Color, Point};
use crate::font_client;
use abi::hid::Key;
use abi::font::FaceId;
use stem::thing::sys::{bytespace_map, bytespace_unmap};
use stem::thing::ThingId;


pub struct KeyOverlay {
    // State
    active_keys: BTreeSet<Key>,
    cached_text: String,
    
    // Resources
    font_face: Option<FaceId>,
    last_font_check: u64,
    
    // Render Cache
    cached_bitmap: Option<Image>,
    cached_geometry: Rect, // The rect of the *overlay box*, including padding
    
    // Damage
    prev_rect: Option<Rect>,
}

impl KeyOverlay {
    pub fn new() -> Self {
        Self {
            active_keys: BTreeSet::new(),
            cached_text: String::new(),
            font_face: None,
            last_font_check: 0,
            cached_bitmap: None,
            cached_geometry: Rect::default(),
            prev_rect: None,
        }
    }

    pub fn update(&mut self, keys: &BTreeSet<Key>, frame_id: u64) -> bool {
        let mut damaged = false;
        
        // 1. Check if keys changed
        if keys != &self.active_keys {
            self.active_keys = keys.clone();
            
            // Rebuild string
            let new_text = build_key_string(keys);
            if new_text != self.cached_text {
                self.cached_text = new_text;
                self.cached_bitmap = None; // Invalidate cache
                damaged = true;
            }
        }
        
        // 2. Ensure we have a font
        if self.font_face.is_none() {
            if frame_id > self.last_font_check + 60 {
                self.last_font_check = frame_id;
                if let Some(list) = super::font_client::list_fonts() {
                    let list: &alloc::vec::Vec<abi::font::FontInfo> = &list;
                    let mut found = false;
                    
                    // 1. Try Hack
                    for info in list.iter() {
                        if info.family.contains("Hack") {
                            self.font_face = Some(info.face_id);
                            found = true;
                            break;
                        }
                    }
                    
                    // 2. Try Noto
                    if !found {
                        for info in list.iter() {
                            if info.family.contains("Noto") {
                                self.font_face = Some(info.face_id);
                                found = true;
                                break;
                            }
                        }
                    }
                    
                    // 3. Fallback to first
                    if !found {
                        if let Some(first) = list.first() {
                            self.font_face = Some(first.face_id);
                        }
                    }
                }
            }
        }
        
        // 3. Re-render if needed and possible (using RenderText directly)
        if self.cached_bitmap.is_none() && !self.cached_text.is_empty() && self.font_face.is_some() {
             let face = self.font_face.unwrap();
             // Render!
             // Color: White text (0xFFFFFFFF)
             // Use 24px font size as per requirements/design
             if let Some(bmp) = font_client::render_text(face, 24, &self.cached_text, 0xFFFFFFFF) {
                 // Convert TextBitmap to Image
                 // Map bytespace
                 if let Ok(ptr) = bytespace_map(ThingId(bmp.buffer_id)) {
                     // Create new Arc<[u32]> with copy of data
                     let len = (bmp.width * bmp.height) as usize;
                     let src_slice = unsafe { core::slice::from_raw_parts(ptr as *const u32, len) };
                     
                     // We must copy because we unmap immediately
                     let pixels: Arc<[u32]> = Arc::from(src_slice);

                     let _ = bytespace_unmap(ThingId(bmp.buffer_id), ptr);
                     
                     self.cached_bitmap = Some(Image {
                         width: bmp.width,
                         height: bmp.height,
                         pixels,
                         gen: AssetGeneration::ZERO
                     });
                     
                     // Check if geometry changed (size changed)
                     let old_geom = self.cached_geometry;
                     // We don't update geometry here, we update it in render or we compute it now.
                     // Let's compute it now to be correct for damage tracking.
                     // screen_w/h is unknown here, but size is known.
                     // The requirement says "bottom-right overlay". Position depends on screen size.
                     // We can't fully know rect without screen size.
                     // But we can know size.
                     
                     damaged = true;
                 }
             }
        }
        
        damaged
    }
    
    // Calculates damage rect (union of previous frame's rect and current frame's rect)
    pub fn damage_rect(&self, screen_w: i32, screen_h: i32) -> Option<Rect> {
        let curr = self.compute_rect(screen_w, screen_h);
        match (self.prev_rect, curr) {
            (Some(p), Some(c)) => {
                // Union
                let min_x = p.x.min(c.x);
                let min_y = p.y.min(c.y);
                let max_x = (p.x + p.w).max(c.x + c.w);
                let max_y = (p.y + p.h).max(c.y + c.h);
                Some(Rect::new(min_x, min_y, max_x - min_x, max_y - min_y))
            },
            (Some(p), None) => Some(p),
            (None, Some(c)) => Some(c),
            (None, None) => None,
        }
    }
    
    pub fn post_present(&mut self) {
        // We assume render() was called and cached_geometry is up to date
        self.prev_rect = if self.active_keys.is_empty() { None } else { Some(self.cached_geometry) };
    }
    
    fn current_overlay_rect(&self) -> Option<Rect> {
        if self.active_keys.is_empty() { return None; }
        if self.cached_geometry.w == 0 { return None; }
        Some(self.cached_geometry)
    }

    fn compute_rect(&self, screen_w: i32, screen_h: i32) -> Option<Rect> {
        if self.active_keys.is_empty() { return None; }
        
        let padding_x = 12;
        let padding_y = 10;
        
        let (content_w, content_h) = if let Some(ref img) = self.cached_bitmap {
            (img.width as i32, img.height as i32)
        } else {
            // Fallback
             let text_w = (self.cached_text.chars().count() as f32 * 24.0 * 0.6) as i32;
            (text_w, 24)
        };
        
        let box_w = content_w + padding_x * 2;
        let box_h = content_h + padding_y * 2;
        
        let x = screen_w - box_w - 16; 
        let y = screen_h - box_h - 16;
        
        Some(Rect::new(x, y, box_w, box_h))
    }

    pub fn render(&mut self, list: &mut DrawList, screen_w: i32, screen_h: i32) {
        let rect = match self.compute_rect(screen_w, screen_h) {
            Some(r) => r,
            None => {
                self.cached_geometry = Rect::default();
                return;
            }
        };
        self.cached_geometry = rect;
        
        // Draw Rounded Rect (Black 0.6 opacity)
        let bg_color = Color::from_u32(0x99000000); 
        let radius = 10;
        
        let x = rect.x;
        let y = rect.y;
        let box_w = rect.w;
        let box_h = rect.h;
        
        // Center rect
        list.rect(x + radius, y, box_w - radius*2, box_h, bg_color);
        list.rect(x, y + radius, radius, box_h - radius*2, bg_color);
        list.rect(x + box_w - radius, y + radius, radius, box_h - radius*2, bg_color);
        
        // Corners
        list.commands().push(DrawCmd::FillCircle { center: Point::new(x + radius, y + radius), radius, color: bg_color });
        list.commands().push(DrawCmd::FillCircle { center: Point::new(x + box_w - radius, y + radius), radius, color: bg_color });
        list.commands().push(DrawCmd::FillCircle { center: Point::new(x + radius, y + box_h - radius), radius, color: bg_color });
        list.commands().push(DrawCmd::FillCircle { center: Point::new(x + box_w - radius, y + box_h - radius), radius, color: bg_color });

        // Draw Text Image
        if let Some(ref img) = self.cached_bitmap {
            // Center text in box
            list.blit_image(img, x + 12, y + 10);
        }
    }
}

fn build_key_string(keys: &BTreeSet<Key>) -> String {
    let mut parts: Vec<&str> = Vec::new();

    // 1. Modifiers
    if keys.contains(&Key::LeftCtrl) || keys.contains(&Key::RightCtrl) {
        parts.push("⌃");
    }
    if keys.contains(&Key::LeftAlt) || keys.contains(&Key::RightAlt) {
        parts.push("⌥");
    }
    if keys.contains(&Key::LeftShift) || keys.contains(&Key::RightShift) {
        parts.push("⇧");
    }
    if keys.contains(&Key::LeftMeta) || keys.contains(&Key::RightMeta) {
        parts.push("⌘");
    }

    // 2. Non-mod keys
    let mut printables = Vec::new();
    let mut f_keys = Vec::new();
    
    for k in keys {
        if is_modifier(*k) { continue; }
        if is_f_key(*k) {
            f_keys.push(k);
            continue;
        }
        printables.push(k);
    }
    
    for k in printables {
        parts.push(key_to_glyph(*k));
    }
    
    for k in f_keys {
        parts.push(key_to_glyph(*k));
    }
    
    parts.join(" ")
}

fn is_modifier(k: Key) -> bool {
    match k {
        Key::LeftCtrl | Key::RightCtrl | Key::LeftAlt | Key::RightAlt |
        Key::LeftShift | Key::RightShift | Key::LeftMeta | Key::RightMeta => true,
        _ => false,
    }
}

fn is_f_key(k: Key) -> bool {
    match k {
        Key::F1 | Key::F2 | Key::F3 | Key::F4 | Key::F5 | Key::F6 |
        Key::F7 | Key::F8 | Key::F9 | Key::F10 | Key::F11 | Key::F12 => true,
        _ => false,
    }
}

fn key_to_glyph(key: Key) -> &'static str {
    match key {
        Key::LeftCtrl | Key::RightCtrl => "⌃",
        Key::LeftAlt | Key::RightAlt => "⌥",
        Key::LeftShift | Key::RightShift => "⇧",
        Key::LeftMeta | Key::RightMeta => "⌘",
        
        Key::Enter => "⏎",
        Key::Tab => "⇥",
        Key::Escape => "⎋",
        Key::Backspace => "⌫",
        Key::Delete => "⌦",
        Key::Space => "␣", 
        Key::Left => "←",
        Key::Right => "→",
        Key::Up => "↑",
        Key::Down => "↓",
        Key::Home => "↖",
        Key::End => "↘",
        Key::PageUp => "⇞",
        Key::PageDown => "⇟",
        
        Key::F1 => "F1", Key::F2 => "F2", Key::F3 => "F3", Key::F4 => "F4",
        Key::F5 => "F5", Key::F6 => "F6", Key::F7 => "F7", Key::F8 => "F8",
        Key::F9 => "F9", Key::F10 => "F10", Key::F11 => "F11", Key::F12 => "F12",
        
        _ => {
            match key {
                 Key::Num0 => "0", Key::Num1 => "1",
                 Key::Num2 => "2", Key::Num3 => "3",
                 Key::Num4 => "4", Key::Num5 => "5",
                 Key::Num6 => "6", Key::Num7 => "7",
                 Key::Num8 => "8", Key::Num9 => "9",
                 Key::A => "A", Key::B => "B", Key::C => "C", Key::D => "D",
                 Key::E => "E", Key::F => "F", Key::G => "G", Key::H => "H",
                 Key::I => "I", Key::J => "J", Key::K => "K", Key::L => "L",
                 Key::M => "M", Key::N => "N", Key::O => "O", Key::P => "P",
                 Key::Q => "Q", Key::R => "R", Key::S => "S", Key::T => "T",
                 Key::U => "U", Key::V => "V", Key::W => "W", Key::X => "X",
                 Key::Y => "Y", Key::Z => "Z",
                 Key::Minus => "-", Key::Equal => "=", Key::LeftBracket => "[",
                 Key::RightBracket => "]", Key::Backslash => "\\", Key::Semicolon => ";",
                 Key::Quote => "'", Key::Grave => "`", Key::Comma => ",",
                 Key::Period => ".", Key::Slash => "/",
                 _ => "□"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::hid::Key;
    use alloc::collections::BTreeSet;

    #[test]
    fn unicode_mapping_modifiers() {
        assert_eq!(key_to_glyph(Key::LeftCtrl), "⌃");
        assert_eq!(key_to_glyph(Key::RightAlt), "⌥");
        assert_eq!(key_to_glyph(Key::LeftShift), "⇧");
        assert_eq!(key_to_glyph(Key::RightMeta), "⌘");
    }

    #[test]
    fn display_text_is_stable_order() {
        let mut keys = BTreeSet::new();
        keys.insert(Key::A);
        keys.insert(Key::LeftCtrl);
        keys.insert(Key::LeftShift);
        
        let s = build_key_string(&keys);
        assert_eq!(s, "⌃ ⇧ A"); 
        
        // Order shouldn't change if inserted differently
        let mut keys2 = BTreeSet::new();
        keys2.insert(Key::LeftShift);
        keys2.insert(Key::A);
        keys2.insert(Key::LeftCtrl);
        
        let s2 = build_key_string(&keys2);
        assert_eq!(s, s2);
    }

    #[test]
    fn damage_includes_old_and_new_rect() {
        let mut overlay = KeyOverlay::new();
        
        // Mock State 1
        overlay.prev_rect = Some(Rect::new(100, 100, 50, 20));
        overlay.active_keys.insert(Key::A); // Make it not empty
        
        // Mock a cached geometry for "current"
        overlay.cached_bitmap = Some(Image {
            width: 100,
            height: 30,
            pixels: Arc::new([]),
            gen: AssetGeneration::ZERO,
        });
        
        let dmg = overlay.damage_rect(1000, 1000).unwrap();
        
        // Should contain prev (100, 100, 50, 20)
        assert!(dmg.x <= 100);
        assert!(dmg.y <= 100);
        assert!(dmg.x + dmg.w >= 150);
        assert!(dmg.y + dmg.h >= 120);
        
        // Should contain curr (calculated in compute_rect)
        // Content: 100x30
        // padding x 12, y 10
        // box_w = 124, box_h = 50
        // x = 1000 - 124 - 16 = 860
        // y = 1000 - 50 - 16 = 934
        
        assert!(dmg.x <= 860);
        assert!(dmg.y <= 934);
        assert!(dmg.x + dmg.w >= 860 + 124);
        assert!(dmg.y + dmg.h >= 934 + 50);
    }
}

