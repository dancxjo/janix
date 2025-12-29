
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let saved_dest = dest;
    core::arch::asm!(
        "rep movsb",
        inout("rcx") n => _,
        inout("rdi") dest => _,
        inout("rsi") src => _,
        options(nostack, preserves_flags)
    );
    saved_dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut u8, c: i32, n: usize) -> *mut u8 {
    let saved_dest = dest;
    let val = c as u8;
    core::arch::asm!(
        "rep stosb",
        inout("rcx") n => _,
        inout("rdi") dest => _,
        in("al") val,
        options(nostack, preserves_flags)
    );
    saved_dest
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if dest as usize <= src as usize {
        memcpy(dest, src, n);
    } else {
        // Backward copy for overlap handling when dest > src
        let mut i = n;
        while i > 0 {
            i -= 1;
            // Use raw pointer arithmetic to avoid debug checks in ptr::add
            let d = (dest as usize).wrapping_add(i) as *mut u8;
            let s = (src as usize).wrapping_add(i) as *const u8;
            *d = *s;
        }
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *((s1 as usize).wrapping_add(i) as *const u8);
        let b = *((s2 as usize).wrapping_add(i) as *const u8);
        if a != b {
            return (a as i32).wrapping_sub(b as i32);
        }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const u8) -> usize {
    let mut len = 0;
    while *((s as usize).wrapping_add(len) as *const u8) != 0 {
        len += 1;
    }
    len
}
