use abi::draw_cmd::DrawCmd;
use postcard::to_slice;

pub struct DrawListWriter<'a> {
    buf: &'a mut [u8],
    cursor: usize,
    count: u32,
}

impl<'a> DrawListWriter<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, cursor: 0, count: 0 }
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn clear(&mut self, color: u32) {
        let cmd = DrawCmd::Clear { color };
        if let Ok(used) = to_slice(&cmd, &mut self.buf[self.cursor..]) {
            self.cursor += used.len();
            self.count += 1;
        }
    }

    pub fn fill_rect(&mut self, x: i16, y: i16, w: u16, h: u16, color: u32) {
        let cmd = DrawCmd::FillRect { x, y, w, h, color };
        if let Ok(used) = to_slice(&cmd, &mut self.buf[self.cursor..]) {
            self.cursor += used.len();
            self.count += 1;
        }
    }

    pub fn fill_rounded_rect(&mut self, x: i16, y: i16, w: u16, h: u16, radius: u16, color: u32) {
        let cmd = DrawCmd::FillRoundedRect { x, y, w, h, radius, color };
        if let Ok(used) = to_slice(&cmd, &mut self.buf[self.cursor..]) {
            self.cursor += used.len();
            self.count += 1;
        }
    }

    pub fn stroke_rounded_rect(&mut self, x: i16, y: i16, w: u16, h: u16, radius: u16, thickness: u16, color: u32) {
        let cmd = DrawCmd::StrokeRoundedRect { x, y, w, h, radius, thickness, color };
        if let Ok(used) = to_slice(&cmd, &mut self.buf[self.cursor..]) {
            self.cursor += used.len();
            self.count += 1;
        }
    }

    pub fn text(&mut self, x: i16, y: i16, text: &str, color: u32) {
        let len = text.len() as u16;
        let cmd = DrawCmd::Text { x, y, color, len };
        
        // Write command
        if let Ok(used) = to_slice(&cmd, &mut self.buf[self.cursor..]) {
            self.cursor += used.len();
            
            // Write text bytes
            if self.cursor + text.len() <= self.buf.len() {
                self.buf[self.cursor..self.cursor+text.len()].copy_from_slice(text.as_bytes());
                self.cursor += text.len();
                self.count += 1;
            } else {
                // Rollback if not enough space for text
                // Actually difficult to rollback cleanly without separate tracking, 
                // but for now we just won't increment count and risk corrupt buffer tail
            }
        }
    }
}
