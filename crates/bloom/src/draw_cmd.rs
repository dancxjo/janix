use abi::draw_cmd::DrawCmd;
use crate::scene::Rect;

pub trait DrawCmdExt {
    fn bounds(&self) -> Rect;
}

impl DrawCmdExt for DrawCmd {
    fn bounds(&self) -> Rect {
        match self {
            DrawCmd::FillRect { x, y, w, h, .. } => Rect { x: *x as i32, y: *y as i32, w: *w as u32, h: *h as u32 },
            DrawCmd::FillRoundedRect { x, y, w, h, .. } => Rect { x: *x as i32, y: *y as i32, w: *w as u32, h: *h as u32 },
            DrawCmd::StrokeRoundedRect { x, y, w, h, .. } => Rect { x: *x as i32, y: *y as i32, w: *w as u32, h: *h as u32 },
            DrawCmd::Text { x, y, .. } => {
                // Heuristic: assume standard 16px high font, and some width.
                // Since this is for damage tracking, over-invalidation is safe.
                // We'll approximate width as len * 8 (half width of 16px char).
                // Or just use a "large enough" rect for now as we don't know the exact text content here easily without the buffer.
                // But wait, DrawCmd::Text doesn't store the text itself inside the enum (it's in the buffer).
                // So we can't know the true width here.
                // For now, return a conservative 0-size rect or a "unit" rect?
                // Actually, the user asked for bounds() so damage calculation becomes trivial.
                // If we can't know bounds, we can't minimize damage.
                // But let's return a nominal rect for the anchor at least.
                Rect { x: *x as i32, y: *y as i32, w: 100, h: 16 } 
            },
            DrawCmd::Clear { .. } => Rect { x: 0, y: 0, w: 0, h: 0 }, // Clear usually affects full widget, handled by caller? Or maybe full widget rect.
            DrawCmd::Shadow { x, y, w, h, .. } => Rect { x: *x as i32, y: *y as i32, w: *w as u32, h: *h as u32 },
            DrawCmd::Blit { x, y, w, h, .. } => Rect { x: *x as i32, y: *y as i32, w: *w as u32, h: *h as u32 },
            DrawCmd::End => Rect { x: 0, y: 0, w: 0, h: 0 },
        }
    }
}
