#![no_std]
#![no_main]

//! # Asset Watcher Service (assetd / ingestd)
//!
//! This service makes assets continuous graph citizens by:
//!
//! 1. **Continuous Discovery**: Watches BOOT_MODULE nodes for new assets
//! 2. **Content Hashing**: Computes SHA-256 hash of asset content for deduplication
//! 3. **Graph Materialization**: Creates canonical Asset nodes in the graph
//! 4. **Change Detection**: Updates assets when content changes, increments generation
//! 5. **Deduplication**: Identifies identical assets by content hash
//!
//! ## Asset Node Properties
//!
//! Each asset in the graph has:
//! - `asset.name`: Stable logical name
//! - `asset.kind`: Type (font, svg, image, cursor, raw)
//! - `asset.hash`: SHA-256 content hash (first 8 bytes as u64)
//! - `asset.size`: Size in bytes
//! - `asset.bytespace`: Reference to asset content
//! - `asset.generation`: Increments on content change
//! - `asset.source`: Where asset came from (e.g., "boot")
//! - `asset.ready`: Boolean indicating asset is ready for use
//!
//! ## Philosophy
//!
//! Assets are not boot-shaped artifacts but living Things. When an asset:
//! - appears → graph node created
//! - changes → generation incremented, consumers notified
//! - disappears → node can be marked stale (future enhancement)
//!
//! The graph is the truth, continuously updated.

extern crate alloc;
mod sniff;

use abi::ids::HandleId;
use abi::schema::{keys, kinds};
use abi::types::{WatchMode, WatchSpec};
use alloc::vec::Vec;
use sha2::{Digest, Sha256};
use sniff::sniff;
use stem::thing::ThingId;
use stem::thing::sys::{
    bytespace_info, bytespace_map, bytespace_unmap, create_node, describe_thing, find, intern,
    prop_get, prop_set,
};
use stem::{info, syscall};
use ttf_parser::Face;

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("INGESTD: Starting unified content provider service...");

    // 1. Create ContentSource for Limine modules
    info!("INGESTD: Initializing Limine module content source...");
    let limine_source = initialize_limine_content_source();

    // 2. Initial scan of boot modules to seed canonical assets
    info!("INGESTD: Performing initial boot module scan...");
    scan_boot_modules();

    // 3. Seed system assets
    info!("INGESTD: Seeding system assets (reactive)...");
    seed_system_assets();

    // 4. Open watches for new boot modules and asset requests
    // This ensures continuous monitoring of asset sources
    let boot_module_pred = intern(kinds::BOOT_MODULE).unwrap_or(0);
    let asset_request_pred = intern(kinds::ASSET_REQUEST).unwrap_or(0);
    let proc_task_pred = intern(kinds::PROC_TASK).unwrap_or(0);

    let mut watch_ids = Vec::new();
    let mut watch_bufs = Vec::new();
    let mut watch_seqs = Vec::new();

    let preds = [boot_module_pred, asset_request_pred, proc_task_pred];
    for &pred in &preds {
        if pred == 0 {
            continue;
        }
        let filter = abi::root::RootWatchFilter::kind(pred as u32);
        let spec = WatchSpec {
            mode: WatchMode::StreamOnly as u32,
            start_seq: 0,
            filter_ptr: &filter as *const _ as u64,
            filter_len: core::mem::size_of::<abi::root::RootWatchFilter>() as u64,
            ..Default::default()
        };
        if let Ok(wid) = syscall::root_watch_open(&spec) {
            watch_ids.push(wid);
            watch_bufs.push([0u8; 4096]);
            watch_seqs.push(0u64);
        }
    }

    info!("INGESTD: Continuous asset watcher loop active. Watching for asset changes...");

    loop {
        let mut any_activity = false;
        for i in 0..watch_ids.len() {
            match syscall::root_watch_next(watch_ids[i], &mut watch_seqs[i], &mut watch_bufs[i]) {
                Ok(len) if len > 0 => {
                    any_activity = true;
                    process_events(&watch_bufs[i][..len], limine_source);
                }
                _ => {}
            }
        }

        if !any_activity {
            stem::sleep_ms(100);
        }
    }
}

