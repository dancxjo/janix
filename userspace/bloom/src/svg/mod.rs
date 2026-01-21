use alloc::{vec, vec::Vec};
use alloc::string::{String, ToString};

pub mod ir;
pub mod parse;
pub mod walk;
pub mod compile;

use stem::xml::{XmlReader, Event, Attributes, parse_color, parse_f32, parse_length_px, parse_transform, TransformCmd};
use crate::drawlist::DrawCmd;
use crate::geometry::{Color, Point, Rect, Transform};

#[derive(Debug, Clone)]
struct SvgState {
    transform: Transform,
    fill: Color,
    stroke: Color,
    stroke_width: i32,
    opacity: u8,
}

impl Default for SvgState {
    fn default() -> Self {
        Self {
            transform: Transform::identity(),
            fill: Color::BLACK,
            stroke: Color::TRANSPARENT,
            stroke_width: 1,
            opacity: 255,
        }
    }
}

pub struct SvgParser {
    state_stack: Vec<SvgState>,
    cmds: Vec<DrawCmd>,
    width: i32,
    height: i32,
}

impl SvgParser {
    pub fn new() -> Self {
        Self {
            state_stack: vec![SvgState::default()],
            cmds: Vec::new(),
            width: 0,
            height: 0,
        }
    }
    
    fn current_state(&self) -> &SvgState {
        self.state_stack.last().expect("State stack empty")
    }

    fn current_state_mut(&mut self) -> &mut SvgState {
        self.state_stack.last_mut().expect("State stack empty")
    }

    fn push_state(&mut self) {
        let current = self.current_state().clone();
        self.state_stack.push(current);
    }
    
    fn pop_state(&mut self) {
        if self.state_stack.len() > 1 {
            self.state_stack.pop();
        }
    }

    fn apply_attributes(&mut self, attrs: &Attributes) {
        // Collect transform cmds first so we can apply them
        let mut transform_cmds = Vec::new();
        
        let state = self.current_state_mut();
        
        // Attributes
        if let Some(fill) = attrs.get("fill") {
            if let Some(c) = parse_color(fill) { state.fill = Color::new(c.r, c.g, c.b, c.a); }
        }
        if let Some(stroke) = attrs.get("stroke") {
            if let Some(c) = parse_color(stroke) { state.stroke = Color::new(c.r, c.g, c.b, c.a); }
        }
        if let Some(sw) = attrs.get("stroke-width") {
            if let Some(w) = parse_length_px(sw) { state.stroke_width = w as i32; }
        }
        if let Some(op) = attrs.get("opacity") {
             if let Some(f) = parse_f32(op) { state.opacity = (f * 255.0) as u8; }
        }
        if let Some(trans) = attrs.get("transform") {
            transform_cmds.extend(parse_transform(trans));
        }

        // Inline style
        if let Some(style) = attrs.get("style") {
             for part in style.split(';') {
                 let kv: Vec<&str> = part.split(':').collect();
                 if kv.len() == 2 {
                     let k = kv[0].trim();
                     let v = kv[1].trim();
                     match k {
                         "fill" => if let Some(c) = parse_color(v) { state.fill = Color::new(c.r, c.g, c.b, c.a); },
                         "stroke" => if let Some(c) = parse_color(v) { state.stroke = Color::new(c.r, c.g, c.b, c.a); },
                         "stroke-width" => if let Some(w) = parse_length_px(v) { state.stroke_width = w as i32; },
                         _ => {}
                     }
                 }
             }
         }
         
         // Apply transform accumulation
         for cmd in transform_cmds {
            match cmd {
                TransformCmd::Translate(tx, ty) => {
                     state.transform = state.transform.multiply(&Transform::translate(tx, ty));
                }
                TransformCmd::Scale(sx, sy) => {
                     // geometry::Transform doesn't have scale helper?
                     // Implement manually
                     state.transform.m11 *= sx;
                     state.transform.m12 *= sx;
                     state.transform.m21 *= sy;
                     state.transform.m22 *= sy;
                }
                TransformCmd::Rotate(angle) => {
                     // geometry::Transform::rotate_degrees is available with svg-cursors feature (which we are in)
                     state.transform = state.transform.multiply(&Transform::rotate_degrees(angle));
                }
            }
         }
    }

    pub fn parse(&mut self, xml: &str) -> Vec<DrawCmd> {
        self.cmds.clear();
        // Preserve initial transform (e.g. set by render_to_buffer for scaling)
        let initial_transform = self.state_stack[0].transform;
        self.state_stack.truncate(1);
        self.state_stack[0] = SvgState::default(); 
        self.state_stack[0].transform = initial_transform;

        let reader = XmlReader::new(xml.as_bytes());
        
        for event in reader {
            match event {
                Ok(Event::StartElement { name, attributes }) => {
                    self.handle_start_element(&name, &attributes);
                }
                Ok(Event::EndElement { name }) => {
                    self.handle_end_element(&name);
                }
                _ => {}
            }
        }
        
        self.cmds.clone()
    }

