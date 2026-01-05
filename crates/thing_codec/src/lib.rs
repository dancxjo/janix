#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use abi::ids::{SymbolId, ThingId, crc64};
use abi::bodies::ThingEnvelopeV1;
use serde::{Serialize, Deserialize};

pub trait GraphClient {
    fn create_thing(&mut self, kind: SymbolId) -> Result<ThingId, i32>;
    fn set_body(&mut self, id: ThingId, body: &[u8]) -> Result<(), i32>;
    fn get_body(&self, id: ThingId) -> Result<Vec<u8>, i32>;
}

pub trait Thing: Serialize + for<'de> Deserialize<'de> {
    const KIND: SymbolId;
    const SCHEMA_HASH: u64;
    const SCHEMA_VERSION: u32;
    const SCHEMA_STR: &'static str = "";

    fn encode(&self) -> Vec<u8> {
        postcard::to_allocvec(self).unwrap()
    }

    fn encode_full(&self) -> Vec<u8> {
        let payload = self.encode();
        encode_envelope(
            Self::KIND,
            Self::SCHEMA_HASH,
            Self::SCHEMA_VERSION,
            Self::SCHEMA_STR,
            &payload
        )
    }

    fn decode_payload(bytes: &[u8]) -> Result<Self, ThingDecodeError> {
        postcard::from_bytes(bytes).map_err(|_| ThingDecodeError::PostcardError)
    }

    fn decode_full(bytes: &[u8]) -> Result<Self, ThingDecodeError> {
        let envelope = decode_envelope(bytes)?;
        
        if envelope.kind != Self::KIND.0 {
            return Err(ThingDecodeError::KindMismatch);
        }
        if envelope.schema_hash != Self::SCHEMA_HASH {
            return Err(ThingDecodeError::SchemaHashMismatch);
        }

        Self::decode_payload(envelope.payload)
    }

    // Static API
    fn create<G: GraphClient>(&self, g: &mut G) -> Result<ThingId, i32> {
        let id = g.create_thing(Self::KIND)?;
        g.set_body(id, &self.encode_full())?;
        Ok(id)
    }

    fn read<G: GraphClient>(g: &G, id: ThingId) -> Result<Self, i32> {
        let body = g.get_body(id)?;
        Self::decode_full(&body).map_err(|_| -1) 
    }

    fn write<G: GraphClient>(&self, g: &mut G, id: ThingId) -> Result<(), i32> {
        g.set_body(id, &self.encode_full())
    }

    fn try_read<G: GraphClient>(g: &G, id: ThingId) -> Result<Option<Self>, i32> {
        match g.get_body(id) {
            Ok(body) => {
                match Self::decode_full(&body) {
                    Ok(val) => Ok(Some(val)),
                    Err(_) => Ok(None),
                }
            }
            Err(e) if e == abi::syscall::err::ENOENT => Ok(None),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThingDecodeError {
    InvalidMagic,
    InvalidVersion,
    InvalidLength,
    IntegrityMismatch,
    KindMismatch,
    SchemaHashMismatch,
    PostcardError,
}

pub struct EnvelopeView<'a> {
    pub kind: u64,
    pub schema_hash: u64,
    pub schema_version: u32,
    pub schema_str: &'a str,
    pub payload: &'a [u8],
}

pub fn encode_envelope(
    kind: SymbolId,
    schema_hash: u64,
    schema_version: u32,
    schema_str: &str,
    payload: &[u8]
) -> Vec<u8> {
    let schema_bytes = schema_str.as_bytes();
    let schema_len = schema_bytes.len();
    
    let schema_str_end = core::mem::size_of::<ThingEnvelopeV1>() + schema_len;
    let payload_start = (schema_str_end + 7) & !7;
    let total_len = payload_start + payload.len();

    let mut bytes = Vec::with_capacity(total_len);
    
    let header = ThingEnvelopeV1 {
        magic: ThingEnvelopeV1::MAGIC,
        env_version: ThingEnvelopeV1::VERSION,
        flags: 0,
        kind: kind.0,
        schema_hash,
        schema_version,
        schema_str_len: schema_len as u16,
        payload_format: 0, 
        reserved0: 0,
        payload_len: payload.len() as u32,
        body_len: total_len as u32,
        integrity: 0,
    };

    unsafe {
        let ptr = &header as *const _ as *const u8;
        bytes.extend_from_slice(core::slice::from_raw_parts(ptr, core::mem::size_of::<ThingEnvelopeV1>()));
    }

    bytes.extend_from_slice(schema_bytes);
    
    while bytes.len() < payload_start {
        bytes.push(0);
    }

    bytes.extend_from_slice(payload);

    let data_for_crc = bytes.clone();
    let integrity_offset = offset_of_integrity();
    let digest = crc64(&data_for_crc);
    
    let digest_bytes = digest.to_le_bytes();
    for i in 0..8 {
        bytes[integrity_offset + i] = digest_bytes[i];
    }

    bytes
}

pub fn decode_envelope(bytes: &[u8]) -> Result<EnvelopeView<'_>, ThingDecodeError> {
    if bytes.len() < core::mem::size_of::<ThingEnvelopeV1>() {
        return Err(ThingDecodeError::InvalidLength);
    }

    let header = unsafe { &*(bytes.as_ptr() as *const ThingEnvelopeV1) };

    if header.magic != ThingEnvelopeV1::MAGIC {
        return Err(ThingDecodeError::InvalidMagic);
    }
    if header.env_version != ThingEnvelopeV1::VERSION {
        return Err(ThingDecodeError::InvalidVersion);
    }
    if header.body_len as usize != bytes.len() {
        return Err(ThingDecodeError::InvalidLength);
    }

    let mut data_for_crc = bytes.to_vec();
    let integrity_offset = offset_of_integrity();
    for i in 0..8 {
        data_for_crc[integrity_offset + i] = 0;
    }
    if crc64(&data_for_crc) != header.integrity {
        return Err(ThingDecodeError::IntegrityMismatch);
    }

    let schema_str_start = core::mem::size_of::<ThingEnvelopeV1>();
    let schema_str_end = schema_str_start + header.schema_str_len as usize;
    if schema_str_end > bytes.len() {
        return Err(ThingDecodeError::InvalidLength);
    }

    let schema_str = core::str::from_utf8(&bytes[schema_str_start..schema_str_end])
        .map_err(|_| ThingDecodeError::InvalidLength)?;

    let payload_start = (schema_str_end + 7) & !7;
    let payload_end = payload_start + header.payload_len as usize;
    if payload_end > bytes.len() {
        return Err(ThingDecodeError::InvalidLength);
    }

    Ok(EnvelopeView {
        kind: header.kind,
        schema_hash: header.schema_hash,
        schema_version: header.schema_version,
        schema_str,
        payload: &bytes[payload_start..payload_end],
    })
}

fn offset_of_integrity() -> usize {
    core::mem::size_of::<ThingEnvelopeV1>() - 8
}
