//! Task and thread spawning functions.

use crate::task::{Affinity, ProcessInfo, StartupArg, Task, TaskId, TaskState};
use crate::{BootRuntime, BootTasking, UserEntry};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use super::SCHEDULER;
use super::types::{DEFAULT_TIMESLICE, Scheduler};
use core::sync::atomic::{AtomicUsize, Ordering};

const KERNEL_STACK_SIZE: usize = 65536;

fn boot_module_matches(name: &str, module_name: &str) -> bool {
    if module_name == name {
        return true;
    }

    module_name.rsplit('/').next().unwrap_or(module_name) == name.rsplit('/').next().unwrap_or(name)
}

fn current_parent_pid<R: BootRuntime>(sched: &Scheduler<R>) -> u32 {
    let cpu_idx = super::current_cpu_index::<R>();
    sched
        .state
        .per_cpu
        .get(cpu_idx)
        .and_then(|pc| pc.current)
        .and_then(|ctid| crate::task::registry::get_task::<R>(ctid))
        .and_then(|t| t.process_info.clone())
        .map(|pi| pi.lock().pid)
        .unwrap_or(0)
}

fn default_process_info(
    pid: u32,
    ppid: u32,
    mappings: alloc::sync::Arc<spin::Mutex<crate::memory::mappings::MappingList>>,
    aspace_raw: u64,
) -> alloc::sync::Arc<spin::Mutex<ProcessInfo>> {
    let console_node: alloc::sync::Arc<dyn crate::vfs::VfsNode> =
        alloc::sync::Arc::new(crate::vfs::devfs::ConsoleNode);
    let mut fd_table = crate::vfs::fd_table::FdTable::new();
    let _ = fd_table.insert_at(
        0,
        console_node.clone(),
        crate::vfs::OpenFlags::read_only(),
        "/dev/console".into(),
    );
    let _ = fd_table.insert_at(
        1,
        console_node.clone(),
        crate::vfs::OpenFlags::write_only(),
        "/dev/console".into(),
    );
    let _ = fd_table.insert_at(
        2,
        console_node,
        crate::vfs::OpenFlags::write_only(),
        "/dev/console".into(),
    );
    alloc::sync::Arc::new(spin::Mutex::new(ProcessInfo {
        pid,
        ppid,
        argv: alloc::vec::Vec::new(),
        env: alloc::collections::BTreeMap::new(),
        auxv: alloc::vec::Vec::new(),
        fd_table,
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: alloc::string::String::from("/"),
        thread_ids: alloc::vec![pid as TaskId],
        exec_in_progress: false,
        exec_path: alloc::string::String::new(),
        mappings,
        aspace_raw,
    }))
}

fn inherit_process_info<R: BootRuntime>(
    pid: u32,
    ppid: u32,
    mappings: alloc::sync::Arc<spin::Mutex<crate::memory::mappings::MappingList>>,
    aspace_raw: u64,
) -> alloc::sync::Arc<spin::Mutex<ProcessInfo>> {
    let tid = crate::runtime::<R>().current_tid();
    let current_pinfo =
        crate::task::registry::get_task::<R>(tid).and_then(|t| t.process_info.clone());

    if let Some(parent_pi) = current_pinfo {
        let parent = parent_pi.lock();
        alloc::sync::Arc::new(spin::Mutex::new(ProcessInfo {
            pid,
            ppid,
            argv: alloc::vec::Vec::new(),
            env: parent.env.clone(),
            auxv: alloc::vec::Vec::new(),
            fd_table: parent.fd_table.clone(),
            namespace: parent.namespace.clone(),
            cwd: parent.cwd.clone(),
            thread_ids: alloc::vec![pid as TaskId],
            exec_in_progress: false,
            exec_path: alloc::string::String::new(),
            mappings,
            aspace_raw,
        }))
    } else {
        default_process_info(pid, ppid, mappings, aspace_raw)
    }
}

// Global round-robin index for CPU selection
pub(crate) static RR_IDX: AtomicUsize = AtomicUsize::new(0);

