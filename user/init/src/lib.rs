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
    load_thing, create_process,
};

const SUPERVISOR_IDLE_NS: u64 = 100_000_000;

#[cfg(feature = "rootfs")]
const ROOTFS_IDENTIFIER: &str = "rootfs";

pub fn init_main() -> ! {
    println!("init: starting");
    
    // Direct syscall test
    // let msg = "Hello from direct syscall\n";
    // unsafe {
    //    thing_os::syscalls::syscall(abi::KernelRequest::Log { message: msg });
    // }
    // loop {}

    ensure_modes();
    println!("init: ensure_modes done");

    #[cfg(feature = "rootfs")]
    {
        let program_images: Vec<ProgramImage> = list_things_by_kind();
        start_rootfs(&program_images);
    }

    let boot_profile = wait_for_boot_profile();
    println!("init: BootProfile version {}", boot_profile.version);

    let launch_ids = link_targets(boot_profile.id, graph_kinds::LINK_LAUNCHES);

    let mut programs = Vec::new();
    collect_boot_programs(&mut programs, launch_ids.as_slice());

    if programs.is_empty() {
        println!("init: no BootProgram links; waiting briefly for rootfs");
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

    let init_process = find_process_by_pid(1)
        .unwrap_or_else(|| fatal("init Process Thing (pid=1) missing"));

    let program_images: Vec<ProgramImage> = list_things_by_kind();

    for _ in programs.iter().filter(|program| is_rootfs(program)) {
        println!("init: skipping rootfs BootProgram entry (already handled)");
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
        println!("init: launching {} driver BootProgram(s) before user", driver_programs.len());
    }

    for program in driver_programs.iter().copied() {
        println!("init: checking program binary='{}'", program.binary);
        if program.binary == "init" {
            continue;
        }
        if program.binary == "ps2_keyboard_driver" {
            println!("init: temporarily skipping ps2_keyboard_driver for conflict test");
            continue;
        }
        if program.binary == "ps2_keyboard_driver" {
            println!("init: temporarily skipping ps2_keyboard_driver for conflict test");
            continue;
        }
        spawn_boot_program(&init_process, &program_images, program);
    }

    if !compositor_programs.is_empty() {
        println!("init: launching compositor early before other user ({} entry/entries)", compositor_programs.len());
    }

    for program in compositor_programs.iter().copied() {
        spawn_boot_program(&init_process, &program_images, program);
    }

    /*
    for program in other_app_programs.iter().copied() {
        println!("init: checking program binary='{}'", program.binary);
        if program.binary == "init" {
            continue;
        }

        // DEBUG: Force disable window_demo for freeze debugging
        if program.binary == "window_demo" {
             println!("init: SKIPPING window_demo (debug disable)");
             continue;
        }
        spawn_boot_program(&init_process, &program_images, program);
    }
    */

    println!("init: entering supervision loop");
    loop {
        sleep(Duration::from_nanos(SUPERVISOR_IDLE_NS));
    }
}

fn ensure_modes() {
    let _ = register_schema_for::<Mode>();
    let _ = register_schema_for::<Place>();

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

fn spawn_boot_program(
    init_process: &ProcessThing,
    program_images: &[ProgramImage],
    program: &BootProgram,
) {
    println!(
        "init: BootProgram name={} app_id={} priority={} binary={}",
        program.name, program.app_id, program.priority, program.binary
    );
    if let Some(image) = program_images
        .iter()
        .find(|img| img.identifier == program.binary)
    {
        println!(
            "init: BootProgram {} backed by ProgramImage id={} module_index={} base_phys={:#x} size={}",
            program.name, image.identifier, image.module_index, image.base_phys, image.size
        );
    } else {
        println!(
            "init: WARNING: no ProgramImage found for BootProgram {} (binary={})",
            program.name, program.binary
        );
    }
    println!(
        "init: spawning BootProgram {} (app_id={}, binary={})",
        program.name, program.app_id, program.binary
    );
    if let Some((process_id, _thread_id)) = create_process(program.id).ok() {
        if !add_link(init_process.id, graph_kinds::LINK_SPAWNED, process_id) {
            println!("init: failed to add SPAWNED link after create_process");
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
    identifier.contains("_driver") || identifier.contains("-driver") || identifier == "pci" || identifier == "usb" || identifier == "framebuffer"
}

fn load_boot_profile() -> Option<BootProfile> {
    let mut profiles: Vec<BootProfile> = list_things_by_kind();
    println!("init: BootProfile query returned {} entries", profiles.len());

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
            println!("init: ignoring missing BootProgram ThingId {}", program_id.0);
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

#[cfg(feature = "rootfs")]
fn start_rootfs(images: &[ProgramImage]) {
    if let Some(existing) = find_thing::<BootProgram>(|bp| bp.binary == ROOTFS_IDENTIFIER) {
        println!("init: rootfs BootProgram already exists as ThingId {}", existing.id.0);
        let _ = create_process(existing.id);
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
        if let Some(program_id) = create_thing(&temp_program) {
            println!("init: created temporary rootfs BootProgram id={}", program_id.0);
            let _ = create_process(program_id);
        } else {
            println!("init: failed to create BootProgram for rootfs");
        }
    } else {
        println!("init: rootfs ProgramImage missing; skipping rootfs launch");
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
