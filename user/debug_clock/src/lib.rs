#![no_std]

use thing_os::prelude::*;
use alloc::format;
use abi::{graph_kinds, PropValue};
use alloc::string::String;

// Import necessary functions
use thing_os::{user_create_thing, add_link};
use thing_os::userland::ui;
extern crate alloc;

pub fn run<S: Sys>(sys: &mut S) -> ! {
    println(sys, "debug_clock: starting...");

    let Some(clock) = SystemClock::discover(sys) else {
        println(sys, "debug_clock: TimeSource not found");
        sys.exit_thread();
    };

    // Create UI window
    let mut title_buf = String::new();
    title_buf.push_str("ThingOS Clock");
    
    println(sys, "debug_clock: creating window...");
    let window = match ui::create_window(sys, &title_buf, 0) { // 0 = Default/Sky mode
        Some(w) => {
            let msg = format!("debug_clock: window created id={:?}", w.id);
            let leaked = Box::leak(msg.into_boxed_str());
            println(sys, leaked);
            w
        },
        None => {
            println(sys, "debug_clock: failed to create window!");
            loop { sys.syscall(abi::KernelRequest::ExitThread); }
        }
    };
    println(sys, "debug_clock: window created");

    // Create Root Widget (container)
    let mut root_props = alloc::vec::Vec::new();
    root_props.push((graph_kinds::PROP_WINDOW_ID, PropValue::U64(window.id.0)));
    root_props.push((graph_kinds::PROP_FLEX_DIRECTION, PropValue::Str(String::from("column"))));
    root_props.push((graph_kinds::PROP_JUSTIFY_CONTENT, PropValue::Str(String::from("center"))));
    root_props.push((graph_kinds::PROP_ALIGN_ITEMS, PropValue::Str(String::from("center"))));
    root_props.push((graph_kinds::PROP_WIDTH, PropValue::I64(100))); // Fill window
    
    let root_props_ref = Box::leak(root_props.into_boxed_slice());
    let root_id = user_create_thing(sys, graph_kinds::KIND_WIDGET, root_props_ref)
        .expect("failed to create root widget");
    
    // Link Window -> Root Widget
    add_link(sys, window.id, graph_kinds::LINK_WIDGET_CHILD, root_id);

    // Create Text Widget
    let text_id = user_create_thing(sys, graph_kinds::KIND_WIDGET, &[])
        .expect("failed to create text widget");

    // Link Root -> Text
    add_link(sys, root_id, graph_kinds::LINK_WIDGET_CHILD, text_id);


    let tick_hz = clock.tick_hz(sys).max(1);
    let start_ticks = clock.uptime_ticks(sys);
    let base_seconds = clock.now(sys).0;

    let mut last_logged = None;
    loop {
        let ticks = clock.uptime_ticks(sys);
        let elapsed_secs = ticks.saturating_sub(start_ticks) / tick_hz as u64;
        
        // Update every second (or frame if we want smooth, but seconds for now)
        if last_logged.map(|prev| prev != elapsed_secs).unwrap_or(true) {
            let (hour, minute, second) = seconds_to_hms(base_seconds + elapsed_secs as i64);
            
            let time_str = format!("{:02}:{:02}:{:02}", hour, minute, second);
            let _ = update_props(sys, text_id, &[
                (graph_kinds::PROP_TEXT, PropValue::Str(time_str)),
                (graph_kinds::PROP_FONT_SIZE, PropValue::I64(24)),
            ]);

            last_logged = Some(elapsed_secs);
        }
        // The instruction's snippet was malformed and introduced undefined variables.
        // Assuming the intent was to add a println based on time,
        // and to fix the `update_props` line which was also malformed in the snippet.
        // The original `update_props` is restored, and a simple `println` is added.
        // If `uptime_ms` was intended, it would need to be defined.
        // For now, a simple tick print is added.
        println(sys, "debug_clock: tick");
        sys.sleep_for_ns(100_000_000); // 10Hz poll
    }
}

fn seconds_to_hms(seconds: i64) -> (u32, u32, u32) {
    let secs_in_day = 86_400_i64;
    let mut secs = seconds.rem_euclid(secs_in_day);
    if secs < 0 {
        secs += secs_in_day;
    }
    let hour = (secs / 3_600) as u32;
    secs %= 3_600;
    let minute = (secs / 60) as u32;
    let second = (secs % 60) as u32;
    (hour, minute, second)
}

#[cfg(test)]
mod tests {
    use super::seconds_to_hms;

    #[test]
    fn formats_small_time_values() {
        assert_eq!(seconds_to_hms(7 * 3_600 + 5 * 60 + 9), (7, 5, 9));
    }

    #[test]
    fn wraps_at_midnight() {
        assert_eq!(seconds_to_hms(86_400), (0, 0, 0));
        assert_eq!(seconds_to_hms(-1), (23, 59, 59));
    }
}
