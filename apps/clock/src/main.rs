#![no_std]
#![no_main]

extern crate alloc;
use alloc::format;
use alloc::string::String;
use thing_std::*;
use models::*;
use thing_codec::GraphClient;

#[no_mangle]
pub fn main() {
    thing_std::init(0);
    log_info("CLOCK: Starting with window...");

    let mut client = SyscallGraphClient;

    // 1. Create Layout first
    let kind_layout = symbol_intern("kind.Layout");
    let layout_id = client.create_thing(kind_layout).expect("create layout");
    let layout = Layout {
        kind: LayoutKind::Column,
        padding: 12,
        gap: 10,
        align: 0,
    };
    layout.write(&mut client, layout_id).expect("save layout");

    // 2. Create Label for time display with classic green digital clock color
    let kind_label = symbol_intern("kind.Label");
    let label_id = client.create_thing(kind_label).expect("create label");
    // Classic green digital clock color: 0xFF00FF00 (RGBA: green with full opacity)
    let mut label = Label {
        text: symbol_intern("Loading..."),
        style: TextStyle { size: 24, color_rgba: 0xFF00FF00 },
    };
    label.write(&mut client, label_id).expect("save label");

    // 3. Create Window
    let kind_window = symbol_intern("kind.Window");
    let window_id = client.create_thing(kind_window).expect("create window");
    
    let window = Window {
        title: symbol_intern("Clock"),
        x: 100,
        y: 400, // Below hello_window
        width: 200,
        height: 100,
        style: WindowStyle {
            bg_rgba: 0xFF111111, // Darker background for contrast with green
            radius: 8,
            shadow: 1, 
            elevation: 2,
        },
        content_root: layout_id,
    };
    window.write(&mut client, window_id).expect("save window");

    // 4. Links
    let rel_content = symbol_intern("content_root");
    relationship_create(rel_content, window_id, layout_id);

    let rel_child = symbol_intern("child");
    relationship_create(rel_child, layout_id, label_id);

    // 5. Publish to graph.windows
    let graph_windows = thing_find("graph.windows").unwrap_or_else(|| {
        let pid = thing_create(symbol_intern("kind.Graph"), ThingId::from_parts(0,0));
        thing_register_name(pid, "graph.windows");
        pid
    });
    
    let rel_contains = symbol_intern("predicate.contains");
    relationship_create(rel_contains, graph_windows, window_id);

    // 6. Frame Pulse Setup
    let kind_frame = symbol_intern("kind.Frame");
    let frame_id = client.create_thing(kind_frame).expect("create frame");
    let rel_has_frame = symbol_intern("has_frame");
    relationship_create(rel_has_frame, window_id, frame_id);

    log_info("CLOCK: Window Published!");

    let mut time_thing = None;
    let mut frame_seq = 0;

    loop {
        // --- Get Time ---
        let mut time_str = String::from("Waiting...");
        
        if time_thing.is_none() {
            time_thing = thing_std::graph::thing_find("system.time");
        }

        if let Some(tid) = time_thing {
             if let Some((body, _)) = thing_std::graph::thing_get_body(tid) {
                 if let Ok(sys_clock) = SystemClock::decode_full(&body) {
                     let now_mono = monotonic_now();
                     let elapsed_ns = now_mono - sys_clock.last_set_mono_ns;
                     let current_ns = (sys_clock.unix_epoch_ns as u64) + elapsed_ns;
                     let current_secs = current_ns / 1_000_000_000;
                     time_str = format_time(current_secs);
                 }
             }
        }

        // --- Update Label every second ---
        label.text = symbol_intern(&time_str);
        label.write(&mut client, label_id).expect("update label");

        // Emit Frame to trigger repaint
        frame_seq += 1;
        let frame = Frame { window: window_id, seq: frame_seq };
        frame.write(&mut client, frame_id).expect("pulse frame");
        
        // Update every second
        sleep_ms(1000);
    }
}

fn format_time(unix_secs: u64) -> String {
    let secs_of_day = unix_secs % 86400;
    let hours = secs_of_day / 3600;
    let minutes = (secs_of_day % 3600) / 60;
    let seconds = secs_of_day % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
