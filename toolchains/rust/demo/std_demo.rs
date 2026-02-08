#![feature(restricted_std)]

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    println!("std demo: hello from ThingOS std");

    let mut values = Vec::new();
    values.push(7_u64);
    values.push(35_u64);
    println!("std demo: alloc ok, sum={}", values.iter().sum::<u64>());

    let start = Instant::now();
    thread::sleep(Duration::from_millis(50));
    println!("std demo: sleep elapsed {:?}", start.elapsed());

    panic!("std demo: intentional panic to verify abort path");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe { core::ptr::copy_nonoverlapping(src, dst, n) };
    dst
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe { core::ptr::copy(src, dst, n) };
    dst
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(dst: *mut u8, c: i32, n: usize) -> *mut u8 {
    unsafe { core::ptr::write_bytes(dst, c as u8, n) };
    dst
}
