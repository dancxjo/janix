#![no_std]
#![no_main]

use standard::prelude::*;
use stem::{sys_spawn_module, sys_yield};

#[no_mangle]
pub fn main() -> i32 {
    println!("sprout: root init");

    // Create a watch
    let watch_id = match watch_create() {
        Ok(id) => {
            println!("sprout: watch id={}", id);
            id
        },
        Err(e) => {
            println!("sprout: watch_create failed err={}", e);
            loop { unsafe { sys_yield() }; }
        }
    };

    // Append initial facts
    // 1: sprout
    // 2: clock
    // Edge: sprout -> clock
    
    // PutNode(sprout)
    let op1 = JournalOp { kind: OpKind::PutNode, a: 1, b: 0, key: 0, value: 0 };
    match graph_append(op1) {
        Ok(idx) => println!("sprout: appended idx={} PutNode sprout", idx),
        Err(e) => println!("sprout: append failed err={}", e),
    }

    // PutNode(clock)
    let op2 = JournalOp { kind: OpKind::PutNode, a: 2, b: 0, key: 0, value: 0 };
    match graph_append(op2) {
        Ok(idx) => println!("sprout: appended idx={} PutNode clock", idx),
        Err(e) => println!("sprout: append failed err={}", e),
    }

    // PutEdge(sprout -> clock)
    let op3 = JournalOp { kind: OpKind::PutEdge, a: 1, b: 2, key: 0, value: 0 };
    match graph_append(op3) {
        Ok(idx) => println!("sprout: appended idx={} PutEdge sprout->clock", idx),
        Err(e) => println!("sprout: append failed err={}", e),
    }

    // Spawn clock
    // For now we assume typical paths.
    let modules = [
        "/boot/modules/rtc_cmos",
        "/boot/modules/clock",
    ];

    for mod_path in modules {
        print!("sprout: spawning {}... ", mod_path);
        let tid = unsafe { sys_spawn_module(mod_path) };
        if tid >= 0 {
            println!("ok tid={}", tid);
        } else {
            println!("failed err={}", tid);
        }
    }

    loop {
        match watch_next(watch_id) {
            Ok(Some(event)) => {
                print!("sprout: event idx={} ", event.index);
                match event.op.kind {
                     OpKind::PutNode => println!("PutNode {}", event.op.a),
                     OpKind::PutEdge => println!("PutEdge {}->{}", event.op.a, event.op.b),
                     OpKind::DelNode => println!("DelNode {}", event.op.a),
                     OpKind::DelEdge => println!("DelEdge {}->{}", event.op.a, event.op.b),
                }
            },
            Ok(None) => {
                // EAGAIN
                unsafe { sys_yield() };
            },
            Err(e) => {
                println!("sprout: watch error {}", e);
                unsafe { sys_yield() };
            }
        }
    }
}
