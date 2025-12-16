#![no_std]

extern crate alloc;

use abi::{ThingId, graph_kinds};
#[cfg(feature = "rootfs")]
use alloc::string::String;
use alloc::vec::Vec;
use thing_models::{BootProfile, BootProgram, Mode, Place, ProgramImage};
use thing_os::prelude::*;
use thing_os::{
    MODE_INDEX_CONSOLE, ProcessThing, add_link, find_thing, link_targets, list_things_by_kind,
    load_thing, spawn_program,
};

const SUPERVISOR_IDLE_NS: u64 = 100_000_000;

#[cfg(feature = "rootfs")]
const ROOTFS_IDENTIFIER: &str = "rootfs";

pub fn main<S: Sys>(sys: &mut S) -> ! {
    println(sys, "init: starting");

    ensure_modes(sys);
    println(sys, "init: ensure_modes done");

    #[cfg(feature = "rootfs")]
    {
        let program_images: Vec<ProgramImage> = list_things_by_kind(sys);
        start_rootfs(sys, &program_images);
    }

    let boot_profile = wait_for_boot_profile(sys);
    log_dynamic(
        sys,
        format_args!("init: BootProfile version {}", boot_profile.version),
    );

    let launch_ids = link_targets(sys, boot_profile.id, graph_kinds::LINK_LAUNCHES);

    let mut programs = Vec::new();
    collect_boot_programs(sys, &mut programs, launch_ids.as_slice());

    if programs.is_empty() {
        println(
            sys,
            "init: no BootProgram links; waiting briefly for rootfs",
        );
        println(sys, "init: entering supervision loop");
        loop {
            sys.sleep_for_ns(SUPERVISOR_IDLE_NS);
            
            let refresh_ids = link_targets(sys, boot_profile.id, graph_kinds::LINK_LAUNCHES);
            collect_boot_programs(sys, &mut programs, refresh_ids.as_slice());
            if !programs.is_empty() {
                break;
            }
        }
    }

    if programs.is_empty() {
        println(
            sys,
            "init: still no BootPrograms after waiting; scanning all BootProgram Things",
        );
        programs = list_things_by_kind(sys);
    }

    if programs.is_empty() {
        println(
            sys,
            "init: no BootPrograms found after full scan; system will idle",
        );
    }

    let init_process = find_process_by_pid(sys, 1)
        .unwrap_or_else(|| fatal(sys, "init Process Thing (pid=1) missing"));

    let program_images: Vec<ProgramImage> = list_things_by_kind(sys);

    for _ in programs.iter().filter(|program| is_rootfs(program)) {
        println(
            sys,
            "init: skipping rootfs BootProgram entry (already handled)",
        );
    }

    let (driver_programs, app_programs): (Vec<&BootProgram>, Vec<&BootProgram>) = programs
        .iter()
        .filter(|program| !is_rootfs(program))
        .partition(|program| is_driver(program));

    let (compositor_programs, other_app_programs): (Vec<&BootProgram>, Vec<&BootProgram>) =
        app_programs
            .iter()
            .copied()
            .partition(|program| is_compositor(program));

    if !driver_programs.is_empty() {
        log_dynamic(
            sys,
            format_args!(
                "init: launching {} driver BootProgram(s) before user",
                driver_programs.len()
            ),
        );
    }

    for program in driver_programs.iter().copied() {
        log_dynamic(
            sys,
            format_args!("init: checking program binary='{}'", program.binary),
        );
        if program.binary == "init" {
            continue;
        }
        spawn_boot_program(sys, &init_process, &program_images, program);
    }

    if !compositor_programs.is_empty() {
        log_dynamic(
            sys,
            format_args!(
                "init: launching compositor early before other user ({} entry/entries)",
                compositor_programs.len()
            ),
        );
    }

    for program in compositor_programs.iter().copied() {
        spawn_boot_program(sys, &init_process, &program_images, program);
    }

    for program in other_app_programs.iter().copied() {
        log_dynamic(
            sys,
            format_args!("init: checking program binary='{}'", program.binary),
        );
        if program.binary == "init" {
            continue;
        }
        // DEBUG: Force disable Geographer for white screen debugging
        if program.binary == "geographer" {
            println(sys, "init: SKIPPING geographer (debug disable)");
            continue;
        }
        // DEBUG: Force disable window_demo for freeze debugging
        if program.binary == "window_demo" {
             println(sys, "init: SKIPPING window_demo (debug disable)");
             continue;
        }
        spawn_boot_program(sys, &init_process, &program_images, program);
    }

    println(sys, "init: entering supervision loop");
    loop {
        sys.sleep_for_ns(SUPERVISOR_IDLE_NS);
    }
}

