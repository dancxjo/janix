#![no_std]
#![no_main]

extern crate alloc;
use stem::syscall;
use abi::types::{WatchSpec, WatchMode};
use abi::watch::{self, WatchOp};
use abi::query::{QueryStep};
use abi::symbols::{SymbolRefWire, SYMBOL_REF_TAG_STR};
mod behavior;
use behavior::sniff::sniff;
use abi::schema::{kinds, keys, rels};
use stem::thing::ThingId;
use abi::ids::HandleId;
use abi::root::RootWatchFilter;
use stem::root_watch;
use stem::thing::sys::{
    bytespace_info, bytespace_map, bytespace_unmap, bytespace_create, bytespace_write, create_node,
    link, prop_get, prop_set, get_edges, find, intern,
};
use alloc::vec::Vec;
use alloc::string::String;
use ttf_parser::{Face, name_id};
use ttf_parser::name::Name;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    let _ = main();
    syscall::exit(0)
}

fn main() -> Result<(), abi::errors::Errno> {
    syscall::log_write("INGESTD: Starting...", 1)?;

    let kind_str = kinds::BOOT_MODULE;
    let symbol = SymbolRefWire {
        tag: SYMBOL_REF_TAG_STR,
        ptr_or_id: kind_str.as_ptr() as u64,
        len: kind_str.len() as u64,
    };
    let steps = [QueryStep {
        op: abi::query::QueryOpKind::Scan as u64,
        arg1: 512,
        arg2: 0,
        symbol,
    }];

    let bytespace_pred = stem::thing::sys::intern(keys::BYTESPACE).unwrap_or(0);
    let filter = RootWatchFilter::predicate(bytespace_pred);

    let spec = WatchSpec {
        mode: WatchMode::QueryThenStream as u32,
        query_ptr: steps.as_ptr() as u64,
        query_len: steps.len() as u64,
        start_seq: 0,
        filter_ptr: &filter as *const _ as u64,
        filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
        ..Default::default()
    };
    
    // We expect this to fail if kernel is not updated yet or similar, but
    // assuming kernel is ready.
    let watch_id = match syscall::root_watch_open(&spec) {
        Ok(id) => id,
        Err(e) => {
            syscall::log_write("INGESTD: Failed to open watch", 1)?;
            return Err(e);
        }
    };
    
    // Pre-intern symbols to avoid allocation in loop
    // BUT root_intern takes &str. stem wrapper handles ptr/len.
    // For root_create_node, we need a SymbolRefWire (ptr or id).
    // We can use SYMBOL_REF_TAG_STR with ptr.
    
    syscall::log_write("INGESTD: Watch active. Loop start.", 1)?;
    
    fn process_payload(payload: &[u8]) {
        let mut cursor = 0usize;
        while cursor < payload.len() {
            match watch::decode_event(&payload[cursor..]) {
                Ok((header, value)) => {
                    cursor += watch::WATCH_EVENT_HEADER_LEN + value.len();
                    if WatchOp::from_u8(header.op) != Some(WatchOp::Upsert) {
                        continue;
                    }
                    let node_id = header.subject.to_u64_lossy();
                    if node_id != 0 {
                        process_asset(node_id);
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
    }

    let mut seq_out = 0u64;
    let mut watch_buf = [0u8; 4096];

    let _ = root_watch::watch_drain(watch_id, &mut watch_buf, |_seq, bytes| {
        process_payload(bytes);
    });

    loop {
        let res = syscall::root_watch_next(watch_id, &mut seq_out, &mut watch_buf);
        match res {
            Ok(len) if len > 0 => {
                process_payload(&watch_buf[..len]);
            }
            Ok(0) => {
                syscall::sleep_ms(100);
            }
            Err(_) => {
                syscall::sleep_ms(1000);
            }
            _ => {}
        }
    }
}

fn process_asset(id: u64) {
    let node_id = ThingId::from_u64(id);
    let bs_val = match prop_get(node_id, "bytespace") {
        Ok(val) if val != 0 => val,
        _ => return,
    };
    let bytespace_id = ThingId::from_u64(bs_val);
    let size = match bytespace_info(bytespace_id) {
        Ok(size) => size,
        Err(_) => return,
    };

    let ptr = match bytespace_map(bytespace_id) {
        Ok(ptr) => ptr,
        Err(_) => return,
    };

    let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8, size) };
    if slice.is_empty() {
        return;
    }

    if let Some(guess) = sniff(slice) {
        let _ = write_fact(id, guess.mime, guess.confidence);
        if is_font_mime(guess.mime) {
            ingest_font(bytespace_id, size, slice);
        }
    } else {
        let _ = write_fact(id, "application/octet-stream", 0);
    }

    let _ = bytespace_unmap(bytespace_id, ptr);
}

fn write_fact(target_id: u64, mime: &str, confidence: u16) -> Result<(), abi::errors::Errno> {
    // 1. Create content_type_fact node
    // kind = "fact.content_type"
    let kind_sym = SymbolRefWire {
        tag: SYMBOL_REF_TAG_STR,
        ptr_or_id: "fact.content_type".as_ptr() as u64,
        len: "fact.content_type".len() as u64,
    };
    // Note: stem::root_create_node expects usize ptr to Wire.
    let kind_ptr = &kind_sym as *const _ as usize;
    let fact_id = syscall::root_create_node(kind_ptr)?;
    
    // 2. Set props
    // mime
    set_prop_str(fact_id as usize, "mime", mime)?;
    // confidence
    set_prop_u64(fact_id as usize, "confidence", confidence as u64)?;
    // detector
    set_prop_str(fact_id as usize, "detector", "magic-v0")?;
    
    // 3. Link
    // rel = "has_fact"
    let rel_sym = SymbolRefWire {
        tag: SYMBOL_REF_TAG_STR,
        ptr_or_id: "has_fact".as_ptr() as u64,
        len: "has_fact".len() as u64,
    };
    syscall::root_link(target_id as usize, &rel_sym as *const _ as usize, fact_id as usize)?;
    
    syscall::log_write("INGESTD: Tagged asset", 1)?;
    Ok(())
}

fn is_font_mime(mime: &str) -> bool {
    mime.starts_with("font/") || mime == "application/font-sfnt"
}

fn ingest_font(bytespace_id: ThingId, size: usize, data: &[u8]) {
    let face = match Face::parse(data, 0) {
        Ok(face) => face,
        Err(_) => return,
    };

    let family_name = extract_name(&face, name_id::TYPOGRAPHIC_FAMILY, name_id::FAMILY)
        .unwrap_or_else(|| "Unknown".into());
    let style_name = extract_name(&face, name_id::TYPOGRAPHIC_SUBFAMILY, name_id::SUBFAMILY)
        .unwrap_or_else(|| "Regular".into());

    let weight = face.weight().to_number();
    let width = face.width().to_number();
    let slope = if face.is_italic() || face.is_oblique() { 1u8 } else { 0u8 };

    let (ranges, count) = coverage_ranges(&face);

    let file_name = find_module_name(bytespace_id)
        .unwrap_or_else(|| alloc::format!("font-{}.bin", bytespace_id.to_u64_lossy()));

    let family_key = intern(&family_name).unwrap_or(0) as u64;
    let face_key = face_key_hash(family_key, weight, width, slope, &style_name);

    let family_id = get_or_create_node_by_prop(kinds::FONT_FAMILY, keys::FONT_FAMILY_KEY, family_key);
    set_prop_bytespace_str(family_id, keys::FONT_NAME, &family_name);
    let _ = prop_set(family_id, keys::FONT_FAMILY_KEY, family_key);

    if let Some(super_name) = superfamily_from_family(&family_name) {
        let super_key = intern(&super_name).unwrap_or(0) as u64;
        let super_id = get_or_create_node_by_prop(kinds::FONT_SUPERFAMILY, keys::FONT_FAMILY_KEY, super_key);
        set_prop_bytespace_str(super_id, keys::FONT_NAME, &super_name);
        let _ = prop_set(super_id, keys::FONT_FAMILY_KEY, super_key);
        ensure_link(super_id, rels::FONT_CONTAINS, family_id);
    }

    let face_id = get_or_create_node_by_prop(kinds::FONT_FACE, keys::FONT_FACE_KEY, face_key);
    let _ = prop_set(face_id, keys::FONT_FACE_KEY, face_key);
    let _ = prop_set(face_id, keys::FONT_WEIGHT, weight as u64);
    let _ = prop_set(face_id, keys::FONT_WIDTH, width as u64);
    let _ = prop_set(face_id, keys::FONT_SLOPE, slope as u64);
    set_prop_bytespace_str(face_id, keys::FONT_STYLE, &style_name);
    ensure_link(family_id, rels::FONT_CONTAINS, face_id);

    let file_id = get_or_create_node_by_prop(
        kinds::FONT_FILE,
        keys::FONT_BYTESPACE,
        bytespace_id.to_u64_lossy(),
    );
    let _ = prop_set(file_id, keys::FONT_BYTESPACE, bytespace_id.to_u64_lossy());
    let _ = prop_set(file_id, keys::FONT_SIZE_BYTES, size as u64);
    set_prop_bytespace_str(file_id, keys::FONT_NAME, &file_name);
    ensure_link(face_id, rels::FONT_CONTAINS, file_id);

    let coverage_id = ensure_coverage_node(face_id);
    set_prop_bytespace_str(coverage_id, keys::FONT_COVERAGE_RANGES, &ranges);
    let _ = prop_set(coverage_id, keys::FONT_COVERAGE_COUNT, count as u64);
    ensure_link(face_id, rels::FONT_COVERS, coverage_id);
}

fn extract_name(face: &Face<'_>, primary: u16, fallback: u16) -> Option<String> {
    let mut best: Option<String> = None;
    for name in face.names() {
        if !name.is_unicode() {
            continue;
        }
        if name.name_id == primary {
            if let Some(s) = name_to_string(&name) {
                return Some(s);
            }
        }
        if best.is_none() && name.name_id == fallback {
            best = name_to_string(&name);
        }
    }
    best
}

fn name_to_string(name: &Name<'_>) -> Option<String> {
    if !name.is_unicode() {
        return None;
    }
    let bytes = name.name;
    if bytes.len() % 2 != 0 {
        return None;
    }
    let mut buf: Vec<u16> = Vec::with_capacity(bytes.len() / 2);
    for chunk in bytes.chunks_exact(2) {
        buf.push(u16::from_be_bytes([chunk[0], chunk[1]]));
    }
    String::from_utf16(&buf).ok()
}

fn coverage_ranges(face: &Face<'_>) -> (String, usize) {
    let mut codepoints: Vec<u32> = Vec::new();
    if let Some(cmap) = face.tables().cmap {
        for subtable in cmap.subtables {
            subtable.codepoints(|cp| {
                if subtable.glyph_index(cp).is_some() {
                    codepoints.push(cp);
                }
            });
        }
    }
    if codepoints.is_empty() {
        return ("0000-0000".into(), 0);
    }
    codepoints.sort_unstable();
    codepoints.dedup();

    let mut out = String::new();
    let mut ranges = 0usize;
    let mut start = codepoints[0];
    let mut prev = codepoints[0];
    for &cp in codepoints.iter().skip(1) {
        if cp == prev + 1 {
            prev = cp;
            continue;
        }
        append_range(&mut out, start, prev, ranges > 0);
        ranges += 1;
        start = cp;
        prev = cp;
    }
    append_range(&mut out, start, prev, ranges > 0);
    (out, codepoints.len())
}

fn append_range(out: &mut String, start: u32, end: u32, comma: bool) {
    use alloc::fmt::Write;
    if comma {
        let _ = out.write_char(',');
    }
    if start == end {
        let _ = write!(out, "{:04X}", start);
    } else {
        let _ = write!(out, "{:04X}-{:04X}", start, end);
    }
}

fn find_module_name(bytespace_id: ThingId) -> Option<String> {
    let mut modules = [ThingId::default(); 128];
    let count = find(kinds::BOOT_MODULE, &mut modules).ok()?;
    for id in modules.iter().take(count) {
        if let Ok(bs_val) = prop_get(*id, "bytespace") {
            if bs_val == bytespace_id.to_u64_lossy() {
                let mut buf = [0u8; 512];
                let len = stem::thing::sys::describe_thing(*id, &mut buf).ok()?;
                let desc = core::str::from_utf8(&buf[..len]).unwrap_or("");
                if let Some(pos) = desc.find("name: \"") {
                    let rest = &desc[pos + 7..];
                    if let Some(end) = rest.find('"') {
                        return Some(rest[..end].into());
                    }
                }
            }
        }
    }
    None
}

fn superfamily_from_family(name: &str) -> Option<String> {
    let mut parts = name.split_whitespace();
    let head = parts.next()?;
    if parts.next().is_some() {
        Some(head.into())
    } else {
        None
    }
}

fn face_key_hash(family_key: u64, weight: u16, width: u16, slope: u8, style: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    hash = fnv_mix(hash, family_key.to_le_bytes().as_ref());
    hash = fnv_mix(hash, weight.to_le_bytes().as_ref());
    hash = fnv_mix(hash, width.to_le_bytes().as_ref());
    hash = fnv_mix(hash, &[slope]);
    hash = fnv_mix(hash, style.as_bytes());
    hash
}

fn fnv_mix(mut hash: u64, bytes: &[u8]) -> u64 {
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn get_or_create_node_by_prop(kind: &str, key: &str, val: u64) -> ThingId {
    if let Some(id) = find_node_by_prop(kind, key, val) {
        return id;
    }
    create_node(kind).unwrap_or_else(|_| ThingId::default())
}

fn find_node_by_prop(kind: &str, key: &str, val: u64) -> Option<ThingId> {
    let mut nodes = [ThingId::default(); 128];
    let count = find(kind, &mut nodes).ok()?;
    for id in nodes.iter().take(count) {
        if let Ok(v) = prop_get(*id, key) {
            if v == val {
                return Some(*id);
            }
        }
    }
    None
}

fn ensure_link(src: ThingId, rel: &str, dst: ThingId) {
    let rel_id = intern(rel).unwrap_or(0) as u64;
    if rel_id == 0 {
        return;
    }
    let mut edges = [abi::types::Edge::default(); 64];
    if let Ok(count) = get_edges(src, &mut edges) {
        for edge in edges.iter().take(count) {
            if edge.predicate.to_u64_lossy() == rel_id && edge.to == dst {
                return;
            }
        }
    }
    let _ = link(src, rel, dst);
}

fn ensure_coverage_node(face_id: ThingId) -> ThingId {
    let rel_id = intern(rels::FONT_COVERS).unwrap_or(0) as u64;
    if rel_id != 0 {
        let mut edges = [abi::types::Edge::default(); 64];
        if let Ok(count) = get_edges(face_id, &mut edges) {
            for edge in edges.iter().take(count) {
                if edge.predicate.to_u64_lossy() == rel_id {
                    return edge.to;
                }
            }
        }
    }
    create_node(kinds::FONT_COVERAGE).unwrap_or_else(|_| ThingId::default())
}

fn set_prop_str(id: usize, key: &str, val: &str) -> Result<(), abi::errors::Errno> {
    // Intern key? Or use STR tag?
    // root_prop_set expects key_ptr (SymbolRefWire).
    // Value is u64. Strings must be interned or passed as blob?
    // Props are currently u64 values?
    // `RootOp::PropSet { value: u64 }`.
    // So strings MUST be interned or we need PropSetStr?
    // The kernel `root_handlers.rs` `prop_set` takes value: usize.
    // If I want to store a string, I must intern it first and store the SymbolId.
    
    let key_sym = SymbolRefWire {
        tag: SYMBOL_REF_TAG_STR,
        ptr_or_id: key.as_ptr() as u64,
        len: key.len() as u64,
    };
    
    let val_id = syscall::root_intern(val)?; // Intern value
    
    syscall::root_prop_set(id, &key_sym as *const _ as usize, val_id as usize)?;
    Ok(())
}

fn set_prop_u64(id: usize, key: &str, val: u64) -> Result<(), abi::errors::Errno> {
    let key_sym = SymbolRefWire {
        tag: SYMBOL_REF_TAG_STR,
        ptr_or_id: key.as_ptr() as u64,
        len: key.len() as u64,
    };
    syscall::root_prop_set(id, &key_sym as *const _ as usize, val as usize)?;
    Ok(())
}

fn set_prop_bytespace_str(id: ThingId, key: &str, val: &str) {
    if val.is_empty() {
        let _ = prop_set(id, key, 0);
        return;
    }
    let bs = match bytespace_create(val.len(), 0, 0) {
        Ok(id) => id,
        Err(_) => return,
    };
    let _ = bytespace_write(bs, 0, val.as_bytes());
    let _ = prop_set(id, key, bs.to_u64_lossy());
}
