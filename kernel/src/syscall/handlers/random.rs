//! Random/entropy syscall handlers.

use crate::syscall::validate::{copyout, validate_user_range};
use abi::errors::{Errno, SysResult};

/// SYS_GETRANDOM: Fill a user buffer with random bytes.
///
/// Args: buf_ptr, buf_len.
/// Returns: 0 on success.
pub fn sys_getrandom(buf_ptr: usize, buf_len: usize) -> SysResult<usize> {
    if buf_len == 0 {
        return Ok(0);
    }

    // Cap at 256 bytes per call to avoid holding kernel resources too long
    let len = buf_len.min(256);
    validate_user_range(buf_ptr, len, true)?;

    let mut kbuf = [0u8; 256];
    crate::entropy::fill(&mut kbuf[..len])?;

    unsafe {
        copyout(buf_ptr, &kbuf[..len])?;
    }
    Ok(0)
}
