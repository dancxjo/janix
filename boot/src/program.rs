extern crate alloc;

use alloc::{boxed::Box, string::String};
use abi::{PropValue, ThingId};
use kernel_core::graph;
use kernel_core::graph_kinds;
use kernel_core::sched::SCHEDULER;

use crate::user;

pub fn spawn_program(boot_program_id: ThingId) -> Result<(ThingId, ThingId), &'static str> {
    let (kind, props) = kernel_core::graph::get_thing(boot_program_id)
        .ok_or("BootProgram Thing not found")?;
    if kind != graph_kinds::KIND_BOOT_PROGRAM {
        return Err("SpawnProgram Thing was not BootProgram");
    }

    let mut name: Option<String> = None;
    let mut app_id: Option<u64> = None;
    let mut priority: u64 = 0;
    let mut binary: Option<String> = None;

    for prop in props.iter().flatten() {
        match prop.0 {
            "name" => {
                if let PropValue::Str(s) = &prop.1 {
                    name = Some(s.clone());
                }
            }
            "app_id" => {
                if let PropValue::U64(v) = prop.1 {
                    app_id = Some(v);
                }
            }
            "priority" => {
                if let PropValue::U64(v) = prop.1 {
                    priority = v;
                }
            }
            "binary" => {
                if let PropValue::Str(s) = &prop.1 {
                    binary = Some(s.clone());
                }
            }
            _ => {}
        }
    }

    let name = name.ok_or("BootProgram missing name")?;
    let app_id = app_id.ok_or("BootProgram missing app_id")?;
    let binary = binary.unwrap_or_default();

    if find_program_image(&binary).is_none() {
        kernel_core::log("SpawnProgram could not find ProgramImage");
    }

    let leaked_name: &'static str = Box::leak(name.into_boxed_str());
    let stack = user::alloc_user_stack();

    let (pid, tid, process_thing, thread_thing) = {
        let mut sched = SCHEDULER.lock();
        let pid = sched.add_process(leaked_name);
        let tid = sched.add_thread(
            pid,
            leaked_name,
            user::user_thread_main,
            app_id,
            stack,
            priority,
        );
        let process_thing = sched
            .process_thing_id(pid)
            .ok_or("Process Thing not recorded")?;
        let thread_thing = sched
            .thread_thing_id(tid)
            .ok_or("Thread Thing not recorded")?;
        (pid, tid, process_thing, thread_thing)
    };

    Ok((process_thing, thread_thing))
}

fn find_program_image(identifier: &str) -> Option<ThingId> {
    let mut found = None;
    graph::iter_things(|thing| {
        if thing.kind != graph_kinds::KIND_PROGRAM_IMAGE {
            return;
        }
        let mut id_value: Option<String> = None;
        for prop in thing.props.iter().flatten() {
            if prop.0 == "identifier" {
                if let PropValue::Str(s) = &prop.1 {
                    id_value = Some(s.clone());
                }
            }
        }
        if let Some(id) = id_value {
            if id == identifier {
                found = Some(thing.id);
            }
        }
    });
    found
}
