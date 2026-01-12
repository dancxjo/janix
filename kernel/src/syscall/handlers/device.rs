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

/// Map a device MMIO BAR into the task's address space
pub fn sys_device_map_mmio(claim_handle: usize, bar_index: usize) -> SysResult<usize> {
    use crate::device_registry::REGISTRY;
    
    if bar_index > 5 {
        return Err(Errno::EINVAL);
    }
    
    let task_id = unsafe { crate::task::scheduler::current_tid_current() };
    let mut reg = REGISTRY.lock();
    
    if !reg.verify_claim(claim_handle, task_id) {
        crate::kinfo!("DEVICE: map_mmio failed - claim {} not owned by task {}", claim_handle, task_id);
        return Err(Errno::EPERM);
    }
    
    let (phys_addr, size) = reg.get_bar_info(claim_handle, bar_index)
        .ok_or(Errno::ENODEV)?;
    
    if phys_addr == 0 || size == 0 {
        crate::kinfo!("DEVICE: BAR{} not present", bar_index);
        return Err(Errno::ENODEV);
    }
    
    let hhdm_offset = crate::boot_info::get()
        .map(|i| i.hhdm_offset)
        .unwrap_or(0);
    let virt_addr = phys_addr + hhdm_offset;
    
    reg.set_bar_mapping(claim_handle, bar_index, virt_addr);
    
    crate::kinfo!("DEVICE: Mapped BAR{} phys=0x{:x} size=0x{:x} -> virt=0x{:x}", 
        bar_index, phys_addr, size, virt_addr);
    
    Ok(virt_addr as usize)
}

/// Subscribe to device interrupts
/// 
/// Args:
///   vector: CPU interrupt vector to subscribe to (0x20-0x2F for legacy IRQs)
/// 
/// Returns: 0 on success
pub fn sys_device_irq_subscribe(vector: usize) -> SysResult<usize> {
    if vector > 255 {
        return Err(Errno::EINVAL);
    }
    
    crate::irq::subscribe(vector as u8).map_err(|_| Errno::EBUSY)?;
    crate::kinfo!("DEVICE: task subscribed to vector 0x{:x}", vector);
    Ok(0)
}

/// Wait for a device interrupt
/// 
/// Args:
///   vector: CPU interrupt vector to wait on
/// 
/// Returns: number of pending interrupts since last wait
pub fn sys_device_irq_wait(vector: usize) -> SysResult<usize> {
    if vector > 255 {
        return Err(Errno::EINVAL);
    }
    
    let count = crate::irq::wait(vector as u8);
    Ok(count as usize)
}

/// Allocate DMA-safe memory for a device
pub fn sys_device_alloc_dma(claim_handle: usize, page_count: usize) -> SysResult<usize> {
    use crate::device_registry::REGISTRY;
    use crate::memory::FRAME_ALLOCATOR;
    
    if page_count == 0 || page_count > 256 {
        return Err(Errno::EINVAL);
    }
    
    let task_id = unsafe { crate::task::scheduler::current_tid_current() };
    
    {
        let reg = REGISTRY.lock();
        if !reg.verify_claim(claim_handle, task_id) {
            return Err(Errno::EPERM);
        }
    }
    
    let mut phys_base = 0u64;
    
    FRAME_ALLOCATOR.with_lock(|alloc| {
        if let Some((phys,)) = alloc.alloc() {
            phys_base = phys;
            for _ in 1..page_count {
                alloc.alloc();
            }
        }
    });
    
    if phys_base == 0 {
        crate::kinfo!("DEVICE: DMA alloc failed - no memory");
        return Err(Errno::ENOMEM);
    }
    
    let hhdm_offset = crate::boot_info::get()
        .map(|i| i.hhdm_offset)
        .unwrap_or(0);
    let virt_addr = phys_base + hhdm_offset;
    
    {
        let mut reg = REGISTRY.lock();
        reg.alloc_dma_slot(claim_handle, phys_base, virt_addr, page_count);
    }
    
    crate::kinfo!("DEVICE: DMA alloc {} pages phys=0x{:x} virt=0x{:x}", 
        page_count, phys_base, virt_addr);
    
    Ok(virt_addr as usize)
}

/// Get physical address of a DMA allocation
pub fn sys_device_dma_phys(virt_addr: usize) -> SysResult<usize> {
    let hhdm_offset = crate::boot_info::get()
        .map(|i| i.hhdm_offset)
        .unwrap_or(0);
    if (virt_addr as u64) < hhdm_offset {
        return Err(Errno::EINVAL);
    }
    let phys = (virt_addr as u64) - hhdm_offset;
    Ok(phys as usize)
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