impl<R: BootRuntime> Scheduler<R> {
    fn pick_cpu_and_bringup(&mut self, affinity: Affinity, _trigger_smp: bool) -> usize {
        let rt = crate::runtime::<R>();
        let count = self.state.online_cpu_count;
        let idx = RR_IDX.fetch_add(1, Ordering::Relaxed);

        match affinity {
            Affinity::Pinned(cpu) => cpu,
            Affinity::Any => {
                // will be brought up manually when needed.
                //     if let Some(next_cpu_id) = rt.next_offline_cpu() {
                //         let target_cpu = next_cpu_id.0 as usize;
                //         unsafe {
                //             let _ = rt.start_cpu(next_cpu_id, crate::kernel_secondary_entry::<R>, target_cpu);
                //         }
                //         return target_cpu;
                //     }
                // }
                let _ = rt; // Suppress unused variable warning
                if count > 1 {
                    (idx % (count - 1)) + 1
                } else {
                    0
                }
            }
        }
    }
    pub fn spawn(
        &mut self,
        entry: extern "C" fn(usize) -> !,
        arg: StartupArg,
        priority: crate::task::TaskPriority,
        affinity: Affinity,
    ) -> TaskId {
        let rt = crate::runtime::<R>();
        let id = self.next_id;
        self.next_id += 1;

        let layout = alloc::alloc::Layout::from_size_align(KERNEL_STACK_SIZE, 8).unwrap();
        let stack_base = unsafe { alloc::alloc::alloc(layout) };
        if stack_base.is_null() {
            panic!("Failed to allocate stack for task {}", id);
        }
        let stack_top = (stack_base as u64) + KERNEL_STACK_SIZE as u64;

        let ctx = rt
            .tasking()
            .init_kernel_context(entry, stack_top, arg.to_raw());

        // Determine target CPU: Balanced among online CPUs.
        let target_cpu = self.pick_cpu_and_bringup(affinity, false);
        crate::kdebug!("SCHED: Task {} assigned to CPU {}", id, target_cpu);

        // Push to target CPU's run queue
        let cpu_count = self.state.per_cpu.len(); // Should match rt.cpu_count()
        let safe_cpu = if target_cpu < cpu_count {
            target_cpu
        } else {
            0
        };

        let task: Task<R> = Task {
            id,
            state: TaskState::Runnable,
            priority,
            kstack_base: stack_base,
            kstack_size: KERNEL_STACK_SIZE,
            kstack_top: stack_top,
            ctx,
            aspace: rt.tasking().active_address_space(),
            simd: crate::simd::SimdState::new(rt),
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: false,
            wake_pending: false,
            pending_interrupt: false,
            stack_info: None,
            mappings: alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            )),
            timeslice_remaining: DEFAULT_TIMESLICE,
            affinity,
            last_cpu: Some(safe_cpu),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            enqueued_at_tick: super::TICK_COUNT.load(Ordering::Relaxed),
            base_priority: priority,
            user_fs_base: 0,
            detached: false,
        };

        let sched_fields = crate::sched::state::TaskSchedFields {
            tid: task.id,
            state: task.state,
            priority: task.priority,
            base_priority: task.base_priority,
            timeslice_remaining: task.timeslice_remaining,
            affinity: task.affinity,
            enqueued_at_tick: task.enqueued_at_tick,
            last_cpu: task.last_cpu,
            runq_location: None,
        };
        self.state.insert_task(sched_fields);
        crate::task::registry::get_registry::<R>().insert(alloc::boxed::Box::new(task));
        self.state.enqueue_task(safe_cpu, priority as usize, id);

        // Ensure the target CPU actually performs a reschedule for this new runnable task.
        if safe_cpu == super::current_cpu_index::<R>() {
            self.state.per_cpu[safe_cpu].need_resched = true;
        } else {
            super::GLOBAL_NEED_RESCHED[safe_cpu]
                .store(true, core::sync::atomic::Ordering::Release);
            super::DIAG_IPI_SENT.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            crate::kdebug!(
                "SCHED: Sending Resched IPI to CPU {} for task {}",
                safe_cpu,
                id
            );
            rt.send_ipi(safe_cpu, 0x30); // Use IRQ_RESCHED_VECTOR
        }

        let parent_tid = self.state.per_cpu[super::current_cpu_index::<R>()].current;
        // Link affinity and initial location
        if let Affinity::Pinned(cpu) = affinity {}
        // Initial location matches target runq

        id
    }

    pub fn spawn_user_thread(
        &mut self,
        entry: usize,
        stack: usize,
        arg: StartupArg,
        stack_info: abi::types::StackInfo,
        priority: crate::task::TaskPriority,
        affinity: Affinity,
        tls_base: u64,
        detached: bool,
    ) -> TaskId {
        let rt = crate::runtime::<R>();
        let id = self.next_id;
        self.next_id += 1;

        let layout = alloc::alloc::Layout::from_size_align(KERNEL_STACK_SIZE, 8).unwrap();
        let stack_base = unsafe { alloc::alloc::alloc(layout) };
        if stack_base.is_null() {
            panic!("Failed to allocate kernel stack for user thread {}", id);
        }
        let kstack_top = (stack_base as u64) + KERNEL_STACK_SIZE as u64;

        let aspace = rt.tasking().active_address_space();

        // Inherit mappings and process_info from the current process (not the
        // current task) so that the canonical VM state is always sourced from
        // Process rather than from an arbitrary thread's cached copy.
        let parent_pinfo =
            if let Some(current_id) = self.state.per_cpu[super::current_cpu_index::<R>()].current {
                crate::task::registry::get_task::<R>(current_id)
                    .and_then(|parent| parent.process_info.clone())
            } else {
                None
            };

        // Clone the mappings Arc from the parent process (same underlying
        // MappingList object).  Fall back to an empty list only when there is
        // no parent process (should not happen for user threads).
        let mappings = parent_pinfo
            .as_ref()
            .map(|pi| pi.lock().mappings.clone())
            .unwrap_or_else(|| {
                alloc::sync::Arc::new(spin::Mutex::new(
                    crate::memory::mappings::MappingList::new(),
                ))
            });

        let spec = crate::UserTaskSpec {
            entry: entry as u64,
            stack_top: stack as u64,
            aspace,
            arg: arg.to_raw(),
        };

        let ctx = rt.tasking().init_user_context(spec, kstack_top);

        let target_cpu = match affinity {
            Affinity::Pinned(cpu) => cpu,
            Affinity::Any => super::current_cpu_index::<R>(),
        };
        crate::kdebug!(
            "SCHED: Task {} (user thread) assigned to CPU {}",
            id,
            target_cpu
        );

        // Push to target CPU's run queue
        let cpu_count = self.state.per_cpu.len();
        let safe_cpu = if target_cpu < cpu_count {
            target_cpu
        } else {
            0
        };

        let task: Task<R> = Task {
            id,
            state: TaskState::Runnable,
            priority,
            kstack_base: stack_base,
            kstack_size: KERNEL_STACK_SIZE,
            kstack_top,
            ctx,
            aspace,
            simd: crate::simd::SimdState::new(rt),
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: true,
            wake_pending: false,
            pending_interrupt: false,
            stack_info: Some(stack_info),
            mappings,
            timeslice_remaining: DEFAULT_TIMESLICE,
            affinity,
            last_cpu: Some(safe_cpu),
            name: [0; 32],
            name_len: 0,
            process_info: parent_pinfo,
            enqueued_at_tick: super::TICK_COUNT.load(Ordering::Relaxed),
            base_priority: priority,
            user_fs_base: tls_base,
            detached,
        };

        // Register this thread's TID in the owning process so exec and exit
        // can enumerate all threads.  This must happen unconditionally —
        // previously the registration was gated on `tls_base != 0` which was
        // a bug: threads spawned without a TLS base were invisible to exec.
        if let Some(pinfo) = task.process_info.as_ref() {
            let mut pi = pinfo.lock();
            if !pi.thread_ids.contains(&id) {
                pi.thread_ids.push(id);
            }
        }

        let sched_fields = crate::sched::state::TaskSchedFields {
            tid: task.id,
            state: task.state,
            priority: task.priority,
            base_priority: task.base_priority,
            timeslice_remaining: task.timeslice_remaining,
            affinity: task.affinity,
            enqueued_at_tick: task.enqueued_at_tick,
            last_cpu: task.last_cpu,
            runq_location: None,
        };
        self.state.insert_task(sched_fields);
        crate::task::registry::get_registry::<R>().insert(alloc::boxed::Box::new(task));
        self.state.enqueue_task(safe_cpu, priority as usize, id);

        // Ensure the target CPU actually performs a reschedule for this new runnable task.
        if safe_cpu == super::current_cpu_index::<R>() {
            self.state.per_cpu[safe_cpu].need_resched = true;
        } else {
            super::GLOBAL_NEED_RESCHED[safe_cpu]
                .store(true, core::sync::atomic::Ordering::Release);
            crate::kdebug!(
                "SCHED: Sending Resched IPI to CPU {} for task {}",
                safe_cpu,
                id
            );
            rt.send_ipi(safe_cpu, 0x30); // Use IRQ_RESCHED_VECTOR
        }

        let parent_tid = self.state.per_cpu[super::current_cpu_index::<R>()].current;
        // Link affinity and initial location
        if let Affinity::Pinned(cpu) = affinity {}
        // Initial location matches target runq

        id
    }

    pub fn spawn_user_task(
        &mut self,
        entry: UserEntry,
        aspace: <R::Tasking as BootTasking>::AddressSpace,
        stack_info: abi::types::StackInfo,
        regions: alloc::vec::Vec<abi::vm::VmRegionInfo>,
        priority: crate::task::TaskPriority,
        affinity: Affinity,
    ) -> Option<TaskId> {
        let rt = crate::runtime::<R>();
        let id = self.next_id;

        self.next_id += 1;
        let layout = alloc::alloc::Layout::from_size_align(KERNEL_STACK_SIZE, 8).unwrap();
        let stack_base = unsafe { alloc::alloc::alloc(layout) };
        if stack_base.is_null() {
            return None;
        }
        let stack_top = (stack_base as u64) + KERNEL_STACK_SIZE as u64;

        let user_entry = alloc::boxed::Box::new(entry);
        let entry_ptr = alloc::boxed::Box::into_raw(user_entry) as usize;

        let ctx =
            rt.tasking()
                .init_kernel_context(user_thread_trampoline::<R>, stack_top, entry_ptr);

        let mapping_list = crate::memory::mappings::MappingList { regions };
        let ppid = current_parent_pid::<R>(self);
        // Create the mappings Arc once — both the Process and the Thread hold a
        // clone of the same Arc so the scheduler's per-CPU CURRENT_MAPPINGS cache
        // works without locking the Process mutex on every context switch.
        let mappings_arc = alloc::sync::Arc::new(spin::Mutex::new(mapping_list));
        // Derive the process-owned address-space token from the typed aspace handle.
        let aspace_raw = rt.tasking().aspace_to_raw(aspace);
        let pinfo = default_process_info(id as u32, ppid, mappings_arc.clone(), aspace_raw);

        let target_cpu = self.pick_cpu_and_bringup(affinity, true);
        // Push to target CPU's run queue
        let cpu_count = self.state.per_cpu.len();
        let safe_cpu = if target_cpu < cpu_count {
            target_cpu
        } else {
            0
        };

        let task: Task<R> = Task {
            id,
            state: TaskState::Runnable,
            priority,
            kstack_base: stack_base,
            kstack_size: KERNEL_STACK_SIZE,
            kstack_top: stack_top,
            ctx,
            aspace,
            simd: crate::simd::SimdState::new(rt),
            exit_code: None,
            exit_waiters: crate::sched::WaitQueue::new(),
            is_user: true,
            wake_pending: false,
            pending_interrupt: false,
            stack_info: Some(stack_info),
            mappings: mappings_arc,
            timeslice_remaining: DEFAULT_TIMESLICE,
            affinity,
            last_cpu: Some(safe_cpu),
            name: [0; 32],
            name_len: 0,
            process_info: Some(pinfo),
            enqueued_at_tick: super::TICK_COUNT.load(Ordering::Relaxed),
            base_priority: priority,
            user_fs_base: 0,
            detached: false,
        };

        let sched_fields = crate::sched::state::TaskSchedFields {
            tid: task.id,
            state: task.state,
            priority: task.priority,
            base_priority: task.base_priority,
            timeslice_remaining: task.timeslice_remaining,
            affinity: task.affinity,
            enqueued_at_tick: task.enqueued_at_tick,
            last_cpu: task.last_cpu,
            runq_location: None,
        };
        self.state.insert_task(sched_fields);
        crate::task::registry::get_registry::<R>().insert(alloc::boxed::Box::new(task));
        self.state.enqueue_task(safe_cpu, priority as usize, id);

        // Ensure the target CPU actually performs a reschedule for this new runnable task.
        if safe_cpu == super::current_cpu_index::<R>() {
            self.state.per_cpu[safe_cpu].need_resched = true;
        } else {
            super::GLOBAL_NEED_RESCHED[safe_cpu]
                .store(true, core::sync::atomic::Ordering::Release);
            crate::kdebug!(
                "SCHED: Sending Resched IPI to CPU {} for task {}",
                safe_cpu,
                id
            );
            rt.send_ipi(safe_cpu, 0x30); // Use IRQ_RESCHED_VECTOR
        }

        let parent_tid = self.state.per_cpu[super::current_cpu_index::<R>()].current;
        // Link affinity and initial location
        if let Affinity::Pinned(cpu) = affinity {}
        // Initial location matches target runq

        Some(id)
    }
}

