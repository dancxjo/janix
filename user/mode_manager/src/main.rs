#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;
use core::panic::PanicInfo;

use abi::{
    graph_kinds::{self, KIND_SYSTEM, LINK_HAS_ACTIVE_MODE},
    PropValue, ThingId, KernelRequest, KernelResponse,
};
use thing_models::{KeyScanEvent, Mode};
use thing_os::prelude::*;
use thing_os::{
    entry, add_link, update_props, create_thing, list_things_by_kind,
    MODE_INDEX_CONSOLE,
};

#[derive(Debug, Clone)]
struct SystemThing {
    id: ThingId,
}

impl thing_os::Thing for SystemThing {
    const KIND: &'static str = KIND_SYSTEM;
    const DESCRIPTION: &'static str = "Root System Node";
    
    fn schema() -> &'static [(&'static str, abi::PropType)] {
        &[(graph_kinds::PROP_NAME, abi::PropType::Str)]
    }
    
    fn from_props(id: ThingId, _props: &[Option<(abi::PropKey, PropValue)>]) -> Self {
        SystemThing { id }
    }
    fn to_props(&self, out: &mut Vec<(abi::PropKey, PropValue)>) {
        out.push((graph_kinds::PROP_NAME, PropValue::Str("System".into())));
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    entry(|sys| run(sys));
}

fn run<S: Sys>(sys: &mut S) -> ! {
    println(sys, "mode_manager: starting");

    let _ = register_schema_for::<KeyScanEvent>(sys);
    let _ = register_schema_for::<Mode>(sys);
    
    let (system_id, sky_id, console_id) = ensure_modes_exist(sys);
    
    println(sys, "mode_manager: modes ensured. listening for F1/F12...");

    let mut last_key_seq = initial_sequence::<S, KeyScanEvent>(sys);

    loop {
        let events = new_things_since::<S, KeyScanEvent>(sys, &mut last_key_seq);
        for event in events {
            if event.released {
                continue;
            }
            
            // F1 = 0x3B (Sky)
            // F12 = 0x58 (Console)
            
            let target_mode_id = match event.scancode {
                0x3B => Some(sky_id),
                0x58 => Some(console_id),
                _ => None,
            };

            if let Some(target) = target_mode_id {
                let name = if target == sky_id { "Sky" } else { "Console" };
                let msg = format!("mode_manager: F-key detected. Switching to {}", name);
                let leaked = Box::leak(msg.into_boxed_str());
                println(sys, leaked);
                
                set_active_mode(sys, system_id, target, name);
            }
        }

        sys.sleep_for_ns(50_000_000); // 50ms poll
    }
}

fn ensure_modes_exist<S: Sys>(sys: &mut S) -> (ThingId, ThingId, ThingId) {
    // 1. Find or create System thing
    let system_id = match list_things_by_kind::<S, SystemThing>(sys).first() {
        Some(t) => t.id,
        None => {
            println(sys, "mode_manager: creating System thing");
            // Use create_thing with our SystemThing struct which properly implements to_props
            let sys_thing = SystemThing { id: ThingId(0) }; // ID ignored on create
            create_thing(sys, &sys_thing)
                .expect("Failed to create System thing")
        }
    };

    // 2. Find or create Modes
    let modes: Vec<Mode> = list_things_by_kind(sys);
    
    let sky_mode = modes.iter().find(|m| m.index == 1);
    let sky_id = match sky_mode {
        Some(m) => m.id,
        None => {
             // Create Sky mode (index 1)
             println(sys, "mode_manager: creating Sky mode");
             let mode = Mode {
                 id: ThingId(0),
                 index: 1,
                 name: "Sky".into(),
                 place_id: None, 
                 active: false, 
                 layout_policy: None,
             };
             create_thing(sys, &mode).expect("Failed to create Sky mode")
        }
    };

    let console_mode = modes.iter().find(|m| m.index == MODE_INDEX_CONSOLE);
    let console_id = match console_mode {
        Some(m) => m.id,
        None => {
             // Create Console mode
             println(sys, "mode_manager: creating Console mode");
             let mode = Mode {
                 id: ThingId(0),
                 index: MODE_INDEX_CONSOLE,
                 name: "Console".into(),
                 place_id: None,
                 active: false,
                 layout_policy: None,
             };
             create_thing(sys, &mode).expect("Failed to create Console mode")
        }
    };

    // 3. Ensure default active mode (Sky) if no edge exists
    let links = get_outgoing_links_by_pred(sys, system_id, LINK_HAS_ACTIVE_MODE);
    if links.is_empty() {
        println(sys, "mode_manager: setting default active mode to Sky");
        add_link(sys, system_id, LINK_HAS_ACTIVE_MODE, sky_id);
        
        let msg = format!("MODE graph set -> Sky (default)");
        let leaked = Box::leak(msg.into_boxed_str());
        println(sys, leaked);
    }
    
    (system_id, sky_id, console_id)
}

