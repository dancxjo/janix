#![no_std]

extern crate alloc;

use abi::ThingId;
#[cfg(feature = "rootfs")]
use alloc::string::String;
use alloc::vec::Vec;
use thing_models::graph_kinds;
use thing_models::{BootProfile, BootProgram, Mode, Place, ProgramImage};
use thing_os::prelude::*;
use thing_os::{
    MODE_INDEX_CONSOLE, ProcessThing, add_link, create_process, find_thing, link_targets,
    list_things_by_kind, load_thing,
};

#[allow(unused_imports)]
use alloc::collections::BTreeSet;

use abi::syscall_defs::SymbolId;
use abi::wire::graph::{WatchEvent, WatchSpec, WatchId, WatchSpecTag, WatchEventKind, WatchFlags};
// import graph_kinds::* if needed
use thing_models::{Host, Module};
use thing_os::{intern, watch_open, watch_next};

pub const SUPERVISOR_IDLE_NS: u64 = 100_000_000;

pub fn init_main() -> ! {
    println!("init: starting (Host model)");

    ensure_modes();
    
    // Find Host
    let host = find_thing::<Host>(|_| true).expect("Host not found");
    println!("init: Found Host id={}", host.id.0);

    // Create Watch on Host for HAS_MODULE
    let pred_has_module = graph_kinds::LINK_HAS_MODULE;
    let spec = WatchSpec {
        tag: WatchSpecTag::Link,
        thing: host.id,
        key: SymbolId(pred_has_module.0 as u32),
        flags: WatchFlags { bits: WatchFlags::LINK_ADDED },
    };

    let watch_id = watch_open(&spec).expect("Failed to open watch on Host");
    println!("init: Watch opened id={}", watch_id.0);

    // Enumerate existing modules
    let modules = link_targets(host.id, graph_kinds::LINK_HAS_MODULE);
    let mut spawned = BTreeSet::new();

    for mod_id in modules {
        process_module(mod_id, &mut spawned);
    }

    // Event Loop
    let dummy = WatchEvent { 
        kind: WatchEventKind::Overflow, 
        src_or_thing: ThingId(0), 
        pred_or_key: SymbolId(0), 
        dst_or_aux: 0 
    };
    let mut buf = [dummy; 16];
    loop {
        if let Some(count) = watch_next(watch_id, &mut buf) {
            for i in 0..count {
                let evt = &buf[i];
                // Check if LinkAdded and correct pred (already filtered by kernel if specific, but useful to double check)
                if evt.kind == WatchEventKind::LinkAdded && evt.pred_or_key == SymbolId(pred_has_module.0 as u32) {
                    process_module(ThingId(evt.dst_or_aux), &mut spawned);
                }
            }
        } else {
             // Block/sleep if watch_next returns None/0? 
             // watch_next should block if implemented that way, or return 0 if non-blocking.
             // Impl said "Handles WOULD_BLOCK by sleeping".
             // So it blocks.
        }
    }
}

fn process_module(mod_id: ThingId, spawned: &mut BTreeSet<ThingId>) {
    if spawned.contains(&mod_id) { return; }
    
    if let Some(module) = load_thing::<Module>(mod_id) {
        println!("init: found module '{}' role='{}'", module.name, module.role);
        
        if module.role == "service" || module.role == "driver" {
            // Check if it is init itself
            if module.name == "init" {
                spawned.insert(mod_id);
                return;
            }
            
            println!("init: spawning {}", module.name);
            match create_process(mod_id) {
                Ok((pid, _)) => {
                    println!("init: spawned {} as pid={}", module.name, pid.0);
                    spawned.insert(mod_id);
                },
                Err(e) => println!("init: failed to spawn {}: {}", module.name, e),
            }
        }
    }
}

// Keep helper functions
fn ensure_modes() {
    if !ensure_schema_exists_for::<Mode>() {
        fatal("Mode schema missing");
    }
    if !ensure_schema_exists_for::<Place>() {
        fatal("Place schema missing");
    }

    let existing: Vec<Mode> = list_things_by_kind();
    if !existing.is_empty() {
        return;
    }

    let main_place = Place {
        id: ThingId(0),
        name: "place-main".to_string(),
        layout_mode: None,
    };
    let console_place = Place {
        id: ThingId(0),
        name: "place-console".to_string(),
        layout_mode: None,
    };

    let main_place_id = create_thing(&main_place).unwrap_or(ThingId(0));
    let console_place_id = create_thing(&console_place).unwrap_or(ThingId(0));

    let main_mode = Mode {
        id: ThingId(0),
        index: 1,
        name: "Desktop".to_string(),
        place_id: Some(main_place_id),
        active: true,
        layout_policy: None,
    };
    if let Some(mode_id) = create_thing(&main_mode) {
        let _ = add_link(mode_id, graph_kinds::LINK_MODE_PLACE, main_place_id);
    }

    let console_mode = Mode {
        id: ThingId(0),
        index: MODE_INDEX_CONSOLE,
        name: "Console".to_string(),
        place_id: Some(console_place_id),
        active: false,
        layout_policy: None,
    };
    if let Some(mode_id) = create_thing(&console_mode) {
        let _ = add_link(mode_id, graph_kinds::LINK_MODE_PLACE, console_place_id);
    }
}

fn fatal(msg: &str) -> ! {
    println!("init fatal: {}", msg);
    loop {
        sleep(Duration::from_nanos(SUPERVISOR_IDLE_NS));
    }
}

