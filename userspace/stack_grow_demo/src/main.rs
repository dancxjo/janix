#![no_std]
#![no_main]

use stem::info;

const FRAME_BYTES: usize = 4096;

fn touch_stack(depth: usize, max: usize) {
    let mut buf = [0u8; FRAME_BYTES];
    unsafe {
        core::ptr::write_volatile(buf.as_mut_ptr(), depth as u8);
    }

    if depth % 64 == 0 {
        info!("[stack_grow_demo] depth={}", depth);
    }

    if depth < max {
        touch_stack(depth + 1, max);
    }

    unsafe {
        core::ptr::read_volatile(buf.as_ptr());
    }
}

#[stem::main]
fn main() -> ! {
    info!("[stack_grow_demo] starting");
    touch_stack(0, 256);
    info!("[stack_grow_demo] completed");
    loop {
        stem::yield_now();
    }
}
