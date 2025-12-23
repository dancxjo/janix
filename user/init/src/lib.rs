#![no_std]

extern crate alloc;

use abi::ThingId;
use alloc::vec::Vec;
use thing_models::graph_kinds;
use thing_models::{BootProfile, BootProgram, Mode, Place, ProgramImage};
use thing_os::prelude::*;
use thing_os::{
    MODE_INDEX_CONSOLE, ProcessThing, add_link, create_process, find_thing, link_targets,
    list_things_by_kind, load_thing,
};

const SUPERVISOR_IDLE_NS: u64 = 100_000_000;

pub fn init_main() -> ! {
    println!("init: starting");

    // Direct syscall test
    // let msg = "Hello from direct syscall\n";
    // unsafe {
    //    thing_os::syscalls::syscall(abi::KernelRequest::Log { message: msg });
    // }
    // loop {}

    ensure_modes();
    // println!("init: ensure_modes done");

    let boot_profile = wait_for_boot_profile();
    println!("init: BootProfile version {}", boot_profile.version);

    let launch_ids = link_targets(boot_profile.id, graph_kinds::LINK_LAUNCHES);

    let mut programs = Vec::new();
    collect_boot_programs(&mut programs, launch_ids.as_slice());

    if programs.is_empty() {
        println!("init: no BootProgram links; waiting for boot graph to populate");
        println!("init: entering supervision loop");
        loop {
            sleep(Duration::from_nanos(SUPERVISOR_IDLE_NS));

            let refresh_ids = link_targets(boot_profile.id, graph_kinds::LINK_LAUNCHES);
            collect_boot_programs(&mut programs, refresh_ids.as_slice());
            if !programs.is_empty() {
                break;
            }
        }
    }

    if programs.is_empty() {
        println!("init: still no BootPrograms after waiting; scanning all BootProgram Things");
        programs = list_things_by_kind();
    }

    if programs.is_empty() {
        println!("init: no BootPrograms found after full scan; system will idle");
    }

    let init_process =
        find_process_by_pid(1).unwrap_or_else(|| fatal("init Process Thing (pid=1) missing"));

    let program_images: Vec<ProgramImage> = list_things_by_kind();

    let (driver_programs, app_programs): (Vec<&BootProgram>, Vec<&BootProgram>) = programs
        .iter()
        .partition(|program| is_driver(program));

    let (compositor_programs, other_app_programs): (Vec<&BootProgram>, Vec<&BootProgram>) =
        app_programs
            .iter()
            .copied()
            .partition(|program| is_compositor(program));

    // 1. Boot Manifest Audit
    validate_boot_manifest(&programs, &program_images);

    if !driver_programs.is_empty() {
        println!(
            "init: launching {} driver BootProgram(s) before user",
            driver_programs.len()
        );
    }

    // 2. Launch Drivers
    for program in driver_programs.iter().copied() {
        if program.binary == "init" {
            continue;
        }
        spawn_boot_program(&init_process, &program_images, program);
    }

    if !compositor_programs.is_empty() {
        println!(
            "init: launching compositor early before other user ({} entry/entries)",
            compositor_programs.len()
        );
    }

    // 3. Launch Compositor
    for program in compositor_programs.iter().copied() {
        spawn_boot_program(&init_process, &program_images, program);
    }

    for program in other_app_programs.iter().copied() {
        if program.binary == "init" {
            continue;
        }
        spawn_boot_program(&init_process, &program_images, program);
    }

    println!("init: entering supervision loop");
    loop {
        sleep(Duration::from_nanos(SUPERVISOR_IDLE_NS));
    }
}

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

fn validate_boot_manifest(programs: &[BootProgram], images: &[ProgramImage]) {
    use alloc::format;
    println!("init: === Boot Manifest Audit ===");
    println!(
        "init: {:<20} | {:<20} | {:<10} | {:<8}",
        "BootProgram", "Binary", "Status", "Size"
    );
    println!("init: {:-<20}-+-{:-<20}-+-{:-<10}-+-{:-<8}", "", "", "", "");

    let mut missing = 0_u32;

    for prog in programs {
        let image = images.iter().find(|img| img.identifier == prog.binary);
        let status = if image.is_some() { "OK" } else { "MISSING" };
        let size = image.map(|i| i.size).unwrap_or(0);
        let size_str = if size > 0 {
            format!("{}b", size)
        } else {
            "-".to_string()
        };

        if image.is_none() {
            missing += 1;
        }

        println!(
            "init: {:<20} | {:<20} | {:<10} | {:<8}",
            prog.name, prog.binary, status, size_str
        );
    }
    println!("init: ===========================");
    if missing > 0 {
        println!(
            "init: WARNING - {} BootProgram(s) missing ProgramImage(s)",
            missing
        );
    }
}

fn spawn_boot_program(
    init_process: &ProcessThing,
    program_images: &[ProgramImage],
    program: &BootProgram,
) {
    // Audit already checked existence, but we check again to find the image for spawning
    if let Some(_image) = program_images
        .iter()
        .find(|img| img.identifier == program.binary)
    {
        // Found
    } else {
        println!(
            "init: ERROR: Skipping spawn for '{}' - binary '{}' missing from images",
            program.name, program.binary
        );
        return;
    }

    println!("init: spawning {} (id={})", program.name, program.app_id);

    if let Some((process_id, _thread_id)) = create_process(program.id).ok() {
        if !add_link(init_process.id, graph_kinds::LINK_SPAWNED, process_id) {
            println!("init: failed to add SPAWNED link");
        }
    } else {
        println!("init: create_process failed for {}", program.name);
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
    identifier.contains("_driver")
        || identifier.contains("-driver")
        || identifier == "pci"
        || identifier == "usb"
        || identifier == "framebuffer"
}

fn load_boot_profile() -> Option<BootProfile> {
    let mut profiles: Vec<BootProfile> = list_things_by_kind();
    println!(
        "init: BootProfile query returned {} entries",
        profiles.len()
    );

    match profiles.len() {
        1 => profiles.pop(),
        0 => {
            println!("init: BootProfile not found");
            None
        }
        _ => {
            println!("init: multiple BootProfile things found");
            None
        }
    }
}

fn find_process_by_pid(pid: u64) -> Option<ProcessThing> {
    find_thing::<ProcessThing>(|p| p.pid == pid)
}

fn fatal(msg: &str) -> ! {
    println!("init fatal: {}", msg);
    loop {
        sleep(Duration::from_nanos(SUPERVISOR_IDLE_NS));
    }
}

fn collect_boot_programs(programs: &mut Vec<BootProgram>, ids: &[ThingId]) {
    for program_id in ids.iter().copied() {
        if let Some(program) = load_thing::<BootProgram>(program_id) {
            programs.push(program);
        } else {
            println!(
                "init: ignoring missing BootProgram ThingId {}",
                program_id.0
            );
        }
    }
}

fn wait_for_boot_profile() -> BootProfile {
    for _ in 0..32 {
        if let Some(profile) = load_boot_profile() {
            return profile;
        }
        sleep(Duration::from_nanos(SUPERVISOR_IDLE_NS));
    }
    fatal("BootProfile missing or duplicated after waiting");
}
