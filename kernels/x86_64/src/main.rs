#![no_std]
#![no_main]

#[cfg(not(test))]
use bridge_x86_64::Bridge;
#[cfg(not(test))]
use kernel_core::Kernel;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[cfg(not(test))]
use linked_list_allocator::LockedHeap;

#[cfg(not(test))]
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize allocator if needed (omitted for stub)
    let k = Kernel::new(Bridge);
    k.boot();
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn main() {}
