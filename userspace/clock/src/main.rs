#![no_std]
#![no_main]

// No alloc!

#[no_mangle]
pub extern "C" fn main(_arg: usize) {
    stem::syscall::debug_write("[CLOCK] v4 no alloc\n", 3).ok();
    
    loop {
        stem::syscall::sleep_ms(5000);
    }
}
