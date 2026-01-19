use alloc::vec::Vec;
use alloc::string::String;
use alloc::string::ToString;
use crate::drawlist::{DrawList, DrawCmd};
use crate::geometry::{Color, Point, Rect, Transform, EdgeAA};

pub struct SvgParser;

impl SvgParser {
    pub fn render(svg: &str, list: &mut DrawList, x: i32, y: i32, scale: f32, target_color: Color) {
        // Global transform for placement and scale
        let global_transform = Transform {
            m11: scale, m12: 0.0,
            m21: 0.0, m22: scale,
            dx: x as f32, dy: y as f32,
        };
        list.commands().push(DrawCmd::PushTransform { transform: global_transform });

        let mut depth = 0;

        let mut rest = svg;
        while let Some(start) = rest.find('<') {
            rest = &rest[start + 1..];
            if let Some(end) = rest.find('>') {
                let tag_content = &rest[..end];
                rest = &rest[end + 1..];

                if tag_content.starts_with("?") || tag_content.starts_with("!") {
                    continue;
                }

                if tag_content.starts_with('/') {
                    // Closing tag
                    let tag_name = tag_content[1..].trim();
                    if tag_name == "g" {
                        if depth > 0 {
                            list.commands().push(DrawCmd::PopTransform);
                            depth -= 1;
                        }
                    }
                    continue;
                }

                let is_self_closing = tag_content.ends_with('/');
                let content_core = if is_self_closing {
                    &tag_content[..tag_content.len() - 1]
                } else {
                    tag_content
                };

                let mut parts = content_core.split_whitespace();
                let tag_name = parts.next().unwrap_or("");
                let attrs = parse_attrs(content_core);

                let transform = if let Some(t_str) = attrs.get("transform") {
                    parse_transform(t_str)
                } else {
                    Transform::identity()
                };

                if tag_name == "g" {
                    list.commands().push(DrawCmd::PushTransform { transform });
                    if !is_self_closing {
                        depth += 1;
                    } else {
                        list.commands().push(DrawCmd::PopTransform);
                    }
                } else if tag_name == "path" {
                     list.commands().push(DrawCmd::PushTransform { transform });
                     if let Some(d) = attrs.get("d") {
                         let fill = parse_color(attrs.get("fill"), target_color);
                         if let Some(c) = fill {
                             list.commands().push(DrawCmd::BeginPath);
                             parse_path(d, list);
                             // Assume standard winding/filling
                             list.commands().push(DrawCmd::FillPath { color: c });
                         }
                     }
                     list.commands().push(DrawCmd::PopTransform);
                } else if tag_name == "rect" {
                    list.commands().push(DrawCmd::PushTransform { transform });
                    let rx = attrs.get("x").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
                    let ry = attrs.get("y").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
                    let w = attrs.get("width").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
                    let h = attrs.get("height").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
                    let r = attrs.get("ry").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);

                    let fill = parse_color(attrs.get("fill"), target_color);

                    if let Some(c) = fill {
                        let rect = Rect::new(rx as i32, ry as i32, w as i32, h as i32);
                        if r > 0.0 {
                            list.commands().push(DrawCmd::FillRoundRect { rect, radius: r as i32, color: c, aa: EdgeAA::Coverage8 });
                        } else {
                             list.commands().push(DrawCmd::FillRect { rect, color: c, aa: EdgeAA::Coverage8 });
                        }
                    }
                    list.commands().push(DrawCmd::PopTransform);
                }
            } else {
                break;
            }
        }

        while depth > 0 {
            list.commands().push(DrawCmd::PopTransform);
            depth -= 1;
        }
        list.commands().push(DrawCmd::PopTransform);
    }
}

