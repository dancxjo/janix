#![no_std]
#![no_main]

use thing_os::prelude::*;
use thing_os::PropValue;
use abi::wire::graph::{WatchSpec, WatchSpecTag, WatchFlags, WatchEvent, WatchEventKind};

#[thing_os::main]
fn main() {
    println!("watch_test: starting...");
    
    // 1. Create a subject to watch
    let kind_test = "WatchTestSubject";
    let subject_id = match user_create_thing(kind_test, &[]) {
        Ok(id) => id,
        Err(e) => {
            println!("watch_test: failed to create subject: {}", e);
            return;
        }
    };
    println!("watch_test: created subject {}", subject_id.0);

    // 2. Prepare watch spec
    let key_test = thing_os::intern("test_prop");
    let spec = WatchSpec {
        tag: WatchSpecTag::Prop,
        thing: subject_id,
        key: key_test,
        flags: WatchFlags { bits: WatchFlags::PROP_SET | WatchFlags::PROP_DEL },
    };
    
    // 3. Open watch
    let watch_id = match thing_os::watch_open(&spec) {
        Some(id) => id,
        None => {
             println!("watch_test: failed to open watch");
             return;
        }
    };
    println!("watch_test: opened watch {}", watch_id.0);

    // 4. Trigger mutation (should generate PropSet event)
    println!("watch_test: triggering mutation...");
    let props = [(String::from("test_prop"), PropValue::U64(123))];
    if let Err(e) = thing_os::user_update_thing(subject_id, &props) {
        println!("watch_test: failed to update: {}", e);
    }
    
    // 5. Read events (blocking)
    let mut buf = [WatchEvent {
        kind: WatchEventKind::Overflow, // dummy default
        src_or_thing: abi::ThingId(0),
        pred_or_key: abi::syscall_defs::SymbolId(0),
        dst_or_aux: 0
    }; 4];
    
    println!("watch_test: waiting for events...");
    match thing_os::watch_next(watch_id, &mut buf) {
        Some(count) => {
            println!("watch_test: got {} events", count);
             for i in 0..count {
                let ev = buf[i];
                println!("Event: kind={:?} thing={} key={} val_tag={}", 
                    ev.kind, ev.src_or_thing.0, ev.pred_or_key.0, ev.dst_or_aux);
             }
        }
        None => println!("watch_test: error reading events"),
    }
    
    // Close watch
    if thing_os::watch_close(watch_id) {
        println!("watch_test: closed watch");
    } else {
        println!("watch_test: failed to close watch");
    }
    
    println!("watch_test: done.");
}
