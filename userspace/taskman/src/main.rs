#![no_std]
#![no_main]

extern crate alloc;

use abi::query::QueryRow;
use abi::schema::{keys, kinds, rels};
use abi::types::HandleId;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::time::Duration;
use stem::info;
use stem::petals::Petals;
use stem::thing::query::{query_nodes_by_kind, RestrictedQuery};
use stem::thing::sys::{
    bytespace_create, bytespace_read, bytespace_write, create_node, describe_thing, find, link,
    prop_get, prop_set,
};
use stem::thing::ThingId;

/// State color constants (ARGB)
const COLOR_RUNNING: u32 = 0xFF4CAF50; // Green
const COLOR_SLEEPING: u32 = 0xFF2196F3; // Blue
const COLOR_BLOCKED: u32 = 0xFFFFC107; // Amber
const COLOR_DEAD: u32 = 0xFF9E9E9E; // Gray
const COLOR_UNKNOWN: u32 = 0xFF808080; // Medium gray

fn set_string_prop(id: ThingId, key_name: &str, value: &str) {
    if value.is_empty() {
        prop_set(id, key_name, 0).ok();
        return;
    }
    let bs_id = bytespace_create(value.len(), 0, 0).expect("create bytespace");
    bytespace_write(bs_id, 0, value.as_bytes()).ok();
    prop_set(id, key_name, bs_id.to_u64_lossy()).ok();
}

/// Read a bytespace-backed string property.
fn read_string_prop(id: ThingId, key: &str) -> Option<String> {
    let bs = prop_get(id, key).ok()?;
    if bs == 0 {
        return None;
    }
    let bs_id = ThingId::from_u64(bs);
    let len = stem::thing::sys::bytespace_info(bs_id).unwrap_or(0);
    if len == 0 {
        return None;
    }
    let mut buf = alloc::vec![0u8; len];
    if bytespace_read(bs_id, 0, &mut buf).is_ok() {
        core::str::from_utf8(&buf).ok().map(|s| String::from(s))
    } else {
        None
    }
}

/// Resolve an interned symbol ID to its string representation.
fn resolve_symbol(sym_id: u64) -> Option<String> {
    if sym_id == 0 {
        return None;
    }
    let mut buf = [0u8; 128];
    if let Ok(len) = stem::thing::sys::describe_symbol(sym_id as u32, &mut buf) {
        if len > 0 {
            return core::str::from_utf8(&buf[..len]).ok().map(String::from);
        }
    }
    None
}

struct TaskInfo {
    name: String,
    state: u64,
}

/// Map task state values to icon colors.
fn state_color(state: u64) -> u32 {
    match state {
        1 => COLOR_RUNNING,  // Running
        2 => COLOR_SLEEPING, // Sleeping / Ready
        3 => COLOR_BLOCKED,  // Blocked
        0 => COLOR_DEAD,     // Dead / Not started
        _ => COLOR_UNKNOWN,
    }
}

/// Map task state values to human-readable labels.
fn state_label(state: u64) -> &'static str {
    match state {
        1 => "run",
        2 => "sleep",
        3 => "block",
        0 => "dead",
        _ => "?",
    }
}

/// Query the graph for tasks by traversing from the scheduler anchor.
/// Pattern: find svc.Scheduler → traverse edges → collect reachable task nodes.
fn collect_tasks() -> Vec<TaskInfo> {
    let mut tasks = Vec::new();
    let mut seen = alloc::collections::BTreeSet::new();

    // Seed: find scheduler, kernel, and CPU nodes (same seeds as anther task_monitor)
    let mut seeds = Vec::new();

    let seed_kinds = [kinds::SVC_SCHEDULER, kinds::PROC_KERNEL, kinds::DEV_CPU];
    for &kind in &seed_kinds {
        let mut ids = [ThingId::default(); 16];
        if let Ok(count) = find(kind, &mut ids) {
            for i in 0..count {
                seeds.push(ids[i]);
            }
        }
    }

    // BFS: traverse edges from seeds, collecting task-like nodes
    let mut queue = seeds;
    let mut processed = 0;

    while processed < queue.len() && processed < 256 {
        let id = queue[processed];
        processed += 1;

        if !seen.insert(id) {
            continue;
        }

        // Check if this node is a task by examining its description
        let mut buf = [0u8; 128];
        if let Ok(len) = describe_thing(id, &mut buf) {
            let desc = core::str::from_utf8(&buf[..len]).unwrap_or("");

            // Skip non-task nodes (bytespaces, mem ranges, log entries, UI nodes)
            if desc.contains(":mem.Range")
                || desc.contains(":Bytespace")
                || desc.contains(":log.Entry")
                || desc.contains(":ui.")
                || desc.contains(":boot.")
                || desc.contains(":dev.")
                || desc.contains(":svc.")
                || desc.contains(":fw.")
            {
                // Still traverse edges from svc/dev nodes to find tasks
                if desc.contains(":svc.") || desc.contains(":dev.") {
                    let mut q_buf = [QueryRow::default(); 64];
                    let mut q = RestrictedQuery::new(&mut q_buf);
                    if let Ok(edge_count) = q.get_edges(id, None, 64) {
                        for i in 0..edge_count {
                            let target = ThingId::from_u64(q.buf[i].val_dst);
                            if !seen.contains(&target) && queue.len() < 512 {
                                queue.push(target);
                            }
                        }
                    }
                }
                continue;
            }

            // If it looks like a proc.Task or proc.Kernel, collect it
            if desc.contains(":proc.") {
                let name = extract_task_name(id, desc);
                let state = prop_get(id, keys::PROC_STATE).unwrap_or(0);
                tasks.push(TaskInfo { name, state });
            }
        }

        // Traverse edges to find more nodes
        let mut q_buf = [QueryRow::default(); 64];
        let mut q = RestrictedQuery::new(&mut q_buf);
        if let Ok(edge_count) = q.get_edges(id, None, 64) {
            for i in 0..edge_count {
                let target = ThingId::from_u64(q.buf[i].val_dst);
                if !seen.contains(&target) && queue.len() < 512 {
                    queue.push(target);
                }
            }
        }
    }

    tasks
}