fn parse_attrs(content: &str) -> alloc::collections::BTreeMap<String, String> {
    let mut attrs = alloc::collections::BTreeMap::new();
    let mut rest = content;
    if let Some(idx) = rest.find(char::is_whitespace) {
        rest = &rest[idx..];
    } else {
        return attrs;
    }

    while !rest.is_empty() {
        rest = rest.trim_start();
        if rest.is_empty() { break; }

        if let Some(eq_idx) = rest.find('=') {
            let key = rest[..eq_idx].trim();
            rest = &rest[eq_idx+1..].trim_start();

            if rest.starts_with('"') {
                if let Some(end_quote) = rest[1..].find('"') {
                    let value = &rest[1..end_quote+1];
                    attrs.insert(key.to_string(), value.to_string());
                    rest = &rest[end_quote+2..];
                } else { break; }
            } else if rest.starts_with('\'') { // Handle single quotes
                 if let Some(end_quote) = rest[1..].find('\'') {
                    let value = &rest[1..end_quote+1];
                    attrs.insert(key.to_string(), value.to_string());
                    rest = &rest[end_quote+2..];
                } else { break; }
            } else {
                if let Some(space) = rest.find(char::is_whitespace) {
                     rest = &rest[space..];
                } else { break; }
            }
        } else { break; }
    }
    attrs
}

fn parse_color(val: Option<&String>, target: Color) -> Option<Color> {
    if let Some(s) = val {
        if s == "none" { return None; }
        if s.starts_with("#") {
            let hex = &s[1..];
            if hex.eq_ignore_ascii_case("ffb900") {
                return Some(target);
            }
            // If it's pure white, maybe we keep it white?
            // The request said "Set the main color (other than white) in the svgs be the target color"
            // So white stays white.
            if hex.len() == 6 {
                let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                return Some(Color::rgb(r, g, b));
            }
        }
    }
    None
}

fn parse_transform(t_str: &str) -> Transform {
    let mut t = Transform::identity();
    let mut rest = t_str;

    while !rest.is_empty() {
        rest = rest.trim();
        if rest.starts_with("translate") {
            if let Some(start) = rest.find('(') {
                if let Some(end) = rest.find(')') {
                    let args = &rest[start+1..end];
                    let parts: Vec<&str> = args.split_whitespace().chain(args.split(',')).filter(|s| !s.trim().is_empty()).collect();
                    if parts.len() >= 1 {
                        let tx = parts[0].parse::<f32>().unwrap_or(0.0);
                        let ty = if parts.len() >= 2 { parts[1].parse::<f32>().unwrap_or(0.0) } else { 0.0 };
                        t = t.multiply(&Transform::translate(tx, ty));
                    }
                    rest = &rest[end+1..];
                    continue;
                }
            }
        } else if rest.starts_with("rotate") {
             if let Some(start) = rest.find('(') {
                if let Some(end) = rest.find(')') {
                    let args = &rest[start+1..end];
                    let parts: Vec<&str> = args.split_whitespace().chain(args.split(',')).filter(|s| !s.trim().is_empty()).collect();
                    if parts.len() >= 1 {
                        let angle = parts[0].parse::<f32>().unwrap_or(0.0);
                        if parts.len() >= 3 {
                             let cx = parts[1].parse::<f32>().unwrap_or(0.0);
                             let cy = parts[2].parse::<f32>().unwrap_or(0.0);
                             let t1 = Transform::translate(cx, cy);
                             let r = Transform::rotate_degrees(angle);
                             let t2 = Transform::translate(-cx, -cy);
                             let combined = t1.multiply(&r).multiply(&t2);
                             t = t.multiply(&combined);
                        } else {
                             t = t.multiply(&Transform::rotate_degrees(angle));
                        }
                    }
                    rest = &rest[end+1..];
                    continue;
                }
            }
        }
        // Advance if no match to avoid infinite loop
        if let Some(idx) = rest.find(')') {
             rest = &rest[idx+1..];
        } else {
             break;
        }
    }
    t
}

#[derive(Debug)]
enum Token {
    Command(char),
    Number(f32),
}

struct PathTokenizer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> PathTokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn next(&mut self) -> Option<Token> {
        while self.pos < self.input.len() {
            let c = self.input.as_bytes()[self.pos] as char;
            if c.is_whitespace() || c == ',' {
                self.pos += 1;
            } else { break; }
        }

        if self.pos >= self.input.len() { return None; }

        let c = self.input.as_bytes()[self.pos] as char;
        if c.is_ascii_alphabetic() && c != 'e' && c != 'E' { // e/E used in numbers
             self.pos += 1;
             return Some(Token::Command(c));
        }

        let start = self.pos;
        if c == '-' || c == '+' { self.pos += 1; }

