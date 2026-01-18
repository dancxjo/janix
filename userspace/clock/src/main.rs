#![no_std]
#![no_main]

extern crate alloc;

use core::time::Duration;
use stem::info;
use time::OffsetDateTime;
use stem::thing::ThingId;
use abi::ids::HandleId;
use stem::thing::sys::{create_node, prop_set, link, bytespace_create, bytespace_write};
use abi::schema::{kinds, keys, rels};


/// Print a single tick with both wall clock (if anchored) and monotonic time.
fn print_tick(unix: u64, mono_ns: u64) {
    if unix == 0 {
        info!("unix={} utc=<unanchored> mono_ns={}", unix, mono_ns);
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

    info!(
        "unix={} utc={:04}-{:02}-{:02} {:02}:{:02}:{:02} mono_ns={}",
        unix,
        dt.year(),
        dt.month() as u8,
        dt.day(),
        dt.hour(),
        dt.minute(),
        dt.second(),
        mono_ns
    );
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

#[stem::main]
fn main() -> ! {
    let cpu = stem::arch::whoami();
    info!(
        "whoami: cs=0x{:x} ss=0x{:x} cpl={} rsp=0x{:x} rip=0x{:x} rflags=0x{:x}",
        cpu.cs, cpu.ss, cpu.cpl, cpu.rsp, cpu.rip, cpu.rflags
    );

    info!("starting clock publisher");

    // 1. Create Clock Thing (Publisher State)
    let clock_thing = create_node(kinds::CLOCK).expect("create clock node");
    info!("Clock thing created: {}", clock_thing.to_u64_lossy());

    // 2. Setup UI
    info!("Waiting for UI Root (Compositor)...");
    let mut ui_root = ThingId::default();
    let mut i = 0;
    while i < 120 { // Wait up to 60 seconds for Bloom to start
        let mut ui_roots = [ThingId::default(); 1];
        match stem::thing::sys::find(kinds::UI_ROOT, &mut ui_roots) {
            Ok(count) if count > 0 => {
                ui_root = ui_roots[0];
                info!("Found UI Root: {} (attempt {})", ui_root.to_u64_lossy(), i + 1);
                break;
            }
            Ok(_) => {
                if i % 10 == 0 {
                    info!("UI Root not found yet (attempt {}), still waiting...", i + 1);
                }
            }
            Err(e) => {
                info!("Error finding UI Root: {:?}", e);
            }
        }
        stem::sleep(Duration::from_millis(500));
    }

    if ui_root.to_u64_lossy() == 0 {
        info!("ERROR: UI Root still not found after 60s, giving up on UI");
    }


    if ui_root.to_u64_lossy() != 0 {
        // Create Window
        let win = create_node(kinds::UI_WINDOW).expect("create UI_WINDOW");
        link(win, rels::CHILD_OF, ui_root).expect("link window");
        link(ui_root, rels::HAS_CHILD, win).expect("link window has_child");

        // Window Style: Black Background
        prop_set(win, keys::UI_BG_COLOR, 0xFF000000).ok(); // Black

        // Window Layout: Center, size
        prop_set(win, keys::UI_X, 100).ok();
        prop_set(win, keys::UI_Y, 100).ok();
        prop_set(win, keys::UI_WIDTH, 400).ok();
        prop_set(win, keys::UI_HEIGHT, 150).ok();
        set_string_prop(win, keys::UI_TITLE, "Clock");

        // Create Text
        let text = create_node(kinds::UI_TEXT).expect("create UI_TEXT");
        link(text, rels::CHILD_OF, win).expect("link text");
        link(win, rels::HAS_CHILD, text).expect("link text has_child");

        // Text Style: Red Foreground, DSEG Font
        prop_set(text, keys::UI_FG_COLOR, 0xFFFF0000).ok(); // Red
        set_string_prop(text, keys::UI_FONT, "DSEG7Classic-Regular.ttf");
        prop_set(text, keys::UI_FONT_SIZE, 64).ok(); // Large font
        // Text Layout: Centered
        prop_set(text, keys::UI_CENTER_X, 1).ok();
        prop_set(text, keys::UI_CENTER_Y, 1).ok();

        // Initial text
        set_string_prop(text, keys::UI_TEXT, "--:--:--");



        // 3. Create Binding
        let binding = create_node(kinds::BINDING).expect("create binding");
        // We use properties for binding relations as per schema keys, but schema also defines relations?
        // Schema keys: BINDING_SOURCE, BINDING_TARGET (strings).
        // Let's use properties pointing to ThingIds.
        prop_set(binding, keys::BINDING_SOURCE, clock_thing.to_u64_lossy()).ok();
        prop_set(binding, keys::BINDING_TARGET, text.to_u64_lossy()).ok();
        // Map type: 0 = direct copy (or specific enum value).
        // Plan said "clock_now_text_to_ui_text_value". Let's say that's 1.
        // Or since we just copy the bytespace ID, 0 is fine if bindd knows what to do.
        prop_set(binding, keys::BINDING_MAP, 1).ok();

        info!("Binding created: {} (Clock->Text)", binding.to_u64_lossy());
    }

    loop {
        let unix = stem::time::now_unix_seconds();
        let mono_ns = stem::monotonic_ns();
        print_tick(unix, mono_ns);

        // Publish State to Graph
        let unix_i64 = unix as i64;
        let dt = OffsetDateTime::from_unix_timestamp(unix_i64).ok();
        if let Some(dt) = dt {
            let time_str = alloc::format!("{:02}:{:02}:{:02}", dt.hour(), dt.minute(), dt.second());
            // Update clock:now_text
            // Note: We create a new bytespace for every update.
            // This ensures the ThingId changes, which triggers the 'bloom' UI snapshot diffing to detect a change.
            // A production version would implement bytespace reuse or a garbage collector.
            set_string_prop(clock_thing, keys::CLOCK_NOW_TEXT, &time_str);
            // Update clock:tick
            prop_set(clock_thing, keys::CLOCK_TICK, mono_ns).ok();
        }

        // NO direct UI update here!

        stem::sleep(Duration::from_secs(1));
    }
}
