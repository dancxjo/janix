#![no_std]

//! User Mode v0 ABI
//!
//! Registers:
//! - Syscall Number: RAX
//! - Args: RDI, RSI, RDX, R10, R8, R9
//! - Return: RAX
//! - Clobbers: RCX (RIP), R11 (RFLAGS)

pub mod root;
pub mod syscall;