        while self.pos < self.input.len() {
            let nc = self.input.as_bytes()[self.pos] as char;
            if nc.is_digit(10) {
                self.pos += 1;
            } else if nc == '.' {
                self.pos += 1;
            } else { break; }
        }

        // Handle scientific
        if self.pos < self.input.len() {
             let nc = self.input.as_bytes()[self.pos] as char;
             if nc == 'e' || nc == 'E' {
                 self.pos += 1;
                 if self.pos < self.input.len() {
                     let sc = self.input.as_bytes()[self.pos] as char;
                     if sc == '+' || sc == '-' { self.pos += 1; }
                 }
                 while self.pos < self.input.len() {
                     if (self.input.as_bytes()[self.pos] as char).is_digit(10) { self.pos += 1; }
                     else { break; }
                 }
             }
        }

        let num_str = &self.input[start..self.pos];
        if let Ok(n) = num_str.parse::<f32>() {
            Some(Token::Number(n))
        } else {
            // Check if we consumed nothing but a sign or dot (invalid)
             // Fallback: if it was a command char mistaken for number start?
             // But we checked alpha.
             // Maybe it's just garbage?
             self.pos += 1;
             None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drawlist::DrawList;

    #[test]
    fn test_parse_default_svg() {
        let svg_data = include_str!("default.svg");
        let mut list = DrawList::new();
        SvgParser::render(svg_data, &mut list, 0, 0, 1.0, Color::WHITE);

        // Basic check: did we get any commands?
        assert!(!list.iter().as_slice().is_empty());

        // Iterate to check for panics or weirdness
        for cmd in list.iter() {
            // println!("{:?}", cmd);
        }
    }

    #[test]
    fn test_tokenizer() {
        let input = "M10 20 L 30.5,40";
        let mut tokenizer = PathTokenizer::new(input);
        match tokenizer.next() {
            Some(Token::Command('M')) => {},
            _ => panic!("Expected M"),
        }
        match tokenizer.next() {
             Some(Token::Number(n)) if n == 10.0 => {},
             _ => panic!("Expected 10"),
        }
        match tokenizer.next() {
             Some(Token::Number(n)) if n == 20.0 => {},
             _ => panic!("Expected 20"),
        }
        match tokenizer.next() {
             Some(Token::Command('L')) => {},
             _ => panic!("Expected L"),
        }
        match tokenizer.next() {
             Some(Token::Number(n)) if n == 30.5 => {},
             _ => panic!("Expected 30.5"),
        }
    }

    #[test]
    fn test_scientific() {
        let input = "l2e-3";
        let mut tokenizer = PathTokenizer::new(input);
        match tokenizer.next() {
            Some(Token::Command('l')) => {},
             _ => panic!("Expected l"),
        }
        match tokenizer.next() {
             Some(Token::Number(n)) => assert!((n - 0.002).abs() < 1e-6),
             _ => panic!("Expected 0.002"),
        }
    }
}

fn parse_path(d: &str, list: &mut DrawList) {
    let mut tokenizer = PathTokenizer::new(d);
    let mut current_cmd = ' ';
    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut start_x = 0.0;
    let mut start_y = 0.0;
    // For smooth curves (S/s)
    let mut last_ctrl_x = 0.0;
    let mut last_ctrl_y = 0.0;
    let mut last_cmd_was_curve = false;

    // We need to handle implicit repetition of commands
    // e.g. "L 10 10 20 20" means "L 10 10 L 20 20"

    let mut expected_args = 0;
    let mut args = Vec::new();

    // Helper to process args for current_cmd
    let mut process_cmd = |cmd: char, args: &[f32], cx: &mut f32, cy: &mut f32, start_x: &mut f32, start_y: &mut f32, last_cx: &mut f32, last_cy: &mut f32, last_curve: &mut bool| {
        match cmd {
            'M' | 'm' => {
                let (nx, ny) = if cmd == 'M' { (args[0], args[1]) } else { (*cx + args[0], *cy + args[1]) };
                list.commands().push(DrawCmd::MoveTo { point: Point::new(nx as i32, ny as i32) });
                *cx = nx; *cy = ny;
                *start_x = nx; *start_y = ny;
                *last_curve = false;
            }
            'L' | 'l' => {
                let (nx, ny) = if cmd == 'L' { (args[0], args[1]) } else { (*cx + args[0], *cy + args[1]) };
                list.commands().push(DrawCmd::LineTo { point: Point::new(nx as i32, ny as i32) });
                *cx = nx; *cy = ny;
                *last_curve = false;
            }
            'H' | 'h' => {
                let nx = if cmd == 'H' { args[0] } else { *cx + args[0] };
                let ny = *cy;
                list.commands().push(DrawCmd::LineTo { point: Point::new(nx as i32, ny as i32) });
                *cx = nx;
                *last_curve = false;
            }
            'V' | 'v' => {
                let nx = *cx;
                let ny = if cmd == 'V' { args[0] } else { *cy + args[0] };
                list.commands().push(DrawCmd::LineTo { point: Point::new(nx as i32, ny as i32) });
                *cy = ny;
                *last_curve = false;
            }
            'C' | 'c' => {
                let (c1x, c1y) = if cmd == 'C' { (args[0], args[1]) } else { (*cx + args[0], *cy + args[1]) };
                let (c2x, c2y) = if cmd == 'C' { (args[2], args[3]) } else { (*cx + args[2], *cy + args[3]) };
                let (ex, ey)   = if cmd == 'C' { (args[4], args[5]) } else { (*cx + args[4], *cy + args[5]) };
                list.commands().push(DrawCmd::CurveTo {
                    c1: Point::new(c1x as i32, c1y as i32),
                    c2: Point::new(c2x as i32, c2y as i32),
                    to: Point::new(ex as i32, ey as i32)
                });
                *last_cx = c2x; *last_cy = c2y;
                *cx = ex; *cy = ey;
                *last_curve = true;
            }
            'S' | 's' => {
                // Smooth cubic
                let (c1x, c1y) = if *last_curve {
                    (2.0 * *cx - *last_cx, 2.0 * *cy - *last_cy)
                } else {
                    (*cx, *cy)
                };
                let (c2x, c2y) = if cmd == 'S' { (args[0], args[1]) } else { (*cx + args[0], *cy + args[1]) };
                let (ex, ey)   = if cmd == 'S' { (args[2], args[3]) } else { (*cx + args[2], *cy + args[3]) };

                list.commands().push(DrawCmd::CurveTo {
                    c1: Point::new(c1x as i32, c1y as i32),
                    c2: Point::new(c2x as i32, c2y as i32),
                    to: Point::new(ex as i32, ey as i32)
                });
                *last_cx = c2x; *last_cy = c2y;
                *cx = ex; *cy = ey;
                *last_curve = true;
            }
            'Z' | 'z' => {
                list.commands().push(DrawCmd::ClosePath);
                *cx = *start_x; *cy = *start_y;
                *last_curve = false;
            }
            _ => {}
        }
    };

    // Arg counts
    fn get_arg_count(c: char) -> usize {
        match c.to_ascii_uppercase() {
            'M' => 2,
            'L' => 2,
            'H' => 1,
            'V' => 1,
            'C' => 6,
            'S' => 4,
            'Q' => 4,
            'T' => 2,
            'A' => 7,
            'Z' => 0,
            _ => 0
        }
    }

    loop {
        let token = tokenizer.next();
        match token {
            Some(Token::Command(c)) => {
                current_cmd = c;
                expected_args = get_arg_count(c);
                args.clear();

                if expected_args == 0 {
                    process_cmd(c, &args, &mut cx, &mut cy, &mut start_x, &mut start_y, &mut last_ctrl_x, &mut last_ctrl_y, &mut last_cmd_was_curve);
                }
            }
            Some(Token::Number(n)) => {
                args.push(n);
                if args.len() == expected_args {
                    process_cmd(current_cmd, &args, &mut cx, &mut cy, &mut start_x, &mut start_y, &mut last_ctrl_x, &mut last_ctrl_y, &mut last_cmd_was_curve);
                    args.clear();

                    // Implicit repetition
                    // If M/m -> implicit L/l
                    if current_cmd == 'M' { current_cmd = 'L'; }
                    else if current_cmd == 'm' { current_cmd = 'l'; }
                    // Others repeat same cmd
                }
            }
            None => break
        }
    }
}
