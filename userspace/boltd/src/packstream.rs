use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Bytes(Vec<u8>),
    String(String),
    List(Vec<Value>),
    Map(BTreeMap<String, Value>),
    Struct { tag: u8, fields: Vec<Value> },
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        if let Value::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_map(&self) -> Option<&BTreeMap<String, Value>> {
        if let Value::Map(m) = self {
            Some(m)
        } else {
            None
        }
    }
}

pub fn encode(item: &Value, out: &mut Vec<u8>) {
    match item {
        Value::Null => out.push(0xC0),
        Value::Boolean(b) => {
            if *b {
                out.push(0xC3);
            } else {
                out.push(0xC2);
            }
        }
        Value::Integer(i) => {
            if *i >= -16 && *i <= 127 {
                out.push(*i as i8 as u8);
            } else if *i >= i8::MIN as i64 && *i <= i8::MAX as i64 {
                out.push(0xC8);
                out.push(*i as u8);
            } else if *i >= i16::MIN as i64 && *i <= i16::MAX as i64 {
                out.push(0xC9);
                out.extend_from_slice(&(*i as i16).to_be_bytes());
            } else if *i >= i32::MIN as i64 && *i <= i32::MAX as i64 {
                out.push(0xCA);
                out.extend_from_slice(&(*i as i32).to_be_bytes());
            } else {
                out.push(0xCB);
                out.extend_from_slice(&i.to_be_bytes());
            }
        }
        Value::Float(f) => {
            out.push(0xC1);
            out.extend_from_slice(&f.to_bits().to_be_bytes());
        }
        Value::Bytes(b) => {
            let len = b.len();
            if len <= u8::MAX as usize {
                out.push(0xCC);
                out.push(len as u8);
            } else if len <= u16::MAX as usize {
                out.push(0xCD);
                out.extend_from_slice(&(len as u16).to_be_bytes());
            } else {
                out.push(0xCE);
                out.extend_from_slice(&(len as u32).to_be_bytes());
            }
            out.extend_from_slice(b);
        }
        Value::String(s) => {
            let bytes = s.as_bytes();
            let len = bytes.len();
            if len <= 15 {
                out.push(0x80 | (len as u8));
            } else if len <= u8::MAX as usize {
                out.push(0xD0);
                out.push(len as u8);
            } else if len <= u16::MAX as usize {
                out.push(0xD1);
                out.extend_from_slice(&(len as u16).to_be_bytes());
            } else {
                out.push(0xD2);
                out.extend_from_slice(&(len as u32).to_be_bytes());
            }
            out.extend_from_slice(bytes);
        }
        Value::List(l) => {
            let len = l.len();
            if len <= 15 {
                out.push(0x90 | (len as u8));
            } else if len <= u8::MAX as usize {
                out.push(0xD4);
                out.push(len as u8);
            } else if len <= u16::MAX as usize {
                out.push(0xD5);
                out.extend_from_slice(&(len as u16).to_be_bytes());
            } else {
                out.push(0xD6);
                out.extend_from_slice(&(len as u32).to_be_bytes());
            }
            for v in l {
                encode(v, out);
            }
        }
        Value::Map(m) => {
            let len = m.len();
            if len <= 15 {
                out.push(0xA0 | (len as u8));
            } else if len <= u8::MAX as usize {
                out.push(0xD8);
                out.push(len as u8);
            } else if len <= u16::MAX as usize {
                out.push(0xD9);
                out.extend_from_slice(&(len as u16).to_be_bytes());
            } else {
                out.push(0xDA);
                out.extend_from_slice(&(len as u32).to_be_bytes());
            }
            for (k, v) in m {
                encode(&Value::String(k.clone()), out);
                encode(v, out);
            }
        }
        Value::Struct { tag, fields } => {
            let len = fields.len();
            if len <= 15 {
                out.push(0xB0 | (len as u8));
            } else {
                // not strictly supported by packstream tiny structs, packstream spec says struct max is 15 fields
                // but if we had to, there is no large struct representation.
                panic!("structs must have <= 15 fields");
            }
            out.push(*tag);
            for v in fields {
                encode(v, out);
            }
        }
    }
}

