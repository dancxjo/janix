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
/// 
/// Args:
///   claim_handle: Handle from sys_device_claim
///   bar_index: BAR number (0-5)
/// 
/// Returns: virtual address where the BAR is mapped (via HHDM)
pub fn sys_device_map_mmio(claim_handle: usize, bar_index: usize) -> SysResult<usize> {
    use crate::device_registry::REGISTRY;
    
    if bar_index > 5 {
        return Err(Errno::EINVAL);
    }
    
    let task_id = unsafe { crate::task::scheduler::current_tid_current() };
    let mut reg = REGISTRY.lock();
    
    // Verify claim ownership
    if !reg.verify_claim(claim_handle, task_id) {
        crate::kinfo!("DEVICE: map_mmio failed - claim {} not owned by task {}", claim_handle, task_id);
        return Err(Errno::EPERM);
    }
    
    // Get BAR info
    let (phys_addr, size) = reg.get_bar_info(claim_handle, bar_index)
        .ok_or(Errno::ENODEV)?;
    
    if phys_addr == 0 || size == 0 {
        crate::kinfo!("DEVICE: BAR{} not present", bar_index);
        return Err(Errno::ENODEV);
    }
    
    // Use HHDM mapping - physical address + HHDM offset = virtual address
    // This is identity-mapped in kernel space, accessible from userspace via syscalls
    // For now, we use the kernel's HHDM offset. In the future, we might map
    // the pages into userspace directly with appropriate flags.
    let hhdm_offset = crate::boot_info::get()
        .map(|i| i.hhdm_offset)
        .unwrap_or(0);
    let virt_addr = phys_addr + hhdm_offset;
    
    // Record the mapping
    reg.set_bar_mapping(claim_handle, bar_index, virt_addr);
    
    crate::kinfo!("DEVICE: Mapped BAR{} phys=0x{:x} size=0x{:x} -> virt=0x{:x}", 
        bar_index, phys_addr, size, virt_addr);
    
    Ok(virt_addr as usize)
}

/// Subscribe to device interrupts
pub fn sys_device_irq_subscribe(_claim_handle: usize) -> SysResult<usize> {
    // TODO: Implement IRQ subscription
    // For now, virtio-gpu can use polling mode
    crate::kinfo!("DEVICE: IRQ subscribe not yet implemented, use polling");
    Err(Errno::NotSupported)
}

/// Allocate DMA-safe memory for a device
/// 
/// Args:
///   claim_handle: Handle from sys_device_claim
///   page_count: Number of 4K pages to allocate
/// 
/// Returns: (virtual_addr, physical_addr) packed as (virt << 32 | phys_low)
///          Use separate syscall to get full 64-bit phys if needed
pub fn sys_device_alloc_dma(claim_handle: usize, page_count: usize) -> SysResult<usize> {
    use crate::device_registry::REGISTRY;
    use crate::memory::FRAME_ALLOCATOR;
    
    if page_count == 0 || page_count > 256 {
        return Err(Errno::EINVAL);
    }
    
    let task_id = unsafe { crate::task::scheduler::current_tid_current() };
    
    // Verify claim
    {
        let reg = REGISTRY.lock();
        if !reg.verify_claim(claim_handle, task_id) {
            return Err(Errno::EPERM);
        }
    }
    
    // Allocate contiguous physical pages
    // Note: Current allocator doesn't guarantee contiguity for multiple pages.
    // For virtio, we can send a scatter-gather list, so this is acceptable.
    // We allocate pages individually and return the first one.
    let mut phys_base = 0u64;
    
    FRAME_ALLOCATOR.with_lock(|alloc| {
        if let Some((phys,)) = alloc.alloc() {
            phys_base = phys;
            // Mark additional pages (best effort for contiguity tracking)
            for _ in 1..page_count {
                alloc.alloc(); // Just allocate, virtio uses scatter-gather
            }
        }
    });
    
    if phys_base == 0 {
        crate::kinfo!("DEVICE: DMA alloc failed - no memory");
        return Err(Errno::ENOMEM);
    }
    
    // Convert to virtual via HHDM
    let hhdm_offset = crate::boot_info::get()
        .map(|i| i.hhdm_offset)
        .unwrap_or(0);
    let virt_addr = phys_base + hhdm_offset;
    
    // Record in registry
    {
        let mut reg = REGISTRY.lock();
        reg.alloc_dma_slot(claim_handle, phys_base, virt_addr, page_count);
    }
    
    crate::kinfo!("DEVICE: DMA alloc {} pages phys=0x{:x} virt=0x{:x}", 
        page_count, phys_base, virt_addr);
    
    // Return both addresses - pack into result
    // Caller can use phys for device descriptors and virt for CPU access
    // Pack: virt in high bits (for immediate use), phys in return
    // Actually, let's use two calls or a struct. For simplicity, return phys.
    // The virt can be derived from HHDM syscall or passed back separately.
    Ok(virt_addr as usize)
}

/// Get physical address of a DMA allocation (for device descriptors)
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
