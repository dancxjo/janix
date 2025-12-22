
use core::ffi::c_void;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void {
    let s = s as *mut u8;
    for i in 0..n {
        unsafe { *s.add(i) = c as u8; }
    }
    s as *mut c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    let dest = dest as *mut u8;
    let src = src as *const u8;
    for i in 0..n {
        unsafe { *dest.add(i) = *src.add(i); }
    }
    dest as *mut c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    let dest = dest as *mut u8;
    let src = src as *const u8;
    if src < dest as *const u8 {
        // Backward copy
        for i in (0..n).rev() {
            unsafe { *dest.add(i) = *src.add(i); }
        }
    } else {
        // Forward copy
        for i in 0..n {
            unsafe { *dest.add(i) = *src.add(i); }
        }
    }
    dest as *mut c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> i32 {
    let s1 = s1 as *const u8;
    let s2 = s2 as *const u8;
    for i in 0..n {
        let a = unsafe { *s1.add(i) };
        let b = unsafe { *s2.add(i) };
        if a != b {
            return a as i32 - b as i32;
        }
    }
    0
}
