use crate::geometry::Rect;

pub const MAX_OVERLAY_RECTS: usize = 256;
pub const MAX_OVERLAY_RAW_RECTS: usize = 64;

#[derive(Clone, Copy, Debug, Default)]
pub struct DebugFlags {
    pub show_damage_rects: bool,
    pub show_raw_damage_rects: bool,
    pub show_damage_stats: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverlayMode {
    DirtyRects,
    Fullframe,
}

impl Default for OverlayMode {
    fn default() -> Self {
        OverlayMode::DirtyRects
    }
}

pub struct DamageOverlayState {
    pub present_rects: [Rect; MAX_OVERLAY_RECTS],
    pub present_len: usize,
    pub raw_rects: [Rect; MAX_OVERLAY_RAW_RECTS],
    pub raw_len: usize,
    pub mode: OverlayMode,
    pub reason: &'static str,
    pub overflowed: bool,
}

impl Default for DamageOverlayState {
    fn default() -> Self {
        Self {
            present_rects: [Rect::default(); MAX_OVERLAY_RECTS],
            present_len: 0,
            raw_rects: [Rect::default(); MAX_OVERLAY_RAW_RECTS],
            raw_len: 0,
            mode: OverlayMode::DirtyRects,
            reason: "",
            overflowed: false,
        }
    }
}

impl DamageOverlayState {
    pub fn update(
        &mut self,
        present: &[Rect],
        raw: &[Rect],
        mode: OverlayMode,
        reason: &'static str,
        overflowed: bool,
    ) {
        let present_len = present.len().min(MAX_OVERLAY_RECTS);
        self.present_len = present_len;
        self.present_rects[..present_len].copy_from_slice(&present[..present_len]);

        let raw_len = raw.len().min(MAX_OVERLAY_RAW_RECTS);
        self.raw_len = raw_len;
        self.raw_rects[..raw_len].copy_from_slice(&raw[..raw_len]);

        self.mode = mode;
        self.reason = reason;
        self.overflowed = overflowed;
    }

    pub fn present(&self) -> &[Rect] {
        &self.present_rects[..self.present_len]
    }

    pub fn raw(&self) -> &[Rect] {
        &self.raw_rects[..self.raw_len]
    }
}
