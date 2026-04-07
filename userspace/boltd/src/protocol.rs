use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;

use crate::packstream::{decode, encode, Value};

#[derive(Debug)]
pub enum BoltMessage {
    Init {
        client_id: alloc::string::String,
        auth: alloc::collections::BTreeMap<alloc::string::String, Value>,
    },
    Run {
        query: alloc::string::String,
        params: alloc::collections::BTreeMap<alloc::string::String, Value>,
    },
    PullAll,
    Pull {
        meta: alloc::collections::BTreeMap<alloc::string::String, Value>,
    },
    Discard {
        meta: alloc::collections::BTreeMap<alloc::string::String, Value>,
    },
    Hello {
        meta: alloc::collections::BTreeMap<alloc::string::String, Value>,
    },
    Success(alloc::collections::BTreeMap<alloc::string::String, Value>),
    Record(Vec<Value>),
    Failure(alloc::collections::BTreeMap<alloc::string::String, Value>),
}

pub fn decode_message(bytes: &[u8]) -> Option<BoltMessage> {
    let (val, _) = decode(bytes)?;
    match val {
        Value::Struct { tag, mut fields } => {
            match tag {
                // v1/v2 INIT
                0x01 if fields.len() >= 1 => {
                    let auth = if fields.len() > 1 {
                        fields.remove(1).as_map().cloned().unwrap_or_default()
                    } else {
                        Default::default()
                    };
                    let client_id = fields.remove(0).as_str().unwrap_or("").to_string();
                    Some(BoltMessage::Hello {
                        meta: auth, // map init to hello
                    })
                }
                // RUN v1/v4
                0x10 if fields.len() >= 1 => {
                    let params = if fields.len() > 1 {
                        fields.remove(1).as_map().cloned().unwrap_or_default()
                    } else {
                        Default::default()
                    };
                    let query = fields.remove(0).as_str().unwrap_or("").to_string();
                    Some(BoltMessage::Run { query, params })
                }
                // PULL_ALL v1/v2
                0x3F if fields.is_empty() => Some(BoltMessage::PullAll),
                // PULL v4
                0x3F if fields.len() == 1 => {
                    let meta = fields.remove(0).as_map().cloned().unwrap_or_default();
                    Some(BoltMessage::Pull { meta })
                }
                // DISCARD_ALL v1/v2
                0x2F if fields.is_empty() => Some(BoltMessage::Discard {
                    meta: Default::default(),
                }),
                // DISCARD v4
                0x2F if fields.len() == 1 => {
                    let meta = fields.remove(0).as_map().cloned().unwrap_or_default();
                    Some(BoltMessage::Discard { meta })
                }
                _ => None,
            }
        }
        _ => None,
    }
}

pub fn encode_message(msg: &BoltMessage, out: &mut Vec<u8>) {
    let val = match msg {
        BoltMessage::Success(map) => Value::Struct {
            tag: 0x70,
            fields: vec![Value::Map(map.clone())],
        },
        BoltMessage::Record(fields) => Value::Struct {
            tag: 0x71,
            fields: vec![Value::List(fields.clone())],
        },
        BoltMessage::Failure(map) => Value::Struct {
            tag: 0x7F,
            fields: vec![Value::Map(map.clone())],
        },
        _ => unimplemented!(),
    };
    encode(&val, out);
}

pub fn create_chunked(msg: &BoltMessage) -> Vec<u8> {
    let mut payload = Vec::new();
    encode_message(msg, &mut payload);

    let mut out = Vec::new();
    // basic chunking: just stick it in one chunk if it's small, otherwise loop
    let mut offset = 0;
    while offset < payload.len() {
        let chunk_size = core::cmp::min(payload.len() - offset, 65535);
        out.extend_from_slice(&(chunk_size as u16).to_be_bytes());
        out.extend_from_slice(&payload[offset..offset + chunk_size]);
        offset += chunk_size;
    }
    // end of message chunk
    out.extend_from_slice(&[0x00, 0x00]);
    out
}