pub fn spawn<R: BootRuntime>(
    entry: extern "C" fn(usize) -> !,
    arg: StartupArg,
    priority: crate::task::TaskPriority,
    affinity: crate::task::Affinity,
) -> TaskId {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
    let id = sched.spawn(entry, arg, priority, affinity);
    rt.irq_restore(_irq);
    id
}

pub fn spawn_with_priority<R: BootRuntime>(
    entry: extern "C" fn(usize) -> !,
    arg: StartupArg,
    priority: crate::task::TaskPriority,
) -> TaskId {
    spawn::<R>(entry, arg, priority, crate::task::Affinity::Any)
}

pub unsafe fn spawn_user_thread<R: BootRuntime>(
    entry: usize,
    stack: usize,
    arg: StartupArg,
    stack_info: abi::types::StackInfo,
    priority: crate::task::TaskPriority,
) -> TaskId {
    spawn_user_thread_ex::<R>(entry, stack, arg, stack_info, priority, 0, false)
}

/// Extended version of `spawn_user_thread` with explicit TLS base and detached flag.
pub unsafe fn spawn_user_thread_ex<R: BootRuntime>(
    entry: usize,
    stack: usize,
    arg: StartupArg,
    stack_info: abi::types::StackInfo,
    priority: crate::task::TaskPriority,
    tls_base: u64,
    detached: bool,
) -> TaskId {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
    let id = sched.spawn_user_thread(
        entry,
        stack,
        arg,
        stack_info,
        priority,
        crate::task::Affinity::Any,
        tls_base,
        detached,
    );
    rt.irq_restore(_irq);
    id
}

