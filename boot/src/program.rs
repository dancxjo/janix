extern crate alloc;

use alloc::{boxed::Box, string::String, string::ToString};
use abi::{PropValue, ThingId};
use kernel_core::graph;
use kernel_core::graph_kinds;
use kernel_core::sched::SCHEDULER;

use crate::elf_loader::{self, LoadedElfProgram, ProgramImageData};
use crate::user;

pub fn spawn_program(boot_program_id: ThingId) -> Result<(ThingId, ThingId), &'static str> {
    let info = load_boot_program_info(boot_program_id)?;

    if let Some(image) = find_program_image(&info.binary) {
        match elf_loader::load_program(&image) {
            Ok(loaded) => {
                kernel_core::log("Loaded ELF ProgramImage, spawning process");
                return spawn_loaded_program(&info, loaded);
            }
            Err(err) => {
                kernel_core::log("ELF load failed; falling back to compat path");
                kernel_core::log(err);
            }
        }
    } else {
        kernel_core::log("No ProgramImage found; falling back to compat path");
    }

    spawn_compat_program(&info)
}

fn spawn_loaded_program(
    info: &BootProgramInfo,
    loaded: LoadedElfProgram,
) -> Result<(ThingId, ThingId), &'static str> {
    let leaked_name: &'static str = leak_name(&info.name);
    let (process_thing, thread_thing) = {
        let mut sched = SCHEDULER.lock();
        let pid = sched.add_process(leaked_name);
        sched.set_process_address_space(pid, loaded.address_space_token);
        sched.set_process_heap(
            pid,
            loaded.heap_base as usize,
            loaded.heap_limit as usize,
        );
        let tid = sched.add_thread_with_entry_point(
            pid,
            leaked_name,
            loaded.entry_point,
            0,
            loaded.user_stack_top,
            info.priority,
        );
        let process_thing = sched
            .process_thing_id(pid)
            .ok_or("Process Thing not recorded")?;
        let thread_thing = sched
            .thread_thing_id(tid)
            .ok_or("Thread Thing not recorded")?;
        (process_thing, thread_thing)
    };
    Ok((process_thing, thread_thing))
}

fn spawn_compat_program(info: &BootProgramInfo) -> Result<(ThingId, ThingId), &'static str> {
    let leaked_name: &'static str = leak_name(&info.name);
    let stack = user::alloc_user_stack();
    let (process_thing, thread_thing) = {
        let mut sched = SCHEDULER.lock();
        let pid = sched.add_process(leaked_name);
        let tid = sched.add_thread(
            pid,
            leaked_name,
            user::user_thread_main,
            info.app_id,
            stack,
            info.priority,
        );
        let process_thing = sched
            .process_thing_id(pid)
            .ok_or("Process Thing not recorded")?;
        let thread_thing = sched
            .thread_thing_id(tid)
            .ok_or("Thread Thing not recorded")?;
        (process_thing, thread_thing)
    };
    Ok((process_thing, thread_thing))
}

fn leak_name(name: &str) -> &'static str {
    Box::leak(name.to_string().into_boxed_str())
}

struct BootProgramInfo {
    name: String,
    app_id: u64,
    priority: u64,
    binary: String,
}

fn load_boot_program_info(id: ThingId) -> Result<BootProgramInfo, &'static str> {
    let (kind, props) = kernel_core::graph::get_thing(id).ok_or("BootProgram Thing not found")?;
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

    Ok(BootProgramInfo {
        name: name.ok_or("BootProgram missing name")?,
        app_id: app_id.ok_or("BootProgram missing app_id")?,
        priority,
        binary: binary.unwrap_or_default(),
    })
}

fn find_program_image(identifier: &str) -> Option<ProgramImageData> {
    let mut found = None;
    graph::iter_things(|thing| {
        if found.is_some() || thing.kind != graph_kinds::KIND_PROGRAM_IMAGE {
            return;
        }
        let mut id_value: Option<String> = None;
        let mut module_index = 0;
        let mut base_phys = 0;
        let mut size = 0;
        for prop in thing.props.iter().flatten() {
            match prop.0 {
                "identifier" => {
                    if let PropValue::Str(s) = &prop.1 {
                        id_value = Some(s.clone());
                    }
                }
                "module_index" => {
                    if let PropValue::U64(v) = prop.1 {
                        module_index = v;
                    }
                }
                "base_phys" => {
                    if let PropValue::U64(v) = prop.1 {
                        base_phys = v;
                    }
                }
                "size" => {
                    if let PropValue::U64(v) = prop.1 {
                        size = v;
                    }
                }
                _ => {}
            }
        }
        if let Some(id) = id_value {
            if id == identifier {
                found = Some(ProgramImageData {
                    identifier: id,
                    module_index,
                    base_phys,
                    size,
                });
            }
        }
    });
    found
}
