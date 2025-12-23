extern crate alloc;

use abi::{Predicate, ThingId};
use alloc::{boxed::Box, string::String, string::ToString};
use kernel::graph;
use kernel::graph_kinds;
use kernel::sched::SCHEDULER;
use kernel::symbols;
use thing_models::PropValue;

use crate::elf_loader::{self, LoadedElfProgram, ProgramImageData};

pub fn spawn_program(boot_program_id: ThingId) -> Result<(ThingId, ThingId), &'static str> {
    let info = load_boot_program_info(boot_program_id)?;

    let image = find_program_image(&info.binary)
        .ok_or("Res ProgramImage pro identificatore non inventa")?;
    let loaded = elf_loader::load_program(&image)?;
    kernel::log("ProgramImage ELF onustus, processum progenerans");
    let (proc, thread) = spawn_loaded_program(&info, loaded)?;

    // Link the new process to the BootProgram as RUNNING
    let pred_running = graph_kinds::LINK_RUNNING;
    let _ = graph::add_link(boot_program_id, pred_running, proc);

    Ok((proc, thread))
}

pub fn spawn_program_by_identifier(
    identifier: &str,
    name: &str,
    priority: u64,
) -> Result<(ThingId, ThingId), &'static str> {
    let image =
        find_program_image(identifier).ok_or("Res ProgramImage pro identificatore non inventa")?;
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
        let t = kernel::time::boot_span_start("spawn_log_format");
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
        kernel::time::boot_span_end("spawn_log_format", t);
    }
    let t_spawn = kernel::time::boot_span_start("spawn_process_setup");
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

        if let Some(thread) = sched.thread_mut(tid) {
            #[cfg(target_arch = "x86_64")]
            {
                use arch::gdt;
                // ... (Context initialization same as before)
                let selectors = gdt::get_selectors();
                let cs = selectors.ucode.0 as u64 | 3;
                let ss = selectors.udata.0 as u64 | 3;

                thread.context[13] = 0;
                thread.context[15] = loaded.entry_point;
                thread.context[16] = cs;
                thread.context[17] = 0x202;
                thread.context[18] = loaded.user_stack_top;
                thread.context[19] = ss;
            }
        }

        let process_thing = sched
            .process_thing_id(pid)
            .ok_or("Res Process non relata")?;
        let thread_thing = sched.thread_thing_id(tid).ok_or("Res Thread non relata")?;
        (process_thing, thread_thing)
    };
    kernel::time::boot_span_end("spawn_process_setup", t_spawn);
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
    respawn_policy: String,
}

fn load_boot_program_info(id: ThingId) -> Result<BootProgramInfo, &'static str> {
    graph::with_thing(id, |thing| {
        let kind_boot_prog = symbols::intern(graph_kinds::KIND_BOOT_PROGRAM);
        if thing.kind != kind_boot_prog {
            return Err("Res SpawnProgram BootProgram non fuit");
        }

        let mut name: Option<String> = None;
        let mut app_id: Option<u64> = None;
        let mut priority: u64 = 0;
        let mut binary: Option<String> = None;
        let mut respawn_policy: Option<String> = None;

        let prop_name = symbols::intern("name");
        let prop_app_id = symbols::intern("app_id");
        let prop_priority = symbols::intern("priority");
        let prop_binary = symbols::intern("binary");
        let prop_respawn = symbols::intern(graph_kinds::PROP_RESPAWN_POLICY);

        for prop in thing.props.iter() {
            if prop.0 == prop_name {
                if let PropValue::Str(s) = &prop.1 {
                    name = Some(s.clone());
                }
            } else if prop.0 == prop_app_id {
                if let PropValue::U64(v) = prop.1 {
                    app_id = Some(v);
                }
            } else if prop.0 == prop_priority {
                if let PropValue::U64(v) = prop.1 {
                    priority = v;
                }
            } else if prop.0 == prop_binary {
                if let PropValue::Str(s) = &prop.1 {
                    binary = Some(s.clone());
                }
            } else if prop.0 == prop_respawn {
                if let PropValue::Str(s) = &prop.1 {
                    respawn_policy = Some(s.clone());
                }
            }
        }

        Ok(BootProgramInfo {
            name: name.ok_or("BootProgram caret nomine")?,
            app_id: app_id.ok_or("BootProgram caret app_id")?,
            priority,
            binary: binary.unwrap_or_default(),
            respawn_policy: respawn_policy
                .unwrap_or_else(|| String::from(graph_kinds::RESPAWN_NEVER)),
        })
    })
    .unwrap_or(Err("Res BootProgram non inventa"))
}

fn find_program_image(identifier: &str) -> Option<ProgramImageData> {
    // Iterate things logic
    // We can't use graph::iter_things easily if it iterates internal store (which is locked).
    // graph::iter_things was a helper?
    // Let's assume store supports iterating.
    // OR we use next_thing_of_kind loop again.

    let kind_prog_img = symbols::intern(graph_kinds::KIND_PROGRAM_IMAGE);
    let mut current = ThingId(0);

    // Pre-intern keys
    let prop_ident = symbols::intern(graph_kinds::PROP_IDENTIFIER);
    let prop_idx = symbols::intern(graph_kinds::PROP_MODULE_INDEX);
    let prop_base = symbols::intern(graph_kinds::PROP_BASE_PHYS);
    let prop_size = symbols::intern(graph_kinds::PROP_SIZE);

    while let Some(next) = graph::next_thing_of_kind_sym(kind_prog_img, current) {
        let mut found_ident: Option<String> = None;
        let mut module_index = 0;
        let mut base_phys = 0;
        let mut size = 0;

        // Access properties via get_prop or with_thing
        let match_found = graph::with_thing(next, |thing| {
            for prop in thing.props.iter() {
                if prop.0 == prop_ident {
                    if let PropValue::Str(s) = &prop.1 {
                        found_ident = Some(s.clone());
                    }
                } else if prop.0 == prop_idx {
                    if let PropValue::U64(v) = prop.1 {
                        module_index = v;
                    }
                } else if prop.0 == prop_base {
                    if let PropValue::U64(v) = prop.1 {
                        base_phys = v;
                    }
                } else if prop.0 == prop_size {
                    if let PropValue::U64(v) = prop.1 {
                        size = v;
                    }
                }
            }
            if let Some(id) = &found_ident {
                if id == identifier {
                    return Some(ProgramImageData {
                        identifier: id.clone(),
                        module_index,
                        base_phys,
                        size,
                    });
                }
            }
            None
        })
        .flatten();

        if let Some(data) = match_found {
            return Some(data);
        }

        current = next;
    }
    None
}
