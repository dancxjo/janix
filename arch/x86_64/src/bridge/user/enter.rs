//! Bridge user entry for x86_64.
//!
//! Forwarding layer from bridge to bringup implementation.

pub use crate::bringup::user::enter::{
    activate_address_space, alloc_user_stack, enter_user_mode, resume_user_mode,
};
pub use crate::bringup::user::UserEntryRegs;

/// Enter user mode with the given parameters.
///
/// # Safety
/// Caller must ensure entry, stack_top, and arg0 are valid.
pub unsafe fn enter_user(entry: u64, stack_top: u64, arg0: u64) -> ! {
    let regs = UserEntryRegs {
        entry_point: entry,
        user_stack: stack_top,
        arg0,
    };
    enter_user_mode(&regs)
}
