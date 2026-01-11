
use alloc::sync::Arc;
use spin::Mutex;

#[derive(Clone)]
pub struct Bytespace {
    pub ptr: usize,
    pub len: usize,
}

pub type BytespaceHandle = Arc<Mutex<Bytespace>>;

pub fn create(len: usize) -> BytespaceHandle {
    // For v0.1, we allocate from heap.
    // Real implementation would map pages.
    let layout = alloc::alloc::Layout::from_size_align(len, 4096).unwrap();
    let ptr = unsafe { alloc::alloc::alloc_zeroed(layout) };
    
    Arc::new(Mutex::new(Bytespace {
        ptr: ptr as usize,
        len,
    }))
}

pub fn create_from_ptr(ptr: usize, len: usize) -> BytespaceHandle {
    Arc::new(Mutex::new(Bytespace {
        ptr,
        len,
    }))
}
