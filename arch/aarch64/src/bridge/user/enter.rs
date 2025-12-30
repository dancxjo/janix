//! Bridge user entry for AArch64.
//!
//! Stub on AArch64; required for API symmetry.
//! The actual user transition lives in bringup/user/enter.rs.

/// Enter user mode.
///
/// # Safety
/// Caller must ensure entry, stack_top, and arg0 are valid.
pub unsafe fn enter_user(entry: u64, stack_top: u64, arg0: u64) -> ! {
    crate::bringup::user::enter::enter_user(entry, stack_top, arg0)
}
