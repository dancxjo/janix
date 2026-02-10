#![no_std]
#![no_main]

extern crate alloc;

use abi::schema::{keys, kinds, rels};
use abi::types::HandleId;
use core::time::Duration;
use stem::info;
use stem::petals::{
    add_rule, attach_window_stylesheet, create_stylesheet, set_node_classes, Declarations, Petals,
    SelectorKind, StyleSelector,
};
use stem::petals::graph::UiKey;
use stem::thing::sys::{
    bytespace_create, bytespace_read, bytespace_write, create_node, describe_thing, find, link,
    prop_get, prop_set,
};
use stem::thing::ThingId;
use time::OffsetDateTime;

fn search_for_icon(suffix: &str) -> Option<ThingId> {
    let mut modules = [ThingId::default(); 128];
    let count = find(kinds::BOOT_MODULE, &mut modules).unwrap_or(0);
    for i in 0..count {
        let mut buf = [0u8; 512];
        let len = describe_thing(modules[i], &mut buf).unwrap_or(0);
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

        // Check if name contains our suffix and ends with .svg
        if mod_name.ends_with(".svg") && mod_name.ends_with(suffix) {
            if let Ok(bs_id) = prop_get(modules[i], "bytespace") {
                return Some(ThingId::from_u64(bs_id));
            }
        }
    }
    None
}

fn find_locale_conf() -> Option<ThingId> {
    let mut modules = [ThingId::default(); 128];
    let count = find(kinds::BOOT_MODULE, &mut modules).unwrap_or(0);
    for i in 0..count {
        let mut buf = [0u8; 512];
        let len = describe_thing(modules[i], &mut buf).unwrap_or(0);
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

        if mod_name == "locale.conf" || mod_name.ends_with("/locale.conf") {
            return Some(modules[i]);
        }
    }
    None
}

fn read_conf_key(key_prefix: &str) -> Option<alloc::string::String> {
    if let Some(mod_id) = find_locale_conf() {
        if let Ok(bs_id) = prop_get(mod_id, "bytespace") {
            let bs_thing = ThingId::from_u64(bs_id);
            let len = stem::thing::sys::bytespace_info(bs_thing).unwrap_or(0);
            if len > 0 {
                let mut buf = alloc::vec![0u8; len];
                if bytespace_read(bs_thing, 0, &mut buf).is_ok() {
                    let content = alloc::string::String::from_utf8(buf).ok()?;
                    for line in content.lines() {
                        if let Some(val) = line.strip_prefix(key_prefix) {
                            return Some(val.trim().into());
                        }
                    }
                }
            }
        }
    }
    None
}

fn read_locale() -> Option<alloc::string::String> {
    read_conf_key("LOCALE=")
}

/// Read timezone offset in hours from locale.conf (e.g. TZ_OFFSET=-8 → -8).
fn read_timezone_offset_hours() -> i32 {
    read_conf_key("TZ_OFFSET=")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0)
}

/// Print a single tick with both wall clock (if anchored) and monotonic time.
fn print_tick(unix: u64, mono_ns: u64) {
    if unix == 0 {
        // info!("unix={} utc=<unanchored> mono_ns={}", unix, mono_ns);
        return;
    }

    let unix_i64 = match i64::try_from(unix) {
        Ok(val) => val,
        Err(_) => {
            info!("unix={} utc=<out_of_range> mono_ns={}", unix, mono_ns);
            return;
        }
    };

    let _dt = match OffsetDateTime::from_unix_timestamp(unix_i64) {
        Ok(val) => val,
        Err(_) => {
            info!("unix={} utc=<invalid> mono_ns={}", unix, mono_ns);
            return;
        }
    };

    // trace!(
    //     "unix={} utc={:04}-{:02}-{:02} {:02}:{:02}:{:02} mono_ns={}",
    //     unix,
    //     dt.year(),
    //     dt.month() as u8,
    //     dt.day(),
    //     dt.hour(),
    //     dt.minute(),
    //     dt.second(),
    //     mono_ns
    // );
}

fn set_string_prop(id: ThingId, key_name: &str, value: &str) {
    if value.is_empty() {
        prop_set(id, key_name, 0).ok();
        return;
    }
    let bs_id = bytespace_create(value.len(), 0, 0).expect("create bytespace");
    bytespace_write(bs_id, 0, value.as_bytes()).ok();
    prop_set(id, key_name, bs_id.to_u64_lossy()).ok();
}

