use crate::asset::AssetBank;
use crate::font_graph;
use abi::root::RootWatchFilter;
use abi::schema::{keys, kinds, rels};
use abi::types::{WatchMode, WatchSpec};
use alloc::string::String;
use alloc::string::ToString;
use stem::thing::sys::{describe_thing, find, prop_get, read, stat};
use stem::thing::{HandleId, ThingId};
use stem::{debug, info, syscall, warn};

pub static ASSETS: AssetBank = AssetBank::new();

// Alias for main.rs compatibility - spawns all loaders then becomes font loader
pub extern "C" fn asset_watcher_entry() -> ! {
    debug!("[bloom] asset_watcher_entry: spawning sub-loaders");
    if let Err(e) = stem::thread::spawn(wallpaper_loader_entry) {
        warn!("[bloom] failed to spawn wallpaper loader: {:?}", e);
    }
    if let Err(e) = stem::thread::spawn(cursor_loader_entry) {
        warn!("[bloom] failed to spawn cursor loader: {:?}", e);
    }
    if let Err(e) = stem::thread::spawn(icon_loader_entry) {
        warn!("[bloom] failed to spawn icon loader: {:?}", e);
    }

    font_loader_entry()
}

pub extern "C" fn wallpaper_loader_entry() -> ! {
    stem::sleep_ms(100);
    debug!("[bloom] wallpaper loader: queueing early wallpapers from boot modules");
    ASSETS.enqueue_wallpaper_load("clouds.bmp");
    ASSETS.enqueue_wallpaper_load("flower.bmp");
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
            let fd = match prop_get(modules[i], "bytespace") {
                Ok(id) => id as u32,
                Err(_) => continue,
            };
            let size = match stat(fd) {
                Ok((_, s, _)) => s,
                Err(_) => continue,
            };
            ASSETS.enqueue_font_load(fd, size as usize, mod_name);
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

    loop {
        stem::syscall::sleep_ms(10000);
    }
}

pub extern "C" fn cursor_loader_entry() -> ! {
    stem::sleep_ms(100);
    debug!("[bloom] cursor loader: loading default cursor");
    ASSETS.enqueue_cursor_load("/assets/cursors/future/default.svg");
    loop {
        stem::syscall::sleep_ms(10000);
    }
}

pub extern "C" fn icon_loader_entry() -> ! {
    stem::sleep_ms(100);
    debug!("[bloom] icon loader started");

    // Explicitly load known icons
    let icons = [
        "bran.bran.svg",
        "dev.bus.platform.svg",
        "dev.cpu.svg",
        "dev.host.svg",
        "dev.input.svg",
        "dev.network.svg",
        "dev.output.svg",
        "dev.storage.svg",
        "kind.bytespace.svg",
        "mem.heap.svg",
        "mem.page.svg",
        "mem.stack.svg",
        "meta.alert.svg",
        "meta.annotation.svg",
        "meta.graph.svg",
        "meta.metric.svg",
        "meta.namespace.svg",
        "meta.trace.svg",
        "meta.version.svg",
        "proc.job.svg",
        "proc.kernel.svg",
        "proc.task.svg",
        "proc.thread.svg",
        "svc.cambium.svg",
        "svc.init.svg",
        "svc.photosynthesis.svg",
        "svc.scheduler.svg",
        "svc.service.svg",
        "svc.shutdown.svg",
        "svc.worker.svg",
        "time.clock.svg",
        "time.deadline.svg",
        "time.interval.svg",
        "time.timer.svg",
        "ui.bloom.svg",
        "ui.cursor.svg",
        "ui.root.svg",
        "ui.scene.svg",
        "ui.theme.svg",
        "ui.widget.svg",
    ];

    for filename in icons.iter() {
        let name = &filename[..filename.len() - 4];
        let path = alloc::format!("/assets/icons/thingos/{}", filename);
        if let Some(cmds) = AssetBank::load_icon_immediate_from_path(&path) {
            ASSETS.publish_icon(name, cmds);
        }
    }

    loop {
        stem::syscall::sleep_ms(10000);
    }
}
