//! Memory and stack allocation syscalls

use abi::errors::{Errno, SysResult};

pub fn sys_alloc_stack(pages: usize) -> SysResult<usize> {
    let top =
        unsafe { crate::task::scheduler::alloc_user_stack_current(pages) }.ok_or(Errno::ENOMEM)?;
    Ok(top)
}