fn ensure_modes<S: Sys>(sys: &mut S) {
    let _ = register_schema_for::<Mode>(sys);
    let _ = register_schema_for::<Place>(sys);

    let existing: Vec<Mode> = list_things_by_kind(sys);
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

    let main_place_id = create_thing(sys, &main_place).unwrap_or(ThingId(0));
    let console_place_id = create_thing(sys, &console_place).unwrap_or(ThingId(0));

    // Spawn mode_manager
    println(sys, "init: spawning mode_manager");
    if let Some(mode_manager_prog) = find_thing::<BootProgram>(sys, |bp| bp.binary == "mode_manager") {
         if let Some(_) = spawn_program(sys, mode_manager_prog.id) {
             println(sys, "init: mode_manager spawned");
         } else {
             println(sys, "init: ERROR spawn_program failed for mode_manager");
         }
    } else {
        println(sys, "init: ERROR mode_manager program not found!");
    }

    // Spawn geographer
    /*
    println(sys, "init: spawning geographer");
    if let Some(geo_prog) = find_thing::<BootProgram>(sys, |bp| bp.binary == "geographer") {
        let _ = spawn_program(sys, geo_prog.id);
    } else {
        println(sys, "init: geographer program not found");
    }
    */

    let main_mode = Mode {
        id: ThingId(0),
        index: 1,
        name: "Desktop".to_string(),
        place_id: Some(main_place_id),
        active: true,
        layout_policy: None,
    };
    if let Some(mode_id) = create_thing(sys, &main_mode) {
        let _ = add_link(sys, mode_id, graph_kinds::LINK_MODE_PLACE, main_place_id);
    }

    let console_mode = Mode {
        id: ThingId(0),
        index: MODE_INDEX_CONSOLE,
        name: "Console".to_string(),
        place_id: Some(console_place_id),
        active: false,
        layout_policy: None,
    };
    if let Some(mode_id) = create_thing(sys, &console_mode) {
        let _ = add_link(sys, mode_id, graph_kinds::LINK_MODE_PLACE, console_place_id);
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
        format_args!(
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
            format_args!(
                "init: BootProgram {} backed by ProgramImage id={} module_index={} base_phys={:#x} size={}",
                program.name, image.identifier, image.module_index, image.base_phys, image.size
            ),
        );
    } else {
        log_dynamic(
            sys,
            format_args!(
                "init: WARNING: no ProgramImage found for BootProgram {} (binary={})",
                program.name, program.binary
            ),
        );
    }
    log_dynamic(
        sys,
        format_args!(
            "init: spawning BootProgram {} (app_id={}, binary={})",
            program.name, program.app_id, program.binary
        ),
    );
    if let Some((process_id, _thread_id)) = spawn_program(sys, program.id) {
        if !add_link(sys, init_process.id, graph_kinds::LINK_SPAWNED, process_id) {
            println(sys, "init: failed to add SPAWNED link after spawn_program");
        }
    } else {
        log_dynamic(
            sys,
            format_args!("init: spawn_program failed for {}", program.name),
        );
    }
}

fn is_driver(program: &BootProgram) -> bool {
    looks_like_driver_identifier(&program.name) || looks_like_driver_identifier(&program.binary)
}

fn is_compositor(program: &BootProgram) -> bool {
    program.name == "compositor" || program.binary == "compositor"
}

fn looks_like_driver_identifier(identifier: &str) -> bool {
    // Drivers currently follow a naming convention like "ps2_keyboard_driver".
    identifier.contains("_driver") || identifier.contains("-driver")
}

fn load_boot_profile<S: Sys>(sys: &mut S) -> Option<BootProfile> {
    let mut profiles: Vec<BootProfile> = list_things_by_kind(sys);
    log_dynamic(
        sys,
        format_args!(
            "init: BootProfile query returned {} entries",
            profiles.len()
        ),
    );
    if let Some(bp) = load_thing::<BootProfile>(sys, abi::ThingId(12)) {
        log_dynamic(
            sys,
            format_args!(
                "init: direct load of ThingId(12) succeeded with version {}",
                bp.version
            ),
        );
    } else {
        println(sys, "init: direct load of ThingId(12) failed");
    }
    match profiles.len() {
        1 => profiles.pop(),
        0 => {
            println(sys, "init: BootProfile not found");
            None
        }
        _ => {
            println(sys, "init: multiple BootProfile things found");
            None
        }
    }
}