pub unsafe fn spawn_user_task_full<R: BootRuntime>(
    entry: UserEntry,
    aspace: <R::Tasking as BootTasking>::AddressSpace,
    stack_info: abi::types::StackInfo,
    regions: alloc::vec::Vec<abi::vm::VmRegionInfo>,
    priority: crate::task::TaskPriority,
) -> Option<TaskId> {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
    let id = sched.spawn_user_task(
        entry,
        aspace,
        stack_info,
        regions,
        priority,
        crate::task::Affinity::Any,
    );
    rt.irq_restore(_irq);
    id
}

pub unsafe fn spawn_process<R: BootRuntime>(name: &str, arg: StartupArg) -> Option<TaskId> {
    unsafe { spawn_process_with_priority::<R>(name, arg, crate::task::TaskPriority::Normal) }
}

pub unsafe fn spawn_process_with_priority<R: BootRuntime>(
    name: &str,
    arg: StartupArg,
    priority: crate::task::TaskPriority,
) -> Option<TaskId> {
    let rt = crate::runtime::<R>();
    let modules = rt.modules();
    let basename = name.rsplit('/').next().unwrap_or(name);
    let module = modules.iter().find(|m| m.name.contains(basename))?;

    let aspace = rt.tasking().make_user_address_space();

    let (mut entry, stack_info, regions, aux_info) = crate::task::loader::load_module(rt, aspace, module)?;
    entry.arg0 = arg.to_raw();

    let _irq = rt.irq_disable();

    let affinity = if name.contains("virtio_sound") || name.contains("beeper") {
        crate::task::Affinity::Pinned(0)
    } else if name == "bloom" {
        if rt.cpu_total_count() > 1 {
            crate::task::Affinity::Pinned(1)
        } else {
            crate::task::Affinity::Any
        }
    } else if name.contains("/sh") {
        if rt.cpu_total_count() > 1 {
            crate::task::Affinity::Pinned(rt.cpu_total_count() - 1)
        } else {
            crate::task::Affinity::Any
        }
    } else {
        crate::task::Affinity::Any
    };

    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };

    let id = sched.spawn_user_task(entry, aspace, stack_info, regions, priority, affinity)?;

    // Determine parent PID from the current task's ProcessInfo
    let ppid = current_parent_pid::<R>(sched);

    // Retrieve the mappings Arc from the task that was just created so the
    // Process and Thread share the same underlying MappingList.
    let task_mappings = crate::task::registry::get_task::<R>(id)
        .map(|t| t.mappings.clone())
        .unwrap_or_else(|| {
            alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            ))
        });

    // Derive the process-owned address-space token (raw u64) from the handle.
    let aspace_raw = rt.tasking().aspace_to_raw(aspace);

    // Create per-process identity
    let pinfo = inherit_process_info::<R>(id as u32, ppid, task_mappings, aspace_raw);
    {
        let page_size = rt.page_size() as u64;
        let mut lock = pinfo.lock();
        lock.argv = alloc::vec![module.name.as_bytes().to_vec()];
        lock.auxv = crate::task::exec::build_auxv(&aux_info, page_size);
        lock.exec_path = alloc::format!("/boot/{}", module.name);
    }

    // Store name, process_info, and initial TLS thread pointer on the task struct.
    if let Some(mut task) = crate::task::registry::get_task_mut::<R>(id) {
        let bytes = module.name.as_bytes();
        let len = bytes.len().min(32);
        task.name[..len].copy_from_slice(&bytes[..len]);
        task.name_len = len as u8;
        task.process_info = Some(pinfo);
        // Apply initial TLS base (FS_BASE on x86_64) for the new process's main thread.
        // Zero means no PT_TLS segment was found; FS_BASE starts at its default state.
        task.user_fs_base = aux_info.tls_tp;
    }

    // Queue setting the process name (processed after scheduler lock released)
    rt.irq_restore(_irq);
    Some(id)
}

