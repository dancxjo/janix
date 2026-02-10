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
use stem::petals::graph::UiKey;
use stem::thing::ThingId;
use abi::root::RootWatchFilter;
use abi::types::{WatchMode, WatchSpec, WATCH_START_LATEST};
use abi::watch;

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
    id: ThingId,
    name: String,
    state: u64,
    tid: u64,
    priority: u64,
    is_user: bool,
    exit_code: Option<i32>,
    current_cpu: Option<u64>,
    affinity: Option<u64>,
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
                let tid = prop_get(id, keys::PROC_TID).unwrap_or(0);
                let priority = prop_get(id, keys::PROC_PRIORITY).unwrap_or(0);
                let is_user = prop_get(id, keys::PROC_IS_USER).unwrap_or(0) != 0;
                let exit_val = prop_get(id, keys::PROC_EXIT_CODE).unwrap_or(0);
                let exit_code = if state == 0 { Some(exit_val as i32) } else { None };

                // Find current CPU (RUNS_ON edge)
                let mut current_cpu = None;
                let mut q_buf = [QueryRow::default(); 16];
                let mut q = RestrictedQuery::new(&mut q_buf);
                if let Ok(count) = q.get_edges(id, Some(rels::RUNS_ON), 16) {
                    for i in 0..count {
                        let cpu_id = ThingId::from_u64(q.buf[i].val_dst);
                        if let Ok(cpu_idx) = prop_get(cpu_id, keys::PROC_TID) { // dev.Cpu uses proc.tid for CPU index
                            current_cpu = Some(cpu_idx);
                            break;
                        }
                    }
                }

                // Find affinity (PINNED_TO edge)
                let mut affinity = None;
                if let Ok(count) = q.get_edges(id, Some(rels::PINNED_TO), 16) {
                    for i in 0..count {
                        let cpu_id = ThingId::from_u64(q.buf[i].val_dst);
                        if let Ok(cpu_idx) = prop_get(cpu_id, keys::PROC_TID) {
                            affinity = Some(cpu_idx);
                            break;
                        }
                    }
                }

                tasks.push(TaskInfo {
                    id,
                    name,
                    state,
                    tid,
                    priority,
                    is_user,
                    exit_code,
                    current_cpu,
                    affinity,
                });
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
    // Try proc.name first – stored as an interned symbol by the kernel's graphify
    if let Ok(name_sym) = prop_get(id, keys::PROC_NAME) {
        if let Some(name) = resolve_symbol(name_sym) {
            return name;
        }
    }

    // Fall back to NAME interned symbol
    if let Ok(name_sym) = prop_get(id, keys::NAME) {
        if let Some(name) = resolve_symbol(name_sym) {
            return name;
        }
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

/// Map task priority values to human-readable labels.
fn priority_label(priority: u64) -> &'static str {
    match priority {
        0 => "Idle",
        1 => "Low",
        2 => "Normal",
        3 => "High",
        4 => "Realtime",
        _ => "?",
    }
}

