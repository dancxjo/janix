#![no_std]
extern crate alloc;
extern crate stem;

pub mod graph_ui;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use stem::thing::{HandleId, ThingId};
use stem::thing::sys::{bytespace_info, bytespace_read, prop_get};

pub fn read_string_prop(node: ThingId, key: &str) -> Option<String> {
    let bs = prop_get(node, key).ok()?;
    if bs == 0 {
        return None;
    }
    let bytes = read_bytespace(ThingId::from_u64(bs)).ok()?;
    core::str::from_utf8(&bytes)
        .ok()
        .map(|s| s.trim_end_matches('\0').to_string())
}

pub fn read_bytespace(bs_id: ThingId) -> Result<Vec<u8>, abi::errors::Errno> {
    let size = bytespace_info(bs_id)?;
    if size == 0 {
        return Ok(Vec::new());
    }
    let mut out = Vec::with_capacity(size);
    out.resize(size, 0);
    let mut offset = 0usize;
    while offset < size {
        let end = core::cmp::min(offset + 4096, size);
        let read = bytespace_read(bs_id, offset, &mut out[offset..end])?;
        if read == 0 {
            break;
        }
        offset = offset.saturating_add(read);
    }
    Ok(out)
}