/// Stdio specification for a single stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StdioSpec {
    /// Inherit parent's handle (child gets a dup of the parent's fd).
    Inherit,
    /// Attach to null sink/source.
    Null,
    /// Create a pipe; returns the pipe_id for the parent's end.
    Pipe,
}

/// Populate the fd_table slots 0, 1, 2 in `fd_table` based on the given specs.
///
/// Returns the pipe IDs allocated for piped stdin/stdout/stderr (0 when not piped).
/// The parent uses these pipe IDs with the legacy `SYS_PIPE_*` syscalls; the child
/// reads/writes through the fd_table nodes which share the same underlying pipe.
fn setup_stdio_fds<R: BootRuntime>(
    fd_table: &mut crate::vfs::fd_table::FdTable,
    stdin_spec: StdioSpec,
    stdout_spec: StdioSpec,
    stderr_spec: StdioSpec,
) -> (u64, u64, u64) {
    use crate::vfs::{OpenFlags, VfsNode};
    use alloc::sync::Arc;

    let console: Arc<dyn VfsNode> = Arc::new(crate::vfs::devfs::ConsoleNode);
    let null: Arc<dyn VfsNode> = Arc::new(crate::vfs::devfs::NullNode);

    // Helper: inherit parent's fd by cloning the node Arc.
    let inherited_node = |fd: u32| -> Option<(Arc<dyn VfsNode>, OpenFlags)> {
        let tid = crate::runtime::<R>().current_tid();
        crate::task::registry::get_task::<R>(tid)
            .and_then(|task| task.process_info.clone())
            .and_then(|pi| {
                let lock = pi.lock();
                lock.fd_table
                    .get(fd)
                    .ok()
                    .map(|f| (f.node.clone(), *f.status_flags.lock()))
            })
    };

    let mut stdin_pipe: u64 = 0;
    let mut stdout_pipe: u64 = 0;
    let mut stderr_pipe: u64 = 0;

    // fd 0 — stdin
    match stdin_spec {
        StdioSpec::Inherit => {
            if let Some((node, flags)) = inherited_node(0) {
                let _ = fd_table.insert_at(0, node, flags, "/dev/console".into());
            } else {
                let _ = fd_table.insert_at(
                    0,
                    console.clone(),
                    OpenFlags::read_only(),
                    "/dev/console".into(),
                );
            }
        }
        StdioSpec::Null => {
            let _ = fd_table.insert_at(0, null.clone(), OpenFlags::read_only(), "/dev/null".into());
        }
        StdioSpec::Pipe => {
            // Create the raw pipe (readers=1, writers=1).  The child's fd 0 is
            // the read end; the parent retains the write end via stdin_pipe.
            let id = crate::ipc::pipe::create(4096, 0);
            if let Some(read_node) = crate::ipc::pipe::read_node_for_id(id) {
                let _ = fd_table.insert_at(
                    0,
                    read_node,
                    OpenFlags::read_only(),
                    alloc::format!("pipe:{}", id),
                );
            }
            stdin_pipe = id;
        }
    }

    // fd 1 — stdout
    match stdout_spec {
        StdioSpec::Inherit => {
            if let Some((node, flags)) = inherited_node(1) {
                let _ = fd_table.insert_at(1, node, flags, "/dev/console".into());
            } else {
                let _ = fd_table.insert_at(
                    1,
                    console.clone(),
                    OpenFlags::write_only(),
                    "/dev/console".into(),
                );
            }
        }
        StdioSpec::Null => {
            let _ =
                fd_table.insert_at(1, null.clone(), OpenFlags::write_only(), "/dev/null".into());
        }
        StdioSpec::Pipe => {
            let id = crate::ipc::pipe::create(4096, 0);
            if let Some(write_node) = crate::ipc::pipe::write_node_for_id(id) {
                let _ = fd_table.insert_at(
                    1,
                    write_node,
                    OpenFlags::write_only(),
                    alloc::format!("pipe:{}", id),
                );
            }
            stdout_pipe = id;
        }
    }

    // fd 2 — stderr
    match stderr_spec {
        StdioSpec::Inherit => {
            if let Some((node, flags)) = inherited_node(2) {
                let _ = fd_table.insert_at(2, node, flags, "/dev/console".into());
            } else {
                let _ =
                    fd_table.insert_at(2, console, OpenFlags::write_only(), "/dev/console".into());
            }
        }
        StdioSpec::Null => {
            let _ = fd_table.insert_at(2, null, OpenFlags::write_only(), "/dev/null".into());
        }
        StdioSpec::Pipe => {
            let id = crate::ipc::pipe::create(4096, 0);
            if let Some(write_node) = crate::ipc::pipe::write_node_for_id(id) {
                let _ = fd_table.insert_at(
                    2,
                    write_node,
                    OpenFlags::write_only(),
                    alloc::format!("pipe:{}", id),
                );
            }
            stderr_pipe = id;
        }
    }

    (stdin_pipe, stdout_pipe, stderr_pipe)
}
/// Result of an enhanced spawn: child tid + pipe IDs for piped stdio.
#[derive(Debug, Clone)]
pub struct SpawnExResult {
    pub child_tid: TaskId,
    pub child_pid: u32,
    /// Parent's fd for stdin (parent writes to this fd). 0 if not piped.
    pub stdin_pipe: u64,
    /// Parent's fd for stdout (parent reads from this fd). 0 if not piped.
    pub stdout_pipe: u64,
    /// Parent's fd for stderr (parent reads from this fd). 0 if not piped.
    pub stderr_pipe: u64,
}

