use crate::asset::AssetBank;
use crate::font_graph;
use abi::schema::{keys, kinds, rels};
use alloc::string::String;
use alloc::string::ToString;
use stem::syscall::vfs::{vfs_open, vfs_readdir, vfs_stat, vfs_close};
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
    debug!("[bloom] font loader: scanning /boot for fonts");
    
    let mut names = [0u8; 4096];
    let boot_fd = vfs_open("/boot", abi::syscall::vfs_flags::O_RDONLY).expect("failed to open /boot");
    if let Ok(count) = vfs_readdir(boot_fd, &mut names) {
        let mut offset = 0;
        for _ in 0..count {
            if offset >= names.len() { break; }
            let name_len = names[offset] as usize;
            if name_len == 0 { break; }
            let name = core::str::from_utf8(&names[offset+1..offset+1+name_len]).unwrap_or("");
            offset += 1 + name_len;

            if name.ends_with(".ttf") || name.ends_with(".TTF") || name.ends_with(".otf") || name.ends_with(".OTF") {
                let path = alloc::format!("/boot/{}", name);
                if let Ok(fd) = vfs_open(&path, 0) {
                    if let Ok((_kind, size, _id)) = vfs_stat(fd) {
                        ASSETS.enqueue_font_load(fd, size as usize, name);
                        // We keep the FD open as it's passed to enqueue_font_load
                    } else {
                        let _ = vfs_close(fd);
                    }
                }
            }
        }
    }
    let _ = vfs_close(boot_fd);

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