    fn handle_start_element(&mut self, name: &str, attrs: &Attributes) {
        match name {
            "svg" => {
                 if let Some(w) = attrs.get("width").and_then(parse_length_px) { self.width = w as i32; }
                 if let Some(h) = attrs.get("height").and_then(parse_length_px) { self.height = h as i32; }
            }
            "g" => {
                self.push_state();
                self.apply_attributes(attrs);
            }
            "rect" => {
                self.push_state();
                self.apply_attributes(attrs);
                
                let x = attrs.get("x").and_then(parse_length_px).unwrap_or(0.0).round() as i32;
                let y = attrs.get("y").and_then(parse_length_px).unwrap_or(0.0).round() as i32;
                let w = attrs.get("width").and_then(parse_length_px).unwrap_or(0.0).round() as i32;
                let h = attrs.get("height").and_then(parse_length_px).unwrap_or(0.0).round() as i32;
                
                self.emit_fill_rect(Rect::new(x, y, w, h));
                // TODO: stroke rect? DrawCmd::StrokeRect exists.
                self.pop_state();
            }
            "line" => {
                 // Line conversion to path or use DrawCmd::Line
                 // DrawCmd::Line exists but is integer coords. SVG line can be float/subpixel.
                 self.push_state();
                 self.apply_attributes(attrs);
                 let x1 = attrs.get("x1").and_then(parse_length_px).unwrap_or(0.0);
                 let y1 = attrs.get("y1").and_then(parse_length_px).unwrap_or(0.0);
                 let x2 = attrs.get("x2").and_then(parse_length_px).unwrap_or(0.0);
                 let y2 = attrs.get("y2").and_then(parse_length_px).unwrap_or(0.0);
                 
                 // Use Path for subpixel line
                 use crate::svg::ir::{PointF, PathCommand}; 
                 let verbs = vec![
                     PathCommand::MoveTo(PointF{x: x1, y: y1}),
                     PathCommand::LineTo(PointF{x: x2, y: y2})
                 ];
                 let path = crate::isa::Path2D { verbs };
                 self.emit_path(path);
                 self.pop_state();
            }
            "circle" => {
                self.push_state();
                self.apply_attributes(attrs);
                
                let cx = attrs.get("cx").and_then(parse_length_px).unwrap_or(0.0).round() as i32;
                let cy = attrs.get("cy").and_then(parse_length_px).unwrap_or(0.0).round() as i32;
                let r = attrs.get("r").and_then(parse_length_px).unwrap_or(0.0).round() as i32;
                
                self.emit_fill_circle(Point::new(cx, cy), r);
                self.pop_state();
            }
            "path" => {
                self.push_state();
                self.apply_attributes(attrs);
                if let Some(d) = attrs.get("d") {
                    let path = crate::svg::parse::parse_path_d(d);
                    self.emit_path(path);
                }
                self.pop_state();
            }
            "polyline" | "polygon" => {
                self.push_state();
                self.apply_attributes(attrs);
                 if let Some(pts_str) = attrs.get("points") {
                    let pts: Vec<f32> = pts_str
                        .split(|c| c == ',' || c == ' ' || c == '\n')
                        .filter_map(parse_f32)
                        .collect();
                    
                    if !pts.is_empty() {
                         use crate::svg::ir::{PointF, PathCommand}; 
                         let mut verbs = Vec::with_capacity(pts.len() / 2 + 1);
                         if pts.len() >= 2 {
                             verbs.push(PathCommand::MoveTo(PointF{x: pts[0], y: pts[1]}));
                             for i in (2..pts.len()).step_by(2) {
                                 if i+1 < pts.len() {
                                     verbs.push(PathCommand::LineTo(PointF{x: pts[i], y: pts[i+1]}));
                                 }
                             }
                         }
                         if name == "polygon" {
                             verbs.push(PathCommand::Close);
                         }
                         let path = crate::isa::Path2D { verbs };
                         self.emit_path(path);
                    }
                }
                self.pop_state();
            }
             _ => { self.push_state(); }
        }
    }

    fn handle_end_element(&mut self, _name: &str) {
        self.pop_state();
    }
    
    fn emit_fill_rect(&mut self, rect: Rect) {
        let (transform, fill, stroke, width) = {
            let s = self.current_state();
            (s.transform, apply_opacity(s.fill, s.opacity), apply_opacity(s.stroke, s.opacity), s.stroke_width)
        };
        
        self.cmds.push(DrawCmd::PushTransform { transform });
        
        if fill.a > 0 {
             self.cmds.push(DrawCmd::FillRect { rect, color: fill, aa: crate::geometry::EdgeAA::None });
        }
        if stroke.a > 0 {
             self.cmds.push(DrawCmd::StrokeRect { rect, color: stroke, width });
        }
        
        self.cmds.push(DrawCmd::PopTransform);
    }
    
