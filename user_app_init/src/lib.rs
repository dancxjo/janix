#![no_std]

extern crate alloc;

use alloc::format;
use alloc::vec::Vec;
use abi::graph_kinds;
use thing_models::{BootProfile, BootProgram};
use userland::prelude::*;
use userland_std::{
    add_edge,
    create_process,
    create_thread,
    edge_targets,
    find_thing,
    list_things_by_kind,
    load_thing,
    ProcessThing,
};

const SUPERVISOR_IDLE_NS: u64 = 100_000_000;

pub fn run<S: Sys>(sys: &mut S) {
    println(sys, "init: starting");

    let boot_profile = match load_boot_profile(sys) {
        Some(profile) => profile,
        None => fatal(sys, "BootProfile missing or duplicated"),
    };

    let launch_ids = edge_targets(sys, boot_profile.id, graph_kinds::EDGE_LAUNCHES);
    if launch_ids.is_empty() {
        log_dynamic(sys, "init: no BootProgram edges; system will idle".into());
    }

    let mut programs = Vec::new();
    for program_id in launch_ids.iter().copied() {
        if let Some(program) = load_thing::<BootProgram>(sys, program_id) {
            programs.push(program);
        } else {
            log_dynamic(
                sys,
                format!("init: ignoring missing BootProgram ThingId {}", program_id.0),
            );
        }
    }

    let init_process = find_process_by_pid(sys, 1)
        .unwrap_or_else(|| fatal(sys, "init Process Thing (pid=1) missing"));

    for program in programs.iter() {
        spawn_program(sys, &init_process, program);
    }

    log_dynamic(sys, "init: entering supervision loop".into());
    loop {
        sys.sleep_for_ns(SUPERVISOR_IDLE_NS);
    }
}

fn load_boot_profile<S: Sys>(sys: &mut S) -> Option<BootProfile> {
    let mut profiles: Vec<BootProfile> = list_things_by_kind(sys);
    match profiles.len() {
        1 => profiles.pop(),
        0 => {
            log_dynamic(sys, "init: BootProfile not found".into());
            None
        }
        _ => {
            log_dynamic(sys, "init: multiple BootProfile nodes found".into());
            None
        }
    }
}

fn find_process_by_pid<S: Sys>(sys: &mut S, pid: u64) -> Option<ProcessThing> {
    find_thing::<ProcessThing>(sys, |p| p.pid == pid)
}

fn spawn_program<S: Sys>(sys: &mut S, init_process: &ProcessThing, program: &BootProgram) {
    log_dynamic(
        sys,
        format!(
            "init: launching {} (binary={}, app_id={}, priority={})",
            program.name, program.binary, program.app_id, program.priority
        ),
    );

    let pid = match create_process(sys, program.name.as_str()) {
        Some(pid) => pid,
        None => fatal(sys, "failed to create process via syscall"),
    };

    let _tid = match create_thread(
        sys,
        pid,
        program.binary.as_str(),
        program.app_id,
        program.priority,
    ) {
        Some(tid) => tid,
        None => fatal(sys, "failed to create thread via syscall"),
    };

    if let Some(proc_thing) = find_process_by_pid(sys, pid) {
        if !add_edge(sys, init_process.id, graph_kinds::EDGE_SPAWNED, proc_thing.id) {
            log_dynamic(
                sys,
                format!("init: failed to add SPAWNED edge to pid {}", pid),
            );
        }
    } else {
        log_dynamic(
            sys,
            format!("init: spawned process pid {} missing from graph", pid),
        );
    }
}

fn fatal<S: Sys>(sys: &mut S, msg: &str) -> ! {
    log_dynamic(sys, format!("init fatal: {}", msg));
    loop {
        sys.sleep_for_ns(SUPERVISOR_IDLE_NS);
    }
}
