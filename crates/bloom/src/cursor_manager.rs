use alloc::string::String;
use alloc::format;
use crate::assets::cursor::{CursorAnimator, CursorAsset};
use crate::assets::cursor::cur::load_cur;
use crate::assets::cursor::ani::load_ani;
use thing_std::graph::thing_find;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CursorKind {
    Default,
    Move,
    ResizeV,
    ResizeH,
    ResizeNWSE,
    ResizeNESW,
    Working,
    Text,
}

pub struct CursorSet {
    default: Option<CursorAnimator>,
    move_cur: Option<CursorAnimator>,
    resize_v: Option<CursorAnimator>,
    resize_h: Option<CursorAnimator>,
    resize_nwse: Option<CursorAnimator>,
    resize_nesw: Option<CursorAnimator>,
    working: Option<CursorAnimator>,
    text: Option<CursorAnimator>,
    
    current: CursorKind,
    override_active: bool, // For force-setting during drag
}

impl CursorSet {
    pub fn new() -> Self {
        Self {
            default: None,
            move_cur: None,
            resize_v: None,
            resize_h: None,
            resize_nwse: None,
            resize_nesw: None,
            working: None,
            text: None,
            current: CursorKind::Default,
            override_active: false,
        }
    }

    pub fn load_all(&mut self, mut vaddr_base: u64) {
        let mut load = |name: &str, is_ani: bool| -> Option<CursorAnimator> {
            let thing_name = format!("bytespace.asset.{}", name);
            if let Some(bs_id) = thing_find(&thing_name) {
                // Reserve 1MB per cursor (generous)
                let len = 1024 * 1024;
                // Basic bump allocator strategy for this loop
                // In a real system we might track this better
                let vaddr = vaddr_base;
                vaddr_base += len;
                
                let buf = crate::assets::map_bytespace(bs_id, vaddr, len);
                
                let asset = if is_ani {
                   load_ani(buf)
                } else {
                    load_cur(buf).map(CursorAsset::static_cursor)
                };
                
                if let Some(asset) = asset {
                    return Some(CursorAnimator::new(asset, 1000)); // 1000 ticks/ms default? Check app.rs
                }
            }
            // thing_std::log_info(&format!("BLOOM: Failed to load cursor {}", name));
            None
        };

        self.default = load("Normal.cur", false);
        self.move_cur = load("Move.cur", false);
        self.resize_v = load("Vertical.ani", true);
        self.resize_h = load("Horizontal.ani", true);
        self.resize_nwse = load("Diagonal1.ani", true);
        self.resize_nesw = load("Diagonal2.ani", true);
        self.working = load("Working.ani", true);
        self.text = load("Text.cur", false);
        
        // Fallback: if default is missing, try to use anything? 
        // We assume at least Normal.cur exists as per old code.
    }

    pub fn set_cursor(&mut self, kind: CursorKind) {
        if !self.override_active {
            self.current = kind;
        }
    }

    pub fn set_override(&mut self, kind: Option<CursorKind>) {
        if let Some(k) = kind {
            self.current = k;
            self.override_active = true;
        } else {
            self.override_active = false;
        }
    }

    pub fn current_animator(&mut self) -> Option<&mut CursorAnimator> {
        match self.current {
            CursorKind::Default => self.default.as_mut().or(self.working.as_mut()),
            CursorKind::Move => self.move_cur.as_mut().or(self.default.as_mut()),
            CursorKind::ResizeV => self.resize_v.as_mut().or(self.default.as_mut()),
            CursorKind::ResizeH => self.resize_h.as_mut().or(self.default.as_mut()),
            CursorKind::ResizeNWSE => self.resize_nwse.as_mut().or(self.default.as_mut()),
            CursorKind::ResizeNESW => self.resize_nesw.as_mut().or(self.default.as_mut()),
            CursorKind::Working => self.working.as_mut().or(self.default.as_mut()),
            CursorKind::Text => self.text.as_mut().or(self.default.as_mut()),
        }
    }
}
