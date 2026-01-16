use alloc::collections::BTreeSet;
use alloc::string::String;
use alloc::vec::Vec;
use crate::drawlist::DrawList;
use crate::damage::Rect;
use crate::geometry::Color;
use abi::hid::Key;


pub struct KeyOverlay {
    // State
    active_keys: BTreeSet<Key>,
    cached_text: String,
    
    cached_geometry: Rect, // The rect of the *overlay box*, including padding
    
    // Damage
    prev_rect: Option<Rect>,
}

impl KeyOverlay {
    pub fn new() -> Self {
        Self {
            active_keys: BTreeSet::new(),
            cached_text: String::new(),
            cached_geometry: Rect::default(),
            prev_rect: None,
        }
    }

    pub fn update(&mut self, keys: &BTreeSet<Key>, _frame_id: u64) -> bool {
        let mut damaged = false;
        
        // 1. Check if keys changed
        if keys != &self.active_keys {
            self.active_keys = keys.clone();
            
            // Rebuild string
            let new_text = build_key_string(keys);
            if new_text != self.cached_text {
                self.cached_text = new_text;
                damaged = true;
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
        
        // Rough estimation of text width if we don't have font metrics yet
        let text_w = (self.cached_text.chars().count() as f32 * 24.0 * 0.6) as i32;
        let content_w = text_w;
        let content_h = 24;
        
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
        
        list.rounded_rect(rect.x, rect.y, rect.w, rect.h, radius, bg_color, crate::geometry::EdgeAA::None);

        // Draw Text
        if !self.cached_text.is_empty() {
             list.text(&self.cached_text, rect.x + 12, rect.y + 10, 24.0, Color::from_u32(0xFFFFFFFF));
        }
    }
}

fn build_key_string(keys: &BTreeSet<Key>) -> String {
    let mut parts: Vec<&str> = Vec::new();

    // 1. Modifiers
    if keys.contains(&Key::LeftCtrl) || keys.contains(&Key::RightCtrl) {
        parts.push("⌃"); // U+2303
    }
    if keys.contains(&Key::LeftAlt) || keys.contains(&Key::RightAlt) {
        parts.push("⌥"); // U+2325
    }
    if keys.contains(&Key::LeftShift) || keys.contains(&Key::RightShift) {
        parts.push("⇧"); // U+21E7
    }
    if keys.contains(&Key::LeftMeta) || keys.contains(&Key::RightMeta) {
        parts.push("⌘"); // U+2318
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
        overlay.cached_geometry = Rect::new(860, 934, 124, 50);
        
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

