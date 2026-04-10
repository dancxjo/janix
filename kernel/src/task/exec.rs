use abi::errors::{Errno, SysResult};
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;

use crate::{BootRuntime, BootTasking};
use crate::task::ProcessInfo;
use crate::vfs::OpenFlags;

/// Core logic for in-place process image replacement.
/// 
/// This implementation follows the Janix-style "fd-first" design:
/// 1. Validate that the process is single-threaded.
/// 2. Read the executable from the given FD.
/// 3. Prepare a new address space and load the ELF.
/// 4. Setup a new user stack with argv/env.
/// 5. Atomically swap the image and return to userspace.
pub fn task_exec_current<R: BootRuntime>(
    fd: u32,
    argv: Vec<Vec<u8>>,
    env: BTreeMap<Vec<u8>, Vec<u8>>,
) -> SysResult<()> {
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOSYS)?;
    let pid = pinfo_arc.lock().pid;

    // 1. Ensure single-threaded
    {
        let registry = crate::task::registry::get_registry::<R>();
        let mut thread_count = 0;
        for task in &registry.tasks {
            if let Some(ref task_pinfo) = task.process_info {
                if task_pinfo.lock().pid == pid {
                    thread_count += 1;
                }
            }
        }
        if thread_count > 1 {
            return Err(Errno::EBUSY);
        }
    }

    // 2. Resolve executable from FD
    let node = {
        let pinfo = pinfo_arc.lock();
        let open_file = pinfo.fd_table.get(fd)?;
        // Simple validation: must be a regular file or something we can read as an ELF
        let stat = open_file.node.stat()?;
        if !stat.is_reg() {
            return Err(Errno::EACCES); // Or EISDIR etc
        }
        open_file.node.clone()
    };

    // 3. Read the entire file into kernel memory (v1)
    let stat = node.stat()?;
    let size = stat.size as usize;
    if size > 64 * 1024 * 1024 { // 64MB limit for now
        return Err(Errno::EFBIG);
    }
    let mut buffer = alloc::vec![0u8; size];
    let mut read_pos = 0;
    while read_pos < size {
        let n = node.read(read_pos as u64, &mut buffer[read_pos..])?;
        if n == 0 { break; }
        read_pos += n;
    }
    if read_pos < size {
        return Err(Errno::EIO);
    }

    // 4. Load ELF into a new address space
    let rt = crate::runtime::<R>();
    let new_aspace = rt.tasking().make_user_address_space();
    
    // We need a BootModuleDesc for load_module
    // SAFETY: load_module is synchronous and does not store the reference.
    let static_bytes: &'static [u8] = unsafe { core::mem::transmute(&buffer as &[u8]) };
    let module_desc = crate::BootModuleDesc {
        name: "exec_image",
        cmdline: "",
        bytes: static_bytes,
        phys_start: 0,
        phys_end: 0,
        kind: crate::BootModuleKind::Elf,
    };
    
    let (entry, stack_info, mappings) = crate::task::loader::load_module::<R>(
        rt,
        new_aspace,
        &module_desc,
    ).ok_or(Errno::ENOEXEC)?;

    // 5. Update ProcessInfo metadata
    {
        let mut pinfo = pinfo_arc.lock();
        pinfo.argv = argv;
        pinfo.env = env;
    }

    // 6. Finalize the new task state
    // We need to replace the current task's aspace, mappings, and context.
    // This is the "Commit Point".
    
    let tid = unsafe { crate::sched::current_tid_current() };
    
    // Copy the context out so we can switch to it after dropping the registry lock
    let (to_ctx, new_aspace_actual) = {
        let mut task_mut = crate::task::registry::get_task_mut::<R>(tid).ok_or(Errno::ESRCH)?;
        
        // Replace address space
        task_mut.aspace = new_aspace;
        
        // Replace mappings
        {
            let mut mlock = task_mut.mappings.lock();
            *mlock = crate::memory::mappings::MappingList::new();
            for m in mappings {
                mlock.insert(m);
            }
        }
        
        // Replace stack info
        task_mut.stack_info = Some(stack_info);
        
        // Initialize new user context
        let spec = crate::UserTaskSpec {
            entry: entry.entry_pc as u64,
            stack_top: entry.user_sp as u64,
            aspace: new_aspace,
            arg: 0, // Not used by stem since it uses SYS_ARGV_GET
        };
        task_mut.ctx = rt.tasking().init_user_context(spec, task_mut.kstack_top);
        
        (task_mut.ctx, task_mut.aspace)
    }; // registry lock (TaskMut) dropped here

    // 7. Perform the actual transition
    // We must never return to the old state.
    rt.tasking().activate_address_space(new_aspace_actual);
    
    let mut dummy_ctx = Default::default();
    unsafe {
        rt.tasking().switch(&mut dummy_ctx, &to_ctx, tid);
    }

    // switch() should never return to this stack because we didn't save it into any task.ctx
    unreachable!("task_exec: switch returned unexpectedly")
}
