//! Syscall handler implementations
//!
//! Organized into focused modules by function category.

mod device;
mod logging;
mod memory;
mod port;
pub mod stream;
mod process;
mod root_handlers;
mod time;
mod trace;

// Re-export all syscall handlers
pub use device::*;
pub use logging::*;
pub use memory::*;
pub use port::*;
pub use process::*;
pub use root_handlers::*;
pub use time::*;
pub use trace::*;

// Shared utilities used by multiple handlers
use crate::root::{self as root_svc, RootOp, SymbolShell};
use crate::syscall::validate::{copyin, copyout};
use abi::errors::{Errno, SysResult};
use abi::symbols::{SYMBOL_REF_TAG_ID, SYMBOL_REF_TAG_STR, SymbolRefWire};
use alloc::string::String;
use core::sync::atomic::Ordering;

/// Blocking call to Root service
pub(crate) fn root_call(op: RootOp) -> SysResult<usize> {
    let reply = root_svc::enqueue(op);
    loop {
        let done = reply.done.load(Ordering::Acquire);
        if done != 0 {
            let status = reply.status.load(Ordering::Relaxed);
            let value = reply.value.load(Ordering::Relaxed);

            #[cfg(feature = "diagnostic-apps")]
            crate::ktrace!("ROOT_CALL_DEBUG: status={} value={:x}", status, value);

            return if status == 0 {
                Ok(value as usize)
            } else {
                Err(Errno::EIO)
            };
        }
        unsafe {
            crate::task::scheduler::yield_now_current();
        }
    }
}

/// Read a symbol reference from userspace
pub(crate) fn read_symbol(ptr: usize) -> SysResult<SymbolShell> {
    use crate::syscall::validate::validate_user_range;

    let size = core::mem::size_of::<SymbolRefWire>();
    validate_user_range(ptr, size, false)?;

    let mut wire: SymbolRefWire = unsafe { core::mem::zeroed() };
    let slice = unsafe { core::slice::from_raw_parts_mut(&mut wire as *mut _ as *mut u8, size) };
    unsafe {
        copyin(slice, ptr)?;
    }

    match wire.tag {
        SYMBOL_REF_TAG_ID => Ok(SymbolShell::Id(wire.ptr_or_id as u32)),
        SYMBOL_REF_TAG_STR => {
            let s_ptr = wire.ptr_or_id as usize;
            let s_len = wire.len as usize;
            if s_len > 256 {
                crate::kprintln!("SYSCALL: Symbol string too long: {}", s_len);
                return Err(Errno::EINVAL);
            }

            if let Err(e) = validate_user_range(s_ptr, s_len, false) {
                crate::kprintln!(
                    "SYSCALL: Symbol string ptr {:x} len {} validation failed: {:?}",
                    s_ptr,
                    s_len,
                    e
                );
                return Err(e);
            }
            let mut buf = [0u8; 256];
            unsafe {
                copyin(&mut buf[..s_len], s_ptr)?;
            }

            match core::str::from_utf8(&buf[..s_len]) {
                Ok(s) => Ok(SymbolShell::Str(String::from(s))),
                Err(_) => {
                    crate::kprintln!("SYSCALL: Symbol string invalid utf8");
                    Err(Errno::EINVAL)
                }
            }
        }
        _ => {
            crate::kprintln!("SYSCALL: Unknown symbol tag: {}", wire.tag);
            Err(Errno::EINVAL)
        }
    }
}
