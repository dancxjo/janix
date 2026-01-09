pub mod paging;

pub fn halt() -> ! {
    loop {
        unsafe { core::arch::asm!("wfi") };
    }
}