/// Initialize the Limine module ContentSource node.
fn initialize_limine_content_source() -> ThingId {
    // Check if ContentSource already exists (buffer size: max 16 sources is sufficient for boot-time sources)
    let mut sources = [ThingId::default(); 16];
    if let Ok(count) = find(kinds::CONTENT_SOURCE, &mut sources) {
        for &source_id in &sources[..count] {
            let kind_sym = prop_get(source_id, keys::CONTENT_SOURCE_KIND).unwrap_or(0);
            if kind_sym != 0 {
                let mut buf = [0u8; 64];
                if let Ok(len) = stem::thing::sys::describe_symbol(kind_sym as u32, &mut buf) {
                    let kind_str = core::str::from_utf8(&buf[..len]).unwrap_or("");
                    if kind_str == "limine_module" {
                        info!("INGESTD: Found existing Limine ContentSource");
                        return source_id;
                    }
                }
            }
        }
    }

    // Create new ContentSource for Limine modules
    match create_node(kinds::CONTENT_SOURCE) {
        Ok(source_id) => {
            let kind_sym = intern("limine_module").unwrap_or(0);
            let name_sym = intern("boot").unwrap_or(0);
            let state_sym = intern("ready").unwrap_or(0);
            
            let _ = prop_set(source_id, keys::CONTENT_SOURCE_KIND, kind_sym as u64);
            let _ = prop_set(source_id, keys::CONTENT_SOURCE_NAME, name_sym as u64);
            let _ = prop_set(source_id, keys::CONTENT_SOURCE_PRIORITY, 100u64); // Default priority
            let _ = prop_set(source_id, keys::CONTENT_SOURCE_STATE, state_sym as u64);
            let _ = prop_set(source_id, keys::CONTENT_SOURCE_GEN, 1u64);
            
            info!("INGESTD: Created Limine ContentSource node");
            source_id
        }
        Err(_) => {
            info!("INGESTD: Failed to create ContentSource, using default");
            ThingId::default()
        }
    }
}

fn scan_boot_modules() {
    let mut modules = [ThingId::default(); 128];
    if let Ok(count) = find(kinds::BOOT_MODULE, &mut modules) {
        for &mod_id in &modules[..count] {
            ingest_boot_module(mod_id);
        }
    }
}

fn process_events(buf: &[u8], _source_id: ThingId) {
    // Note: _source_id reserved for future use when event filtering by source is needed
    let mut cursor = 0;
    while cursor < buf.len() {
        if let Ok((header, value)) = abi::watch::decode_event(&buf[cursor..]) {
            cursor += abi::watch::WATCH_EVENT_HEADER_LEN + value.len();

            if header.op == abi::watch::WatchOp::Upsert as u8 {
                let subject = header.subject;
                let kind = prop_get(subject, keys::KIND).unwrap_or(0);

                // BOOT_MODULE kind (we need to intern it to compare)
                // Actually, the watch filter already limited it to these.
                if let Ok(k_boot) = intern(kinds::BOOT_MODULE) {
                    if kind == k_boot as u64 {
                        ingest_boot_module(subject);
                        continue;
                    }
                }

                if let Ok(k_req) = intern(kinds::ASSET_REQUEST) {
                    if kind == k_req as u64 {
                        fulfill_request(subject);
                        continue;
                    }
                }

                if let Ok(k_task) = intern(kinds::PROC_TASK) {
                    if kind == k_task as u64 {
                        seed_app_assets(subject);
                        continue;
                    }
                }
            }
        } else {
            break;
        }
    }
}