/// Build or rebuild the UI tree with the current task list.
/// Returns the index of the currently-selected task (if any).
fn render_task_list(window_id: ThingId, tasks: &[TaskInfo]) -> Option<usize> {
    let mut ui = Petals::begin_window(window_id);
    let mut selected_index: Option<usize> = None;

    let root = ui.row(|ui| {
        // ── Left column: task list ──
        let left = ui.column(|ui| {
            if tasks.is_empty() {
                ui.text("No tasks found")?;
            } else {
                for (i, task) in tasks.iter().enumerate() {
                    let color = state_color(task.state);
                    let label = format!(
                        "{} [{}]",
                        task.name,
                        state_label(task.state)
                    );
                    // Stable key based on process ThingId so bloom's selection survives rebuilds
                    let key_str = format!("task_{:X}", task.id.to_u64_lossy());
                    let item_id = ui.list_item_keyed(UiKey(&key_str), &label, color)?;
                    // Check if this item is selected in the graph
                    if prop_get(item_id, keys::UI_SELECTED).unwrap_or(0) != 0 {
                        selected_index = Some(i);
                    }
                }
            }
            Ok(())
        })?;
        let _ = ui.set_gap(left, 2);
        let _ = ui.set_padding(left, 8);

        // ── Right column: detail pane ──
        let right = ui.column(|ui| {
            if let Some(idx) = selected_index {
                let t = &tasks[idx];
                ui.text(&format!("Name: {}", t.name))?;
                ui.text(&format!("Type: {}", if t.is_user { "User" } else { "System" }))?;
                ui.text(&format!("State: {}", state_label(t.state)))?;
                ui.text(&format!("TID: {}", t.tid))?;
                ui.text(&format!("Priority: {}", priority_label(t.priority)))?;
                
                if let Some(cpu) = t.current_cpu {
                    ui.text(&format!("CPU: {}", cpu))?;
                } else {
                    ui.text("CPU: -")?;
                }
                
                if let Some(aff) = t.affinity {
                    ui.text(&format!("Affinity: CPU {}", aff))?;
                } else {
                    ui.text("Affinity: Any")?;
                }

                if let Some(exit) = t.exit_code {
                    ui.text(&format!("Exit Code: {}", exit))?;
                }
            } else {
                ui.text("Select a process")?;
            }
            Ok(())
        })?;
        let _ = ui.set_gap(right, 4);
        let _ = ui.set_padding(right, 8);

        Ok(())
    });

    if let Ok(root) = root {
        let _ = ui.set_gap(root, 4);
    }

    if ui.finish().is_err() {
        info!("TASKMAN: Failed to build UI tree");
    }

    selected_index
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
    prop_set(win, keys::UI_WIDTH, 500).ok();
    prop_set(win, keys::UI_HEIGHT, 400).ok();
    prop_set(win, keys::UI_X, 8).ok();
    prop_set(win, keys::UI_Y, 30).ok();

    // 4. Initial render
    let mut tasks = collect_tasks();
    render_task_list(win, &tasks);

    info!(
        "TASKMAN: Window created, found {} initial tasks",
        tasks.len()
    );

    // 5. Setup watches for reactive updates
    let mut watchers = Vec::new();

    // Watch for selection changes (UI_SCENE_GEN on the window)
    if let Ok(pred) = stem::thing::sys::intern(keys::UI_SCENE_GEN) {
        let filter = RootWatchFilter::predicate(pred);
        let spec = WatchSpec {
            mode: WatchMode::StreamOnly as u32,
            start_seq: WATCH_START_LATEST,
            filter_ptr: &filter as *const _ as u64,
            filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
            ..Default::default()
        };
        if let Ok(id) = stem::syscall::root_watch_open(&spec) {
            watchers.push(id);
        }
    }

    // Watch for task changes (proc.Thread kind)
    if let Ok(kind_id) = stem::thing::sys::intern(kinds::PROC_THREAD) {
        let filter = RootWatchFilter::kind(kind_id);
        let spec = WatchSpec {
            mode: WatchMode::StreamOnly as u32,
            start_seq: WATCH_START_LATEST,
            filter_ptr: &filter as *const _ as u64,
            filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
            ..Default::default()
        };
        if let Ok(id) = stem::syscall::root_watch_open(&spec) {
            watchers.push(id);
        }
    }

    // 6. Reactive main loop
    let mut watch_buf = [0u8; 4096];
    let mut seq = 0u64;

    loop {
        let mut dirty = false;

        for &watcher in &watchers {
            // Non-blocking poll of the watch stream
            if let Ok(len) = stem::syscall::root_watch_next(watcher, &mut seq, &mut watch_buf) {
                if len > 0 {
                    dirty = true;
                }
            }
        }

        if dirty {
            tasks = collect_tasks();
            render_task_list(win, &tasks);
        }

        stem::sleep(Duration::from_millis(50));
    }
}
