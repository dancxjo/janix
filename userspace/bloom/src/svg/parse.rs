use crate::geometry::{Color, Transform};
use crate::svg::ir::{Path2D, PathCommand};
use alloc::string::String;
use alloc::vec::Vec;

pub fn parse_path_d(d: &str) -> Path2D {
    use crate::svg::ir::PointF;
    let mut verbs = Vec::new();
    let mut parser = TokenParser::new(d);

    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut last_cp: Option<PointF> = None;
    let mut start_p: Option<PointF> = None;

    while let Some(cmd) = parser.next_command() {
        match cmd {
            'M' | 'm' => {
                let is_rel = cmd == 'm';
                let mut first = true;
                while let (Some(x), Some(y)) = (parser.next_number(), parser.next_number()) {
                    let p = if is_rel {
                        PointF {
                            x: cx + x,
                            y: cy + y,
                        }
                    } else {
                        PointF { x, y }
                    };
                    if first {
                        verbs.push(PathCommand::MoveTo(p));
                        start_p = Some(p);
                        first = false;
                    } else {
                        verbs.push(PathCommand::LineTo(p));
                    }
                    cx = p.x;
                    cy = p.y;
                    last_cp = None;
                }
            }
            'L' | 'l' => {
                let is_rel = cmd == 'l';
                while let (Some(x), Some(y)) = (parser.next_number(), parser.next_number()) {
                    let p = if is_rel {
                        PointF {
                            x: cx + x,
                            y: cy + y,
                        }
                    } else {
                        PointF { x, y }
                    };
                    verbs.push(PathCommand::LineTo(p));
                    cx = p.x;
                    cy = p.y;
                    last_cp = None;
                }
            }
            'H' | 'h' => {
                let is_rel = cmd == 'h';
                while let Some(x) = parser.next_number() {
                    let nx = if is_rel { cx + x } else { x };
                    verbs.push(PathCommand::LineTo(PointF { x: nx, y: cy }));
                    cx = nx;
                    last_cp = None;
                }
            }
            'V' | 'v' => {
                let is_rel = cmd == 'v';
                while let Some(y) = parser.next_number() {
                    let ny = if is_rel { cy + y } else { y };
                    verbs.push(PathCommand::LineTo(PointF { x: cx, y: ny }));
                    cy = ny;
                    last_cp = None;
                }
            }
            'C' | 'c' => {
                let is_rel = cmd == 'c';
                while let (Some(x1), Some(y1), Some(x2), Some(y2), Some(x), Some(y)) = (
                    parser.next_number(),
                    parser.next_number(),
                    parser.next_number(),
                    parser.next_number(),
                    parser.next_number(),
                    parser.next_number(),
                ) {
                    let p1 = if is_rel {
                        PointF {
                            x: cx + x1,
                            y: cy + y1,
                        }
                    } else {
                        PointF { x: x1, y: y1 }
                    };
                    let p2 = if is_rel {
                        PointF {
                            x: cx + x2,
                            y: cy + y2,
                        }
                    } else {
                        PointF { x: x2, y: y2 }
                    };
                    let p = if is_rel {
                        PointF {
                            x: cx + x,
                            y: cy + y,
                        }
                    } else {
                        PointF { x, y }
                    };
                    verbs.push(PathCommand::CubicTo(p1, p2, p));
                    cx = p.x;
                    cy = p.y;
                    last_cp = Some(p2);
                }
            }
            'S' | 's' => {
                let is_rel = cmd == 's';
                while let (Some(x2), Some(y2), Some(x), Some(y)) = (
                    parser.next_number(),
                    parser.next_number(),
                    parser.next_number(),
                    parser.next_number(),
                ) {
                    let p1 = if let Some(cp) = last_cp {
                        PointF {
                            x: 2.0 * cx - cp.x,
                            y: 2.0 * cy - cp.y,
                        }
                    } else {
                        PointF { x: cx, y: cy }
                    };
                    let p2 = if is_rel {
                        PointF {
                            x: cx + x2,
                            y: cy + y2,
                        }
                    } else {
                        PointF { x: x2, y: y2 }
                    };
                    let p = if is_rel {
                        PointF {
                            x: cx + x,
                            y: cy + y,
                        }
                    } else {
                        PointF { x, y }
                    };
                    verbs.push(PathCommand::CubicTo(p1, p2, p));
                    cx = p.x;
                    cy = p.y;
                    last_cp = Some(p2);
                }
            }
            'Q' | 'q' => {
                let is_rel = cmd == 'q';
                while let (Some(x1), Some(y1), Some(x), Some(y)) = (
                    parser.next_number(),
                    parser.next_number(),
                    parser.next_number(),
                    parser.next_number(),
                ) {
                    let p1 = if is_rel {
                        PointF {
                            x: cx + x1,
                            y: cy + y1,
                        }
                    } else {
                        PointF { x: x1, y: y1 }
                    };
                    let p = if is_rel {
                        PointF {
                            x: cx + x,
                            y: cy + y,
                        }
                    } else {
                        PointF { x, y }
                    };
                    verbs.push(PathCommand::QuadTo(p1, p));
                    cx = p.x;
                    cy = p.y;
                    last_cp = Some(p1);
                }
            }
            'T' | 't' => {
                let is_rel = cmd == 't';
                while let (Some(x), Some(y)) = (parser.next_number(), parser.next_number()) {
                    let p1 = if let Some(cp) = last_cp {
                        PointF {
                            x: 2.0 * cx - cp.x,
                            y: 2.0 * cy - cp.y,
                        }
                    } else {
                        PointF { x: cx, y: cy }
                    };
                    let p = if is_rel {
                        PointF {
                            x: cx + x,
                            y: cy + y,
                        }
                    } else {
                        PointF { x, y }
                    };
                    verbs.push(PathCommand::QuadTo(p1, p));
                    cx = p.x;
                    cy = p.y;
                    last_cp = Some(p1);
                }
            }
            'Z' | 'z' => {
                verbs.push(PathCommand::Close);
                if let Some(s) = start_p {
                    cx = s.x;
                    cy = s.y;
                }
                last_cp = None;
            }
            _ => {
                last_cp = None;
            } // Reset on unknown
        }
    }

    Path2D { verbs }
}

