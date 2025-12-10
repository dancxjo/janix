#![no_std]

extern crate alloc;

use abi::{ThingId, graph_kinds};
use alloc::format;
#[cfg(feature = "rootfs")]
use alloc::string::String;
use alloc::vec::Vec;
use thing_models::{BootProfile, BootProgram, Mode, Place, ProgramImage};
use userland::prelude::*;
use userland_std::{
    ProcessThing, add_edge, edge_targets, find_thing, list_things_by_kind, load_thing,
    spawn_program,
};

const SUPERVISOR_IDLE_NS: u64 = 100_000_000;

#[cfg(feature = "rootfs")]
const ROOTFS_IDENTIFIER: &str = "rootfs";

pub fn run<S: Sys>(sys: &mut S) -> ! {
    println(sys, "init: starting");

    seed_modes(sys);

    #[cfg(feature = "rootfs")]
    {
        let program_images: Vec<ProgramImage> = list_things_by_kind(sys);
        start_rootfs(sys, &program_images);
    }

    let boot_profile = wait_for_boot_profile(sys);
    log_dynamic(
        sys,
        format!("init: BootProfile version {}", boot_profile.version),
    );

    let launch_ids = edge_targets(sys, boot_profile.id, graph_kinds::EDGE_LAUNCHES);

    let mut programs = Vec::new();
    collect_boot_programs(sys, &mut programs, launch_ids.as_slice());

    if programs.is_empty() {
        log_dynamic(
            sys,
            "init: no BootProgram edges; waiting briefly for rootfs".into(),
        );
        for _ in 0..8 {
            sys.sleep_for_ns(SUPERVISOR_IDLE_NS);
            let refresh_ids = edge_targets(sys, boot_profile.id, graph_kinds::EDGE_LAUNCHES);
            collect_boot_programs(sys, &mut programs, refresh_ids.as_slice());
            if !programs.is_empty() {
                break;
            }
        }
    }

    if programs.is_empty() {
        log_dynamic(
            sys,
            "init: still no BootPrograms after waiting; scanning all BootProgram Things".into(),
        );
        programs = list_things_by_kind(sys);
    }

    if programs.is_empty() {
        log_dynamic(
            sys,
            "init: no BootPrograms found after full scan; system will idle".into(),
        );
    }

    let init_process = find_process_by_pid(sys, 1)
        .unwrap_or_else(|| fatal(sys, "init Process Thing (pid=1) missing"));

    let program_images: Vec<ProgramImage> = list_things_by_kind(sys);

    for _ in programs.iter().filter(|program| is_rootfs(program)) {
        log_dynamic(
            sys,
            "init: skipping rootfs BootProgram entry (already handled)".into(),
        );
    }

    let (driver_programs, app_programs): (Vec<&BootProgram>, Vec<&BootProgram>) = programs
        .iter()
        .filter(|program| !is_rootfs(program))
        .partition(|program| is_driver(program));

    if !driver_programs.is_empty() {
        log_dynamic(
            sys,
            format!(
                "init: launching {} driver BootProgram(s) before apps",
                driver_programs.len()
            ),
        );
    }

    for program in driver_programs.iter().copied() {
        spawn_boot_program(sys, &init_process, &program_images, program);
    }

    for program in app_programs.iter().copied() {
        spawn_boot_program(sys, &init_process, &program_images, program);
    }

    log_dynamic(sys, "init: entering supervision loop".into());
    loop {
        sys.sleep_for_ns(SUPERVISOR_IDLE_NS);
    }
}

fn seed_modes<S: Sys>(sys: &mut S) {
    let _ = register_schema_for::<Mode>(sys);
    let _ = register_schema_for::<Place>(sys);

    let existing: Vec<Mode> = list_things_by_kind(sys);
    if !existing.is_empty() {
        return;
    }

    for idx in 1..=12_u8 {
        let name = match idx {
            1 => "Desktop".to_string(),
            12 => "Console".to_string(),
            _ => format!("Mode {}", idx),
        };

        let place = Place {
            id: ThingId(0),
            name: format!("place-{}", idx),
        };
        let place_id = create_thing(sys, &place).unwrap_or(ThingId(0));

        let mode = Mode {
            id: ThingId(0),
            index: idx,
            name,
            place_id: Some(place_id),
            active: idx == 1,
        };
        if let Some(mode_id) = create_thing(sys, &mode) {
            let _ = add_edge(sys, mode_id, graph_kinds::EDGE_MODE_PLACE, place_id);
        }
    }
}