/// Enhanced process spawn with explicit argv, env, and stdio piping.
///
/// # Safety
/// Must be called with scheduler lock expectations satisfied.
pub unsafe fn spawn_process_ex<R: BootRuntime>(
    name: &str,
    argv: Vec<Vec<u8>>,
    env: BTreeMap<Vec<u8>, Vec<u8>>,
    stdin_spec: StdioSpec,
    stdout_spec: StdioSpec,
    stderr_spec: StdioSpec,
    boot_arg: u64,
    inherited_handles: Vec<u64>,
    cwd: Option<alloc::string::String>,
) -> Result<SpawnExResult, abi::errors::Errno> {
    let rt = crate::runtime::<R>();
    let modules = rt.modules();
    let module = modules
        .iter()
        .find(|m| boot_module_matches(name, m.name))
        .ok_or(abi::errors::Errno::ENOENT)?;

    let aspace = rt.tasking().make_user_address_space();

    let (mut entry, stack_info, regions, aux_info) =
        crate::task::loader::load_module(rt, aspace, module).ok_or(abi::errors::Errno::ENOEXEC)?;
    entry.arg0 = boot_arg as usize;

    let _irq = rt.irq_disable();

    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut super::types::Scheduler<R>) };

    let id = sched
        .spawn_user_task(
            entry,
            aspace,
            stack_info,
            regions,
            crate::task::TaskPriority::Normal,
            crate::task::Affinity::Any,
        )
        .ok_or(abi::errors::Errno::EAGAIN)?;

    // Determine parent PID
    let cpu_idx = super::current_cpu_index::<R>();
    let ppid = sched
        .state
        .per_cpu
        .get(cpu_idx)
        .and_then(|pc| pc.current)
        .and_then(|ctid| crate::task::registry::get_task::<R>(ctid))
        .and_then(|t| t.process_info.clone())
        .map(|pi| pi.lock().pid)
        .unwrap_or(0);

    // Use provided argv, or fall back to module name
    let final_argv = if argv.is_empty() {
        alloc::vec![module.name.as_bytes().to_vec()]
    } else {
        argv
    };

    // Populate stdio fds in the child's fd_table.
    let tid = crate::runtime::<R>().current_tid();
    let parent_pinfo =
        crate::task::registry::get_task::<R>(tid).and_then(|t| t.process_info.clone());

    let mut fd_table = if let Some(parent_pi) = &parent_pinfo {
        parent_pi.lock().fd_table.clone()
    } else {
        crate::vfs::fd_table::FdTable::new()
    };

    let (stdin_pipe_id, stdout_pipe_id, stderr_pipe_id) =
        setup_stdio_fds::<R>(&mut fd_table, stdin_spec, stdout_spec, stderr_spec);

    // Open the parent-side pipe ends in the parent's fd table so the parent
    // can communicate with the child via normal file descriptors.
    //
    // For stdin PIPE:  parent holds the WRITE end (fd is returned as stdin_pipe).
    // For stdout PIPE: parent holds the READ end (fd is returned as stdout_pipe).
    // For stderr PIPE: parent holds the READ end (fd is returned as stderr_pipe).
    //
    // If there is no parent process, or an end cannot be opened, the pipe will
    // still work from the child's side (it will see EOF when the write end is
    // never written to / the read end is never read from).
    let mut parent_stdin_fd: u64 = 0;
    let mut parent_stdout_fd: u64 = 0;
    let mut parent_stderr_fd: u64 = 0;
    if let Some(parent_pi) = &parent_pinfo {
        let mut plk = parent_pi.lock();
        if stdin_pipe_id != 0 {
            if let Some(write_node) = crate::ipc::pipe::write_node_for_id(stdin_pipe_id) {
                if let Ok(fd) = plk.fd_table.open(
                    write_node,
                    crate::vfs::OpenFlags::write_only(),
                    alloc::format!("pipe:{}", stdin_pipe_id),
                ) {
                    parent_stdin_fd = fd as u64;
                }
            }
        }
        if stdout_pipe_id != 0 {
            if let Some(read_node) = crate::ipc::pipe::read_node_for_id(stdout_pipe_id) {
                if let Ok(fd) = plk.fd_table.open(
                    read_node,
                    crate::vfs::OpenFlags::read_only(),
                    alloc::format!("pipe:{}", stdout_pipe_id),
                ) {
                    parent_stdout_fd = fd as u64;
                }
            }
        }
        if stderr_pipe_id != 0 {
            if let Some(read_node) = crate::ipc::pipe::read_node_for_id(stderr_pipe_id) {
                if let Ok(fd) = plk.fd_table.open(
                    read_node,
                    crate::vfs::OpenFlags::read_only(),
                    alloc::format!("pipe:{}", stderr_pipe_id),
                ) {
                    parent_stderr_fd = fd as u64;
                }
            }
        }
    }

    // Retrieve the mappings Arc from the task so Process and Thread share the
    // same underlying MappingList.
    let task_mappings = crate::task::registry::get_task::<R>(id)
        .map(|t| t.mappings.clone())
        .unwrap_or_else(|| {
            alloc::sync::Arc::new(spin::Mutex::new(
                crate::memory::mappings::MappingList::new(),
            ))
        });

    // Derive the process-owned address-space token (raw u64) from the handle.
    let aspace_raw = rt.tasking().aspace_to_raw(aspace);

    // Create per-process identity with provided argv & env
    let pinfo = alloc::sync::Arc::new(spin::Mutex::new(ProcessInfo {
        pid: id as u32,
        ppid,
        argv: final_argv,
        env,
        auxv: crate::task::exec::build_auxv(&aux_info, rt.page_size() as u64),

        fd_table,
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: if let Some(explicit_cwd) = cwd {
            explicit_cwd
        } else if let Some(parent_pi) = &parent_pinfo {
            parent_pi.lock().cwd.clone()
        } else {
            alloc::string::String::from("/")
        },
        thread_ids: alloc::vec![id],
        exec_in_progress: false,
        exec_path: alloc::format!("/boot/{}", module.name),
        mappings: task_mappings,
        aspace_raw,
    }));

    // Store name, process_info, and initial TLS thread pointer on the task struct.
    if let Some(mut task) = crate::task::registry::get_task_mut::<R>(id) {
        let bytes = module.name.as_bytes();
        let len = bytes.len().min(32);
        task.name[..len].copy_from_slice(&bytes[..len]);
        task.name_len = len as u8;
        task.process_info = Some(pinfo);
        // Apply initial TLS base (FS_BASE on x86_64) for the new process's main thread.
        task.user_fs_base = aux_info.tls_tp;
    }

    rt.irq_restore(_irq);

    Ok(SpawnExResult {
        child_tid: id,
        child_pid: id as u32,
        stdin_pipe: parent_stdin_fd,
        stdout_pipe: parent_stdout_fd,
        stderr_pipe: parent_stderr_fd,
    })
}

