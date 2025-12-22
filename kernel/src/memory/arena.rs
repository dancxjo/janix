use core::mem;
use core::ptr;

// 1MB scratch buffer
pub const ARENA_SIZE: usize = 1024 * 1024;

pub struct FrameArena {
    buffer: [u8; ARENA_SIZE],
    offset: usize,
}

impl FrameArena {
    const fn new() -> Self {
        Self {
            buffer: [0; ARENA_SIZE],
            offset: 0,
        }
    }
}

static mut GLOBAL_ARENA: FrameArena = FrameArena::new();

/// Reset the arena offset to 0.
/// CAUTION: Invalidates all references previously returned by `alloc`.
pub fn reset() {
    unsafe {
        let ptr = &raw mut GLOBAL_ARENA;
        (*ptr).offset = 0;
    }
}

/// Allocate a value in the arena.
/// Returns a mutable reference with 'static lifetime, but it is only valid until next reset.
pub fn alloc<T>(value: T) -> Option<&'static mut T> {
    unsafe {
        let ptr = &raw mut GLOBAL_ARENA;
        let arena = &mut *ptr;

        let layout = core::alloc::Layout::new::<T>();
        let current_ptr = arena.buffer.as_ptr().add(arena.offset) as usize;

        // Align up
        let align_offset = (layout.align() - (current_ptr % layout.align())) % layout.align();
        let new_offset = arena.offset + align_offset;

        if new_offset + layout.size() > ARENA_SIZE {
            return None;
        }

        // Write value
        let dest_ptr = arena.buffer.as_mut_ptr().add(new_offset) as *mut T;
        ptr::write(dest_ptr, value);

        arena.offset = new_offset + layout.size();

        Some(&mut *dest_ptr)
    }
}

/// Get a slice of raw bytes from the arena.
pub fn alloc_bytes(size: usize, align: usize) -> Option<&'static mut [u8]> {
    unsafe {
        let ptr = &raw mut GLOBAL_ARENA;
        let arena = &mut *ptr;

        let current_ptr = arena.buffer.as_ptr().add(arena.offset) as usize;
        let align_offset = (align - (current_ptr % align)) % align;
        let new_offset = arena.offset + align_offset;

        if new_offset + size > ARENA_SIZE {
            return None;
        }

        let slice_ptr = arena.buffer.as_mut_ptr().add(new_offset);
        let slice = core::slice::from_raw_parts_mut(slice_ptr, size);

        arena.offset = new_offset + size;

        Some(slice)
    }
}

/// Get current usage
pub fn usage() -> usize {
    unsafe {
        let ptr = &raw const GLOBAL_ARENA;
        (*ptr).offset
    }
}
