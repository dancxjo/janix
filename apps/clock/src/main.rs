#![no_std]
#![no_main]

extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use thing_std::*;
use models::*;
use thing_codec::GraphClient;
use abi::draw_cmd::DrawCmd;

#[no_mangle]
pub fn main() {
    thing_std::init(0);
    log_info("CLOCK: Starting with DrawList window...");

    let mut client = SyscallGraphClient;

    // 1. Create bytespace for DrawList commands
    let cmd_bytespace = thing_std::memory::bytespace_create(4096);
    log_info(&format!("CLOCK: Created bytespace id={}", cmd_bytespace.low()));

    // 2. Create Layout
    let kind_layout = symbol_intern("kind.Layout");
    let layout_id = client.create_thing(kind_layout).expect("create layout");
    let layout = Layout {
        kind: LayoutKind::Column,
        padding: 12,
        gap: 10,
        align: 0,
    };
    layout.write(&mut client, layout_id).expect("save layout");

    // 3. Create DrawList widget for clock display
    let kind_drawlist = symbol_intern("kind.DrawList");
    let drawlist_id = client.create_thing(kind_drawlist).expect("create drawlist");
    let drawlist = DrawList {
        width: 180,
        height: 40,
        bytespace: cmd_bytespace,
        cmd_count: 0,
    };
    drawlist.write(&mut client, drawlist_id).expect("save drawlist");

    // 4. Create Window
    let kind_window = symbol_intern("kind.Window");
    let window_id = client.create_thing(kind_window).expect("create window");
    
    let mut window = Window {
        title: symbol_intern("Clock"),
        x: 100,
        y: 400,
        width: 200,
        height: 100,
        style: WindowStyle {
            bg_rgba: 0xFF111111,
            radius: 8,
            shadow: 1, 
            elevation: 2,
        },
        content_root: layout_id,
    };
    window.write(&mut client, window_id).expect("save window");

    // 5. Links
    let rel_content = symbol_intern("content_root");
    relationship_create(rel_content, window_id, layout_id);

    let rel_child = symbol_intern("child");
    relationship_create(rel_child, layout_id, drawlist_id);

    // 6. Publish to graph.windows
    let graph_windows = thing_find("graph.windows").unwrap_or_else(|| {
        let pid = thing_create(symbol_intern("kind.Graph"), ThingId::from_parts(0,0));
        thing_register_name(pid, "graph.windows");
        pid
    });
    
    let rel_contains = symbol_intern("predicate.contains");
    relationship_create(rel_contains, graph_windows, window_id);

    log_info("CLOCK: Window Published!");

    // Map the bytespace so we can write commands
    let cmd_vaddr = 0x6000_0000u64;
    let mapped = thing_std::memory::space_map(cmd_bytespace, cmd_vaddr, 0, 4096);
    if mapped == 0 {
        log_info("CLOCK: Failed to map bytespace");
        return;
    }

    let mut time_thing = None;
    let mut last_time_str = String::new();

    loop {
        // --- Get Time ---
        let mut time_str = String::from("--:--:--");
        
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

        // Only update if time changed
        if time_str != last_time_str {
            last_time_str = time_str.clone();

            // Build draw commands
            let mut cmds: Vec<DrawCmd> = Vec::new();
            
            // Clear background (optional, window bg already covers this)
            cmds.push(DrawCmd::Clear { color: 0xFF111111 });
            
            // Draw time text in classic green digital clock color
            // Text command with inline bytes
            cmds.push(DrawCmd::Text { 
                x: 10, 
                y: 12, 
                color: 0xFF00FF00, 
                len: time_str.len() as u16 
            });
            
            cmds.push(DrawCmd::End);

            // Serialize commands to bytespace
            let cmd_buf = unsafe { 
                core::slice::from_raw_parts_mut(cmd_vaddr as *mut u8, 4096) 
            };
            let mut cursor = 0usize;
            let mut cmd_count = 0u32;

            for cmd in &cmds {
                match postcard::to_slice(cmd, &mut cmd_buf[cursor..]) {
                    Ok(used) => {
                        cursor += used.len();
                        cmd_count += 1;
                        
                        // For Text command, append the text bytes immediately after
                        if let DrawCmd::Text { len, .. } = cmd {
                            let text_bytes = time_str.as_bytes();
                            cmd_buf[cursor..cursor + *len as usize].copy_from_slice(&text_bytes[..*len as usize]);
                            cursor += *len as usize;
                        }
                    }
                    Err(_) => break,
                }
            }

            // Update DrawList cmd_count and touch window
            let updated_drawlist = DrawList {
                width: 180,
                height: 40,
                bytespace: cmd_bytespace,
                cmd_count,
            };
            updated_drawlist.write(&mut client, drawlist_id).expect("update drawlist");
            
            // Touch window to trigger repaint
            window.write(&mut client, window_id).expect("touch window");
        }
        
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