struct TokenParser<'a> {
    input: &'a str,
}

impl<'a> TokenParser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input }
    }

    fn skip_separators(&mut self) {
        let mut n = 0;
        for c in self.input.chars() {
            if c.is_ascii_whitespace() || c == ',' {
                n += c.len_utf8();
            } else {
                break;
            }
        }
        self.input = &self.input[n..];
    }

    fn next_command(&mut self) -> Option<char> {
        self.skip_separators();
        let c = self.input.chars().next()?;
        if c.is_alphabetic() {
            self.input = &self.input[c.len_utf8()..];
            Some(c)
        } else {
            None
        }
    }

    fn next_number(&mut self) -> Option<f32> {
        self.skip_separators();

        let mut end = 0;
        let mut chars = self.input.chars().peekable();

        // optional sign
        if let Some(&c) = chars.peek() {
            if c == '-' || c == '+' {
                end += 1;
                chars.next();
            }
        }

        let mut has_digits = false;

        // digits before dot
        while let Some(&c) = chars.peek() {
            if c.is_ascii_digit() {
                end += 1;
                chars.next();
                has_digits = true;
            } else {
                break;
            }
        }

        // dot
        if let Some(&c) = chars.peek() {
            if c == '.' {
                end += 1;
                chars.next();
                // digits after dot
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() {
                        end += 1;
                        chars.next();
                        has_digits = true;
                    } else {
                        break;
                    }
                }
            }
        }

        // exponent? e-10
        if has_digits {
            if let Some(&c) = chars.peek() {
                if c == 'e' || c == 'E' {
                    end += 1;
                    chars.next();
                    // sign
                    if let Some(&c) = chars.peek() {
                        if c == '-' || c == '+' {
                            end += 1;
                            chars.next();
                        }
                    }
                    // digits
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_digit() {
                            end += 1;
                            chars.next();
                        } else {
                            break;
                        }
                    }
                }
            }

            let num_str = &self.input[..end];
            self.input = &self.input[end..];
            num_str.parse::<f32>().ok()
        } else {
            None
        }
    }
}

pub fn parse_color(s: &str) -> Option<Color> {
    let s = s.trim();
    if s == "none" {
        return None;
    }
    // Simple hex support
    if s.starts_with('#') {
        let hex = &s[1..];
        match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
                return Some(Color::new((r << 4) | r, (g << 4) | g, (b << 4) | b, 255));
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                return Some(Color::new(r, g, b, 255));
            }
            _ => return None,
        }
    }
    // Named colors
    match s {
        "black" => Some(Color::BLACK),
        "white" => Some(Color::WHITE),
        "red" => Some(Color::new(255, 0, 0, 255)),
        "green" => Some(Color::new(0, 128, 0, 255)),
        "blue" => Some(Color::new(0, 0, 255, 255)),
        _ => None,
    }
}

pub fn parse_transform(s: &str) -> Transform {
    // Reuse token parser slightly or manual
    // Just simple translate/scale support
    let mut t = Transform::identity();
    let s = s.trim();

    // Hacky parse for "translate(x, y)"
    if let Some(idx) = s.find("translate(") {
        let after = &s[idx + 10..];
        if let Some(end) = after.find(')') {
            let args = &after[..end];
            let parts: Vec<&str> = args.split([',', ' ']).filter(|s| !s.is_empty()).collect();
            if parts.len() >= 2 {
                if let (Ok(x), Ok(y)) = (parts[0].parse::<f32>(), parts[1].parse::<f32>()) {
                    t = t.multiply(&Transform::translate(x, y));
                }
            } else if parts.len() == 1 {
                if let Ok(x) = parts[0].parse::<f32>() {
                    t = t.multiply(&Transform::translate(x, 0.0));
                }
            }
        }
    }
    if let Some(idx) = s.find("scale(") {
        let after = &s[idx + 6..];
        if let Some(end) = after.find(')') {
            let args = &after[..end];
            let parts: Vec<&str> = args.split([',', ' ']).filter(|s| !s.is_empty()).collect();
            if !parts.is_empty() {
                let sx = parts[0].parse().unwrap_or(1.0);
                let sy = if parts.len() >= 2 {
                    parts[1].parse().unwrap_or(sx)
                } else {
                    sx
                };
                t = t.multiply(&Transform::scale(sx, sy));
            }
        }
    }

    t
}