fn find_process_by_pid<S: Sys>(sys: &mut S, pid: u64) -> Option<ProcessThing> {
    find_thing::<ProcessThing>(sys, |p| p.pid == pid)
}

fn fatal<S: Sys>(sys: &mut S, msg: &str) -> ! {
    log_dynamic(sys, format_args!("init fatal: {}", msg));
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
                format_args!(
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

#[cfg(test)]
mod tests {
    use super::*;
    use abi::{KernelRequest, KernelResponse, ThingId};
    use alloc::vec::Vec;
    use thing_models::{BootProgram, ProgramImage};
    use thing_os::{ProcessThing, doc_helpers::DocSys, graph_kinds};

    #[test]
    fn ensure_modes_creates_places_and_modes() {
        let mut responses = Vec::new();
        responses.push(KernelResponse::SchemaRegistered { kind: Mode::KIND });
        responses.push(KernelResponse::SchemaRegistered { kind: Place::KIND });
        responses.push(KernelResponse::ThingListEntry { id: None });
        responses.push(KernelResponse::ThingCreated { id: ThingId(10) });
        responses.push(KernelResponse::ThingCreated { id: ThingId(11) });
        responses.push(KernelResponse::ThingCreated { id: ThingId(20) });
        responses.push(KernelResponse::Success { data: None });
        responses.push(KernelResponse::ThingCreated { id: ThingId(21) });
        responses.push(KernelResponse::Success { data: None });

        let mut sys = DocSys::with_responses(responses);
        ensure_modes(&mut sys);

        let requests = sys.requests.borrow();
        assert!(requests.iter().any(|req| match req {
            KernelRequest::ThingCreate { kind, .. } => *kind == Place::KIND,
            _ => false,
        }));
        assert!(requests.iter().any(|req| match req {
            KernelRequest::ThingCreate { kind, .. } => *kind == Mode::KIND,
            _ => false,
        }));
        assert!(requests.iter().any(|req| match req {
            KernelRequest::AddLink { pred, .. } => *pred == graph_kinds::LINK_MODE_PLACE,
            _ => false,
        }));
    }

    #[test]
    fn spawn_boot_program_links_process_and_logs() {
        let mut sys = DocSys::with_responses(vec![
            KernelResponse::Success { data: None },
            KernelResponse::Success { data: None },
            KernelResponse::Success { data: None },
            KernelResponse::ProgramSpawned {
                process_id: ThingId(30),
                thread_id: ThingId(31),
            },
            KernelResponse::Success { data: None },
        ]);

        let init_process = ProcessThing {
            id: ThingId(5),
            pid: 1,
        };
        let program = BootProgram {
            id: ThingId(6),
            name: "demo".to_string(),
            app_id: 7,
            priority: 0,
            binary: "demo_bin".to_string(),
        };
        let images = vec![ProgramImage {
            id: ThingId(7),
            identifier: "demo_bin".to_string(),
            module_index: 0,
            base_phys: 0,
            size: 0,
        }];

        spawn_boot_program(&mut sys, &init_process, &images, &program);

        let requests = sys.requests.borrow();
        assert!(requests.iter().any(|req| match req {
            KernelRequest::SpawnProgram { boot_program_id } => *boot_program_id == program.id,
            _ => false,
        }));
        assert!(requests.iter().any(|req| match req {
            KernelRequest::AddLink { src, pred, dst } => {
                *pred == graph_kinds::LINK_SPAWNED && *src == init_process.id && *dst == ThingId(30)
            }
            _ => false,
        }));
    }
}

#[cfg(feature = "rootfs")]
fn start_rootfs<S: Sys>(sys: &mut S, images: &[ProgramImage]) {
    if let Some(existing) = find_thing::<BootProgram>(sys, |bp| bp.binary == ROOTFS_IDENTIFIER) {
        log_dynamic(
            sys,
            format_args!(
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
                format_args!(
                    "init: created temporary rootfs BootProgram id={}",
                    program_id.0
                ),
            );
            let _ = spawn_program(sys, program_id);
        } else {
            println(sys, "init: failed to create BootProgram for rootfs");
        }
    } else {
        println(
            sys,
            "init: rootfs ProgramImage missing; skipping rootfs launch",
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