/// Build the initial UI tree for the clock window, returning the text node id.
fn render_window_init(window_id: ThingId, time_text: &str) -> Option<ThingId> {
    if let Ok(stylesheet) = create_stylesheet() {
        let _ = attach_window_stylesheet(window_id, stylesheet);
        let _ = add_rule(
            stylesheet,
            &StyleSelector {
                kind: Some(SelectorKind::Text),
                class: None,
                key: None,
                focused: false,
            },
            &Declarations {
                color: Some(0xFF202020),
                font_size_px: Some(24),
                ..Default::default()
            },
        );
        let _ = add_rule(
            stylesheet,
            &StyleSelector {
                kind: None,
                class: Some("clock"),
                key: None,
                focused: false,
            },
            &Declarations {
                font_name: Some("DSEG7Classic-Regular"),
                font_size_px: Some(64),
                ..Default::default()
            },
        );
        let _ = add_rule(
            stylesheet,
            &StyleSelector {
                kind: None,
                class: None,
                key: Some("time"),
                focused: false,
            },
            &Declarations {
                color: Some(0xFFF04040),
                ..Default::default()
            },
        );
    }

    let mut ui = Petals::begin_window(window_id);
    let mut text_id = None;
    let root = ui.column(|ui| {
        let text = ui.text(time_text)?;
        let _ = ui.key_node(text, UiKey("time"));
        text_id = Some(text);
        Ok(())
    });
    if let Ok(root) = root {
        let _ = ui.set_gap(root, 8);
        let _ = ui.set_padding(root, 16);
    }
    if ui.finish().is_err() {
        return None;
    }
    if let Some(id) = text_id {
        let _ = set_node_classes(id, &["clock"]);
    }
    text_id
}

/// Update just the text content and bump scene gen (no new nodes created).
fn render_window_update(window_id: ThingId, text_node: ThingId, time_text: &str) {
    set_string_prop(text_node, keys::UI_TEXT, time_text);
    // Bump scene gen so blossom re-renders
    let gen = prop_get(window_id, keys::UI_SCENE_GEN).unwrap_or(0);
    prop_set(window_id, keys::UI_SCENE_GEN, gen.wrapping_add(1)).ok();
}