fn spawn_boot_program<S: Sys>(
    sys: &mut S,
    init_process: &ProcessThing,
    program_images: &[ProgramImage],
    program: &BootProgram,
) {
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
                program.name, image.identifier, image.module_index, image.base_phys, image.size
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

fn is_driver(program: &BootProgram) -> bool {
    looks_like_driver_identifier(&program.name) || looks_like_driver_identifier(&program.binary)
}

fn looks_like_driver_identifier(identifier: &str) -> bool {
    // Drivers currently follow a naming convention like "ps2_keyboard_driver".
    identifier.contains("_driver") || identifier.contains("-driver")
}

fn load_boot_profile<S: Sys>(sys: &mut S) -> Option<BootProfile> {
    let mut profiles: Vec<BootProfile> = list_things_by_kind(sys);
    log_dynamic(
        sys,
        format!(
            "init: BootProfile query returned {} entries",
            profiles.len()
        ),
    );
    if let Some(bp) = load_thing::<BootProfile>(sys, abi::ThingId(12)) {
        log_dynamic(
            sys,
            format!(
                "init: direct load of ThingId(12) succeeded with version {}",
                bp.version
            ),
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

fn collect_boot_programs<S: Sys>(sys: &mut S, programs: &mut Vec<BootProgram>, ids: &[ThingId]) {
    for program_id in ids.iter().copied() {
        if let Some(program) = load_thing::<BootProgram>(sys, program_id) {
            programs.push(program);
        } else {
            log_dynamic(
                sys,
                format!(
                    "init: ignoring missing BootProgram ThingId {}",
                    program_id.0
                ),
            );
        }
    }
}

fn wait_for_boot_profile<S: Sys>(sys: &mut S) -> BootProfile {
    for _ in 0..32 {
        if let Some(profile) = load_boot_profile(sys) {
            return profile;
        }
        sys.sleep_for_ns(SUPERVISOR_IDLE_NS);
    }
    fatal(sys, "BootProfile missing or duplicated after waiting");
}

#[cfg(feature = "rootfs")]
fn start_rootfs<S: Sys>(sys: &mut S, images: &[ProgramImage]) {
    if let Some(existing) = find_thing::<BootProgram>(sys, |bp| bp.binary == ROOTFS_IDENTIFIER) {
        log_dynamic(
            sys,
            format!(
                "init: rootfs BootProgram already exists as ThingId {}",
                existing.id.0
            ),
        );
        let _ = spawn_program(sys, existing.id);
        return;
    }

    if let Some(image) = images
        .iter()
        .find(|img| img.identifier == ROOTFS_IDENTIFIER)
    {
        let temp_program = BootProgram {
            id: ThingId(0),
            name: String::from(ROOTFS_IDENTIFIER),
            app_id: 0,
            priority: 0,
            binary: image.identifier.clone(),
        };
        if let Some(program_id) = create_thing(sys, &temp_program) {
            log_dynamic(
                sys,
                format!(
                    "init: created temporary rootfs BootProgram id={}",
                    program_id.0
                ),
            );
            let _ = spawn_program(sys, program_id);
        } else {
            log_dynamic(sys, "init: failed to create BootProgram for rootfs".into());
        }
    } else {
        log_dynamic(
            sys,
            "init: rootfs ProgramImage missing; skipping rootfs launch".into(),
        );
    }
}

#[cfg(feature = "rootfs")]
fn is_rootfs(program: &BootProgram) -> bool {
    program.binary == ROOTFS_IDENTIFIER
}

#[cfg(not(feature = "rootfs"))]
fn is_rootfs(_program: &BootProgram) -> bool {
    false
}
