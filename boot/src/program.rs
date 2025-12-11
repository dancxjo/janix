extern crate alloc;

use abi::{PropValue, ThingId};
use alloc::{boxed::Box, string::String, string::ToString};
use kernel::graph;
use kernel::graph_kinds;
use kernel::sched::SCHEDULER;

use crate::elf_loader::{self, LoadedElfProgram, ProgramImageData};

pub fn spawn_program(boot_program_id: ThingId) -> Result<(ThingId, ThingId), &'static str> {
    let info = load_boot_program_info(boot_program_id)?;

    let image =
        find_program_image(&info.binary).ok_or("ProgramImage Thing not found for identifier")?;
    let loaded = elf_loader::load_program(&image)?;
    kernel::log("Loaded ELF ProgramImage, spawning process");
    spawn_loaded_program(&info, loaded)
}

pub fn spawn_program_by_identifier(
    identifier: &str,
    name: &str,
    priority: u64,
) -> Result<(ThingId, ThingId), &'static str> {
    let image =
        find_program_image(identifier).ok_or("ProgramImage Thing not found for identifier")?;
    let loaded = elf_loader::load_program(&image)?;
    spawn_loaded_program_named(name, priority, loaded)
}

fn spawn_loaded_program(
    info: &BootProgramInfo,
    loaded: LoadedElfProgram,
) -> Result<(ThingId, ThingId), &'static str> {
    spawn_loaded_program_named(&info.name, info.priority, loaded)
}

fn spawn_loaded_program_named(
    name: &str,
    priority: u64,
    loaded: LoadedElfProgram,
) -> Result<(ThingId, ThingId), &'static str> {
    {
        let msg = alloc::format!(
            "spawn_loaded_program_named: name={} entry={:#x} stack_top={:#x} cr3={:#x} heap=[{:#x},{:#x})",
            name,
            loaded.entry_point,
            loaded.user_stack_top,
            loaded.address_space_token,
            loaded.heap_base,
            loaded.heap_limit
        );
        kernel::log(Box::leak(msg.into_boxed_str()));
    }
    let leaked_name: &'static str = leak_name(name);
    let (process_thing, thread_thing) = {
        let mut sched = SCHEDULER.lock();
        let pid = sched.add_process(leaked_name);
        sched.set_process_address_space(pid, loaded.address_space_token);
        sched.set_process_heap(pid, loaded.heap_base as usize, loaded.heap_limit as usize);
        let tid = sched.add_thread_with_entry_point(
            pid,
            leaked_name,
            loaded.entry_point,
            0,
            loaded.user_stack_top,
            priority,
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
    let (kind, props) = kernel::graph::get_thing(id).ok_or("BootProgram Thing not found")?;
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
