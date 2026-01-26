use stem::thing::{ThingId, HandleId};
use stem::thing::sys::{bytespace_info, describe_thing, find, prop_get, bytespace_read};
use abi::schema::{kinds, keys, rels};
use abi::root::RootWatchFilter;
use abi::types::{WatchMode, WatchSpec};
use stem::{root_watch, syscall, info};
use crate::font_graph;
use crate::asset::AssetBank;

pub static ASSETS: AssetBank = AssetBank::new();

pub extern "C" fn wallpaper_loader_entry() -> ! {
    stem::sleep_ms(200);
    let candidates = [
        "/assets/wallpapers/clouds.bmp",
        "wallpapers/clouds.bmp",
        "clouds.bmp",
    ];
    for path in candidates.iter() {
        if ASSETS.probe_asset_exists(path) {
            ASSETS.enqueue_wallpaper_load(path);
            break;
        }
    }
    loop {
        stem::syscall::sleep_ms(10000);
    }
}

pub extern "C" fn font_loader_entry() -> ! {
    let mut modules = [ThingId::default(); 64];
    let count = find(kinds::BOOT_MODULE, &mut modules).unwrap_or(0);
    for i in 0..count {
        let mut buf = [0u8; 512];
        let len = match describe_thing(modules[i], &mut buf) {
            Ok(l) => l,
            Err(_) => continue,
        };
        let desc = core::str::from_utf8(&buf[..len]).unwrap_or("");
        let mod_name = if let Some(pos) = desc.find("name: \"") {
            let rest = &desc[pos + 7..];
            if let Some(end) = rest.find('"') {
                &rest[..end]
            } else {
                continue;
            }
        } else {
            continue;
        };
        if mod_name.ends_with(".ttf")
            || mod_name.ends_with(".TTF")
            || mod_name.ends_with(".otf")
            || mod_name.ends_with(".OTF")
        {
            let bs_id = match prop_get(modules[i], "bytespace") {
                Ok(id) => ThingId::from_u64(id),
                Err(_) => continue,
            };
            let size = match bytespace_info(bs_id) {
                Ok(s) => s,
                Err(_) => continue,
            };
            ASSETS.enqueue_font_load(bs_id, size as usize, mod_name);
        }
    }

    let k_file = stem::thing::sys::intern(kinds::FONT_FILE).unwrap_or(0);
    let k_family = stem::thing::sys::intern(kinds::FONT_FAMILY).unwrap_or(0);
    let k_face = stem::thing::sys::intern(kinds::FONT_FACE).unwrap_or(0);
    let k_super = stem::thing::sys::intern(kinds::FONT_SUPERFAMILY).unwrap_or(0);
    let p_bs = stem::thing::sys::intern(keys::FONT_BYTESPACE).unwrap_or(0);
    let p_sz = stem::thing::sys::intern(keys::FONT_SIZE_BYTES).unwrap_or(0);
    let p_name = stem::thing::sys::intern(keys::FONT_NAME).unwrap_or(0);
    let dirty_keys = [
        p_name,
        stem::thing::sys::intern(keys::FONT_STYLE).unwrap_or(0),
        stem::thing::sys::intern(keys::FONT_WEIGHT).unwrap_or(0),
        stem::thing::sys::intern(keys::FONT_WIDTH).unwrap_or(0),
        stem::thing::sys::intern(keys::FONT_SLOPE).unwrap_or(0),
        stem::thing::sys::intern(keys::FONT_COVERAGE_RANGES).unwrap_or(0),
        stem::thing::sys::intern(rels::FONT_CONTAINS).unwrap_or(0),
        stem::thing::sys::intern(rels::FONT_COVERS).unwrap_or(0),
    ];
    let watch_kinds = [k_file, k_family, k_face, k_super];
    let mut watch_ids: alloc::vec::Vec<usize> = alloc::vec::Vec::new();
    let mut watch_bufs: alloc::vec::Vec<[u8; 4096]> = alloc::vec::Vec::new();
    let mut watch_seq: alloc::vec::Vec<u64> = alloc::vec::Vec::new();

    let process_payload = |buf: &[u8]| {
        let mut cursor = 0usize;
        while cursor < buf.len() {
            if let Ok((header, value)) = abi::watch::decode_event(&buf[cursor..]) {
                cursor += abi::watch::WATCH_EVENT_HEADER_LEN + value.len();
                let mut is_dirty = false;
                if header.predicate == abi::watch::WATCH_PRED_KIND && value.len() == 4 {
                    let kid = u32::from_le_bytes(value.try_into().unwrap());
                    if kid == k_file || kid == k_family || kid == k_face || kid == k_super {
                        is_dirty = true;
                    }
                } else {
                    let pred = header.predicate.to_u32_lossy();
                    for &k in &dirty_keys {
                        if pred == k {
                            is_dirty = true;
                            break;
                        }
                    }
                    if pred == p_bs || pred == p_sz || pred == p_name {
                        is_dirty = true;
                    }
                }
                if is_dirty {
                    font_graph::mark_dirty();
                    let node_id = header.subject;
                    if let (Ok(bs), Ok(sz)) = (
                        prop_get(node_id, keys::FONT_BYTESPACE),
                        prop_get(node_id, keys::FONT_SIZE_BYTES),
                    ) {
                        let name = prop_get(node_id, keys::FONT_NAME)
                            .ok()
                            .and_then(|id| {
                                let bs_id = ThingId::from_u64(id);
                                let size = bytespace_info(bs_id).ok()?;
                                let mut b = alloc::vec![0u8; size];
                                let l = bytespace_read(bs_id, 0, &mut b).ok()?;
                                Some(alloc::string::String::from(
                                    core::str::from_utf8(&b[..l]).ok()?,
                                ))
                            })
                            .unwrap_or_else(|| "font.bin".into());
                        ASSETS.enqueue_font_load(ThingId::from_u64(bs), sz as usize, &name);
                    }
                }
            } else {
                break;
            }
        }
    };

    for kid in watch_kinds.into_iter().filter(|k| *k != 0) {
        let filter = RootWatchFilter::kind(kid);
        let spec = WatchSpec {
            mode: WatchMode::StreamOnly as u32,
            start_seq: 0,
            filter_ptr: &filter as *const _ as u64,
            filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
            ..Default::default()
        };
        if let Ok(wid) = syscall::root_watch_open(&spec) {
            watch_ids.push(wid);
            watch_bufs.push([0u8; 4096]);
            watch_seq.push(0);
            let _ = root_watch::watch_drain(wid, watch_bufs.last_mut().unwrap(), |_, bytes| {
                process_payload(bytes)
            });
        }
    }
    loop {
        let mut any_activity = false;
        for idx in 0..watch_ids.len() {
            match syscall::root_watch_next(
                watch_ids[idx],
                &mut watch_seq[idx],
                &mut watch_bufs[idx],
            ) {
                Ok(len) if len > 0 => {
                    any_activity = true;
                    process_payload(&watch_bufs[idx][..len]);
                }
                Err(abi::errors::Errno::EAGAIN) => {}
                Err(abi::errors::Errno::EOVERFLOW) => {
                    font_graph::mark_dirty();
                }
                _ => {}
            }
        }
        if !any_activity {
            stem::sleep_ms(100);
        }
    }
}

pub extern "C" fn cursor_loader_entry() -> ! {
    stem::sleep_ms(300);
    ASSETS.enqueue_cursor_load("/assets/cursors/future/default.svg");
    loop {
        stem::syscall::sleep_ms(10000);
    }
}
