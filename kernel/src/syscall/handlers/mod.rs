//! Graph handlers helper

pub mod device;
pub mod logging;
pub mod memory;
pub mod port;
pub mod process;
pub mod root_handlers;
pub mod stream;
pub mod time;
pub mod trace;

pub use device::*;
pub use logging::*;
pub use memory::*;
pub use port::*;
pub use process::*;
pub use root_handlers::*;
pub use stream::*;
pub use time::*;
pub use trace::*;

use crate::root::SymbolShell;
use abi::symbols::{SymbolRefWire, SYMBOL_REF_TAG_ID, SYMBOL_REF_TAG_STR};
use abi::errors::{Errno, SysResult};
use crate::syscall::validate::validate_user_range;
use alloc::string::String;
use abi::wire::SymbolId;

// Helper to copy data in from user
pub unsafe fn copyin(dest: &mut [u8], src_ptr: usize) -> Result<(), Errno> {
    crate::memory::copy_from_user(dest, src_ptr)
}

// Helper to copy data out to user
pub unsafe fn copyout(dest_ptr: usize, src: &[u8]) -> Result<(), Errno> {
    crate::memory::copy_to_user(dest_ptr, src)
}

pub fn root_call(op: crate::root::RootOp) -> SysResult<usize> {
    let reply = crate::root::enqueue(op);
    loop {
        let done = reply.done.load(core::sync::atomic::Ordering::Acquire);
        if done != 0 {
            let status = reply.status.load(core::sync::atomic::Ordering::Relaxed);
            let value = reply.value.load(core::sync::atomic::Ordering::Relaxed);
            if status == 0 {
                return Ok(value as usize);
            } else {
                return Err(abi::errors::errno(status as isize).unwrap_err());
            }
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
    }
}

pub fn read_symbol(ptr: usize) -> SysResult<SymbolShell> {
    validate_user_range(ptr, core::mem::size_of::<SymbolRefWire>(), false)?;
    let mut wire = SymbolRefWire { tag: 0, ptr_or_id: 0, len: 0 };
    let slice = unsafe {
        core::slice::from_raw_parts_mut(&mut wire as *mut _ as *mut u8, core::mem::size_of::<SymbolRefWire>())
    };
    unsafe { copyin(slice, ptr)? };

    match wire.tag {
        SYMBOL_REF_TAG_ID => {
            // ptr_or_id points to SymbolId (16 bytes)
            let id_ptr = wire.ptr_or_id as usize;
            validate_user_range(id_ptr, 16, false)?;
            let mut bytes = [0u8; 16];
            unsafe { copyin(&mut bytes, id_ptr)? };
            Ok(SymbolShell::Id(SymbolId(bytes)))
        },
        SYMBOL_REF_TAG_STR => {
            let len = wire.len as usize;
            if len > 256 {
                return Err(Errno::EINVAL);
            }
            let s_ptr = wire.ptr_or_id as usize;
            validate_user_range(s_ptr, len, false)?;
            let mut buf = [0u8; 256];
            unsafe { copyin(&mut buf[..len], s_ptr)? };
            let s = core::str::from_utf8(&buf[..len]).map_err(|_| Errno::EINVAL)?;
            Ok(SymbolShell::Str(String::from(s)))
        },
        _ => Err(Errno::EINVAL),
    }
}
