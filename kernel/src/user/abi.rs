//! User Mode v0 ABI
//!
//! Registers:
//! - Syscall Number: RAX
//! - Args: RDI, RSI, RDX, R10, R8, R9
//! - Return: RAX
//! - Clobbers: RCX (RIP), R11 (RFLAGS)

pub const SYSCALL_PUTCHAR: u64 = 0;
pub const SYSCALL_TICKS: u64 = 1;
pub const SYSCALL_YIELD: u64 = 2;
pub const SYSCALL_EXIT: u64 = 3;
