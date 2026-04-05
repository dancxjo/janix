//! Task and thread spawning functions.

use crate::task::{
    Affinity, ProcessInfo, StartupArg, StdioBinding, StdioPipeMode, Task, TaskId, TaskPriority,
    TaskState,
};
use crate::{BootRuntime, BootTasking, UserEntry};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use super::types::{Scheduler, DEFAULT_TIMESLICE};
use super::SCHEDULER;
use core::sync::atomic::{AtomicUsize, Ordering};

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
                // NOTE: Automatic SMP bring-up disabled for now. Additional processors
                // will be brought up manually when needed.
                // if trigger_smp && self.state.online_cpu_count < self.total_cpu_count && !self.bringup_in_progress {
                //     if let Some(next_cpu_id) = rt.next_offline_cpu() {
                //         let target_cpu = next_cpu_id.0 as usize;
                //         self.bringup_in_progress = true;
                //         crate::kinfo!("SMP: Spawn triggered bring-up of CPU {}", target_cpu);
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

        let layout = alloc::alloc::Layout::from_size_align(65536, 16).unwrap();
        let stack_base = unsafe { alloc::alloc::alloc(layout) };
        if stack_base.is_null() {
            panic!("Failed to allocate stack for task {}", id);
        }
        let stack_top = (stack_base as u64) + 65536;

        let ctx = rt
            .tasking()
            .init_kernel_context(entry, stack_top, arg.to_raw());

        // Determine target CPU: Balanced among online CPUs.
        // Kernel threads do NOT trigger bring-up by default unless balanced carefully.
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
            kstack_size: 16384,
            kstack_top: stack_top,
            ctx,
            aspace: rt.tasking().active_address_space(),
            simd: crate::simd::SimdState::new(rt),
            exit_code: None,
            is_user: false,
            wake_pending: false,
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

        // If the target CPU is not the current one, send an IPI to wake it up
        if safe_cpu != super::current_cpu_index::<R>() {
            super::DIAG_IPI_SENT.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            rt.send_ipi(safe_cpu, 0x30); // Use IRQ_RESCHED_VECTOR
        }

        // Queue graph node creation (processed after scheduler lock released)
        let parent_tid = self.state.per_cpu[super::current_cpu_index::<R>()].current;
        crate::sched::ring::push_task_created::<R>(id, priority as u8, false, None, parent_tid);
        // Link affinity and initial location
        if let Affinity::Pinned(cpu) = affinity {
            crate::sched::ring::push_task_affinity::<R>(id, cpu);
        }
        // Initial location matches target runq
        crate::sched::ring::push_task_location::<R>(id, safe_cpu);

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
    ) -> TaskId {
        let rt = crate::runtime::<R>();
        let id = self.next_id;
        self.next_id += 1;

        let layout = alloc::alloc::Layout::from_size_align(65536, 16).unwrap();
        let stack_base = unsafe { alloc::alloc::alloc(layout) };
        if stack_base.is_null() {
            panic!("Failed to allocate kernel stack for user thread {}", id);
        }
        let kstack_top = (stack_base as u64) + 65536;

        let aspace = rt.tasking().active_address_space();

        // Inherit mappings and process_info from current task
        let (mappings, parent_pinfo) =
            if let Some(current_id) = self.state.per_cpu[super::current_cpu_index::<R>()].current {
                if let Some(parent) = crate::task::registry::get_task::<R>(current_id) {
                    (parent.mappings.clone(), parent.process_info.clone())
                } else {
                    (
                        alloc::sync::Arc::new(spin::Mutex::new(
                            crate::memory::mappings::MappingList::new(),
                        )),
                        None,
                    )
                }
            } else {
                (
                    alloc::sync::Arc::new(spin::Mutex::new(
                        crate::memory::mappings::MappingList::new(),
                    )),
                    None,
                )
            };

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
            kstack_size: 16384,
            kstack_top,
            ctx,
            aspace,
            simd: crate::simd::SimdState::new(rt),
            exit_code: None,
            is_user: true,
            wake_pending: false,
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

        // If the target CPU is not the current one, send an IPI to wake it up
        if safe_cpu != super::current_cpu_index::<R>() {
            rt.send_ipi(safe_cpu, 0x30); // Use IRQ_RESCHED_VECTOR
        }

        // Queue graph node creation (processed after scheduler lock released)
        let parent_tid = self.state.per_cpu[super::current_cpu_index::<R>()].current;
        crate::sched::ring::push_task_created::<R>(id, priority as u8, true, None, parent_tid);
        // Link affinity and initial location
        if let Affinity::Pinned(cpu) = affinity {
            crate::sched::ring::push_task_affinity::<R>(id, cpu);
        }
        // Initial location matches target runq
        crate::sched::ring::push_task_location::<R>(id, safe_cpu);

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
        let layout = alloc::alloc::Layout::from_size_align(65536, 16).unwrap();
        let stack_base = unsafe { alloc::alloc::alloc(layout) };
        if stack_base.is_null() {
            return None;
        }
        let stack_top = (stack_base as u64) + 65536;

        let user_entry = alloc::boxed::Box::new(entry);
        let entry_ptr = alloc::boxed::Box::into_raw(user_entry) as usize;

        let ctx =
            rt.tasking()
                .init_kernel_context(user_thread_trampoline::<R>, stack_top, entry_ptr);

        let mapping_list = crate::memory::mappings::MappingList { regions };

        // Determine target CPU: New processes trigger bring-up of offline CPUs
        let target_cpu = self.pick_cpu_and_bringup(affinity, true);
        crate::kdebug!(
            "SCHED: Task {} (user task/process) assigned to CPU {}",
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
            kstack_size: 16384,
            kstack_top: stack_top,
            ctx,
            aspace,
            simd: crate::simd::SimdState::new(rt),
            exit_code: None,
            is_user: true,
            wake_pending: false,
            stack_info: Some(stack_info),
            mappings: alloc::sync::Arc::new(spin::Mutex::new(mapping_list)),
            timeslice_remaining: DEFAULT_TIMESLICE,
            affinity,
            last_cpu: Some(safe_cpu),
            name: [0; 32],
            name_len: 0,
            process_info: None,
            enqueued_at_tick: super::TICK_COUNT.load(Ordering::Relaxed),
            base_priority: priority,
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

        // If the target CPU is not the current one, send an IPI to wake it up
        if safe_cpu != super::current_cpu_index::<R>() {
            rt.send_ipi(safe_cpu, 0x30); // Use IRQ_RESCHED_VECTOR
        }

        // Queue graph node creation (processed after scheduler lock released)
        let parent_tid = self.state.per_cpu[super::current_cpu_index::<R>()].current;
        crate::sched::ring::push_task_created::<R>(id, priority as u8, true, None, parent_tid);
        // Link affinity and initial location
        if let Affinity::Pinned(cpu) = affinity {
            crate::sched::ring::push_task_affinity::<R>(id, cpu);
        }
        // Initial location matches target runq
        crate::sched::ring::push_task_location::<R>(id, safe_cpu);

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
    let module = modules.iter().find(|m| m.name.contains(name))?;

    let aspace = rt.tasking().make_user_address_space();

    let (mut entry, stack_info, regions) = crate::task::loader::load_module(rt, aspace, module)?;
    entry.arg0 = arg.to_raw();

    let _irq = rt.irq_disable();

    // Audio pipeline: Pin to CPU 0 to avoid waiting for SMP bring-up
    let affinity = if name.contains("virtio_sound") || name.contains("beeper") {
        // Audio proof-of-life: Run on CPU 0 immediately, don't trigger SMP bring-up
        crate::task::Affinity::Pinned(0)
    } else if name == "bloom" {
        if rt.cpu_total_count() > 1 {
            crate::task::Affinity::Pinned(1)
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
    let cpu_idx = super::current_cpu_index::<R>();
    let ppid = sched
        .state
        .per_cpu
        .get(cpu_idx)
        .and_then(|pc| pc.current)
        .and_then(|ctid| crate::task::registry::get_task::<R>(ctid))
        .and_then(|t| t.process_info.as_ref())
        .map(|pi| pi.lock().pid)
        .unwrap_or(0);

    // Create per-process identity
    let pinfo = alloc::sync::Arc::new(spin::Mutex::new(ProcessInfo {
        pid: id as u32,
        ppid,
        argv: alloc::vec![module.name.as_bytes().to_vec()],
        env: alloc::collections::BTreeMap::new(),
        stdio: [StdioBinding::Console; 3],
        console_stdin: alloc::collections::VecDeque::new(),
    }));

    // Store name and process_info on the task struct
    if let Some(task) = crate::task::registry::get_task_mut::<R>(id) {
        let bytes = module.name.as_bytes();
        let len = bytes.len().min(32);
        task.name[..len].copy_from_slice(&bytes[..len]);
        task.name_len = len as u8;
        task.process_info = Some(pinfo);
    }

    // Queue setting the process name (processed after scheduler lock released)
    crate::sched::ring::push_task_name::<R>(id, Some(module.name));

    rt.irq_restore(_irq);
    Some(id)
}

/// Stdio specification for a single stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StdioSpec {
    /// Inherit parent's handle (no-op for v1; child gets kernel console).
    Inherit,
    /// Attach to null sink/source.
    Null,
    /// Create a pipe; returns the pipe_id for the parent's end.
    Pipe,
}

fn inherited_stdio_or_console<R: BootRuntime>() -> [StdioBinding; 3] {
    let tid = crate::runtime::<R>().current_tid();
    crate::task::registry::get_task::<R>(tid)
        .and_then(|task| task.process_info.as_ref())
        .map(|pi| pi.lock().stdio)
        .unwrap_or([StdioBinding::Console; 3])
}

fn bindings_from_specs<R: BootRuntime>(
    stdin_spec: StdioSpec,
    stdout_spec: StdioSpec,
    stderr_spec: StdioSpec,
    stdin_pipe: u64,
    stdout_pipe: u64,
    stderr_pipe: u64,
) -> [StdioBinding; 3] {
    let inherited = inherited_stdio_or_console::<R>();
    [
        match stdin_spec {
            StdioSpec::Inherit => inherited[0],
            StdioSpec::Null => StdioBinding::Null,
            StdioSpec::Pipe => StdioBinding::Pipe {
                pipe_id: stdin_pipe,
                mode: StdioPipeMode::Read,
            },
        },
        match stdout_spec {
            StdioSpec::Inherit => inherited[1],
            StdioSpec::Null => StdioBinding::Null,
            StdioSpec::Pipe => StdioBinding::Pipe {
                pipe_id: stdout_pipe,
                mode: StdioPipeMode::Write,
            },
        },
        match stderr_spec {
            StdioSpec::Inherit => inherited[2],
            StdioSpec::Null => StdioBinding::Null,
            StdioSpec::Pipe => StdioBinding::Pipe {
                pipe_id: stderr_pipe,
                mode: StdioPipeMode::Write,
            },
        },
    ]
}

/// Result of an enhanced spawn: child tid + pipe IDs for piped stdio.
#[derive(Debug, Clone)]
pub struct SpawnExResult {
    pub child_tid: TaskId,
    /// Pipe ID for stdin (parent writes). 0 if not piped.
    pub stdin_pipe: u64,
    /// Pipe ID for stdout (parent reads). 0 if not piped.
    pub stdout_pipe: u64,
    /// Pipe ID for stderr (parent reads). 0 if not piped.
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
) -> Result<SpawnExResult, abi::errors::Errno> {
    let rt = crate::runtime::<R>();
    let modules = rt.modules();
    let module = modules
        .iter()
        .find(|m| m.name.contains(name))
        .ok_or(abi::errors::Errno::ENOENT)?;

    let aspace = rt.tasking().make_user_address_space();

    let (mut entry, stack_info, regions) =
        crate::task::loader::load_module(rt, aspace, module).ok_or(abi::errors::Errno::ENOEXEC)?;
    entry.arg0 = 0; // No raw arg for ex spawn

    // Create pipes for piped stdio
    let mut stdin_pipe: u64 = 0;
    let mut stdout_pipe: u64 = 0;
    let mut stderr_pipe: u64 = 0;

    if stdin_spec == StdioSpec::Pipe {
        stdin_pipe = crate::ipc::pipe::create(4096, abi::syscall::pipe_flags::NONBLOCK);
    }
    if stdout_spec == StdioSpec::Pipe {
        stdout_pipe = crate::ipc::pipe::create(4096, abi::syscall::pipe_flags::NONBLOCK);
    }
    if stderr_spec == StdioSpec::Pipe {
        stderr_pipe = crate::ipc::pipe::create(4096, abi::syscall::pipe_flags::NONBLOCK);
    }

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
        .and_then(|t| t.process_info.as_ref())
        .map(|pi| pi.lock().pid)
        .unwrap_or(0);

    // Use provided argv, or fall back to module name
    let final_argv = if argv.is_empty() {
        alloc::vec![module.name.as_bytes().to_vec()]
    } else {
        argv
    };

    let stdio = bindings_from_specs::<R>(
        stdin_spec,
        stdout_spec,
        stderr_spec,
        stdin_pipe,
        stdout_pipe,
        stderr_pipe,
    );

    // Create per-process identity with provided argv & env
    let pinfo = alloc::sync::Arc::new(spin::Mutex::new(ProcessInfo {
        pid: id as u32,
        ppid,
        argv: final_argv,
        env,
        stdio,
        console_stdin: alloc::collections::VecDeque::new(),
    }));

    // Store name and process_info on the task struct
    if let Some(task) = crate::task::registry::get_task_mut::<R>(id) {
        let bytes = module.name.as_bytes();
        let len = bytes.len().min(32);
        task.name[..len].copy_from_slice(&bytes[..len]);
        task.name_len = len as u8;
        task.process_info = Some(pinfo);
    }

    // Queue setting the process name
    crate::sched::ring::push_task_name::<R>(id, Some(module.name));

    rt.irq_restore(_irq);

    Ok(SpawnExResult {
        child_tid: id,
        stdin_pipe,
        stdout_pipe,
        stderr_pipe,
    })
}

pub extern "C" fn user_thread_trampoline<R: BootRuntime>(arg: usize) -> ! {
    crate::kdebug!("Trampoline entered. Arg: 0x{:x}", arg);
    let rt = crate::runtime::<R>();
    let entry_ptr = arg as *mut UserEntry;
    let entry = unsafe { *alloc::boxed::Box::from_raw(entry_ptr) };

    crate::kprintln!(
        "USER_TRAMPOLINE: PC=0x{:x} SP=0x{:x} ARG0=0x{:x}",
        entry.entry_pc,
        entry.user_sp,
        entry.arg0
    );

    // Safety: we are entering user mode with the provided entry point
    unsafe { rt.tasking().enter_user(entry) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::TaskPriority;
    use crate::{BootRuntime, BootRuntimeBase, BootTasking, UserEntry, UserTaskSpec};

    #[derive(Clone, Copy, Default)]
    struct MockContext(usize);
    #[derive(Clone, Copy, Default)]
    struct MockAddressSpace(u64);

    struct MockRuntime;
    impl BootRuntimeBase for MockRuntime {
        fn putchar(&self, _c: u8) {}
        fn mono_ticks(&self) -> u64 {
            0
        }
        fn mono_freq_hz(&self) -> u64 {
            1
        }
        fn init_secondary_cpu(&self, _cpu_index: usize) {}
    }
    impl BootRuntime for MockRuntime {
        type Tasking = MockRuntime;
        fn tasking(&self) -> &Self {
            self
        }
        fn halt(&self) -> ! {
            loop {}
        }
        fn irq_disable(&self) -> crate::IrqState {
            crate::IrqState(0)
        }
        fn irq_restore(&self, _state: crate::IrqState) {}
        fn phys_memory_map(&self) -> &'static [crate::PhysRange] {
            &[]
        }
        fn phys_to_virt_offset(&self) -> u64 {
            0
        }
        fn modules(&self) -> &'static [crate::BootModuleDesc] {
            &[]
        }
        fn framebuffer(&self) -> Option<crate::FramebufferInfo> {
            None
        }
    }
    impl BootTasking for MockRuntime {
        type Runtime = MockRuntime;
        type Context = MockContext;
        type AddressSpace = MockAddressSpace;
        fn init(&self, _hhdm: u64) {}
        fn init_kernel_context(
            &self,
            _entry: extern "C" fn(usize) -> !,
            _st: u64,
            _arg: usize,
        ) -> Self::Context {
            MockContext(_arg)
        }
        fn init_user_context(
            &self,
            _spec: UserTaskSpec<Self::AddressSpace>,
            _kst: u64,
        ) -> Self::Context {
            MockContext(_spec.arg)
        }
        unsafe fn switch(&self, _f: &mut Self::Context, _t: &Self::Context, _tid: u64) {}
        unsafe fn enter_user(&self, _e: UserEntry) -> ! {
            loop {}
        }
        fn make_user_address_space(&self) -> Self::AddressSpace {
            MockAddressSpace(0)
        }
        fn active_address_space(&self) -> Self::AddressSpace {
            MockAddressSpace(0)
        }
        fn activate_address_space(&self, _as: Self::AddressSpace) {}
        fn map_page(
            &self,
            _as: Self::AddressSpace,
            _v: u64,
            _p: u64,
            _pr: crate::MapPerms,
            _k: crate::MapKind,
            _a: &dyn crate::FrameAllocatorHook,
        ) -> Result<(), ()> {
            Ok(())
        }
        fn unmap_page(&self, _as: Self::AddressSpace, _v: u64) -> Result<Option<u64>, ()> {
            Ok(None)
        }
        fn translate(&self, _as: Self::AddressSpace, _v: u64) -> Option<u64> {
            None
        }
        fn tlb_flush_page(&self, _v: u64) {}
    }

    static INIT_TESTS: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

    #[test]
    fn test_spawn_arg_semantics() {
        if !INIT_TESTS.swap(true, core::sync::atomic::Ordering::SeqCst) {
            crate::task::registry::init::<MockRuntime>();
        }
        // Mock runtime pointer for the SCHEDULER lock expectation if needed?
        // Scheduler::new() doesn't need the runtime, but Scheduler<R>::spawn needs rt.tasking()
        // We need to set up the global RUNTIME for current() etc to work if used.
        // But here we call sched.spawn directly.

        let mut sched = Scheduler::<MockRuntime>::new();
        sched.next_id = 5000;
        // Manually initialize PerCpu state for the mock
        sched.state.per_cpu.push(crate::sched::state::PerCpu::new());
        sched.state.per_cpu[0].current = Some(0); // Set a dummy current task ID for parent linking

        // We need a way to mock crate::runtime::<MockRuntime>()
        // In kernel/src/lib.rs:
        // pub fn runtime<R: BootRuntime>() -> &'static R { ... RUNTIME.downcast_ref::<R>() ... }

        static RUNTIME: MockRuntime = MockRuntime;
        unsafe { crate::init_runtime(&RUNTIME) };

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

    extern "C" fn mock_entry(_arg: usize) -> ! {
        loop {}
    }
}
