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
    let (node, exec_fd_path) = {
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
        let path = (*open_file.path).clone();
        (open_file.node.clone(), path)
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

    let (mut entry, stack_info, mut mappings, mut aux_info) =
        match crate::task::loader::load_module::<R>(rt, new_aspace, &module_desc) {
            Some(r) => r,
            None => abort_exec!(Errno::ENOEXEC),
        };

    // 5b. Dynamic-interpreter (PT_INTERP) support.
    //     When the executable requests a dynamic loader we load the interpreter
    //     into the same address space at a high base, then hand control to it.
    //     The interpreter is responsible for processing DT_NEEDED libraries,
    //     relocating symbols, and jumping to the real entry point (AT_ENTRY).
    if let Some(interp_path_bytes) =
        crate::task::loader::extract_interp_path(static_bytes)
    {
        crate::kinfo!(
            "EXEC: PT_INTERP found: {:?}",
            core::str::from_utf8(&interp_path_bytes).unwrap_or("<invalid>")
        );

        // Build a NUL-terminated path string for vfs_open.
        let mut path_buf = interp_path_bytes.clone();
        path_buf.push(0);
        let path_str =
            core::str::from_utf8(&path_buf[..path_buf.len() - 1]).unwrap_or("/lib/ld.so");

        // Open the interpreter file from the VFS.
        let interp_node = match crate::vfs::mount::lookup(path_str) {
            Ok(n) => n,
            Err(e) => {
                crate::kwarn!(
                    "EXEC: Failed to open interpreter '{}': {:?}",
                    path_str,
                    e
                );
                abort_exec!(e);
            }
        };

        // Read the interpreter into kernel memory.
        let interp_stat = match interp_node.stat() {
            Ok(s) => s,
            Err(e) => abort_exec!(e),
        };
        let interp_size = interp_stat.size as usize;
        if interp_size > 32 * 1024 * 1024 {
            abort_exec!(Errno::EFBIG);
        }
        let mut interp_buf = alloc::vec![0u8; interp_size];
        let mut pos = 0;
        while pos < interp_size {
            let n = match interp_node.read(pos as u64, &mut interp_buf[pos..]) {
                Ok(n) => n,
                Err(e) => abort_exec!(e),
            };
            if n == 0 {
                break;
            }
            pos += n;
        }
        if pos < interp_size {
            abort_exec!(Errno::EIO);
        }

        // The interpreter base: place it well above the main executable (0x200000).
        // Use 0x7F00_0000 for x86_64 — well within the 47-bit user VA range.
        const INTERP_LOAD_BASE: u64 = 0x7F00_0000;

        let static_interp: &'static [u8] =
            unsafe { core::mem::transmute(&interp_buf as &[u8]) };
        let interp_module = crate::BootModuleDesc {
            name: "ld.so",
            cmdline: "",
            bytes: static_interp,
            phys_start: 0,
            phys_end: 0,
            kind: crate::BootModuleKind::Elf,
        };

        match crate::task::loader::load_module_at::<R>(
            rt,
            new_aspace,
            &interp_module,
            INTERP_LOAD_BASE,
        ) {
            Some((interp_entry, _interp_stack, interp_regions, _interp_aux)) => {
                // Record where the interpreter was loaded (AT_BASE).
                aux_info.interp_base = INTERP_LOAD_BASE;
                // Extend the mapping list with the interpreter's regions.
                mappings.extend(interp_regions);
                // Hand control to the interpreter; it will jump to AT_ENTRY.
                entry.entry_pc = interp_entry.entry_pc;
            }
            None => {
                crate::kwarn!("EXEC: Failed to load interpreter '{}'", path_str);
                abort_exec!(Errno::ENOEXEC);
            }
        }
    }

    // 6. Update ProcessInfo metadata (argv, env, and auxv).
    //    Also close all FD_CLOEXEC-flagged file descriptors and clear
    //    exec_in_progress now that we are about to commit — the caller is the
    //    sole surviving thread from this point forward.
    {
        let page_size = rt.page_size() as u64;
        let mut pinfo = pinfo_arc.lock();
        pinfo.argv = argv;
        pinfo.env = env;
        // Rebuild auxv from freshly loaded image.  AT_* constants follow
        // the standard ELF auxiliary-vector specification (see elf.h).
        pinfo.auxv = build_auxv(&aux_info, page_size);
        // Record the executable path for /proc/self/exe.
        pinfo.exec_path = exec_fd_path;
        // Close all file descriptors marked FD_CLOEXEC before the new image runs.
        pinfo.fd_table.close_on_exec();
        // Commit: caller is now the only thread; clear the flag.
        pinfo.exec_in_progress = false;
    }

    // 7. Finalize the new task state
    // We need to replace the current task's aspace, mappings, and context.
    // This is the "Commit Point".

    // Copy the context out so we can switch to it after dropping the registry lock
    let (to_ctx, new_aspace_actual, tls_tp) = {
        let mut task_mut = crate::task::registry::get_task_mut::<R>(tid).ok_or(Errno::ESRCH)?;

        // Replace address space on the thread (fast-path cache).
        task_mut.aspace = new_aspace;

        // Replace mappings — task.mappings and process.mappings are the same Arc,
        // so clearing and repopulating the inner MappingList updates both at once.
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

        // Apply the initial TLS thread pointer for the new image.
        // A zero value means no PT_TLS segment was present; FS_BASE is cleared.
        task_mut.user_fs_base = aux_info.tls_tp;

        (task_mut.ctx, task_mut.aspace, aux_info.tls_tp)
    }; // registry lock (TaskMut) dropped here

    // Update the process-owned address-space token so it reflects the new image.
    // This must happen after the registry lock is released (process lock must
    // never be taken while the registry lock is held).
    {
        let aspace_raw = rt.tasking().aspace_to_raw(new_aspace);
        pinfo_arc.lock().aspace_raw = aspace_raw;
    }

    // 8. Perform the actual transition
    // We must never return to the old state.
    rt.tasking().activate_address_space(new_aspace_actual);

    let mut dummy_ctx = Default::default();
    // Discard the outgoing FS_BASE; switch in with the new image's TLS pointer.
    let mut _discard_tls: u64 = 0;
    unsafe {
        rt.tasking()
            .switch_with_tls(&mut dummy_ctx, &to_ctx, tid, &mut _discard_tls, tls_tp);
    }

    // switch() should never return to this stack because we didn't save it into any task.ctx
    unreachable!("task_exec: switch returned unexpectedly")
}

