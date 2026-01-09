pub mod paging;

pub fn halt() -> ! {
    loop {
        unsafe { core::arch::asm!("idle 0") };
    }
}
