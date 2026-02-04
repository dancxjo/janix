
//! Task and thread spawning functions.

use crate::task::{StartupArg, Task, TaskId, TaskPriority, TaskState, Affinity};
use crate::{BootRuntime, BootTasking, UserEntry};

use super::SCHEDULER;
use super::types::{DEFAULT_TIMESLICE, Scheduler};
use core::sync::atomic::{AtomicUsize, Ordering};

// Global round-robin index for CPU selection
static RR_IDX: AtomicUsize = AtomicUsize::new(0);

impl<R: BootRuntime> Scheduler<R> {
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

        let layout = alloc::alloc::Layout::from_size_align(16384, 16).unwrap();
        let stack_base = unsafe { alloc::alloc::alloc(layout) };
        if stack_base.is_null() {
            panic!("Failed to allocate stack for task {}", id);
        }
        let stack_top = (stack_base as u64) + 16384;

        let ctx = rt
            .tasking()
            .init_kernel_context(entry, stack_top, arg.to_raw());

        // Determine target CPU
        let target_cpu = match affinity {
            Affinity::Pinned(cpu) => cpu,
            Affinity::Any => {
                let count = rt.cpu_count();
                // Simple Round Robin
                let idx = RR_IDX.fetch_add(1, Ordering::Relaxed);
                idx % count
            }
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
        };

        self.tasks.push(task);
        // Push to target CPU's run queue
        let cpu_count = self.per_cpu.len(); // Should match rt.cpu_count()
        let safe_cpu = if target_cpu < cpu_count { target_cpu } else { 0 };
        self.per_cpu[safe_cpu].runq[priority as usize].push_back(id);

        // Queue graph node creation (processed after scheduler lock released)
        let parent_tid = self.per_cpu[super::current_cpu_index::<R>()].current;
        super::graphify::create_thread_node(id, priority as u8, false, None, parent_tid);
        // Link affinity and initial location
        if let Affinity::Pinned(cpu) = affinity {
             super::graphify::set_affinity_node(id, cpu);
        }
        // Initial location matches target runq
        super::graphify::update_task_location(id, safe_cpu);

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

        let layout = alloc::alloc::Layout::from_size_align(16384, 16).unwrap();
        let stack_base = unsafe { alloc::alloc::alloc(layout) };
        if stack_base.is_null() {
            panic!("Failed to allocate kernel stack for user thread {}", id);
        }
        let kstack_top = (stack_base as u64) + 16384;

        let aspace = rt.tasking().active_address_space();

        // Inherit mappings from current task
        let mappings = if let Some(current_id) = self.per_cpu[super::current_cpu_index::<R>()].current {
            if let Some(parent) = self.tasks.iter().find(|t| t.id == current_id) {
                parent.mappings.clone()
            } else {
                alloc::sync::Arc::new(spin::Mutex::new(crate::memory::mappings::MappingList::new()))
            }
        } else {
            alloc::sync::Arc::new(spin::Mutex::new(crate::memory::mappings::MappingList::new()))
        };

        let spec = crate::UserTaskSpec {
            entry: entry as u64,
            stack_top: stack as u64,
            aspace,
            arg: arg.to_raw(),
        };

        let ctx = rt.tasking().init_user_context(spec, kstack_top);

        // Determine target CPU
        let target_cpu = match affinity {
            Affinity::Pinned(cpu) => cpu,
            Affinity::Any => {
                let count = rt.cpu_count();
                let idx = RR_IDX.fetch_add(1, Ordering::Relaxed);
                idx % count
            }
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
        };

        self.tasks.push(task);
        // Push to target CPU's run queue
        let cpu_count = self.per_cpu.len();
        let safe_cpu = if target_cpu < cpu_count { target_cpu } else { 0 };
        self.per_cpu[safe_cpu].runq[priority as usize].push_back(id);

        // Queue graph node creation (processed after scheduler lock released)
        let parent_tid = self.per_cpu[super::current_cpu_index::<R>()].current;
        super::graphify::create_thread_node(id, priority as u8, true, None, parent_tid);
        // Link affinity and initial location
        if let Affinity::Pinned(cpu) = affinity {
             super::graphify::set_affinity_node(id, cpu);
        }
        // Initial location matches target runq
        super::graphify::update_task_location(id, safe_cpu);

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
        let layout = alloc::alloc::Layout::from_size_align(16384, 16).unwrap();
        let stack_base = unsafe { alloc::alloc::alloc(layout) };
        if stack_base.is_null() {
            return None;
        }
        let stack_top = (stack_base as u64) + 16384;

