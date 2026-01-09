pub mod paging;
pub mod task;

pub fn halt() -> ! {
    loop {
        unsafe { core::arch::asm!("wfi") };
    }
}
