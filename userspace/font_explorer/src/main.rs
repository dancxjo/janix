#![no_std]
#![no_main]

//! # Font Explorer
//!
//! A system utility for browsing available fonts in Thing-OS.
//!
//! ## Architecture Compliance
//!
//! This application follows the UI Intent Contract (see `docs/UI_INTENT_CONTRACT.md`):
//! - Uses graph-native Petals APIs to declare UI intent
//! - Publishes intent directly to graph nodes/edges
//! - Does NOT perform layout calculations or paint operations
//! - Does NOT import from blossom::layout or blossom::emit_paint
//!
//! The app only:
//! 1. Queries fonts from the graph
//! 2. Builds a declarative UI tree using Petals
//! 3. Publishes the tree to the graph
//! 4. Lets Blossom handle all layout and painting

extern crate alloc;

use abi::root::RootWatchFilter;
use abi::schema::{keys, kinds, rels};
use abi::types::HandleId;
use abi::types::{WatchMode, WatchSpec};
use abi::watch;
use alloc::string::String;
use alloc::vec::Vec;
use stem::petals::Petals;
use stem::thing::ThingId;
use stem::thing::sys::{
    bytespace_create, bytespace_info, bytespace_read, bytespace_write, create_node, find, intern,
    link, prop_get, prop_set,
};

fn set_string_prop(id: ThingId, key: &str, value: &str) {
    if value.is_empty() {
        let _ = prop_set(id, key, 0);
        return;
    }
    let bs_id = match bytespace_create(value.len(), 0, 0) {
        Ok(id) => id,
        Err(_) => return,
    };
    let _ = bytespace_write(bs_id, 0, value.as_bytes());
    let _ = prop_set(id, key, bs_id.to_u64_lossy());
}

fn read_string_prop(id: ThingId, key: &str) -> Option<String> {
    let val = prop_get(id, key).ok()?;
    if val == 0 {
        return None;
    }
    read_bytespace_string(ThingId::from_u64(val))
}

fn read_bytespace_string(id: ThingId) -> Option<String> {
    let size = bytespace_info(id).ok()?;
    if size == 0 {
        return Some(String::new());
    }
    let mut buf = alloc::vec![0u8; size];
    let len = bytespace_read(id, 0, &mut buf).ok()?;
    let text = core::str::from_utf8(&buf[..len]).unwrap_or("");
    Some(text.into())
}

#[derive(Clone, Debug)]
struct FontEntry {
    id: ThingId,
    name: String,
}

fn list_fonts(kind_id: u32) -> Vec<FontEntry> {
    let mut entries = Vec::new();
    let mut families = [ThingId::default(); 256];
    let count = find(kind_id, &mut families).unwrap_or(0);
    for id in families.iter().take(count) {
        if let Some(name) = read_string_prop(*id, keys::FONT_NAME) {
            entries.push(FontEntry { id: *id, name });
        } else {
            // Fallback for nodes without names yet, so counting still works
            entries.push(FontEntry {
                id: *id,
                name: alloc::format!("Unnamed ({:?})", id),
            });
        }
    }
    entries.sort_by(|a, b| a.name.cmp(&b.name));
    entries.dedup_by(|a, b| a.name == b.name);
    entries
}