pub extern "C" fn user_thread_trampoline<R: BootRuntime>(arg: usize) -> ! {
    let rt = crate::runtime::<R>();
    let entry_ptr = arg as *mut UserEntry;
    let entry = unsafe { *alloc::boxed::Box::from_raw(entry_ptr) };

    crate::kdebug!(
        "USER_TRAMPOLINE: PC=0x{:x} SP=0x{:x} ARG0=0x{:x}",
        entry.entry_pc,
        entry.user_sp,
        entry.arg0
    );

    unsafe { rt.tasking().enter_user(entry) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::TaskPriority;

    // Re-use the shared mock runtime defined in `sched::tests` so that both
    // this module and `sched::mod` share a single `init_runtime` call and a
    // single `MockRuntime` type.  This prevents the "Runtime type mismatch" /
    // double-init panics that occur when each module defines its own mock.
    use crate::sched::tests::{MockRuntime, init_test_env};

    #[test]
    fn boot_module_match_requires_exact_basename() {
        assert!(boot_module_matches("ls", "/bin/ls"));
        assert!(boot_module_matches("/bin/ls", "/bin/ls"));
        assert!(!boot_module_matches("ls", "/bin/smallsh"));
        assert!(!boot_module_matches("/bin/ls", "/bin/smallsh"));
    }

    #[test]
    fn test_spawn_arg_semantics() {
        let _g = init_test_env();

        let mut sched = Scheduler::<MockRuntime>::new();
        sched.next_id = 5000;
        // Manually initialize PerCpu state for the mock
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(0); // Set a dummy current task ID for parent linking

        let cases = [
            (StartupArg::None, 0),
            (StartupArg::BootRegistry, 0x600000),
            (StartupArg::DeviceId(0x123), 0x123),
            (StartupArg::Raw(0x42), 0x42),
        ];

        for (arg, expected) in cases {
            let id = sched.spawn(mock_entry, arg, TaskPriority::Normal, Affinity::Any);
            let task = crate::task::registry::get_task::<MockRuntime>(id).unwrap();

            // In our MockTasking.init_kernel_context, we store arg in MockContext.0
            assert_eq!(task.ctx.0, expected);
            assert_eq!(arg.to_raw(), expected);
        }
    }

    /// Verify that `spawn_user_thread` correctly routes the startup argument
    /// into the task context so the entry function receives it in the first
    /// argument register (e.g. `rdi` on x86_64).
    ///
    /// The `MockRuntime::init_user_context` stores `spec.arg` directly in the
    /// mock context, so asserting `task.ctx.0 == expected` confirms the full
    /// pipeline:
    ///   `spawn_with_arg(entry, arg)`
    ///   → `SYS_SPAWN_THREAD` with `SpawnThreadReq { arg }`
    ///   → `StartupArg::Raw(arg)` passed to `spawn_user_thread`
    ///   → `UserTaskSpec { arg }` passed to `init_user_context`
    ///   → arg placed in first argument register on the target arch
    #[test]
    fn test_spawn_user_thread_arg() {
        let _g = init_test_env();

        let mut sched = Scheduler::<MockRuntime>::new();
        sched.next_id = 9000;
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(0);

        let stack_info = abi::types::StackInfo::default();

        let cases: &[(usize, usize)] = &[
            (0, 0),
            (0x1234, 0x1234),
            (0xDEAD_BEEF, 0xDEAD_BEEF),
            (usize::MAX, usize::MAX),
        ];

        for &(raw_arg, expected) in cases {
            let id = sched.spawn_user_thread(
                0x4000, // mock entry address
                0x8000, // mock user stack pointer
                StartupArg::Raw(raw_arg),
                stack_info,
                TaskPriority::Normal,
                Affinity::Any,
                0,     // tls_base
                false, // detached
            );
            let task = crate::task::registry::get_task::<MockRuntime>(id).unwrap();

            // MockRuntime::init_user_context stores spec.arg in MockContext.0
            assert_eq!(
                task.ctx.0, expected,
                "user thread arg mismatch for raw_arg={:#x}",
                raw_arg
            );
        }
    }

    extern "C" fn mock_entry(_arg: usize) -> ! {
        loop {}
    }
}