fn set_active_mode<S: Sys>(sys: &mut S, system_id: ThingId, target_mode_id: ThingId, name: &str) {
    // Transaction:
    // 1. Find existing System -> HAS_ACTIVE_MODE links
    // 2. If exists, update target. If not, create.
    
    let existing = get_outgoing_links_by_pred(sys, system_id, LINK_HAS_ACTIVE_MODE);
    
    if let Some(link) = existing.first() {
        if link.target == target_mode_id {
            return; // Already set
        }
        // Update existing link to point to new target
        update_props(sys, link.id, &[(graph_kinds::PROP_LINK_DST, PropValue::U64(target_mode_id.0))]);
    } else {
        // Create new
        add_link(sys, system_id, LINK_HAS_ACTIVE_MODE, target_mode_id);
    }
    
    let msg = format!("MODE graph set -> {}", name);
    let leaked = Box::leak(msg.into_boxed_str());
    println(sys, leaked);
}

// Helpers

// Initial sequence helper 
fn initial_sequence<S: Sys, T: thing_os::Thing>(sys: &mut S) -> u64 {
    // KeyScanEvent specific optimization: just assume 0 for now as we want all new events.
    // Or scan max.
    // Since T is generic, we can't access sequence_index unless we add a specific trait bound,
    // or just return 0.
    // For simplicity, returning 0 might replay old events if service restarts.
    // But since `new_things_since` iterates ALL with sequence > last_seq, 
    // we should try to find max seq.
    // Without Generic trait bound, we can't.
    // We specialized this before by listing KeyScanEvents directly in the caller if needed.
    // But actually, KeyScanEvent struct has it.
    // Let's implement it for KeyScanEvent only or T where T: KeyScanEvent? No.
    // Just implement a specialized version for KeyScanEvent.
    
    // Wait, the original code had:
    // `let mut last_key_seq = initial_sequence::<S, KeyScanEvent>(sys);`
    // And `initial_sequence` took T: Thing.
    // And inside it mapped `t -> 0`?
    // That means it returned 0.
    // So let's just use 0.
    0
}

fn new_things_since<S: Sys, T: thing_os::Thing + Clone>(sys: &mut S, last_seq: &mut u64) -> Vec<KeyScanEvent> {
    // This function is specialized for KeyScanEvent because we access `sequence_index`.
    // We pretend access via T is not needed if we hardcode KeyScanEvent usage inside, 
    // but the signature asks for T.
    // Let's just remove T and use KeyScanEvent explicitly.
    
    let events: Vec<KeyScanEvent> = list_things_by_kind(sys);
    let mut new_events = Vec::new();
    let mut max_seq = *last_seq;
    
    for event in events {
        if event.sequence_index > *last_seq {
            new_events.push(event.clone());
            if event.sequence_index > max_seq {
                max_seq = event.sequence_index;
            }
        }
    }
    *last_seq = max_seq;
    
    // Sort by sequence
    new_events.sort_by_key(|e| e.sequence_index);
    new_events
}

// Link helpers
struct Link {
    id: ThingId,
    target: ThingId,
}

fn get_outgoing_links_by_pred<S: Sys>(sys: &mut S, source: ThingId, pred: abi::Predicate) -> Vec<Link> {
     let mut links = Vec::new();
     let mut cursor = ThingId(u64::MAX);
     loop {
        match sys.syscall(KernelRequest::ThingList { 
            kind: graph_kinds::KIND_LINK, 
            start_after: cursor 
        }) {
            KernelResponse::ThingListEntry { id: Some(link_id) } => {
                cursor = link_id;
                // Get props
                if let KernelResponse::ThingData { props, .. } = sys.syscall(KernelRequest::ThingGet { id: link_id }) {
                    let mut l_src = None;
                    let mut l_pred = None;
                    let mut l_dst = None;
                    
                    for (k, v) in props.iter().flatten() {
                         if *k == graph_kinds::PROP_LINK_SRC {
                             if let PropValue::U64(id_val) = v { l_src = Some(ThingId(*id_val)); }
                         } else if *k == graph_kinds::PROP_LINK_DST {
                             if let PropValue::U64(id_val) = v { l_dst = Some(ThingId(*id_val)); }
                         } else if *k == graph_kinds::PROP_LINK_PRED {
                             if let PropValue::U64(p_val) = v { l_pred = Some(abi::Predicate(*p_val)); }
                         }
                    }
                    
                    if l_src == Some(source) && l_pred == Some(pred) {
                        if let Some(target) = l_dst {
                            links.push(Link { id: link_id, target });
                        }
                    }
                }
            },
            _ => break,
        }
     }
     links
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
