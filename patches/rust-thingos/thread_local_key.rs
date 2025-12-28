// library/std/src/sys/pal/thingos/thread_local_key.rs
use crate::cell::Cell;
use crate::ptr;

pub type Key = usize;

#[inline]
pub unsafe fn create(_dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    0 // Unsupported
}

#[inline]
pub unsafe fn set(_key: Key, _value: *mut u8) {
}

#[inline]
pub unsafe fn get(_key: Key) -> *mut u8 {
    ptr::null_mut()
}

#[inline]
pub unsafe fn destroy(_key: Key) {
}
