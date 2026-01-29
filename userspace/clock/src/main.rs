#![no_std]
#![no_main]

extern crate alloc;

use abi::schema::{keys, kinds, rels};
use abi::types::HandleId;
use core::time::Duration;
use stem::info;
use stem::petals::{AlignItems, Color, Flex, FontKey, JustifyContent, Scene, Styled, Text, Window};
use stem::thing::sys::{
    bytespace_create, bytespace_write, create_node, describe_thing, find, link, prop_get, prop_set,
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

    let dt = match OffsetDateTime::from_unix_timestamp(unix_i64) {
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

fn build_scene(window_id: ThingId, time_text: &str) -> Scene {
    Scene::new().window(
        Window::new(window_id)
            .title("Clock")
            .initial_size(400, 150)
            .root(
                Flex::column()
                    .gap(8)
                    .padding(16)
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::Center)
                    .push(
                        Text::new(time_text)
                            .font(FontKey::new("DSEG7Classic-Regular").size(64))
                            .color(Color::rgb(240, 64, 64)),
                    ),
            ),
    )
}

#[stem::main]
fn main() -> ! {
    let cpu = stem::arch::whoami();
    // info!(
    //     "whoami: cs=0x{:x} ss=0x{:x} cpl={} rsp=0x{:x} rip=0x{:x} rflags=0x{:x}",
    //     cpu.cs, cpu.ss, cpu.cpl, cpu.rsp, cpu.rip, cpu.rflags
    // );

    // info!("starting clock publisher");

    // 1. Create Clock Thing (Publisher State)
    let clock_thing = create_node(kinds::CLOCK).expect("create clock node");
    // info!("Clock thing created: {}", clock_thing.to_u64_lossy());

    let mut window_id: Option<ThingId> = None;

    // 2. Setup UI
    // info!("Waiting for UI Root (Compositor)...");
    let mut ui_root = ThingId::default();
    let mut i = 0;
    while i < 120 {
        // Wait up to 60 seconds for Bloom to start
        let mut ui_roots = [ThingId::default(); 1];
        match stem::thing::sys::find(kinds::UI_ROOT, &mut ui_roots) {
            Ok(count) if count > 0 => {
                ui_root = ui_roots[0];
                // info!(
                //     "Found UI Root: {} (attempt {})",
                //     ui_root.to_u64_lossy(),
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

    if ui_root.to_u64_lossy() == 0 {
        info!("ERROR: UI Root still not found after 60s, giving up on UI");
    }

    if ui_root.to_u64_lossy() != 0 {
        // Create Window
        let win = create_node(kinds::UI_WINDOW).expect("create UI_WINDOW");
        link(win, rels::CHILD_OF, ui_root).expect("link window");
        link(ui_root, rels::HAS_CHILD, win).expect("link window has_child");
        window_id = Some(win);

        // Window Style: Black Background (explicit override)
        prop_set(win, keys::UI_BG_COLOR, 0xFF000000).ok(); // Black
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
        let scene = build_scene(win, "--:--:--");
        if let Err(e) = stem::petals::publish_window(&scene) {
            info!("CLOCK: initial scene publish failed: {:?}", e);
        }
    }

    // info!(
    //     "CLOCK: Entering main loop, publishing to thing_id={}",
    //     clock_thing.to_u64_lossy()
    // );

    loop {
        // 1. Get precise system time
        let now_ns = stem::time::now_unix_nanos();
        let unix = now_ns / 1_000_000_000;
        let mono_ns = stem::monotonic_ns();

        if now_ns > 0 {
            print_tick(unix, mono_ns);

            // 2. Publish State to Graph
            let dt = OffsetDateTime::from_unix_timestamp(unix as i64).ok();
            if let Some(dt) = dt {
                let time_str =
                    alloc::format!("{:02}:{:02}:{:02}", dt.hour(), dt.minute(), dt.second());
                if let Some(win) = window_id {
                    let scene = build_scene(win, &time_str);
                    if let Err(e) = stem::petals::publish_window(&scene) {
                        info!("CLOCK: scene publish failed: {:?}", e);
                    }
                }
                // Update clock:tick
                if prop_set(clock_thing, keys::CLOCK_TICK, mono_ns).is_ok() {
                    // info!(
                    //     "CLOCK PUBLISH: thing={} now_text='{}' tick={}",
                    //     clock_thing.to_u64_lossy(),
                    //     time_str,
                    //     mono_ns
                    // );
                }
            }
        }

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
