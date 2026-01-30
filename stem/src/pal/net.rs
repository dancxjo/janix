//! Network platform abstraction.
//!
//! Provides access to the primary NIC via syscalls.

use crate::syscall;
use abi::errors::SysResult;

/// Get the MAC address of the primary NIC.
pub fn nic_mac(out: &mut [u8; 6]) -> SysResult<()> {
    let result = unsafe {
        syscall::syscall6(
            abi::syscall::SYS_NIC_MAC,
            out.as_mut_ptr() as usize,
            0,
            0,
            0,
            0,
            0,
        )
    };
    
    if result >= 0 {
        Ok(())
    } else {
        Err(abi::errors::from_syscall_result(result))
    }
}

/// Check if the link is up.
pub fn nic_link_up() -> SysResult<bool> {
    let result = unsafe {
        syscall::syscall6(abi::syscall::SYS_NIC_LINK_UP, 0, 0, 0, 0, 0, 0)
    };
    
    if result >= 0 {
        Ok(result != 0)
    } else {
        Err(abi::errors::from_syscall_result(result))
    }
}

/// Poll for a received frame.
///
/// Returns the frame length if available, or Ok(0) if no frame.
pub fn nic_poll_rx(buffer: &mut [u8]) -> SysResult<usize> {
    let result = unsafe {
        syscall::syscall6(
            abi::syscall::SYS_NIC_POLL_RX,
            buffer.as_mut_ptr() as usize,
            buffer.len(),
            0,
            0,
            0,
            0,
        )
    };
    
    if result >= 0 {
        Ok(result as usize)
    } else {
        Err(abi::errors::from_syscall_result(result))
    }
}

/// Transmit a frame.
pub fn nic_tx(frame: &[u8]) -> SysResult<()> {
    let result = unsafe {
        syscall::syscall6(
            abi::syscall::SYS_NIC_TX,
            frame.as_ptr() as usize,
            frame.len(),
            0,
            0,
            0,
            0,
        )
    };
    
    if result >= 0 {
        Ok(())
    } else {
        Err(abi::errors::from_syscall_result(result))
    }
}