/// Extract a human-readable name for a task node.
fn extract_task_name(id: ThingId, desc: &str) -> String {
    // Try NAME property (interned symbol)
    if let Ok(name_sym) = prop_get(id, keys::NAME) {
        if let Some(name) = resolve_symbol(name_sym) {
            return name;
        }
    }

    // Try proc.name bytespace string
    if let Some(name) = read_string_prop(id, keys::PROC_NAME) {
        return name;
    }

    // Extract from description like "(var_ID:proc.Task { ... })"
    if let Some(start) = desc.find('(') {
        if let Some(end) = desc.find(" {") {
            let identity = &desc[start + 1..end];
            if let Some(colon) = identity.find(':') {
                return String::from(&identity[..colon]);
            }
        }
    }

    format!("task_{:X}", id.to_u64_lossy())
}

/// Build or rebuild the UI tree with the current task list.
fn render_task_list(window_id: ThingId, tasks: &[TaskInfo]) {
    let mut ui = Petals::begin_window(window_id);

    let root = ui.column(|ui| {
        if tasks.is_empty() {
            ui.text("No tasks found")?;
        } else {
            for task in tasks {
                let color = state_color(task.state);
                let label = format!(
                    "{} [{}]",
                    task.name,
                    state_label(task.state)
                );
                ui.list_item(&label, color)?;
            }
        }
        Ok(())
    });

    if let Ok(root) = root {
        let _ = ui.set_gap(root, 2);
        let _ = ui.set_padding(root, 8);
    }

    if ui.finish().is_err() {
        info!("TASKMAN: Failed to build UI tree");
    }
}

#[stem::main]
fn main() -> ! {
    info!("TASKMAN: starting task manager");

    // 1. Wait for UI Crown (Bloom compositor)
    let mut ui_crown = ThingId::default();
    for attempt in 0..120 {
        let mut crowns = [ThingId::default(); 1];
        match find(kinds::UI_CROWN, &mut crowns) {
            Ok(count) if count > 0 => {
                ui_crown = crowns[0];
                info!("TASKMAN: Found UI Crown (attempt {})", attempt + 1);
                break;
            }
            _ => {}
        }
        stem::sleep(Duration::from_millis(500));
    }

    if ui_crown.to_u64_lossy() == 0 {
        info!("TASKMAN: UI Crown not found after 60s, exiting");
        loop {
            stem::sleep(Duration::from_secs(60));
        }
    }

    // 2. Create window anchored to top-left
    let win = create_node(kinds::UI_WINDOW).expect("create UI_WINDOW");
    link(win, rels::CHILD_OF, ui_crown).expect("link window to crown");
    link(ui_crown, rels::HAS_CHILD, win).expect("link crown has_child window");

    // Window style
    prop_set(win, keys::UI_BG_COLOR, 0xF0F0F0F0).ok(); // Light semi-transparent background
    set_string_prop(win, keys::UI_TITLE, "Task Manager");

    // Position: top-left using absolute coordinates
    prop_set(win, keys::UI_WIDTH, 280).ok();
    prop_set(win, keys::UI_HEIGHT, 400).ok();
    prop_set(win, keys::UI_X, 8).ok();
    prop_set(win, keys::UI_Y, 30).ok();

    // Initial render with empty state
    let initial_tasks = collect_tasks();
    render_task_list(win, &initial_tasks);

    info!(
        "TASKMAN: Window created, found {} initial tasks",
        initial_tasks.len()
    );

    // 3. Main loop: refresh task list every 2 seconds
    loop {
        stem::sleep(Duration::from_secs(2));

        let tasks = collect_tasks();
        render_task_list(win, &tasks);
    }
}
