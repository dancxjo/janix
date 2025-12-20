use core::arch::asm;

// SBI Extension IDs
const SBI_EID_TIME: usize = 0x54494D45;

// SBI Function IDs
const SBI_FID_SET_TIMER: usize = 0;

#[inline(always)]
pub fn set_timer(stime_value: u64) {
    unsafe {
        // SBI Call: sbi_set_timer(stime_value)
        // a7 = EID, a6 = FID, a0 = stime_value
        // Returns: a0 = error, a1 = value (unused here)
        asm!(
            "ecall",
            in("a7") SBI_EID_TIME,
            in("a6") SBI_FID_SET_TIMER,
            in("a0") stime_value,
            lateout("a0") _,
            lateout("a1") _,
            options(nostack)
        );
    }
}
