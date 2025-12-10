#![no_std]

extern crate alloc;

use alloc::format;
use alloc::vec::Vec;
use abi::graph_kinds;
use thing_models::{BootProfile, BootProgram, ProgramImage};
use userland::prelude::*;
use userland_std::{
    add_edge, edge_targets, find_thing, list_things_by_kind, load_thing, spawn_program, ProcessThing,
};

const SUPERVISOR_IDLE_NS: u64 = 100_000_000;

pub fn run<S: Sys>(sys: &mut S) -> ! {
    println(sys, "init: starting");

    let boot_profile = match load_boot_profile(sys) {
        Some(profile) => profile,
        None => fatal(sys, "BootProfile missing or duplicated"),
    };
    log_dynamic(
        sys,
        format!("init: BootProfile version {}", boot_profile.version),
    );

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

    let program_images: Vec<ProgramImage> = list_things_by_kind(sys);

    for program in programs.iter() {
        log_dynamic(
            sys,
            format!(
                "init: BootProgram name={} app_id={} priority={} binary={}",
                program.name, program.app_id, program.priority, program.binary
            ),
        );
        if let Some(image) = program_images
            .iter()
            .find(|img| img.identifier == program.binary)
        {
            log_dynamic(
                sys,
                format!(
                    "init: BootProgram {} backed by ProgramImage id={} module_index={} base_phys={:#x} size={}",
                    program.name,
                    image.identifier,
                    image.module_index,
                    image.base_phys,
                    image.size
                ),
            );
        } else {
            log_dynamic(
                sys,
                format!(
                    "init: WARNING: no ProgramImage found for BootProgram {} (binary={})",
                    program.name, program.binary
                ),
            );
        }
        log_dynamic(
            sys,
            format!(
                "init: spawning BootProgram {} (app_id={}, binary={})",
                program.name, program.app_id, program.binary
            ),
        );
        if let Some((process_id, _thread_id)) = spawn_program(sys, program.id) {
            if !add_edge(sys, init_process.id, graph_kinds::EDGE_SPAWNED, process_id) {
                log_dynamic(
                    sys,
                    "init: failed to add SPAWNED edge after spawn_program".into(),
                );
            }
        } else {
            log_dynamic(
                sys,
                format!("init: spawn_program failed for {}", program.name),
            );
        }
    }

    log_dynamic(sys, "init: entering supervision loop".into());
    loop {
        sys.sleep_for_ns(SUPERVISOR_IDLE_NS);
    }
}

fn load_boot_profile<S: Sys>(sys: &mut S) -> Option<BootProfile> {
    let mut profiles: Vec<BootProfile> = list_things_by_kind(sys);
    log_dynamic(
        sys,
        format!("init: BootProfile query returned {} entries", profiles.len()),
    );
    if let Some(bp) = load_thing::<BootProfile>(sys, abi::ThingId(12)) {
        log_dynamic(
            sys,
            format!("init: direct load of ThingId(12) succeeded with version {}", bp.version),
        );
    } else {
        log_dynamic(sys, "init: direct load of ThingId(12) failed".into());
    }
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

fn fatal<S: Sys>(sys: &mut S, msg: &str) -> ! {
    log_dynamic(sys, format!("init fatal: {}", msg));
    loop {
        sys.sleep_for_ns(SUPERVISOR_IDLE_NS);
    }
}