    fn emit_fill_circle(&mut self, center: Point, radius: i32) {
        let (transform, fill, stroke, width) = {
            let s = self.current_state();
            (s.transform, apply_opacity(s.fill, s.opacity), apply_opacity(s.stroke, s.opacity), s.stroke_width)
        };
        
        self.cmds.push(DrawCmd::PushTransform { transform });
        
        if fill.a > 0 {
             self.cmds.push(DrawCmd::FillCircle { center, radius, color: fill });
        }
        if stroke.a > 0 {
             self.cmds.push(DrawCmd::StrokeCircle { center, radius, color: stroke, width });
        }
        
        self.cmds.push(DrawCmd::PopTransform);
    }
    
    fn emit_path(&mut self, path: crate::isa::Path2D) {
        let (transform, fill, stroke, width) = {
            let s = self.current_state();
            (s.transform, apply_opacity(s.fill, s.opacity), apply_opacity(s.stroke, s.opacity), s.stroke_width)
        };
        
        self.cmds.push(DrawCmd::PushTransform { transform });
        
        let path_arc = alloc::sync::Arc::new(path);
        
        if fill.a > 0 {
            self.cmds.push(DrawCmd::FillPath { 
                path: path_arc.clone(), 
                color: fill, 
                fill_rule: crate::isa::FillRule::NonZero, 
                aa: crate::geometry::EdgeAA::Coverage8 
            });
        }
        
        if stroke.a > 0 {
            self.cmds.push(DrawCmd::StrokePath { 
                path: path_arc, 
                color: stroke, 
                width, 
                cap: crate::isa::LineCap::Butt, 
                join: crate::isa::LineJoin::Miter, 
                miter_limit: 4.0, 
                aa: crate::geometry::EdgeAA::Coverage8 
            });
        }
        
        self.cmds.push(DrawCmd::PopTransform);
    }
}

fn apply_opacity(c: Color, opacity: u8) -> Color {
    crate::geometry::Color::new(c.r, c.g, c.b, ((c.a as u16 * opacity as u16) / 255) as u8)
}

/// Render SVG string to a raw buffer (ARGB pre-allocation)
/// Useful for asset loading.
pub fn render_to_buffer(xml: &str, width: i32, height: i32, scale: f32) -> alloc::vec::Vec<u32> {
    let mut pixels = alloc::vec![0u32; (width * height) as usize];
    let ptr = pixels.as_mut_ptr() as *mut u8;
    // Create unsafe surface. Ensure lifetime of pixels outlives surface usage.
    let mut surface = unsafe { crate::surface::Surface::new(ptr, (width * height * 4) as usize, width as u32, height as u32, (width * 4) as u32) };
    
    // Parse
    let mut parser = SvgParser::new();
    // let cmds = parser.parse(xml); // Removed unused call
    
    // Create DrawList
    let mut list = crate::drawlist::DrawList::new();
    
    // Wrap cmds in PushTransform/PopTransform for scale?
    // We already baked transforms into the primitives in SvgParser (because we are flattening).
    // So we just need a GLOBAL scale transform if requested.
    // Wait, SvgParser starts with Identity.
    // If we want detailed rendering, we should maybe set the initial state of the parser?
    // But `render_to_buffer` takes a `scale`.
    // If I use DrawCmd::PushTransform, my `FillRect`s (scanlines) will get scaled.
    // Since `LowLevelOp` ONLY supports translation, `PushTransform` with scale will BE IGNORED by `lower()`!
    //
    // CRITICAL: `lower()` drops scale.
    // So I MUST apply the global scale INSIDE `SvgParser` or wrap the input SVG in a `<g transform="scale(...)">` ...
    // Or just modify `state_stack` init.
    //
    // Let's modify `parser.parse` to allow injecting initial transform?
    // Or just prepend a group tag? String manipulation is alloc-heavy.
    //
    // Let's manually apply scale to the generated cmds?
    // No, that's hard.
    //
    // Best way: Initialize SvgParser state with the scale.
    // I'll add `set_initial_transform` to SvgParser.
    
    parser.state_stack[0].transform = Transform::default();
    parser.state_stack[0].transform.m11 = scale; 
    parser.state_stack[0].transform.m22 = scale; 
    
    // Re-parse with scale
    let cmds = parser.parse(xml);
    list.commands().extend(cmds);
     
    crate::raster::execute(&mut surface, &list, false);
    
    pixels
}