fn ingest_boot_module(mod_id: ThingId) {
    let mut buf = [0u8; 512];
    let len = match describe_thing(mod_id, &mut buf) {
        Ok(l) => l,
        Err(_) => return,
    };
    let desc = core::str::from_utf8(&buf[..len]).unwrap_or("");
    let mod_name = if let Some(pos) = desc.find("name: \"") {
        let rest = &desc[pos + 7..];
        if let Some(end) = rest.find('"') {
            &rest[..end]
        } else {
            return;
        }
    } else {
        return;
    };

    let bs_id = match prop_get(mod_id, keys::BYTESPACE) {
        Ok(id) => ThingId::from_u64(id),
        Err(_) => return,
    };

    let size = match bytespace_info(bs_id) {
        Ok(size) => size,
        Err(_) => return,
    };

    let ptr = match bytespace_map(bs_id) {
        Ok(ptr) => ptr,
        Err(_) => return,
    };

    let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8, size) };
    
    // Compute content hash
    let mut hasher = Sha256::new();
    hasher.update(slice);
    let hash_bytes = hasher.finalize();
    // Convert first 8 bytes to u64 for storage
    let hash = u64::from_le_bytes([
        hash_bytes[0], hash_bytes[1], hash_bytes[2], hash_bytes[3],
        hash_bytes[4], hash_bytes[5], hash_bytes[6], hash_bytes[7],
    ]);
    
    let guess = sniff(slice);

    let kind = if let Some(ref g) = guess {
        if g.mime.starts_with("font/")
            || g.mime == "application/font-sfnt"
            || g.mime == "application/x-font-ttf"
        {
            "font"
        } else if g.mime == "image/svg+xml" || g.mime == "text/xml" && mod_name.ends_with(".svg") {
            "svg"
        } else if g.mime.starts_with("image/") {
            "image"
        } else {
            "raw"
        }
    } else {
        if mod_name.ends_with(".ttf") || mod_name.ends_with(".otf") {
            "font"
        } else if mod_name.ends_with(".svg") {
            "svg"
        } else if mod_name.ends_with(".bmp") || mod_name.ends_with(".png") {
            "image"
        } else {
            "raw"
        }
    };

    let asset_id = publish_asset(mod_name, kind, bs_id, "boot", size, hash);
    
    // Also create a File node in the content graph for unified access
    // Cache lookup: 16 sources is sufficient for boot-time sources (Limine, ISO, etc.)
    let mut sources = [ThingId::default(); 16];
    if let Ok(count) = find(kinds::CONTENT_SOURCE, &mut sources) {
        for &source_id in &sources[..count] {
            let kind_sym = prop_get(source_id, keys::CONTENT_SOURCE_KIND).unwrap_or(0);
            if kind_sym != 0 {
                let mut buf = [0u8; 64];
                if let Ok(len) = stem::thing::sys::describe_symbol(kind_sym as u32, &mut buf) {
                    let kind_str = core::str::from_utf8(&buf[..len]).unwrap_or("");
                    if kind_str == "limine_module" {
                        // Determine MIME type from kind and extension
                        let mime = if kind == "font" {
                            Some("application/font-sfnt")
                        } else if kind == "svg" {
                            Some("image/svg+xml")
                        } else if kind == "image" {
                            Some("image/bmp")
                        } else {
                            None
                        };
                        
                        if publish_content_file(source_id, mod_name, bs_id, size, hash, mime).is_none() {
                            info!("INGESTD: Failed to create File node for '{}'", mod_name);
                        }
                        break;
                    }
                }
            }
        }
    }
    
    if mod_name.contains("fonts") || mod_name.ends_with(".ttf") {
        info!(
            "INGESTD: Font debug - name='{}' kind='{}' guess={:?} first4={:02x?}",
            mod_name,
            kind,
            guess.as_ref().map(|g| g.mime),
            &slice[..4.min(slice.len())]
        );
    }
    info!("INGESTD: Published asset '{}' ({}, {} bytes, hash={:016x})", mod_name, kind, size, hash);

    // Metadata enrichment for fonts
    if kind == "font" && !slice.is_empty() {
        if let Ok(face) = Face::parse(slice, 0) {
            let family = face
                .names()
                .into_iter()
                .find(|n| n.name_id == ttf_parser::name_id::FAMILY && n.is_unicode())
                .and_then(|n| {
                    let mut buf = Vec::with_capacity(n.name.len() / 2);
                    for chunk in n.name.chunks_exact(2) {
                        buf.push(u16::from_be_bytes([chunk[0], chunk[1]]));
                    }
                    alloc::string::String::from_utf16(&buf).ok()
                });

            if let Some(name) = family {
                let _ = prop_set(asset_id, keys::FONT_NAME, intern(&name).unwrap_or(0) as u64);
            } else {
                let _ = prop_set(
                    asset_id,
                    keys::FONT_NAME,
                    intern(mod_name).unwrap_or(0) as u64,
                );
            }
        } else {
            let _ = prop_set(
                asset_id,
                keys::FONT_NAME,
                intern(mod_name).unwrap_or(0) as u64,
            );
        }
    }

    let _ = bytespace_unmap(bs_id, ptr);
}

