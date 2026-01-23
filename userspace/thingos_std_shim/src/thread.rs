use alloc::boxed::Box;
use core::any::Any;
use core::cell::UnsafeCell;

use crate::time::Duration;

pub type Result<T> = core::result::Result<T, Box<dyn Any + Send + 'static>>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ThreadId(u64);

#[derive(Clone, Debug)]
pub struct Thread {
    id: ThreadId,
}

impl Thread {
    pub fn id(&self) -> ThreadId {
        self.id
    }
}

struct ThreadResult<T> {
    value: UnsafeCell<Option<T>>,
}

impl<T> ThreadResult<T> {
    fn new() -> Self {
        Self {
            value: UnsafeCell::new(None),
        }
    }

    unsafe fn write(&self, value: T) {
        *self.value.get() = Some(value);
    }

    fn take(self: Box<Self>) -> T {
        unsafe { (*self.value.get()).take() }.expect("thread result missing")
    }
}

unsafe impl<T: Send> Send for ThreadResult<T> {}
unsafe impl<T: Send> Sync for ThreadResult<T> {}

struct ThreadPayload<F, T> {
    func: Option<F>,
    result: *mut ThreadResult<T>,
}

extern "C" fn thread_entry<F, T>(arg: usize) -> !
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let mut payload = unsafe { Box::from_raw(arg as *mut ThreadPayload<F, T>) };
    let func = payload.func.take().expect("thread entry missing closure");
    let result = func();
    unsafe {
        (*payload.result).write(result);
    }
    drop(payload);
    stem::syscall::exit(0)
}

pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let result = Box::new(ThreadResult::new());
    let result_ptr = Box::into_raw(result);
    let payload = Box::new(ThreadPayload {
        func: Some(f),
        result: result_ptr,
    });
    let payload_ptr = Box::into_raw(payload) as usize;
    let stack = stem::stack::Stack::alloc_growing_stack(stem::stack::StackSpec::default())
        .expect("failed to allocate thread stack");
    let tid = stem::syscall::spawn_thread_with_arg(thread_entry::<F, T>, payload_ptr, &stack)
        .expect("failed to spawn thread");
    JoinHandle { tid, result: result_ptr }
}

pub fn current() -> Thread {
    let id = stem::thread::current_id().unwrap_or(0);
    Thread {
        id: ThreadId(id),
    }
}

pub fn yield_now() {
    stem::thread::yield_now();
}

pub fn sleep(duration: Duration) {
    stem::syscall::sleep_ns(duration.as_nanos() as u64);
}

pub struct JoinHandle<T> {
    tid: u64,
    result: *mut ThreadResult<T>,
}

impl<T> JoinHandle<T> {
    pub fn join(self) -> Result<T> {
        stem::thread::wait(self.tid)
            .map_err(|_| Box::new("thread join failed") as Box<dyn Any + Send + 'static>)?;
        let boxed = unsafe { Box::from_raw(self.result) };
        Ok(boxed.take())
    }
}