#[stem::main]
fn main() -> ! {
    let _cpu = stem::arch::whoami();
    // info!(
    //     "whoami: cs=0x{:x} ss=0x{:x} cpl={} rsp=0x{:x} rip=0x{:x} rflags=0x{:x}",
    //     cpu.cs, cpu.ss, cpu.cpl, cpu.rsp, cpu.rip, cpu.rflags
    // );

    // info!("starting clock publisher");

    // 1. Create Clock Thing (Publisher State)
    let clock_thing = create_node(kinds::CLOCK).expect("create clock node");
    // info!("Clock thing created: {}", clock_thing.to_u64_lossy());

    let mut window_id: Option<ThingId> = None;
    let mut text_node_id: Option<ThingId> = None;

    // 2. Setup UI
    // info!("Waiting for UI Root (Compositor)...");
    let mut ui_crown = ThingId::default();
    let mut i = 0;
    while i < 120 {
        // Wait up to 60 seconds for Bloom to start
        let mut ui_crowns = [ThingId::default(); 1];
        match stem::thing::sys::find(kinds::UI_CROWN, &mut ui_crowns) {
            Ok(count) if count > 0 => {
                ui_crown = ui_crowns[0];
                // info!(
                //     "Found UI Root: {} (attempt {})",
                //     ui_crown.to_u64_lossy(),
                //     i + 1
                // );
                break;
            }
            Ok(_) => {
                if i % 10 == 0 {
                    // info!(
                    //     "UI Root not found yet (attempt {}), still waiting...",
                    //     i + 1
                    // );
                }
            }
            Err(e) => {
                info!("Error finding UI Root: {:?}", e);
            }
        }
        stem::sleep(Duration::from_millis(500));
        i += 1;
    }

    if ui_crown.to_u64_lossy() == 0 {
        info!("ERROR: UI Root still not found after 60s, giving up on UI");
    }

    if ui_crown.to_u64_lossy() != 0 {
        // Create Window
        let win = create_node(kinds::UI_WINDOW).expect("create UI_WINDOW");
        link(win, rels::CHILD_OF, ui_crown).expect("link window");
        link(ui_crown, rels::HAS_CHILD, win).expect("link window has_child");
        window_id = Some(win);

        // Window Style: White Background
        prop_set(win, keys::UI_BG_COLOR, 0xFFFFFFFF).ok(); // White
        set_string_prop(win, keys::UI_TITLE, "Clock");

        // Window Layout: Bottom-right area (to avoid overlap with font_explorer)
        prop_set(win, keys::UI_WIDTH, 400).ok();
        prop_set(win, keys::UI_HEIGHT, 150).ok();
        prop_set(win, keys::UI_X, 0).ok(); // Base at 0 (inset will override)
        prop_set(win, keys::UI_Y, 0).ok(); // Base at 0 (inset will override)
        prop_set(win, keys::UI_INSET_RIGHT, 20).ok(); // 20px from right edge
        prop_set(win, keys::UI_INSET_BOTTOM, 30).ok(); // 30px from bottom edge

        // Window Icon
        if let Some(icon_id) = search_for_icon("office-calendar.svg") {
            // info!("Found clock icon: {}", icon_id.to_u64_lossy());
            prop_set(win, keys::UI_WINDOW_ICON, icon_id.to_u64_lossy()).ok();
        } else {
            // info!("Clock icon not found");
        }

        // Initial scene publish
        text_node_id = render_window_init(win, "--:--:--");
        if text_node_id.is_none() {
            info!("CLOCK: initial scene publish failed");
        }
    }

    // info!(
    //     "CLOCK: Entering main loop, publishing to thing_id={}",
    //     clock_thing.to_u64_lossy()
    // );

    let locale = read_locale().unwrap_or_else(|| "en_GB".into());
    let is_12h = locale == "en_US";
    let tz_offset_secs: i64 = read_timezone_offset_hours() as i64 * 3600;

    loop {
        // 1. Get precise system time
        let now_ns = stem::time::now_unix_nanos();
        let unix = now_ns / 1_000_000_000;
        let mono_ns = stem::monotonic_ns();

        print_tick(unix, mono_ns);

        // 2. Publish State to Graph and always invalidate UI, even before wall-clock anchor.
        // Apply timezone offset to convert UTC → local time.
        let local_unix = (unix as i64).wrapping_add(tz_offset_secs);
        let time_str = match OffsetDateTime::from_unix_timestamp(local_unix).ok() {
            Some(dt) => {
                if is_12h {
                    let (h, am) = if dt.hour() == 0 {
                        (12, true)
                    } else if dt.hour() == 12 {
                        (12, false)
                    } else if dt.hour() > 12 {
                        (dt.hour() - 12, false)
                    } else {
                        (dt.hour(), true)
                    };
                    alloc::format!(
                        "{:02}:{:02}:{:02} {}",
                        h,
                        dt.minute(),
                        dt.second(),
                        if am { "AM" } else { "PM" }
                    )
                } else {
                    alloc::format!("{:02}:{:02}:{:02}", dt.hour(), dt.minute(), dt.second())
                }
            }
            None => {
                // Monotonic fallback keeps scene_gen moving until RTC/NTP anchoring is ready.
                let secs = (mono_ns / 1_000_000_000) % 86_400;
                let h = secs / 3600;
                let m = (secs % 3600) / 60;
                let s = secs % 60;
                alloc::format!("{:02}:{:02}:{:02}", h, m, s)
            }
        };

        if let (Some(win), Some(text_node)) = (window_id, text_node_id) {
            render_window_update(win, text_node, &time_str);
        }

        // Update clock:tick regardless of wall-clock anchoring.
        let _ = prop_set(clock_thing, keys::CLOCK_TICK, mono_ns);

        // 3. Sleep until the next whole second boundary
        let now_ns_recheck = stem::time::now_unix_nanos();
        let nanos_into_second = now_ns_recheck % 1_000_000_000;
        let sleep_nanos = 1_000_000_000 - nanos_into_second;

        // Add a tiny buffer (1ms) if we are extremely close to the boundary to avoid double-ticks
        // or busy-looping if the timer granularity is coarse.
        let sleep_nanos = if sleep_nanos < 1_000_000 {
            sleep_nanos + 1_000_000_000
        } else {
            sleep_nanos
        };

        stem::sleep(Duration::from_nanos(sleep_nanos));
    }
}