fn seed_system_assets() {
    let root_ui = match find(kinds::UI_CROWN, &mut [ThingId::default(); 1]) {
        Ok(1) => {
            let mut buf = [ThingId::default(); 1];
            let _ = find(kinds::UI_CROWN, &mut buf);
            buf[0]
        }
        _ => {
            if let Ok(id) = create_node(kinds::UI_CROWN) {
                id
            } else {
                return;
            }
        }
    };

    let mut assets = [ThingId::default(); 256];
    if let Ok(count) = find(kinds::ASSET, &mut assets) {
        let mut leather_id = ThingId::default();
        let mut cursor_id = ThingId::default();

        for &asset_id in &assets[..count] {
            let name_sym = prop_get(asset_id, keys::ASSET_NAME).unwrap_or(0);
            let mut name_buf = [0u8; 256];
            if let Ok(len) = stem::thing::sys::describe_symbol(name_sym as u32, &mut name_buf) {
                let name = core::str::from_utf8(&name_buf[..len]).unwrap_or("");
                if name.contains("leather.bmp") {
                    leather_id = asset_id;
                }
                if name.contains("pointer.svg") || name.contains("arrow.svg") {
                    if cursor_id.to_u64_lossy() == 0 {
                        cursor_id = asset_id;
                    }
                }
            }
        }

        if leather_id.to_u64_lossy() != 0 {
            info!("INGESTD: Seeding desktop wallpaper 'leather.bmp'");
            let _ = prop_set(root_ui, "ui.wallpaper", leather_id.to_u64_lossy());
        }

        if cursor_id.to_u64_lossy() != 0 {
            info!("INGESTD: Seeding global cursor");
            let _ = prop_set(root_ui, "ui.cursor", cursor_id.to_u64_lossy());
        }

        // Initial scan for already running apps
        let mut apps = [ThingId::default(); 64];
        if let Ok(count) = find(kinds::PROC_TASK, &mut apps) {
            for &app_id in &apps[..count] {
                seed_app_assets(app_id);
            }
        }
    }
}

fn seed_app_assets(app_id: ThingId) {
    let mut linen_id = ThingId::default();
    let mut assets = [ThingId::default(); 256];
    if let Ok(count) = find(kinds::ASSET, &mut assets) {
        for &id in &assets[..count] {
            let name_sym = prop_get(id, keys::ASSET_NAME).unwrap_or(0);
            let mut name_buf = [0u8; 256];
            if let Ok(len) = stem::thing::sys::describe_symbol(name_sym as u32, &mut name_buf) {
                let name = core::str::from_utf8(&name_buf[..len]).unwrap_or("");
                if name.contains("linen.bmp") {
                    linen_id = id;
                    break;
                }
            }
        }
    }

    if linen_id.to_u64_lossy() == 0 {
        return;
    }

    let mut name_buf = [0u8; 128];
    if let Ok(len) = describe_thing(app_id, &mut name_buf) {
        let name = core::str::from_utf8(&name_buf[..len]).unwrap_or("");
        if name.contains("photosynthesis") {
            info!("INGESTD: Seeding photosynthesis wallpaper 'linen.bmp'");
            let _ = prop_set(app_id, "ui.wallpaper", linen_id.to_u64_lossy());
        }
    }
}

