
#![allow(unused)]

pub mod os;
pub mod common;
pub mod os_str;
pub use common::*;
pub mod time;
pub mod syscall;
pub mod thread;
pub mod stdio;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if src < dest as *const u8 {
        for i in (0..n).rev() {
            *dest.add(i) = *src.add(i);
        }
    } else {
        for i in 0..n {
            *dest.add(i) = *src.add(i);
        }
    }
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    for i in 0..n {
        *dest.add(i) = *src.add(i);
    }
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(dest: *mut u8, c: i32, n: usize) -> *mut u8 {
    for i in 0..n {
        *dest.add(i) = c as u8;
    }
    dest
}

// Basic entry point since we don't have crt0
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    unsafe extern "C" {
        fn main(argc: isize, argv: *const *const u8) -> i32;
    }
    // TODO: Parse args from stack/registers if available
    let argc = 0;
    let argv = core::ptr::null();
    
    // Call the rustc-generated main wrapper (which calls lang_start)
    main(argc, argv);
    
    // If main returns, exit
    crate::process::exit(0);
}
pub mod fs;
pub mod io;
pub mod net;
pub mod process;
pub mod args;
pub mod env;

pub fn fill_bytes(_v: &mut [u8]) {}
