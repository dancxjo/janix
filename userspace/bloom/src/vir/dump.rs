//! VIR dump utilities for debugging and inspection

use super::*;
use alloc::format;
use alloc::string::{String, ToString};

impl VirDocument {
    /// Dump VIR document in a human-readable format
    pub fn dump_text(&self) -> String {
        let mut s = String::new();
        s.push_str("VirDocument {\n");

        if let Some(w) = self.width {
            s.push_str(&format!("  width: {}\n", w));
        }
        if let Some(h) = self.height {
            s.push_str(&format!("  height: {}\n", h));
        }
        if let Some(vb) = &self.view_box {
            s.push_str(&format!(
                "  viewBox: {{ x: {}, y: {}, width: {}, height: {} }}\n",
                vb.x, vb.y, vb.width, vb.height
            ));
        }

        s.push_str(&format!("  elements: {} items\n", self.elements.len()));
        for (i, elem) in self.elements.iter().enumerate() {
            s.push_str(&format!("  [{}] {}\n", i, elem.dump_text()));
        }

        s.push_str("}\n");
        s
    }
}

impl VirElement {
    fn dump_text(&self) -> String {
        let mut s = String::new();
        s.push_str("Element { ");

        s.push_str(&format!("segments: {}, ", self.path.segments.len()));

        if let Some(fill) = &self.fill {
            s.push_str(&format!("fill: {:?}, ", fill.paint));
        }

        if let Some(stroke) = &self.stroke {
            s.push_str(&format!("stroke: width={}, ", stroke.width));
        }

        if self.opacity < 1.0 {
            s.push_str(&format!("opacity: {}, ", self.opacity));
        }

        s.push_str("}");
        s
    }
}

impl VirPath {
    /// Get a summary of the path
    pub fn summary(&self) -> String {
        let mut move_count = 0;
        let mut line_count = 0;
        let mut quad_count = 0;
        let mut cubic_count = 0;
        let mut close_count = 0;

        for seg in &self.segments {
            match seg {
                VirSegment::MoveTo(_) => move_count += 1,
                VirSegment::LineTo(_) => line_count += 1,
                VirSegment::QuadTo(_, _) => quad_count += 1,
                VirSegment::CubicTo(_, _, _) => cubic_count += 1,
                VirSegment::Close => close_count += 1,
            }
        }

        format!(
            "Path(M:{} L:{} Q:{} C:{} Z:{})",
            move_count, line_count, quad_count, cubic_count, close_count
        )
    }
}