pub fn decode(bytes: &[u8]) -> Option<(Value, usize)> {
    if bytes.is_empty() {
        return None;
    }

    let marker = bytes[0];
    let pos = 1;

    match marker {
        0xc0 => Some((Value::Null, pos)),
        0xc2 => Some((Value::Boolean(false), pos)),
        0xc3 => Some((Value::Boolean(true), pos)),
        0xc1 => {
            if bytes.len() < pos + 8 {
                return None;
            }
            let bits = u64::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());
            Some((Value::Float(f64::from_bits(bits)), pos + 8))
        }
        0xc8 => {
            if bytes.len() < pos + 1 {
                return None;
            }
            Some((Value::Integer(bytes[pos] as i8 as i64), pos + 1))
        }
        0xc9 => {
            if bytes.len() < pos + 2 {
                return None;
            }
            let val = i16::from_be_bytes(bytes[pos..pos + 2].try_into().unwrap());
            Some((Value::Integer(val as i64), pos + 2))
        }
        0xca => {
            if bytes.len() < pos + 4 {
                return None;
            }
            let val = i32::from_be_bytes(bytes[pos..pos + 4].try_into().unwrap());
            Some((Value::Integer(val as i64), pos + 4))
        }
        0xcb => {
            if bytes.len() < pos + 8 {
                return None;
            }
            let val = i64::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());
            Some((Value::Integer(val), pos + 8))
        }
        0xcc | 0xcd | 0xce => {
            // Bytes
            let (len, head) = match marker {
                0xcc => {
                    if bytes.len() < pos + 1 {
                        return None;
                    }
                    (bytes[pos] as usize, pos + 1)
                }
                0xcd => {
                    if bytes.len() < pos + 2 {
                        return None;
                    }
                    (
                        u16::from_be_bytes(bytes[pos..pos + 2].try_into().unwrap()) as usize,
                        pos + 2,
                    )
                }
                0xce => {
                    if bytes.len() < pos + 4 {
                        return None;
                    }
                    (
                        u32::from_be_bytes(bytes[pos..pos + 4].try_into().unwrap()) as usize,
                        pos + 4,
                    )
                }
                _ => unreachable!(),
            };
            if bytes.len() < head + len {
                return None;
            }
            Some((Value::Bytes(bytes[head..head + len].to_vec()), head + len))
        }
        0xd0 | 0xd1 | 0xd2 | 0x80..=0x8f => {
            // Strings
            let (len, head) = if marker >= 0x80 && marker <= 0x8f {
                ((marker & 0x0f) as usize, pos)
            } else {
                match marker {
                    0xd0 => {
                        if bytes.len() < pos + 1 {
                            return None;
                        }
                        (bytes[pos] as usize, pos + 1)
                    }
                    0xd1 => {
                        if bytes.len() < pos + 2 {
                            return None;
                        }
                        (
                            u16::from_be_bytes(bytes[pos..pos + 2].try_into().unwrap()) as usize,
                            pos + 2,
                        )
                    }
                    0xd2 => {
                        if bytes.len() < pos + 4 {
                            return None;
                        }
                        (
                            u32::from_be_bytes(bytes[pos..pos + 4].try_into().unwrap()) as usize,
                            pos + 4,
                        )
                    }
                    _ => unreachable!(),
                }
            };
            if bytes.len() < head + len {
                return None;
            }
            let s = String::from_utf8(bytes[head..head + len].to_vec()).ok()?;
            Some((Value::String(s), head + len))
        }
        0xd4 | 0xd5 | 0xd6 | 0x90..=0x9f => {
            // Lists
            let (len, mut head) = if marker >= 0x90 && marker <= 0x9f {
                ((marker & 0x0f) as usize, pos)
            } else {
                match marker {
                    0xd4 => {
                        if bytes.len() < pos + 1 {
                            return None;
                        }
                        (bytes[pos] as usize, pos + 1)
                    }
                    0xd5 => {
                        if bytes.len() < pos + 2 {
                            return None;
                        }
                        (
                            u16::from_be_bytes(bytes[pos..pos + 2].try_into().unwrap()) as usize,
                            pos + 2,
                        )
                    }
                    0xd6 => {
                        if bytes.len() < pos + 4 {
                            return None;
                        }
                        (
                            u32::from_be_bytes(bytes[pos..pos + 4].try_into().unwrap()) as usize,
                            pos + 4,
                        )
                    }
                    _ => unreachable!(),
                }
            };
            let mut arr = Vec::with_capacity(len);
            for _ in 0..len {
                let (val, consumed) = decode(&bytes[head..])?;
                arr.push(val);
                head += consumed;
            }
            Some((Value::List(arr), head))
        }
        0xd8 | 0xd9 | 0xda | 0xa0..=0xaf => {
            // Maps
            let (len, mut head) = if marker >= 0xa0 && marker <= 0xaf {
                ((marker & 0x0f) as usize, pos)
            } else {
                match marker {
                    0xd8 => {
                        if bytes.len() < pos + 1 {
                            return None;
                        }
                        (bytes[pos] as usize, pos + 1)
                    }
                    0xd9 => {
                        if bytes.len() < pos + 2 {
                            return None;
                        }
                        (
                            u16::from_be_bytes(bytes[pos..pos + 2].try_into().unwrap()) as usize,
                            pos + 2,
                        )
                    }
                    0xda => {
                        if bytes.len() < pos + 4 {
                            return None;
                        }
                        (
                            u32::from_be_bytes(bytes[pos..pos + 4].try_into().unwrap()) as usize,
                            pos + 4,
                        )
                    }
                    _ => unreachable!(),
                }
            };
            let mut map = BTreeMap::new();
            for _ in 0..len {
                let (k_val, k_cons) = decode(&bytes[head..])?;
                head += k_cons;
                let (v_val, v_cons) = decode(&bytes[head..])?;
                head += v_cons;

                if let Value::String(s) = k_val {
                    map.insert(s, v_val);
                } else {
                    return None; // invalid map key
                }
            }
            Some((Value::Map(map), head))
        }
        0xb0..=0xbf => {
            // Structs
            let len = marker & 0x0f;
            if bytes.len() < pos + 1 {
                return None;
            }
            let tag = bytes[pos];
            let mut head = pos + 1;
            let mut fields = Vec::with_capacity(len as usize);
            for _ in 0..len {
                let (val, cons) = decode(&bytes[head..])?;
                fields.push(val);
                head += cons;
            }
            Some((Value::Struct { tag, fields }, head))
        }
        0x00..=0x7f => {
            // Positive tiny int
            Some((Value::Integer(marker as i64), pos))
        }
        0xf0..=0xff => {
            // Negative tiny int (-16 .. -1)
            let val = marker as i8;
            Some((Value::Integer(val as i64), pos))
        }
        _ => None,
    }
}
