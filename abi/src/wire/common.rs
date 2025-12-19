

use core::marker::PhantomData;

/// A pointer to a T in user memory.
/// This type is explicitly #[repr(C)] with a u64 layout for strict ABI stability.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UserPtr<T> {
    pub ptr: u64,
    pub _phantom: PhantomData<T>,
}

impl<T> UserPtr<T> {
    pub const fn new(ptr: u64) -> Self {
        Self {
            ptr,
            _phantom: PhantomData,
        }
    }
    
    pub fn is_null(&self) -> bool {
        self.ptr == 0
    }
}

impl<T> Default for UserPtr<T> {
    fn default() -> Self {
        Self::new(0)
    }
}

/// A slice of T in user memory, defined by pointer and length.
/// #[repr(C)] ensures layout is (u64, u64).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UserSlice<T> {
    pub ptr: u64,
    pub len: u64,
    pub _phantom: PhantomData<T>,
}

impl<T> UserSlice<T> {
    pub fn from_slice(slice: &[T]) -> Self {
        Self {
            ptr: slice.as_ptr() as u64,
            len: slice.len() as u64,
            _phantom: PhantomData,
        }
    }

    pub const fn new(ptr: UserPtr<T>, len: u64) -> Self {
        Self {
            ptr: ptr.ptr,
            len,
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for UserSlice<T> {
    fn default() -> Self {
        Self { ptr: 0, len: 0, _phantom: PhantomData }
    }
}

/// A string in user memory (UTF-8 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UserStr {
    pub ptr: u64,
    pub len: u64,
}

impl Default for UserStr {
    fn default() -> Self {
        Self { ptr: 0, len: 0 }
    }
}
