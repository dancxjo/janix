use crate::syscall::validate::{copyout, validate_user_range};
use crate::trace::irq_ring;
use abi::errors::{Errno, SysResult};
use abi::trace::TraceEvent;
use alloc::vec;

pub fn sys_trace_read(ptr: usize, len: usize) -> SysResult<usize> {
    if len > 8192 {
        // Sanity cap
        return Err(Errno::EINVAL);
    }

    // Validate output buffer
    let size_struct = core::mem::size_of::<TraceEvent>();
    let total_bytes = len.checked_mul(size_struct).ok_or(Errno::EINVAL)?;
    validate_user_range(ptr, total_bytes, true)?;

    let mut temp = vec![TraceEvent::Empty; len];

    let count = {
        let ring = irq_ring::IRQ_RING.lock();
        ring.read_all(&mut temp)
    };

    // now copyout
    let bytes = count * size_struct;
    unsafe {
        let src = core::slice::from_raw_parts(temp.as_ptr() as *const u8, bytes);
        copyout(ptr, src)?;
    }

    Ok(count)
}