#[stem::main]
fn main() -> ! {
    // Initialize i18n system
    stem::i18n::init();
    
    let mut ui_crown = ThingId::default();
    let mut attempts = 0;
    while attempts < 120 {
        let mut roots = [ThingId::default(); 1];
        if find(kinds::UI_CROWN, &mut roots)
            .ok()
            .filter(|c| *c > 0)
            .is_some()
        {
            ui_crown = roots[0];
            break;
        }
        stem::sleep_ms(500);
        attempts += 1;
    }

    if ui_crown.to_u64_lossy() == 0 {
        loop {
            stem::sleep_ms(10000);
        }
    }

    // Localized text constants
    use stem::i18n::LocalizedText;
    const EXPLORER: LocalizedText = stem::t!("ui.fonts.explorer", "Font Explorer");
    const COUNT_LABEL: LocalizedText = stem::t!("ui.fonts.count", "Fonts");
    const SAMPLE: LocalizedText = stem::t!("ui.fonts.sample", "Sphinx of black quartz, judge my vow. 0123456789 😀 ܐܠܦ ܒܝܬÕøØœ");

    let win = create_node(kinds::UI_WINDOW).expect("create UI_WINDOW");
    link(win, rels::CHILD_OF, ui_crown).expect("link window");
    link(ui_crown, rels::HAS_CHILD, win).expect("link window has_child");
    prop_set(win, keys::UI_BG_COLOR, 0xFFF5F5F0).ok();
    prop_set(win, keys::UI_WIDTH, 900).ok();
    prop_set(win, keys::UI_HEIGHT, 520).ok();
    prop_set(win, keys::UI_X, 50).ok();
    prop_set(win, keys::UI_Y, 50).ok();

    let font_kind = intern(kinds::FONT_FAMILY).unwrap_or(0);
    let mut fonts: Vec<FontEntry> = Vec::new();
    let mut last_font_count = usize::MAX;
    let mut last_discovery = 0u64;
    let mut last_tick = stem::monotonic_ns();
    let mut font_watch = None;
    let mut last_i18n_gen = 0u64;

    if font_kind != 0 {
        let filter = RootWatchFilter::kind(font_kind);
        let spec = WatchSpec {
            mode: WatchMode::StreamOnly as u32,
            filter_ptr: &filter as *const _ as u64,
            filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
            ..Default::default()
        };
        font_watch = stem::syscall::root_watch_open(&spec).ok();
    }

    loop {
        let now = stem::monotonic_ns();

        let mut dirty = last_font_count == usize::MAX;
        
        // Check if locale changed
        let current_i18n_gen = stem::i18n::generation();
        if current_i18n_gen != last_i18n_gen {
            dirty = true;
            last_i18n_gen = current_i18n_gen;
        }
        
        if let Some(watch_id) = font_watch {
            let mut seq = 0u64;
            let mut buf = [0u8; 1024];
            let mut drained = 0u32;
            while let Ok(len) = stem::syscall::root_watch_next(watch_id, &mut seq, &mut buf) {
                if len > 0 {
                    drained += 1;
                } else {
                    break;
                }
            }
            if drained > 0 {
                dirty = true;
            }
        }

        // Fallback periodic discovery in case of watch overflow/miss.
        if now.saturating_sub(last_discovery) > 2_000_000_000 {
            dirty = true;
        }

        if dirty {
            last_discovery = now;
            fonts = list_fonts(font_kind);
        }

        if fonts.len() != last_font_count {
            let win_title = alloc::format!("{}", EXPLORER.get());
            set_string_prop(win, keys::UI_TITLE, &win_title);
            last_font_count = fonts.len();
        }

        last_tick = now;

        let header_text = alloc::format!("{} ({})", COUNT_LABEL.get(), fonts.len());
        let mut ui = Petals::begin_window(win);
        let root = ui.column(|ui| {
            let header = ui.text(&header_text)?;
            let _ = ui.set_font_name(header, "NotoSans-Regular");
            let _ = ui.set_font_size(header, 20);
            let _ = ui.set_color(header, 0xFF000000);

            for entry in fonts.iter().take(32) {
                let sample = alloc::format!("{}  [{}]", SAMPLE.get(), entry.name);
                let text = ui.text(&sample)?;
                let _ = ui.set_font_name(text, "NotoSans-Regular");
                let _ = ui.set_font_size(text, 14);
                let _ = ui.set_color(text, 0xFF222222);
            }
            Ok(())
        });
        if let Ok(root) = root {
            let _ = ui.set_gap(root, 8);
            let _ = ui.set_padding(root, 16);
            let _ = ui.finish();
        }

        stem::sleep_ms(33); // ~30 FPS
    }
}