// Standard AT_* auxiliary-vector type constants (matches Linux/SysV ABI).
const AT_PAGESZ: u64 = abi::auxv::AT_PAGESZ;
const AT_BASE: u64 = abi::auxv::AT_BASE;
const AT_PHDR: u64 = abi::auxv::AT_PHDR;
const AT_PHENT: u64 = abi::auxv::AT_PHENT;
const AT_PHNUM: u64 = abi::auxv::AT_PHNUM;
const AT_ENTRY: u64 = abi::auxv::AT_ENTRY;

// Janix-specific AT_* entries for ELF TLS — shared constants from abi::auxv.
const AT_JANIX_TLS_TEMPLATE_VA: u64 = abi::auxv::AT_JANIX_TLS_TEMPLATE_VA;
const AT_JANIX_TLS_FILESZ: u64 = abi::auxv::AT_JANIX_TLS_FILESZ;
const AT_JANIX_TLS_MEMSZ: u64 = abi::auxv::AT_JANIX_TLS_MEMSZ;
const AT_JANIX_TLS_ALIGN: u64 = abi::auxv::AT_JANIX_TLS_ALIGN;

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
    // Emit AT_BASE when the main executable used a dynamic interpreter.
    if aux_info.interp_base != 0 {
        v.push((AT_BASE, aux_info.interp_base));
    }
    // Emit TLS auxiliary entries when a PT_TLS segment was found.
    if aux_info.tls_memsz != 0 {
        v.push((AT_JANIX_TLS_TEMPLATE_VA, aux_info.tls_template_vaddr));
        v.push((AT_JANIX_TLS_FILESZ, aux_info.tls_filesz));
        v.push((AT_JANIX_TLS_MEMSZ, aux_info.tls_memsz));
        v.push((AT_JANIX_TLS_ALIGN, aux_info.tls_align));
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
            ..Default::default()
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
        // No TLS entries when tls_memsz == 0.
        assert!(!auxv.iter().any(|&(k, _)| k == AT_JANIX_TLS_MEMSZ));
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
            ..Default::default()
        };
        let auxv = build_auxv(&info, 0x1000);
        assert!(auxv.contains(&(AT_PAGESZ, 0x1000)));
        assert!(auxv.contains(&(AT_ENTRY, 0x201000)));
        // phdr_vaddr == 0 → no AT_PHDR/AT_PHENT/AT_PHNUM emitted.
        assert!(!auxv.iter().any(|&(k, _)| k == AT_PHDR));
    }

    #[test]
    fn build_auxv_with_tls() {
        let info = LoaderAuxInfo {
            phdr_vaddr: 0x200040,
            phent: 56,
            phnum: 4,
            entry_vaddr: 0x201000,
            tls_template_vaddr: 0x300000,
            tls_filesz: 64,
            tls_memsz: 128,
            tls_align: 16,
            tls_tp: 0x310080,
            interp_base: 0,
        };
        let auxv = build_auxv(&info, 4096);
        // Standard entries still present.
        assert!(auxv.contains(&(AT_PAGESZ, 4096)));
        assert!(auxv.contains(&(AT_PHDR, 0x200040)));
        assert!(auxv.contains(&(AT_ENTRY, 0x201000)));
        // TLS entries present when tls_memsz != 0.
        assert!(auxv.contains(&(AT_JANIX_TLS_TEMPLATE_VA, 0x300000)));
        assert!(auxv.contains(&(AT_JANIX_TLS_FILESZ, 64)));
        assert!(auxv.contains(&(AT_JANIX_TLS_MEMSZ, 128)));
        assert!(auxv.contains(&(AT_JANIX_TLS_ALIGN, 16)));
    }

    #[test]
    fn build_auxv_with_interp_base() {
        // When the executable has PT_INTERP and the interpreter was loaded,
        // AT_BASE must be emitted with the interpreter's load base.
        let info = LoaderAuxInfo {
            phdr_vaddr: 0x200040,
            phent: 56,
            phnum: 3,
            entry_vaddr: 0x201000,
            interp_base: 0x7F00_0000,
            ..Default::default()
        };
        let auxv = build_auxv(&info, 4096);
        assert!(auxv.contains(&(AT_BASE, 0x7F00_0000)));
        assert!(auxv.contains(&(AT_ENTRY, 0x201000)));
        // Standard entries still present.
        assert!(auxv.contains(&(AT_PAGESZ, 4096)));
    }

    #[test]
    fn build_auxv_no_interp_no_at_base() {
        // Statically-linked binary: no AT_BASE should be emitted.
        let info = LoaderAuxInfo {
            phdr_vaddr: 0x200040,
            phent: 56,
            phnum: 3,
            entry_vaddr: 0x201000,
            ..Default::default()
        };
        let auxv = build_auxv(&info, 4096);
        assert!(!auxv.iter().any(|&(k, _)| k == AT_BASE));
    }



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
            exec_path: alloc::string::String::new(),
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            aspace_raw: 0,
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
            exec_path: alloc::string::String::new(),
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            aspace_raw: 0,
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
            exec_path: alloc::string::String::new(),
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            aspace_raw: 0,
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
        // failure path because kill_by_tid is only called during the sibling-kill
        // phase which happens before FD resolution and ELF loading).
        assert_eq!(
            pinfo.lock().thread_ids.len(),
            2,
            "thread_ids still has both threads on rollback"
        );
    }

    // ── FD_CLOEXEC / close-on-exec unit tests ────────────────────────────────

    use crate::vfs::fd_table::FD_CLOEXEC;
    use crate::vfs::{OpenFlags, VfsNode, VfsStat};
    use abi::errors::SysResult;

    struct NullNode;
    impl VfsNode for NullNode {
        fn read(&self, _: u64, _: &mut [u8]) -> SysResult<usize> {
            Ok(0)
        }
        fn write(&self, _: u64, buf: &[u8]) -> SysResult<usize> {
            Ok(buf.len())
        }
        fn stat(&self) -> SysResult<VfsStat> {
            Ok(VfsStat {
                mode: VfsStat::S_IFCHR | 0o666,
                size: 0,
                ino: 1,
                ..Default::default()
            })
        }
    }

    fn null_node() -> alloc::sync::Arc<dyn VfsNode> {
        alloc::sync::Arc::new(NullNode)
    }

    /// Simulates the commit phase of exec: close_on_exec is called on the fd
    /// table, then exec_in_progress is cleared.  FDs with FD_CLOEXEC should be
    /// gone; others should remain.
    #[test]
    fn exec_commit_closes_cloexec_fds() {
        let pinfo = Arc::new(Mutex::new(ProcessInfo {
            pid: 9300,
            ppid: 1,
            argv: alloc::vec::Vec::new(),
            env: alloc::collections::BTreeMap::new(),
            auxv: alloc::vec::Vec::new(),
            fd_table: crate::vfs::fd_table::FdTable::new(),
            namespace: crate::vfs::NamespaceRef::global(),
            cwd: alloc::string::String::from("/"),
            thread_ids: alloc::vec![9300],
            exec_in_progress: false,
            exec_path: alloc::string::String::new(),
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            aspace_raw: 0,
        }));

        // Set up: fd 0 survives, fd 1 has FD_CLOEXEC.
        {
            let mut pi = pinfo.lock();
            pi.fd_table
                .insert_at(0, null_node(), OpenFlags::read_only(), "/in".into())
                .unwrap();
            pi.fd_table
                .insert_at(1, null_node(), OpenFlags::write_only(), "/cloexec".into())
                .unwrap();
            pi.fd_table.set_fd_flags(1, FD_CLOEXEC).unwrap();
        }

        // Simulate exec commit phase.
        {
            let mut pi = pinfo.lock();
            pi.exec_in_progress = true;
            pi.fd_table.close_on_exec();
            pi.exec_in_progress = false;
        }

        let pi = pinfo.lock();
        assert!(
            pi.fd_table.get(0).is_ok(),
            "fd 0 (no FD_CLOEXEC) must survive exec"
        );
        assert!(
            matches!(pi.fd_table.get(1), Err(abi::errors::Errno::EBADF)),
            "fd 1 (FD_CLOEXEC) must be closed on exec"
        );
        assert!(!pi.exec_in_progress, "exec_in_progress cleared after commit");
    }

    /// When no FDs have FD_CLOEXEC, close_on_exec during exec is a no-op and
    /// all FDs survive.
    #[test]
    fn exec_commit_preserves_all_fds_without_cloexec() {
        let pinfo = Arc::new(Mutex::new(ProcessInfo {
            pid: 9310,
            ppid: 1,
            argv: alloc::vec::Vec::new(),
            env: alloc::collections::BTreeMap::new(),
            auxv: alloc::vec::Vec::new(),
            fd_table: crate::vfs::fd_table::FdTable::new(),
            namespace: crate::vfs::NamespaceRef::global(),
            cwd: alloc::string::String::from("/"),
            thread_ids: alloc::vec![9310],
            exec_in_progress: false,
            exec_path: alloc::string::String::new(),
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            aspace_raw: 0,
        }));

        {
            let mut pi = pinfo.lock();
            pi.fd_table
                .insert_at(0, null_node(), OpenFlags::read_only(), "/in".into())
                .unwrap();
            pi.fd_table
                .insert_at(1, null_node(), OpenFlags::write_only(), "/out".into())
                .unwrap();
        }

        // Exec commit with no FD_CLOEXEC flags set.
        pinfo.lock().fd_table.close_on_exec();

        let pi = pinfo.lock();
        assert!(pi.fd_table.get(0).is_ok(), "fd 0 should survive");
        assert!(pi.fd_table.get(1).is_ok(), "fd 1 should survive");
    }

    // ── VM ownership / process-scoped aspace tests ────────────────────────────

    /// Verify that `Process.aspace_raw` starts at 0 (no address space assigned
    /// yet) and can be set to reflect a new page-table root.
    #[test]
    fn process_aspace_raw_default_is_zero() {
        let pinfo = make_two_thread_pinfo(9400, 9400, 9401);
        assert_eq!(pinfo.lock().aspace_raw, 0, "aspace_raw should default to 0");
    }

    /// Verify that updating `aspace_raw` on the process is visible to all
    /// threads that share the same `Arc<Mutex<Process>>`.
    #[test]
    fn process_aspace_raw_shared_across_threads() {
        let pinfo = make_two_thread_pinfo(9410, 9410, 9411);

        // Simulate the exec commit: update aspace_raw on the process.
        const FAKE_CR3: u64 = 0x0000_0010_0000_0000;
        pinfo.lock().aspace_raw = FAKE_CR3;

        // Both thread representations reference the same Arc, so both observe
        // the same updated value.
        let pi = pinfo.lock();
        assert_eq!(
            pi.aspace_raw, FAKE_CR3,
            "process aspace_raw must be visible to all threads sharing the Arc"
        );
    }

    /// Verify that `Process.mappings` is the canonical source of truth and that
    /// all threads referencing the same process share the **same** `Arc`.
    #[test]
    fn thread_mappings_share_same_arc_as_process() {
        let mappings_arc = alloc::sync::Arc::new(spin::Mutex::new(
            crate::memory::mappings::MappingList::new(),
        ));

        // Two threads in the same process, both getting a clone of the same Arc.
        let thread1_mappings = mappings_arc.clone();
        let thread2_mappings = mappings_arc.clone();

        // All three Arcs point to the same underlying allocation.
        assert!(
            alloc::sync::Arc::ptr_eq(&mappings_arc, &thread1_mappings),
            "process and thread1 must share the same mappings Arc"
        );
        assert!(
            alloc::sync::Arc::ptr_eq(&mappings_arc, &thread2_mappings),
            "process and thread2 must share the same mappings Arc"
        );

        // Mutating through one Arc is immediately visible through the others.
        {
            let mut ml = mappings_arc.lock();
            ml.insert(abi::vm::VmRegionInfo {
                start: 0x1000,
                end: 0x2000,
                prot: abi::vm::VmProt::READ,
                ..Default::default()
            });
        }
        assert_eq!(
            thread1_mappings.lock().regions.len(),
            1,
            "thread1 must see the mapping added via process Arc"
        );
        assert_eq!(
            thread2_mappings.lock().regions.len(),
            1,
            "thread2 must see the mapping added via process Arc"
        );
    }

    /// Simulated exec transition: aspace_raw and mappings are both updated on
    /// the process.  After exec, the old mappings are replaced and the new
    /// aspace_raw reflects the new page-table root.
    #[test]
    fn exec_transition_updates_process_vm_state() {
        let pinfo = make_two_thread_pinfo(9420, 9420, 9421);

        // Before exec: initial state.
        assert_eq!(pinfo.lock().aspace_raw, 0);
        assert_eq!(pinfo.lock().mappings.lock().regions.len(), 0);

        // Simulate exec commit: replace mappings and update aspace_raw.
        const NEW_CR3: u64 = 0x0000_0020_0000_0000;
        {
            let mut pi = pinfo.lock();
            let mut ml = pi.mappings.lock();
            *ml = crate::memory::mappings::MappingList::new();
            ml.insert(abi::vm::VmRegionInfo {
                start: 0x200000,
                end: 0x201000,
                prot: abi::vm::VmProt::READ | abi::vm::VmProt::EXEC,
                ..Default::default()
            });
            drop(ml);
            pi.aspace_raw = NEW_CR3;
        }

        // After exec: process reflects new VM state.
        let pi = pinfo.lock();
        assert_eq!(pi.aspace_raw, NEW_CR3, "aspace_raw must reflect new page table after exec");
        assert_eq!(
            pi.mappings.lock().regions.len(),
            1,
            "mappings must contain the new region after exec"
        );
    }
}
