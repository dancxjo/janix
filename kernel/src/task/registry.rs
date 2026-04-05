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

pub fn get_registry<'a, R: BootRuntime>() -> &'a mut TaskRegistry<R> {
    let lock = REGISTRY.lock();
    let ptr = lock.expect("TaskRegistry not initialized");
    unsafe { &mut *(ptr as *mut TaskRegistry<R>) }
}

pub fn get_task<'a, R: BootRuntime>(id: u64) -> Option<&'a Task<R>> {
    get_registry::<R>().get(id)
}

pub fn get_task_mut<'a, R: BootRuntime>(id: u64) -> Option<&'a mut Task<R>> {
    get_registry::<R>().get_mut(id)
}