        let user_entry = alloc::boxed::Box::new(entry);
        let entry_ptr = alloc::boxed::Box::into_raw(user_entry) as usize;

        let ctx =
            rt.tasking()
                .init_kernel_context(user_thread_trampoline::<R>, stack_top, entry_ptr);

        let mapping_list = crate::memory::mappings::MappingList { regions };

        // Determine target CPU
        let target_cpu = match affinity {
            Affinity::Pinned(cpu) => cpu,
            Affinity::Any => {
                let count = rt.cpu_count();
                let idx = RR_IDX.fetch_add(1, Ordering::Relaxed);
                idx % count
            }
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
        };

        self.tasks.push(task);
        // Push to target CPU's run queue
        let cpu_count = self.per_cpu.len();
        let safe_cpu = if target_cpu < cpu_count { target_cpu } else { 0 };
        self.per_cpu[safe_cpu].runq[priority as usize].push_back(id);

        // Queue graph node creation (processed after scheduler lock released)
        let parent_tid = self.per_cpu[super::current_cpu_index::<R>()].current;
        super::graphify::create_thread_node(id, priority as u8, true, None, parent_tid);
        // Link affinity and initial location
        if let Affinity::Pinned(cpu) = affinity {
             super::graphify::set_affinity_node(id, cpu);
        }
        // Initial location matches target runq
        super::graphify::update_task_location(id, safe_cpu);

        Some(id)
    }
}

pub fn spawn<R: BootRuntime>(entry: extern "C" fn(usize) -> !, arg: StartupArg) -> TaskId {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
    let id = sched.spawn(entry, arg, crate::task::TaskPriority::Normal, crate::task::Affinity::Any);
    rt.irq_restore(_irq);
    id
}

pub fn spawn_with_priority<R: BootRuntime>(
    entry: extern "C" fn(usize) -> !,
    arg: StartupArg,
    priority: crate::task::TaskPriority,
) -> TaskId {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    let ptr = lock.expect("Scheduler not initialized");
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
    let id = sched.spawn(entry, arg, priority, crate::task::Affinity::Any);
    rt.irq_restore(_irq);
    id
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
    let id = sched.spawn_user_thread(entry, stack, arg, stack_info, priority, crate::task::Affinity::Any);
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
    let id = sched.spawn_user_task(entry, aspace, stack_info, regions, priority, crate::task::Affinity::Any);
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
    
    // HACK: Bloom pinning for smoke test
    let affinity = if name == "bloom" {
        if rt.cpu_count() > 1 {
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

    // Queue setting the process name (processed after scheduler lock released)
    super::graphify::set_name(id, module.name);

    rt.irq_restore(_irq);
    Some(id)
}

pub extern "C" fn user_thread_trampoline<R: BootRuntime>(arg: usize) -> ! {
    crate::kinfo!("Trampoline entered. Arg: 0x{:x}", arg);
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

    struct MockArch;
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
        unsafe fn switch(&self, _f: &mut Self::Context, _t: &Self::Context) {}
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

    #[test]
    fn test_spawn_arg_semantics() {
        // Mock runtime pointer for the SCHEDULER lock expectation if needed?
        // Scheduler::new() doesn't need the runtime, but Scheduler<R>::spawn needs rt.tasking()
        // We need to set up the global RUNTIME for current() etc to work if used.
        // But here we call sched.spawn directly.

        let mut sched = Scheduler::<MockRuntime>::new();

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
            let id = sched.spawn(mock_entry, arg, TaskPriority::Normal);
            let task = sched.tasks.iter().find(|t| t.id == id).unwrap();

            // In our MockTasking.init_kernel_context, we store arg in MockContext.0
            assert_eq!(task.ctx.0, expected);
            assert_eq!(arg.to_raw(), expected);
        }
    }

    extern "C" fn mock_entry(_arg: usize) -> ! {
        loop {}
    }
}