fn fulfill_request(req_id: ThingId) {
    // Basic request fulfillment based on name lookup in boot modules
    let name_sym = prop_get(req_id, keys::ASSET_NAME).unwrap_or(0);
    if name_sym == 0 {
        return;
    }

    // Try to find a boot module with this name suffix
    let mut modules = [ThingId::default(); 128];
    if let Ok(count) = find(kinds::BOOT_MODULE, &mut modules) {
        for &mod_id in &modules[..count] {
            let mut buf = [0u8; 512];
            if let Ok(len) = describe_thing(mod_id, &mut buf) {
                let desc = core::str::from_utf8(&buf[..len]).unwrap_or("");
                if let Some(pos) = desc.find("name: \"") {
                    let rest = &desc[pos + 7..];
                    if let Some(end) = rest.find('"') {
                        let mod_name = &rest[..end];
                        // If we had the string for name_sym, we could compare.
                        // For now, assume requests used interned symbols that match boot module names.
                        // This is a bit weak but fits the current graph model.
                        if let Ok(m_sym) = intern(mod_name) {
                            if m_sym as u64 == name_sym {
                                ingest_boot_module(mod_id);
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn publish_asset(name: &str, kind: &str, bs_id: ThingId, source: &str, size: usize, hash: u64) -> ThingId {
    let name_sym = intern(name).unwrap_or(0) as u64;
    let kind_sym = intern(kind).unwrap_or(0) as u64;
    let src_sym = intern(source).unwrap_or(0) as u64;

    if name_sym == 0 {
        return ThingId::default();
    }

    // Check if an asset with the same name already exists
    let mut existing_by_name = ThingId::default();
    let mut assets = [ThingId::default(); 512];
    if let Ok(count) = find(kinds::ASSET, &mut assets) {
        for &id in &assets[..count] {
            let existing_hash = prop_get(id, keys::ASSET_HASH).unwrap_or(0);
            let existing_name = prop_get(id, keys::ASSET_NAME).unwrap_or(0);
            
            // If same name and same hash, asset is unchanged
            if existing_name == name_sym && existing_hash == hash && existing_hash != 0 {
                info!("INGESTD: Asset '{}' unchanged (hash match)", name);
                return id;
            }
            
            if existing_name == name_sym {
                existing_by_name = id;
            }
        }
    }

    if existing_by_name.to_u64_lossy() != 0 {
        // Update existing asset with same name
        let old_bs = prop_get(existing_by_name, keys::ASSET_BYTESPACE).unwrap_or(0);
        let old_hash = prop_get(existing_by_name, keys::ASSET_HASH).unwrap_or(0);
        
        if old_bs != bs_id.to_u64_lossy() || old_hash != hash {
            let _ = prop_set(existing_by_name, keys::ASSET_BYTESPACE, bs_id.to_u64_lossy());
            let _ = prop_set(existing_by_name, keys::ASSET_HASH, hash);
            let _ = prop_set(existing_by_name, keys::ASSET_SIZE, size as u64);
            let _ = prop_set(existing_by_name, keys::ASSET_READY, 1);
            let generation = prop_get(existing_by_name, keys::ASSET_GENERATION).unwrap_or(0);
            let _ = prop_set(existing_by_name, keys::ASSET_GENERATION, generation + 1);
            info!("INGESTD: Updated asset '{}' (gen={}, hash={:016x})", name, generation + 1, hash);
        }
        existing_by_name
    } else {
        // Initial publication - create new asset node
        if let Ok(asset_id) = create_node(kinds::ASSET) {
            let _ = prop_set(asset_id, keys::ASSET_NAME, name_sym);
            let _ = prop_set(asset_id, keys::ASSET_KIND, kind_sym);
            let _ = prop_set(asset_id, keys::ASSET_SOURCE, src_sym);
            let _ = prop_set(asset_id, keys::ASSET_BYTESPACE, bs_id.to_u64_lossy());
            let _ = prop_set(asset_id, keys::ASSET_HASH, hash);
            let _ = prop_set(asset_id, keys::ASSET_SIZE, size as u64);
            let _ = prop_set(asset_id, keys::ASSET_GENERATION, 1);
            let _ = prop_set(asset_id, keys::ASSET_READY, 1);
            info!("INGESTD: Published new asset '{}' (hash={:016x})", name, hash);
            asset_id
        } else {
            ThingId::default()
        }
    }
}

/// Create or update a File node in the content graph.
/// This provides a unified file abstraction across all content sources.
fn publish_content_file(
    source_id: ThingId,
    name: &str,
    bs_id: ThingId,
    size: usize,
    hash: u64,
    mime: Option<&str>,
) -> Option<ThingId> {
    // Check if file already exists with same source and name
    // Buffer size: 512 is reasonable for boot-time assets; larger systems may need pagination
    let mut files = [ThingId::default(); 512];
    if let Ok(count) = find(kinds::CONTENT_FILE, &mut files) {
        let name_sym = intern(name).unwrap_or(0) as u64;
        for &file_id in &files[..count] {
            let existing_name = prop_get(file_id, keys::FILE_NAME).unwrap_or(0);
            let existing_source = prop_get(file_id, keys::FILE_SOURCE).unwrap_or(0);
            
            if existing_name == name_sym && existing_source == source_id.to_u64_lossy() {
                // Update existing file
                let old_hash = prop_get(file_id, keys::FILE_HASH).unwrap_or(0);
                if old_hash != hash {
                    let _ = prop_set(file_id, keys::FILE_BYTESPACE, bs_id.to_u64_lossy());
                    let _ = prop_set(file_id, keys::FILE_HASH, hash);
                    let _ = prop_set(file_id, keys::FILE_SIZE, size as u64);
                }
                return Some(file_id);
            }
        }
    }

    // Create new file node
    match create_node(kinds::CONTENT_FILE) {
        Ok(file_id) => {
            let name_sym = intern(name).unwrap_or(0) as u64;
            let _ = prop_set(file_id, keys::FILE_NAME, name_sym);
            let _ = prop_set(file_id, keys::FILE_SIZE, size as u64);
            let _ = prop_set(file_id, keys::FILE_HASH, hash);
            let _ = prop_set(file_id, keys::FILE_BYTESPACE, bs_id.to_u64_lossy());
            let _ = prop_set(file_id, keys::FILE_SOURCE, source_id.to_u64_lossy());
            
            if let Some(mime_str) = mime {
                if let Ok(mime_sym) = intern(mime_str) {
                    let _ = prop_set(file_id, keys::FILE_MIME, mime_sym as u64);
                }
            }
            
            info!("INGESTD: Created File node '{}' ({} bytes, hash={:016x})", name, size, hash);
            Some(file_id)
        }
        Err(_) => {
            info!("INGESTD: Failed to create File node for '{}'", name);
            None
        }
    }
}
