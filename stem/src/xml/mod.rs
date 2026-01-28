use alloc::string::String;
use alloc::vec::Vec;
pub mod ingest;
pub mod model;

use xml_no_std::reader::{EventReader, XmlEvent};

pub struct XmlReader<'a> {
    parser: EventReader<'a, core::slice::Iter<'a, u8>>,
}

impl<'a> XmlReader<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            parser: EventReader::new(source.iter()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    StartElement {
        name: String,
        attributes: Attributes,
    },
    EndElement {
        name: String,
    },
    Text(String),
}

#[derive(Debug, Clone)]
pub struct Attributes {
    inner: Vec<(String, String)>,
}

impl Attributes {
    pub fn get(&self, name: &str) -> Option<&str> {
        for (k, v) in &self.inner {
            if k == name {
                return Some(v);
            }
        }
        None
    }
}

impl IntoIterator for Attributes {
    type Item = (String, String);
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> Iterator for XmlReader<'a> {
    type Item = Result<Event, xml_no_std::reader::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.parser.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    let attrs = attributes
                        .into_iter()
                        .map(|a| (a.name.local_name, a.value))
                        .collect();
                    return Some(Ok(Event::StartElement {
                        name: name.local_name,
                        attributes: Attributes { inner: attrs },
                    }));
                }
                Ok(XmlEvent::EndElement { name }) => {
                    return Some(Ok(Event::EndElement {
                        name: name.local_name,
                    }));
                }
                Ok(XmlEvent::Characters(data)) => {
                    return Some(Ok(Event::Text(data)));
                }
                Ok(XmlEvent::EndDocument) => return None,
                Err(e) => return Some(Err(e)),
                _ => continue, // skip comments, processing instructions, etc.
            }
        }
    }
}

// Helpers
pub fn parse_f32(s: &str) -> Option<f32> {
    s.parse().ok()
}

pub fn parse_u32(s: &str) -> Option<u32> {
    s.parse().ok()
}

/// Parse a length string. Returns pixels.
/// "10" -> 10.0
/// "10px" -> 10.0
/// Other units are ignored/unsupported for now.
pub fn parse_length_px(s: &str) -> Option<f32> {
    let s = s.trim();
    if s.ends_with("px") {
        parse_f32(&s[..s.len() - 2])
    } else {
        parse_f32(s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const TRANSPARENT: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
}

pub fn parse_color(s: &str) -> Option<Color> {
    let s = s.trim();
    if s.starts_with('#') {
        let hex = &s[1..];
        match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
                Some(Color {
                    r: (r << 4) | r,
                    g: (g << 4) | g,
                    b: (b << 4) | b,
                    a: 255,
                })
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Color { r, g, b, a: 255 })
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                Some(Color { r, g, b, a })
            }
            _ => None,
        }
    } else {
        match s {
            "none" => Some(Color::TRANSPARENT),
            "black" => Some(Color::BLACK),
            "white" => Some(Color::WHITE),
            // TODO: generic rgb() parsing
            _ => None,
        }
    }
}

pub enum TransformCmd {
    Translate(f32, f32),
    Scale(f32, f32),
    Rotate(f32),
}

/// Simple parser for "translate(x, y) scale(s) ..."
pub fn parse_transform(mut s: &str) -> Vec<TransformCmd> {
    let mut cmds = Vec::new();

    // Helper to find next '('
    while let Some(idx) = s.find('(') {
        let name = s[..idx].trim();
        s = &s[idx + 1..];

        // Find matching ')'
        let end = match s.find(')') {
            Some(i) => i,
            None => break, // Malformed
        };

        let args_str = &s[..end];
        s = &s[end + 1..];

        // Split args by comma or whitespace
        let args: Vec<f32> = args_str
            .split(|c| c == ',' || c == ' ' || c == '\t' || c == '\n')
            .filter(|p| !p.is_empty())
            .filter_map(parse_f32)
            .collect();

        match name {
            "translate" => {
                if args.len() == 1 {
                    cmds.push(TransformCmd::Translate(args[0], 0.0));
                } else if args.len() >= 2 {
                    cmds.push(TransformCmd::Translate(args[0], args[1]));
                }
            }
            "scale" => {
                if args.len() == 1 {
                    cmds.push(TransformCmd::Scale(args[0], args[0]));
                } else if args.len() >= 2 {
                    cmds.push(TransformCmd::Scale(args[0], args[1]));
                }
            }
            "rotate" => {
                if !args.is_empty() {
                    cmds.push(TransformCmd::Rotate(args[0]));
                }
            }
            _ => {} // Ignore unknown transforms
        }
    }

    cmds
}
