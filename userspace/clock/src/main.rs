#![no_std]
#![no_main]

extern crate alloc;

use core::time::Duration;
use stem::info;
use time::OffsetDateTime;

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

#[stem::main]
fn main() -> ! {
    let cpu = stem::arch::whoami();
    info!(
        "whoami: cs=0x{:x} ss=0x{:x} cpl={} rsp=0x{:x} rip=0x{:x} rflags=0x{:x}",
        cpu.cs, cpu.ss, cpu.cpl, cpu.rsp, cpu.rip, cpu.rflags
    );

    info!("starting");

    // UI Setup
    use stem::thing::ThingId;
    use stem::ui::UiBuilder;
    
    // Find UI Root
    let mut ui_roots = [ThingId(0); 1];
    let ui_root = if stem::thing::sys::find(abi::schema::kinds::UI_ROOT, &mut ui_roots).unwrap_or(0) > 0 {
        ui_roots[0]
    } else {
        info!("WARN: UI Root not found, retrying in 1s...");
        stem::sleep(Duration::from_secs(1));
        // Simple retry once
        if stem::thing::sys::find(abi::schema::kinds::UI_ROOT, &mut ui_roots).unwrap_or(0) > 0 {
            ui_roots[0]
        } else {
            info!("ERROR: UI Root still not found, giving up on UI");
            ThingId(0)
        }
    };

    let mut text_node = None;
    if ui_root.0 != 0 {
        let win = UiBuilder::create_window(ui_root, "Clock");
        UiBuilder::set_pos(win, 100, 100);
        UiBuilder::set_size(win, 300, 100);
        UiBuilder::set_color(win, 0xCC333333); // Dark gray

        let text = UiBuilder::create_text(win, "Initializing...");
        UiBuilder::set_pos(text, 110, 140);
        UiBuilder::set_color(text, 0xFFFFFFFF);
        text_node = Some(text);
    }

    loop {
        let unix = stem::time::now_unix_seconds();
        let mono_ns = stem::monotonic_ns();
        print_tick(unix, mono_ns);

        if let Some(txt) = text_node {
            let unix_i64 = unix as i64;
            let dt = OffsetDateTime::from_unix_timestamp(unix_i64).ok();
            if let Some(dt) = dt {
                let time_str = alloc::format!("{:02}:{:02}:{:02}", dt.hour(), dt.minute(), dt.second());
                UiBuilder::set_text(txt, &time_str);
            }
        }

        stem::sleep(Duration::from_secs(1));
    }
}
