//! Network syscall handlers

use crate::net::{primary_nic, Nic};
use crate::syscall::validate::{copyout, validate_user_range};
use abi::errors::{Errno, SysResult};

/// SYS_NIC_MAC: Get the MAC address of the primary NIC
///
/// Args:
///   - args[0]: Output buffer pointer (must be at least 6 bytes)
///
/// Returns: 0 on success
pub fn sys_nic_mac(out_ptr: usize) -> SysResult<usize> {
    validate_user_range(out_ptr, 6, true)?;
    
    let nic_lock = primary_nic().ok_or(Errno::ENODEV)?;
    let nic_guard = nic_lock.lock();
    let nic = nic_guard.as_ref().ok_or(Errno::ENODEV)?;
    
    let mac = nic.mac();
    unsafe {
        copyout(out_ptr, &mac)?;
    }
    
    Ok(0)
}

/// SYS_NIC_LINK_UP: Check if the link is up
///
/// Returns: 1 if link is up, 0 if down
pub fn sys_nic_link_up() -> SysResult<usize> {
    let nic_lock = primary_nic().ok_or(Errno::ENODEV)?;
    let nic_guard = nic_lock.lock();
    let nic = nic_guard.as_ref().ok_or(Errno::ENODEV)?;
    
    Ok(if nic.link_up() { 1 } else { 0 })
}

/// SYS_NIC_POLL_RX: Poll for received frame
///
/// Args:
///   - args[0]: Output buffer pointer
///   - args[1]: Buffer length
///
/// Returns: Frame length if available, 0 if no frame
pub fn sys_nic_poll_rx(out_ptr: usize, out_len: usize) -> SysResult<usize> {
    if out_len > 0 {
        validate_user_range(out_ptr, out_len, true)?;
    }
    
    let nic_lock = primary_nic().ok_or(Errno::ENODEV)?;
    let mut nic_guard = nic_lock.lock();
    let nic = nic_guard.as_mut().ok_or(Errno::ENODEV)?;
    
    match nic.poll_rx() {
        Some(frame) => {
            let frame_len = frame.len();
            if out_len < frame_len {
                return Err(Errno::ENOBUFS);
            }
            unsafe {
                copyout(out_ptr, &frame[..frame_len])?;
            }
            Ok(frame_len)
        }
        None => Ok(0),
    }
}

/// SYS_NIC_TX: Transmit a frame
///
/// Args:
///   - args[0]: Frame buffer pointer
///   - args[1]: Frame length
///
/// Returns: 0 on success
pub fn sys_nic_tx(frame_ptr: usize, frame_len: usize) -> SysResult<usize> {
    if frame_len == 0 || frame_len > 2048 {
        return Err(Errno::EINVAL);
    }
    
    validate_user_range(frame_ptr, frame_len, false)?;
    
    let nic_lock = primary_nic().ok_or(Errno::ENODEV)?;
    let mut nic_guard = nic_lock.lock();
    let nic = nic_guard.as_mut().ok_or(Errno::ENODEV)?;
    
    // Copy frame from userspace
    let mut buffer = [0u8; 2048];
    unsafe {
        crate::syscall::validate::copyin(&mut buffer[..frame_len], frame_ptr)?;
    }
    
    nic.tx(&buffer[..frame_len]).map_err(|e| {
        match e {
            crate::net::TxError::QueueFull => Errno::EAGAIN,
            crate::net::TxError::FrameTooLarge => Errno::EMSGSIZE,
            crate::net::TxError::NotReady => Errno::ENODEV,
        }
    })?;
    
    Ok(0)
}
