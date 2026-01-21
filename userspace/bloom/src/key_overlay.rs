use alloc::collections::BTreeSet;
use alloc::string::String;
use alloc::vec::Vec;
use crate::drawlist::DrawList;
use crate::damage::Rect;
use crate::geometry::Color;
use stem::ui::UiBuilder;
use stem::thing::sys::prop_set;
use abi::schema::keys;
use abi::hid::Key;
use stem::thing::ThingId;

pub struct KeyOverlay {
    // State
    active_keys: BTreeSet<Key>,
    cached_text: String,
    
    // UI Graph Handles
    root_node: Option<ThingId>,
    text_node: Option<ThingId>,
    perf_node: Option<ThingId>,
    pub show_perf: bool,
}

impl KeyOverlay {
    pub fn new() -> Self {
        Self {
            active_keys: BTreeSet::new(),
            cached_text: String::new(),
            root_node: None,
            text_node: None,
            perf_node: None,
            show_perf: false,
        }
    }

    pub fn setup(&mut self, ui_root: ThingId) {
        let window = UiBuilder::create_window(ui_root, "Overlay");
        let text = UiBuilder::create_text(window, "");
        let perf = UiBuilder::create_text(window, "");
        
        prop_set(window, keys::UI_BG_COLOR, Color::from_u32(0x99000000).to_u32() as u64).ok();
        prop_set(text, keys::UI_FG_COLOR, Color::from_u32(0xFFFFFFFF).to_u32() as u64).ok();
        prop_set(text, keys::UI_FONT_SIZE, 24).ok();

        prop_set(perf, keys::UI_FG_COLOR, Color::from_u32(0xFF00FF00).to_u32() as u64).ok();
        prop_set(perf, keys::UI_FONT_SIZE, 14).ok();
        
        self.root_node = Some(window);
        self.text_node = Some(text);
        self.perf_node = Some(perf);
    }

    pub fn update(&mut self, keys: &BTreeSet<Key>, screen_w: i32, screen_h: i32) -> bool {
        let mut changed = false;
        
        if keys != &self.active_keys {
            self.active_keys = keys.clone();
            let new_text = build_key_string(keys);
            if new_text != self.cached_text {
                self.cached_text = new_text;
                changed = true;
                
                if let Some(txt_id) = self.text_node {
                    UiBuilder::set_text(txt_id, &self.cached_text);
                }
            }
        }

        if self.show_perf {
            if let Some(perf_id) = self.perf_node {
                if let Some(report) = crate::perf::get_last_report() {
                    let mut s = String::new();
                    use core::fmt::Write;
                    // Cap to 20 lines total to avoid running off screen
                    let mut lines_left = 20;
                    writeln!(s, "PERF REPORT (avg 120f):").ok();
                    lines_left -= 1;

                    for (name, avg) in report.avg_spans {
                        if lines_left == 0 { break; }
                        // Truncate name to 18 chars to fit box
                        writeln!(s, "{:<18.18} {:>6.2}ms", name, avg).ok();
                        lines_left -= 1;
                    }
                    for (name, avg) in report.avg_counters {
                        if lines_left == 0 { break; }
                        writeln!(s, "{:<18.18} {:>6.1}", name, avg).ok();
                        lines_left -= 1;
                    }
                    UiBuilder::set_text(perf_id, &s);
                    changed = true;
                }
            }
        } else {
             if let Some(perf_id) = self.perf_node {
                 UiBuilder::set_text(perf_id, "");
             }
        }
        
        // Always ensure layout is correct
        if let Some(root_id) = self.root_node {
            if let Some(rect) = self.compute_rect(screen_w, screen_h) {
                UiBuilder::set_pos(root_id, rect.x, rect.y);
                UiBuilder::set_size(root_id, rect.w, rect.h);
                
                if let Some(txt_id) = self.text_node {
                     UiBuilder::set_pos(txt_id, 12, 6);
                }
                if let Some(perf_id) = self.perf_node {
                     UiBuilder::set_pos(perf_id, 12, 40);
                }
            }
        }
        
        changed
    }
    
    pub fn post_present(&mut self) {}

    fn compute_rect(&self, screen_w: i32, screen_h: i32) -> Option<Rect> {
        if self.active_keys.is_empty() && !self.show_perf { return None; }
        
        let padding_x = 12;
        let padding_y = 10;
        
        let text_w = (self.cached_text.chars().count() as f32 * 24.0 * 0.6) as i32;
        let content_w = text_w;
        let content_h = 24;
        
        let mut box_w = content_w + padding_x * 2;
        let mut box_h = content_h + padding_y * 2;
        
        if self.show_perf {
            box_w = box_w.max(280);
            box_h = box_h.max(300);
        }
        
        let x = screen_w - box_w - 16; 
        let y = screen_h - box_h - 16;
        
        Some(Rect::new(x, y, box_w, box_h))
    }

    pub fn render(&mut self, _list: &mut DrawList, _screen_w: i32, _screen_h: i32) {
        // No-op: rendered via UI pipeline
    }
}

fn build_key_string(keys: &BTreeSet<Key>) -> String {
    let mut parts: Vec<&str> = Vec::new();

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
