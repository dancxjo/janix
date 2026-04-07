use super::Task;
use crate::BootRuntime;
use alloc::boxed::Box;
use alloc::vec::Vec;
use spin::Mutex;

pub struct TaskRegistry<R: BootRuntime> {
    pub tasks: Vec<Box<Task<R>>>,
}

impl<R: BootRuntime> TaskRegistry<R> {
    pub fn new() -> Self {
        Self {
            tasks: Vec::with_capacity(1024),
        }
    }

    pub fn insert(&mut self, task: Box<Task<R>>) {
        let id = task.id;
        match self.tasks.binary_search_by_key(&id, |t| t.id) {
            Ok(_) => panic!("Task ID {} already exists in registry", id),
            Err(idx) => self.tasks.insert(idx, task),
        }
    }

    pub fn get(&self, id: u64) -> Option<&Task<R>> {
        self.tasks
            .binary_search_by_key(&id, |t| t.id)
            .ok()
            .map(|idx| &*self.tasks[idx])
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut Task<R>> {
        self.tasks
            .binary_search_by_key(&id, |t| t.id)
            .ok()
            .map(move |idx| &mut *self.tasks[idx])
    }

    pub fn remove(&mut self, id: u64) -> Option<Box<Task<R>>> {
        if let Ok(idx) = self.tasks.binary_search_by_key(&id, |t| t.id) {
            Some(self.tasks.remove(idx))
        } else {
            None
        }
    }
}

pub static REGISTRY: Mutex<Option<usize>> = Mutex::new(None);

pub fn init<R: BootRuntime>() {
    let registry = Box::new(TaskRegistry::<R>::new());
    *REGISTRY.lock() = Some(Box::into_raw(registry) as usize);
}

pub struct RegistryGuard<R: BootRuntime> {
    guard: Option<spin::MutexGuard<'static, Option<usize>>>,
    irq_state: crate::IrqState,
    _marker: core::marker::PhantomData<R>,
}

impl<R: BootRuntime> Drop for RegistryGuard<R> {
    fn drop(&mut self) {
        self.guard.take(); // Drop the lock before restoring interrupts
        unsafe {
            crate::irq::irq_restore_erased(self.irq_state);
        }
    }
}

impl<R: BootRuntime> core::ops::Deref for RegistryGuard<R> {
    type Target = TaskRegistry<R>;
    fn deref(&self) -> &Self::Target {
        let ptr = self
            .guard
            .as_ref()
            .unwrap()
            .expect("TaskRegistry not initialized");
        unsafe { &*(ptr as *const TaskRegistry<R>) }
    }
}

impl<R: BootRuntime> core::ops::DerefMut for RegistryGuard<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr = self
            .guard
            .as_mut()
            .unwrap()
            .expect("TaskRegistry not initialized");
        unsafe { &mut *(ptr as *mut TaskRegistry<R>) }
    }
}

pub fn get_registry<R: BootRuntime>() -> RegistryGuard<R> {
    let irq_state = unsafe { crate::irq::irq_disable_erased() };
    RegistryGuard {
        guard: Some(REGISTRY.lock()),
        irq_state,
        _marker: core::marker::PhantomData,
    }
}

pub struct TaskRef<R: BootRuntime> {
    guard: RegistryGuard<R>,
    idx: usize,
}

impl<R: BootRuntime> core::ops::Deref for TaskRef<R> {
    type Target = Task<R>;
    fn deref(&self) -> &Self::Target {
        &self.guard.tasks[self.idx]
    }
}

pub struct TaskMut<R: BootRuntime> {
    guard: RegistryGuard<R>,
    idx: usize,
}

impl<R: BootRuntime> core::ops::Deref for TaskMut<R> {
    type Target = Task<R>;
    fn deref(&self) -> &Self::Target {
        &self.guard.tasks[self.idx]
    }
}

impl<R: BootRuntime> core::ops::DerefMut for TaskMut<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard.tasks[self.idx]
    }
}

pub fn get_task<R: BootRuntime>(id: u64) -> Option<TaskRef<R>> {
    let guard = get_registry::<R>();
    let idx = guard.tasks.binary_search_by_key(&id, |t| t.id).ok()?;
    Some(TaskRef { guard, idx })
}

pub fn get_task_mut<R: BootRuntime>(id: u64) -> Option<TaskMut<R>> {
    let guard = get_registry::<R>();
    let idx = guard.tasks.binary_search_by_key(&id, |t| t.id).ok()?;
    Some(TaskMut { guard, idx })
}
