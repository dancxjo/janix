use crate::machine::input;
use crate::syscall::user_mem;
use abi::cap::CapOp;

pub fn sys_input_read(buf: *mut u8, len: usize) -> Result<usize, user_mem::SysError> {
    user_mem::require_current_cap(CapOp::InputRead, None)?;

    if len == 0 {
        return Ok(0);
    }

    let ptr = buf as u64;
    user_mem::validate_user_write(ptr, len)?;

    let mut scratch = alloc::vec![0u8; len];
    let written = input::read_scancodes(&mut scratch);
    user_mem::copy_to_user(ptr, &scratch, written)?;

    Ok(written)
}
