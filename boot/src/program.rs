extern crate alloc;

use abi::{Predicate, ThingId};
use alloc::{boxed::Box, string::String, string::ToString};
use kernel::graph;
use kernel::graph_kinds;
use kernel::sched::SCHEDULER;
use kernel::symbols;
use thing_models::{Content, Module, PropValue, SharedBuffer};
use kernel::Thing;

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
        find_program_image(identifier).ok_or("Res Module pro identificatore non inventa")?;
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
        let kind_module = symbols::intern(Module::KIND);

        if thing.kind == kind_boot_prog {
            // ... existing BootProgram logic ...
            let mut name: Option<String> = None;
            let mut app_id: Option<u64> = None;
            let mut priority: u64 = 0;
            let mut binary: Option<String> = None;
            let mut respawn_policy: Option<String> = None;

            let prop_name = symbols::intern(graph_kinds::PROP_NAME);
            let prop_app_id = symbols::intern("app_id");
            let prop_priority = symbols::intern("priority");
            let prop_binary = symbols::intern("binary");
            let prop_respawn = symbols::intern(graph_kinds::PROP_RESPAWN_POLICY);

            for prop in thing.props.iter() {
                if prop.0 == prop_name {
                    if let PropValue::Str(s) = &prop.1 { name = Some(s.clone()); }
                } else if prop.0 == prop_app_id {
                    if let PropValue::U64(v) = prop.1 { app_id = Some(v); }
                } else if prop.0 == prop_priority {
                    if let PropValue::U64(v) = prop.1 { priority = v; }
                } else if prop.0 == prop_binary {
                    if let PropValue::Str(s) = &prop.1 { binary = Some(s.clone()); }
                } else if prop.0 == prop_respawn {
                    if let PropValue::Str(s) = &prop.1 { respawn_policy = Some(s.clone()); }
                }
            }

            Ok(BootProgramInfo {
                name: name.ok_or("BootProgram caret nomine")?,
                app_id: app_id.unwrap_or(0),
                priority,
                binary: binary.unwrap_or_default(),
                respawn_policy: respawn_policy.unwrap_or_else(|| String::from(graph_kinds::RESPAWN_NEVER)),
            })
        } else if thing.kind == kind_module {
            // Adapt Module to BootProgramInfo
             let mut name: Option<String> = None;
             let prop_name = symbols::intern("name");
             for prop in thing.props.iter() {
                 if prop.0 == prop_name {
                     if let PropValue::Str(s) = &prop.1 { name = Some(s.clone()); }
                 }
             }
             let name = name.ok_or("Module caret nomine")?;
             Ok(BootProgramInfo {
                 name: name.clone(),
                 app_id: 0, // Generated or unused?
                 priority: 0,
                 binary: name, // Binary is identifier for Module lookup
                 respawn_policy: String::from(graph_kinds::RESPAWN_NEVER),
             })
        } else {
            Err("Res id non est BootProgram vel Module")
        }
    }).unwrap_or(Err("Res non inventa"))
}

fn find_program_image(identifier: &str) -> Option<ProgramImageData> {
    // New logic: Find Module by name, get Content, get SharedBuffer
    let kind_module = symbols::intern(Module::KIND);
    let mut current = ThingId(0);

    let prop_name = symbols::intern("name"); // Module.name

    while let Some(next) = graph::next_thing_of_kind_sym(kind_module, current) {
        
        let mut found_name: Option<String> = None;
        let mut content_id: Option<ThingId> = None;

        graph::with_thing(next, |thing| {
            for prop in thing.props.iter() {
                if prop.0 == prop_name {
                    if let PropValue::Str(s) = &prop.1 {
                        found_name = Some(s.clone());
                    }
                }
            }
            // Check link to Content directly to avoid deadlock (graph::link_target_at tries to lock again)
            // graph_kinds::LINK_HAS_CONTENT is a Predicate
            if let Some((_, c_id)) = thing.links.iter().find(|(p, _)| *p == graph_kinds::LINK_HAS_CONTENT) {
                 content_id = Some(*c_id);
            }
        });

        if let Some(ref name) = found_name {
             if name == identifier {
                 // Found the module, now get buffer params
                 if let Some(c_id) = content_id {
                     return get_buffer_data(c_id, identifier);
                 }
             }
        }
        current = next;
    }


    None
}

fn get_buffer_data(content_id: ThingId, identifier: &str) -> Option<ProgramImageData> {
     // Get buffer_id from Content
    let prop_buf = symbols::intern(graph_kinds::PROP_BUFFER_ID); // buffer_id
    let prop_len = symbols::intern("len");
    let mut buffer_id = 0u64;
    let mut content_len = 0u64;

    let found = graph::with_thing(content_id, |thing| {
        for prop in thing.props.iter() {
            if prop.0 == prop_buf {
                if let PropValue::U64(v) = prop.1 {
                    buffer_id = v;
                }
            } else if prop.0 == prop_len {
                if let PropValue::U64(v) = prop.1 {
                    content_len = v;
                }
            }
        }
        buffer_id != 0
    }).unwrap_or(false);

    if !found { return None; }

    // Look up SharedBuffer
    let manager = kernel::shared_buffer::manager().lock();
    if let Some(sb) = manager.get(&ThingId(buffer_id)) {
        // Verify contiguous
        if sb.frames.is_empty() { return None; }
        let start = sb.frames[0].start_address;
        
        // Use Content.len if available, otherwise fallback to buffer metadata matches
        let size = if content_len > 0 {
            content_len
        } else {
            sb.size_bytes()
        };
          
          // Simple contiguous check: expected end = start + ptr_range
          // Actually, SharedBuffer frames are PhysFrame.
          // We assume they are contiguous for program loading.
          // We can check:
          let mut addr = start;
          for frame in &sb.frames {
               if frame.start_address != addr {
                   kernel::log("SharedBuffer non-contiguous, cannot load ELF");
                   return None;
               }
               addr += 4096;
          }
          
          Some(ProgramImageData {
              identifier: String::from(identifier),
              module_index: 0,
              base_phys: start,
              size,
          })
     } else {
         None
     }
}
