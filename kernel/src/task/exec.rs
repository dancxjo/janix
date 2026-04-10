use abi::errors::{Errno, SysResult};
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;

use crate::task::ProcessInfo;
use crate::vfs::OpenFlags;
use crate::{BootRuntime, BootTasking};

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
    if size > 64 * 1024 * 1024 {
        // 64MB limit for now
        return Err(Errno::EFBIG);
    }
    let mut buffer = alloc::vec![0u8; size];
    let mut read_pos = 0;
    while read_pos < size {
        let n = node.read(read_pos as u64, &mut buffer[read_pos..])?;
        if n == 0 {
            break;
        }
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

    let (entry, stack_info, mappings, aux_info) =
        crate::task::loader::load_module::<R>(rt, new_aspace, &module_desc)
            .ok_or(Errno::ENOEXEC)?;

    // 5. Update ProcessInfo metadata (argv, env, and auxv)
    {
        let page_size = rt.page_size() as u64;
        let mut pinfo = pinfo_arc.lock();
        pinfo.argv = argv;
        pinfo.env = env;
        // Rebuild auxv from freshly loaded image.  AT_* constants follow
        // the standard ELF auxiliary-vector specification (see elf.h).
        pinfo.auxv = build_auxv(&aux_info, page_size);
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
    // TLS base for the new image starts at 0; the dummy save target is discarded.
    let mut _discard_tls: u64 = 0;
    unsafe {
        rt.tasking()
            .switch_with_tls(&mut dummy_ctx, &to_ctx, tid, &mut _discard_tls, 0);
    }

    // switch() should never return to this stack because we didn't save it into any task.ctx
    unreachable!("task_exec: switch returned unexpectedly")
}

// Standard AT_* auxiliary-vector type constants (matches Linux/SysV ABI).
const AT_PAGESZ: u64 = 6;
const AT_PHDR: u64 = 3;
const AT_PHENT: u64 = 4;
const AT_PHNUM: u64 = 5;
const AT_ENTRY: u64 = 9;

/// Build the standard auxiliary-vector entries for a freshly loaded image.
///
/// Returns a `Vec<(type, value)>` ready to store in [`ProcessInfo::auxv`].
/// Only entries with non-zero values are included (e.g. phdr info is omitted
/// for flat-binary fallback loads where `aux_info.phdr_vaddr == 0`).
pub fn build_auxv(
    aux_info: &crate::task::loader::LoaderAuxInfo,
    page_size: u64,
) -> Vec<(u64, u64)> {
    let mut v = Vec::new();
    v.push((AT_PAGESZ, page_size));
    if aux_info.phdr_vaddr != 0 {
        v.push((AT_PHDR, aux_info.phdr_vaddr));
        v.push((AT_PHENT, aux_info.phent));
        v.push((AT_PHNUM, aux_info.phnum));
    }
    if aux_info.entry_vaddr != 0 {
        v.push((AT_ENTRY, aux_info.entry_vaddr));
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::loader::LoaderAuxInfo;

    #[test]
    fn build_auxv_elf_image() {
        let info = LoaderAuxInfo {
            phdr_vaddr: 0x200040,
            phent: 56,
            phnum: 3,
            entry_vaddr: 0x201000,
        };
        let auxv = build_auxv(&info, 4096);

        // AT_PAGESZ is always present.
        assert!(auxv.contains(&(AT_PAGESZ, 4096)));
        // ELF-specific entries present when phdr_vaddr != 0.
        assert!(auxv.contains(&(AT_PHDR, 0x200040)));
        assert!(auxv.contains(&(AT_PHENT, 56)));
        assert!(auxv.contains(&(AT_PHNUM, 3)));
        // AT_ENTRY present when entry_vaddr != 0.
        assert!(auxv.contains(&(AT_ENTRY, 0x201000)));
    }

    #[test]
    fn build_auxv_flat_binary() {
        // Flat-binary fallback: phdr_vaddr and entry_vaddr are both 0.
        let info = LoaderAuxInfo::default();
        let auxv = build_auxv(&info, 4096);

        // Only AT_PAGESZ should be emitted.
        assert_eq!(auxv.len(), 1);
        assert!(auxv.contains(&(AT_PAGESZ, 4096)));
        // No ELF-specific or entry entries.
        assert!(!auxv.iter().any(|&(k, _)| k == AT_PHDR));
        assert!(!auxv.iter().any(|&(k, _)| k == AT_ENTRY));
    }

    #[test]
    fn build_auxv_no_phdr_but_has_entry() {
        // Edge case: entry known but no phdr info (should not happen in practice
        // but the function must not panic).
        let info = LoaderAuxInfo {
            phdr_vaddr: 0,
            phent: 56,
            phnum: 0,
            entry_vaddr: 0x201000,
        };
        let auxv = build_auxv(&info, 0x1000);
        assert!(auxv.contains(&(AT_PAGESZ, 0x1000)));
        assert!(auxv.contains(&(AT_ENTRY, 0x201000)));
        // phdr_vaddr == 0 → no AT_PHDR/AT_PHENT/AT_PHNUM emitted.
        assert!(!auxv.iter().any(|&(k, _)| k == AT_PHDR));
    }
}
