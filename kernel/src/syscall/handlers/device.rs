//! Device capability syscalls

use crate::syscall::validate::validate_user_range;
use super::copyin;
use abi::device::{DeviceCall, DeviceKind};
use abi::errors::{Errno, SysResult};

pub fn sys_device_call(call_ptr: usize) -> SysResult<usize> {
    let size = core::mem::size_of::<DeviceCall>();
    validate_user_range(call_ptr, size, true)?;
    let mut call: DeviceCall = unsafe { core::mem::zeroed() };
    let slice = unsafe { core::slice::from_raw_parts_mut(&mut call as *mut _ as *mut u8, size) };
    unsafe {
        copyin(slice, call_ptr)?;
    }
    match call.kind {
        DeviceKind::RtcCmos => Err(Errno::NotSupported),
        _ => Err(Errno::NotSupported),
    }
}

pub fn sys_device_claim(graph_id: usize) -> SysResult<usize> {
    use crate::device_registry::REGISTRY;
    
    let task_id = unsafe { crate::task::scheduler::current_tid_current() };
    
    let mut reg = REGISTRY.lock();
    
    if let Some(device_idx) = reg.find_by_graph_id(graph_id as u64) {
        if let Some(claim_handle) = reg.claim(device_idx, task_id) {
            crate::kinfo!("DEVICE: task {} claimed device {} (handle {})", task_id, graph_id, claim_handle);
            return Ok(claim_handle);
        } else {
            crate::kinfo!("DEVICE: device {} already claimed", graph_id);
            return Err(Errno::EBUSY);
        }
    }
    
    crate::kinfo!("DEVICE: device {} not found in registry", graph_id);
    Err(Errno::ENODEV)
}

pub fn sys_device_map_mmio(_id: usize, _flags: usize) -> SysResult<usize> {
    Err(Errno::NotSupported)
}

pub fn sys_device_irq_subscribe(_id: usize) -> SysResult<usize> {
    Err(Errno::NotSupported)
}

pub fn sys_device_ioport(port: usize, val: usize, write: bool, width: usize) -> SysResult<usize> {
    if write {
        match width {
            1 => crate::ioport_write_u8(port as u16, val as u8),
            2 => crate::ioport_write_u16(port as u16, val as u16),
            4 => crate::ioport_write_u32(port as u16, val as u32),
            _ => return Err(Errno::EINVAL),
        }
        Ok(0)
    } else {
        let ret = match width {
            1 => crate::ioport_read_u8(port as u16) as usize,
            2 => crate::ioport_read_u16(port as u16) as usize,
            4 => crate::ioport_read_u32(port as u16) as usize,
            _ => return Err(Errno::EINVAL),
        };
        Ok(ret)
    }
}
