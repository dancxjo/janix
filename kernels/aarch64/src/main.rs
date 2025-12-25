#![no_std]
#![no_main]

use bridge_aarch64::Bridge;
use kernel_core::Kernel;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let k = Kernel::new(Bridge);
    k.boot();
}
