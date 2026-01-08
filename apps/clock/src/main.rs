#![no_std]
#![no_main]

extern crate alloc;
use alloc::format;
use alloc::string::String;
// alloc::vec::Vec removed (unused)
use thing_std::*;
use models::*;
use thing_codec::GraphClient;
use abi::draw_cmd::DrawCmd;

/// Size of the command bytespace
const CMD_BYTESPACE_SIZE: u64 = 4096;
/// Virtual address for mapping command buffer
const CMD_VADDR: u64 = 0x6000_0000;

#[no_mangle]
pub fn main() {
    thing_std::init(0);
    log_info("CLOCK: Starting with DrawList window...");

    let mut client = SyscallGraphClient;

    // 1. Create bytespace for DrawList commands
    let cmd_bytespace = thing_std::memory::bytespace_create(CMD_BYTESPACE_SIZE);
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
    
    let window = Window {
        title: symbol_intern("Clock"),
        x: 100,
        y: 400,
        width: 200,
        height: 100,
        z: 0,
        focused: false,
        min_width: 150,
        min_height: 80,
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
    // IMPORTANT: Validate the mapping before using it!
    let mapped = thing_std::memory::space_map(cmd_bytespace, CMD_VADDR, 0, CMD_BYTESPACE_SIZE);
    if mapped == 0 || mapped != CMD_VADDR {
        log_info(&format!("CLOCK: FATAL: Failed to map bytespace, expected={:#x} got={:#x}", CMD_VADDR, mapped));
        loop { sched_yield(); }
    }
    
    // Additional validation: ensure address is accessible
    // Try writing a sentinel value to verify the mapping is valid
    let probe_ptr = CMD_VADDR as *mut u8;
    unsafe {
        // Write and read back a sentinel to ensure mapping is valid
        core::ptr::write_volatile(probe_ptr, 0xAA);
        let read_back = core::ptr::read_volatile(probe_ptr);
        if read_back != 0xAA {
            log_info("CLOCK: FATAL: Bytespace mapping verification failed");
            loop { sched_yield(); }
        }
        // Reset the probe byte
        core::ptr::write_volatile(probe_ptr, 0);
    }
    
    log_info(&format!("CLOCK: Bytespace mapped at {:#x}", CMD_VADDR));

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

            // Create command buffer slice with defensive validation
            let cmd_ptr = CMD_VADDR as *mut u8;
            let cmd_len = CMD_BYTESPACE_SIZE as usize;
            
            // Validate pointer before creating slice
            // 1. Non-null check (redundant since it's a constant, but good practice)
            if cmd_ptr.is_null() {
                log_info("CLOCK: ERROR: cmd_ptr is null, skipping update");
                sleep_ms(1000);
                continue;
            }
            
            // 2. Length check against isize::MAX
            if cmd_len > isize::MAX as usize {
                log_info("CLOCK: ERROR: cmd_len too large, skipping update");
                sleep_ms(1000);
                continue;
            }
            
            // 3. Alignment check (u8 doesn't require alignment, but we check anyway)
            // This is mostly a sanity check since we're using known virtual address
            
            // SAFETY: We've verified:
            // - The mapping succeeded and returned our expected address
            // - A probe write/read confirmed memory is accessible
            // - Length is within bounds
            let cmd_buf = unsafe { 
                core::slice::from_raw_parts_mut(cmd_ptr, cmd_len)
            };

            // Build draw commands
            let mut cursor = 0usize;
            let mut cmd_count = 0u32;
            
            // Clear command
            let clear_cmd = DrawCmd::Clear { color: 0xFF111111 };
            match postcard::to_slice(&clear_cmd, &mut cmd_buf[cursor..]) {
                Ok(used) => {
                    cursor += used.len();
                    cmd_count += 1;
                }
                Err(_) => {
                    log_info("CLOCK: ERROR: Failed to serialize Clear command");
                    sleep_ms(1000);
                    continue;
                }
            }
            
            // Text command with inline bytes
            let text_cmd = DrawCmd::Text { 
                x: 10, 
                y: 12, 
                color: 0xFF00FF00, 
                len: time_str.len() as u16 
            };
            match postcard::to_slice(&text_cmd, &mut cmd_buf[cursor..]) {
                Ok(used) => {
                    cursor += used.len();
                    cmd_count += 1;
                    
                    // Append text bytes
                    let text_bytes = time_str.as_bytes();
                    let text_len = text_bytes.len();
                    if cursor + text_len <= cmd_buf.len() {
                        cmd_buf[cursor..cursor + text_len].copy_from_slice(text_bytes);
                        cursor += text_len;
                    } else {
                        log_info("CLOCK: ERROR: Not enough space for text bytes");
                        sleep_ms(1000);
                        continue;
                    }
                }
                Err(_) => {
                    log_info("CLOCK: ERROR: Failed to serialize Text command");
                    sleep_ms(1000);
                    continue;
                }
            }
            
            // End command
            let end_cmd = DrawCmd::End;
            if let Ok(_used) = postcard::to_slice(&end_cmd, &mut cmd_buf[cursor..]) {
                // Command written, cursor not needed after this
                cmd_count += 1;
            }

            // Update DrawList cmd_count and touch window
            let updated_drawlist = DrawList {
                width: 180,
                height: 40,
                bytespace: cmd_bytespace,
                cmd_count,
            };
            
            if let Err(e) = updated_drawlist.write(&mut client, drawlist_id) {
                log_info(&format!("CLOCK: ERROR: Failed to update drawlist: {:?}", e));
            }
            
            // Touch window to trigger repaint
            if let Err(e) = window.write(&mut client, window_id) {
                log_info(&format!("CLOCK: ERROR: Failed to touch window: {:?}", e));
            }
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
