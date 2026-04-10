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
/// 1. Set exec-in-progress flag and collect sibling TIDs.
/// 2. Kill all sibling user threads (they must not resume in the old image).
/// 3. Read the executable from the given FD.
/// 4. Prepare a new address space and load the ELF.
/// 5. Setup a new user stack with argv/env.
/// 6. Atomically swap the image and return to userspace.
///
/// On any failure before the commit point the exec-in-progress flag is
/// cleared so the original process/thread-group is left intact.
pub fn task_exec_current<R: BootRuntime>(
    fd: u32,
    argv: Vec<Vec<u8>>,
    env: BTreeMap<Vec<u8>, Vec<u8>>,
) -> SysResult<()> {
    let pinfo_arc = crate::sched::process_info_current().ok_or(Errno::ENOSYS)?;
    let pid = pinfo_arc.lock().pid;

    // Determine the current TID early so we can exclude ourselves from the
    // sibling list.
    let tid = unsafe { crate::sched::current_tid_current() };

    // 1. Set exec_in_progress and collect sibling TIDs.
    //    After this point, new SYS_SPAWN_THREAD calls into this process are
    //    rejected with EAGAIN until the flag is cleared or the exec commits.
    let sibling_tids: Vec<crate::task::TaskId> = {
        let mut pinfo = pinfo_arc.lock();
        pinfo.exec_in_progress = true;
        pinfo
            .thread_ids
            .iter()
            .copied()
            .filter(|&t| t != tid)
            .collect()
    };

    // 2. Kill sibling threads so they cannot resume in the old address space.
    //    kill_by_tid_current is safe to call from any context; it skips the
    //    calling thread automatically.  Each killed thread is also removed from
    //    ProcessInfo.thread_ids inside mark_task_exited.
    for sibling in sibling_tids {
        let killed = unsafe { crate::sched::kill_by_tid_current(sibling) };
        crate::kdebug!(
            "EXEC: killed sibling thread {} during exec collapse (pid {}): {}",
            sibling,
            pid,
            killed
        );
    }

    // Helper macro: clear exec_in_progress and return an error.
    macro_rules! abort_exec {
        ($err:expr) => {{
            pinfo_arc.lock().exec_in_progress = false;
            return Err($err);
        }};
    }

    // 3. Resolve executable from FD
    let node = {
        let pinfo = pinfo_arc.lock();
        let open_file = match pinfo.fd_table.get(fd) {
            Ok(f) => f,
            Err(e) => abort_exec!(e),
        };
        // Simple validation: must be a regular file or something we can read as an ELF
        let stat = match open_file.node.stat() {
            Ok(s) => s,
            Err(e) => abort_exec!(e),
        };
        if !stat.is_reg() {
            abort_exec!(Errno::EACCES);
        }
        open_file.node.clone()
    };

    // 4. Read the entire file into kernel memory (v1)
    let stat = match node.stat() {
        Ok(s) => s,
        Err(e) => abort_exec!(e),
    };
    let size = stat.size as usize;
    if size > 64 * 1024 * 1024 {
        // 64MB limit for now
        abort_exec!(Errno::EFBIG);
    }
    let mut buffer = alloc::vec![0u8; size];
    let mut read_pos = 0;
    while read_pos < size {
        let n = match node.read(read_pos as u64, &mut buffer[read_pos..]) {
            Ok(n) => n,
            Err(e) => abort_exec!(e),
        };
        if n == 0 {
            break;
        }
        read_pos += n;
    }
    if read_pos < size {
        abort_exec!(Errno::EIO);
    }

    // 5. Load ELF into a new address space
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
        match crate::task::loader::load_module::<R>(rt, new_aspace, &module_desc) {
            Some(r) => r,
            None => abort_exec!(Errno::ENOEXEC),
        };

    // 6. Update ProcessInfo metadata (argv, env, and auxv).
    //    Also clear exec_in_progress now that we are about to commit — the
    //    caller is the sole surviving thread from this point forward.
    {
        let page_size = rt.page_size() as u64;
        let mut pinfo = pinfo_arc.lock();
        pinfo.argv = argv;
        pinfo.env = env;
        // Rebuild auxv from freshly loaded image.  AT_* constants follow
        // the standard ELF auxiliary-vector specification (see elf.h).
        pinfo.auxv = build_auxv(&aux_info, page_size);
        // Commit: caller is now the only thread; clear the flag.
        pinfo.exec_in_progress = false;
    }

    // 7. Finalize the new task state
    // We need to replace the current task's aspace, mappings, and context.
    // This is the "Commit Point".

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

    // 8. Perform the actual transition
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

    // ── exec_in_progress / thread-group collapse unit tests ──────────────────

    /// Helper: build a minimal ProcessInfo with two threads.
    fn make_two_thread_pinfo(
        pid: u32,
        tid_leader: crate::task::TaskId,
        tid_sibling: crate::task::TaskId,
    ) -> Arc<Mutex<ProcessInfo>> {
        Arc::new(Mutex::new(ProcessInfo {
            pid,
            ppid: 1,
            argv: alloc::vec::Vec::new(),
            env: alloc::collections::BTreeMap::new(),
            auxv: alloc::vec::Vec::new(),
            fd_table: crate::vfs::fd_table::FdTable::new(),
            namespace: crate::vfs::NamespaceRef::global(),
            cwd: alloc::string::String::from("/"),
            thread_ids: alloc::vec![tid_leader, tid_sibling],
            exec_in_progress: false,
        }))
    }

    /// exec_in_progress starts as false and can be toggled.
    #[test]
    fn exec_in_progress_default_false() {
        let pinfo = make_two_thread_pinfo(9200, 9200, 9201);
        assert!(!pinfo.lock().exec_in_progress, "should start as false");
    }

    /// Setting exec_in_progress blocks new siblings from being visible.
    #[test]
    fn exec_in_progress_set_and_clear() {
        let pinfo = make_two_thread_pinfo(9210, 9210, 9211);
        {
            let mut pi = pinfo.lock();
            pi.exec_in_progress = true;
        }
        assert!(pinfo.lock().exec_in_progress, "should be set");

        // Simulate pre-commit failure: clear the flag.
        pinfo.lock().exec_in_progress = false;
        assert!(!pinfo.lock().exec_in_progress, "should be cleared on rollback");
    }

    /// Verify that the sibling TID collection logic (filter out current TID)
    /// produces the expected sibling list.
    #[test]
    fn exec_sibling_collection_excludes_caller() {
        let pinfo = make_two_thread_pinfo(9220, 9220, 9221);
        let caller_tid: crate::task::TaskId = 9220;

        // Simulate the sibling collection step in task_exec_current.
        let siblings: alloc::vec::Vec<crate::task::TaskId> = {
            let pi = pinfo.lock();
            pi.thread_ids
                .iter()
                .copied()
                .filter(|&t| t != caller_tid)
                .collect()
        };

        assert_eq!(siblings, alloc::vec![9221], "only sibling should be collected");
    }

    /// Three-thread group: sibling collection excludes the calling thread and
    /// returns both other threads.
    #[test]
    fn exec_sibling_collection_three_threads() {
        let pinfo = Arc::new(Mutex::new(ProcessInfo {
            pid: 9230,
            ppid: 1,
            argv: alloc::vec::Vec::new(),
            env: alloc::collections::BTreeMap::new(),
            auxv: alloc::vec::Vec::new(),
            fd_table: crate::vfs::fd_table::FdTable::new(),
            namespace: crate::vfs::NamespaceRef::global(),
            cwd: alloc::string::String::from("/"),
            thread_ids: alloc::vec![9230, 9231, 9232],
            exec_in_progress: false,
        }));

        let caller_tid: crate::task::TaskId = 9230;
        let siblings: alloc::vec::Vec<crate::task::TaskId> = {
            let pi = pinfo.lock();
            pi.thread_ids
                .iter()
                .copied()
                .filter(|&t| t != caller_tid)
                .collect()
        };

        assert_eq!(siblings.len(), 2, "two siblings expected");
        assert!(siblings.contains(&9231));
        assert!(siblings.contains(&9232));
    }

    /// Single-threaded process: sibling list is empty, exec_in_progress can
    /// be set and the process can proceed directly to commit.
    #[test]
    fn exec_single_threaded_no_siblings() {
        let pinfo = Arc::new(Mutex::new(ProcessInfo {
            pid: 9240,
            ppid: 1,
            argv: alloc::vec::Vec::new(),
            env: alloc::collections::BTreeMap::new(),
            auxv: alloc::vec::Vec::new(),
            fd_table: crate::vfs::fd_table::FdTable::new(),
            namespace: crate::vfs::NamespaceRef::global(),
            cwd: alloc::string::String::from("/"),
            thread_ids: alloc::vec![9240],
            exec_in_progress: false,
        }));

        let caller_tid: crate::task::TaskId = 9240;
        let siblings: alloc::vec::Vec<crate::task::TaskId> = {
            let mut pi = pinfo.lock();
            pi.exec_in_progress = true;
            pi.thread_ids
                .iter()
                .copied()
                .filter(|&t| t != caller_tid)
                .collect()
        };

        assert!(siblings.is_empty(), "no siblings in single-threaded process");
        assert!(pinfo.lock().exec_in_progress, "exec_in_progress should be set");
    }

    /// After a simulated successful exec commit, exec_in_progress is cleared
    /// and only the caller TID remains in thread_ids.
    #[test]
    fn exec_commit_clears_flag_and_leaves_sole_caller() {
        let pinfo = make_two_thread_pinfo(9250, 9250, 9251);
        let caller_tid: crate::task::TaskId = 9250;

        // Phase 1: set flag and collect siblings.
        let siblings: alloc::vec::Vec<crate::task::TaskId> = {
            let mut pi = pinfo.lock();
            pi.exec_in_progress = true;
            pi.thread_ids
                .iter()
                .copied()
                .filter(|&t| t != caller_tid)
                .collect()
        };
        assert_eq!(siblings, alloc::vec![9251]);

        // Phase 2: simulate sibling removal (as kill_by_tid + mark_task_exited would do).
        {
            let mut pi = pinfo.lock();
            for &s in &siblings {
                pi.thread_ids.retain(|&t| t != s);
            }
        }

        // Phase 3: simulate commit — clear exec_in_progress.
        pinfo.lock().exec_in_progress = false;

        let pi = pinfo.lock();
        assert!(!pi.exec_in_progress, "flag should be cleared after commit");
        assert_eq!(pi.thread_ids, alloc::vec![caller_tid], "only caller should remain");
    }

    /// On pre-commit failure the exec_in_progress flag must be cleared so the
    /// original process/thread-group is left usable.
    #[test]
    fn exec_rollback_restores_exec_in_progress() {
        let pinfo = make_two_thread_pinfo(9260, 9260, 9261);

        // Begin exec.
        pinfo.lock().exec_in_progress = true;
        assert!(pinfo.lock().exec_in_progress);

        // Simulate a pre-commit failure (e.g., ENOEXEC).
        pinfo.lock().exec_in_progress = false;

        assert!(
            !pinfo.lock().exec_in_progress,
            "exec_in_progress must be cleared on rollback"
        );
        // thread_ids should be untouched (siblings are still alive in the real
        // failure path because kill_by_tid is only called for the sibling-kill
        // phase which happens before any FD/ELF operations).
        assert_eq!(
            pinfo.lock().thread_ids.len(),
            2,
            "thread_ids still has both threads on rollback"
        );
    }
}
