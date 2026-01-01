//! Syscall dispatch
//!
//! Provides the kernel's system call interface. Syscalls are dispatched
//! by number and return (status, val0, val1).

use core::slice;

use crate::log::{self, Level};
use crate::machine::{self, MmioFlags, MmioRange};
use crate::symbols;

/// Syscall numbers
pub mod nr {
    pub const SYS_VERSION_GET: u32 = 0;
    pub const SYS_LOG_EMIT: u32 = 1;
    pub const SYS_SYMBOL_INTERN: u32 = 2;
    pub const SYS_MACHINE: u32 = 3;
    pub const SYS_THING_CREATE: u32 = 10;
    pub const SYS_THING_SET_PAYLOAD: u32 = 11;
    pub const SYS_LINK_CREATE: u32 = 20;
    pub const SYS_PROC_SPAWN: u32 = 100;
    pub const SYS_PROC_EXIT: u32 = 101;
    pub const SYS_SCHED_YIELD: u32 = 200;
}

/// Machine syscall operations
pub mod machine_op {
    pub const CONSOLE_WRITE: u64 = 0;
    pub const MMIO_MAP: u64 = 1;
}

/// Error codes
pub mod err {
    /// Operation not implemented
    pub const ENOSYS: i32 = -38;
    /// Invalid argument
    pub const EINVAL: i32 = -22;
    /// Bad address
    pub const EFAULT: i32 = -14;
}

/// Syscall result type
pub type SyscallResult = (i32, u64, u64);

/// Initialize syscall dispatch
pub fn init() {
    // Nothing to initialize yet - dispatch table is static
}

/// Main syscall dispatch function
///
/// Called by the architecture layer when a syscall trap occurs.
/// Returns (status, value0, value1).
pub fn dispatch(nr: u32, a0: u64, a1: u64, a2: u64, a3: u64, _a4: u64, _a5: u64) -> SyscallResult {
    match nr {
        nr::SYS_VERSION_GET => sys_version_get(),
        nr::SYS_LOG_EMIT => sys_log_emit(a0, a1, a2),
        nr::SYS_SYMBOL_INTERN => sys_symbol_intern(a0, a1),
        nr::SYS_MACHINE => sys_machine(a0, a1, a2, a3),
        _ => (err::ENOSYS, 0, 0),
    }
}

/// SYS_VERSION_GET: Get kernel version
///
/// Returns: (0, major, minor)
fn sys_version_get() -> SyscallResult {
    const MAJOR: u64 = 0;
    const MINOR: u64 = 3;
    (0, MAJOR, MINOR)
}

/// SYS_LOG_EMIT: Emit a log message
///
/// a0: level (0=trace, 1=debug, 2=info, 3=warn, 4=error)
/// a1: pointer to message
/// a2: message length
///
/// Returns: (0, thing_id_high, thing_id_low) or (error, 0, 0)
fn sys_log_emit(level_raw: u64, _msg_ptr: u64, _msg_len: u64) -> SyscallResult {
    // Validate level
    let level = match level_raw {
        0 => Level::Trace,
        1 => Level::Debug,
        2 => Level::Info,
        3 => Level::Warn,
        4 => Level::Error,
        _ => return (err::EINVAL, 0, 0),
    };

    // For now, we can't safely read user memory, so just log a placeholder
    // In a real implementation, we'd validate the pointer and copy the data
    let subsystem = symbols::well_known(b"userland");

    // TODO: Properly read from user memory with validation
    // For now, log that we received a log request
    let placeholder = b"[user log request]";

    if let Some(id) = log::log_emit(level, subsystem, placeholder) {
        (0, id.high(), id.low())
    } else {
        (err::EFAULT, 0, 0)
    }
}

/// SYS_SYMBOL_INTERN: Intern a symbol string
///
/// a0: pointer to string
/// a1: string length
///
/// Returns: (0, symbol_id, 0) or (error, 0, 0)
fn sys_symbol_intern(_str_ptr: u64, _str_len: u64) -> SyscallResult {
    // TODO: Properly read from user memory with validation
    // For now, return a placeholder symbol
    let placeholder = b"user_symbol";
    let id = symbols::intern(placeholder);
    (0, id.0, 0)
}

/// SYS_MACHINE: Machine operations
///
/// a0: op (0=console_write, 1=mmio_map)
/// a1..a3: op-specific arguments
fn sys_machine(op: u64, a1: u64, a2: u64, a3: u64) -> SyscallResult {
    match op {
        machine_op::CONSOLE_WRITE => sys_machine_console_write(a1, a2),
        machine_op::MMIO_MAP => sys_machine_mmio_map(a1, a2, a3),
        _ => (err::EINVAL, 0, 0),
    }
}

/// SYS_MACHINE[console_write]: write bytes to the machine console
fn sys_machine_console_write(ptr: u64, len: u64) -> SyscallResult {
    if ptr == 0 {
        return (err::EFAULT, 0, 0);
    }

    // For now, assume kernel/user share address space for early logging.
    let bytes = unsafe { slice::from_raw_parts(ptr as *const u8, len as usize) };
    let written = machine::machine().console_write(bytes) as u64;
    (0, written, 0)
}

/// SYS_MACHINE[mmio_map]: map a physical MMIO range
fn sys_machine_mmio_map(phys: u64, len: u64, flags_raw: u64) -> SyscallResult {
    if len == 0 {
        return (err::EINVAL, 0, 0);
    }

    let flags = match MmioFlags::from_bits(flags_raw as u32) {
        Some(f) => f,
        None => return (err::EINVAL, 0, 0),
    };

    let range = MmioRange {
        phys,
        len: len as usize,
    };

    if let Some(mapping) = machine::machine().mmio_map(range, flags) {
        (0, mapping.virt, mapping.len as u64)
    } else {
        (err::EFAULT, 0, 0)
    }
}
