//! Device capability syscalls

use super::{copyin, copyout};
use crate::syscall::validate::validate_user_range;
use abi::device::{
    DEVICE_IRQ_SUBSCRIBE_DEVICE, DEVICE_IRQ_SUBSCRIBE_VECTOR, DeviceCall, DeviceKind,
    PCI_IRQ_MODE_MSI, PCI_IRQ_MODE_MSIX, PCI_OP_ENABLE_MSI, PciEnableMsiRequest,
    PciEnableMsiResponse,
};
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
        DeviceKind::Pci => sys_pci_call(&call),
        _ => Err(Errno::NotSupported),
    }
}

pub fn sys_device_claim(graph_id: usize) -> SysResult<usize> {
    use crate::device_registry::REGISTRY;

    let task_id = unsafe { crate::task::scheduler::current_tid_current() };

    let mut reg = REGISTRY.lock();

    if let Some(device_idx) = reg.find_by_graph_id(graph_id as u64) {
        if let Some(claim_handle) = reg.claim(device_idx, task_id) {
            crate::kinfo!(
                "DEVICE: task {} claimed device {} (handle {})",
                task_id,
                graph_id,
                claim_handle
            );
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
        crate::kinfo!(
            "DEVICE: map_mmio failed - claim {} not owned by task {}",
            claim_handle,
            task_id
        );
        return Err(Errno::EPERM);
    }

    let (phys_addr, size) = reg
        .get_bar_info(claim_handle, bar_index)
        .ok_or(Errno::ENODEV)?;

    if phys_addr == 0 || size == 0 {
        crate::kinfo!("DEVICE: BAR{} not present", bar_index);
        return Err(Errno::ENODEV);
    }

    let page_count = (size + 4095) / 4096;
    let user_va = crate::memory::alloc_user_va((page_count * 4096) as usize);

    // Map pages
    for i in 0..page_count {
        let phys = phys_addr as u64 + (i * 4096) as u64;
        let virt = user_va + (i * 4096) as u64;
        unsafe {
            crate::memory::map_user_page_with_perms(
                virt,
                phys,
                crate::MapPerms {
                    user: true,
                    read: true,
                    write: true,
                    exec: false,
                },
            )
            .map_err(|_| Errno::ENOMEM)?;
        }
    }

    reg.set_bar_mapping(claim_handle, bar_index, user_va);

    crate::kinfo!(
        "DEVICE: Mapped BAR{} phys=0x{:x} size=0x{:x} -> virt=0x{:x}",
        bar_index,
        phys_addr,
        size,
        user_va
    );

    Ok(user_va as usize)
}

/// Subscribe to device interrupts
///
/// Args (mode=DEVICE_IRQ_SUBSCRIBE_VECTOR):
///   arg0: CPU interrupt vector to subscribe to
///
/// Args (mode=DEVICE_IRQ_SUBSCRIBE_DEVICE):
///   arg0: claim handle
///   arg1: device interrupt index
///
/// Returns: 0 on success
pub fn sys_device_irq_subscribe(arg0: usize, arg1: usize, mode: usize) -> SysResult<usize> {
    match mode as u8 {
        DEVICE_IRQ_SUBSCRIBE_DEVICE => {
            let claim_handle = arg0;
            let irq_index = arg1;
            let task_id = unsafe { crate::task::scheduler::current_tid_current() };
            let (irq_mode, vector) = {
                let reg = crate::device_registry::REGISTRY.lock();
                if !reg.verify_claim(claim_handle, task_id) {
                    return Err(Errno::EPERM);
                }
                reg.get_irq_vector(claim_handle, irq_index)
                    .ok_or(Errno::ENODEV)?
            };
            crate::irq::subscribe(vector).map_err(|_| Errno::EBUSY)?;
            crate::kinfo!(
                "DEVICE: task subscribed to device irq {} (mode={:?}, vector=0x{:x})",
                irq_index,
                irq_mode,
                vector
            );
            Ok(0)
        }
        DEVICE_IRQ_SUBSCRIBE_VECTOR | _ => {
            let vector = arg0;
            if vector > 255 {
                return Err(Errno::EINVAL);
            }
            crate::irq::subscribe(vector as u8).map_err(|_| Errno::EBUSY)?;
            crate::kinfo!("DEVICE: task subscribed to vector 0x{:x}", vector);
            Ok(0)
        }
    }
}

/// Wait for a device interrupt
///
/// Args follow sys_device_irq_subscribe
///
/// Returns: number of pending interrupts since last wait
pub fn sys_device_irq_wait(arg0: usize, arg1: usize, mode: usize) -> SysResult<usize> {
    let vector = match mode as u8 {
        DEVICE_IRQ_SUBSCRIBE_DEVICE => {
            let claim_handle = arg0;
            let irq_index = arg1;
            let task_id = unsafe { crate::task::scheduler::current_tid_current() };
            let (_mode, vector) = {
                let reg = crate::device_registry::REGISTRY.lock();
                if !reg.verify_claim(claim_handle, task_id) {
                    return Err(Errno::EPERM);
                }
                reg.get_irq_vector(claim_handle, irq_index)
                    .ok_or(Errno::ENODEV)?
            };
            vector
        }
        DEVICE_IRQ_SUBSCRIBE_VECTOR | _ => {
            if arg0 > 255 {
                return Err(Errno::EINVAL);
            }
            arg0 as u8
        }
    };

    let count = crate::irq::wait(vector);
    Ok(count as usize)
}

fn sys_pci_call(call: &DeviceCall) -> SysResult<usize> {
    match call.op {
        PCI_OP_ENABLE_MSI => {
            if call.in_len as usize != core::mem::size_of::<PciEnableMsiRequest>() {
                return Err(Errno::EINVAL);
            }
            if call.out_len as usize != core::mem::size_of::<PciEnableMsiResponse>() {
                return Err(Errno::EINVAL);
            }
            validate_user_range(call.in_ptr as usize, call.in_len as usize, true)?;
            validate_user_range(call.out_ptr as usize, call.out_len as usize, true)?;

            let mut req: PciEnableMsiRequest = unsafe { core::mem::zeroed() };
            let in_slice = unsafe {
                core::slice::from_raw_parts_mut(
                    &mut req as *mut _ as *mut u8,
                    core::mem::size_of::<PciEnableMsiRequest>(),
                )
            };
            unsafe {
                copyin(in_slice, call.in_ptr as usize)?;
            }

            let res = crate::irq::msi::enable_for_claim(
                req.claim_handle as usize,
                req.requested_vectors,
                req.prefer_msix != 0,
            )?;

            let irq_mode = match res.mode {
                crate::device_registry::IrqMode::Msi => PCI_IRQ_MODE_MSI,
                crate::device_registry::IrqMode::Msix => PCI_IRQ_MODE_MSIX,
                crate::device_registry::IrqMode::Legacy => 0,
            };
            let out = PciEnableMsiResponse {
                vector: res.vector,
                irq_mode,
                _reserved: [0; 2],
            };
            let out_slice = unsafe {
                core::slice::from_raw_parts(
                    &out as *const _ as *const u8,
                    core::mem::size_of::<PciEnableMsiResponse>(),
                )
            };
            unsafe {
                copyout(call.out_ptr as usize, out_slice)?;
            }
            Ok(0)
        }
        _ => Err(Errno::NotSupported),
    }
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

    let hhdm_offset = crate::boot_info::get().map(|i| i.hhdm_offset).unwrap_or(0);
    let virt_addr = phys_base + hhdm_offset;

    {
        let mut reg = REGISTRY.lock();
        reg.alloc_dma_slot(claim_handle, phys_base, virt_addr, page_count);
    }

    crate::kinfo!(
        "DEVICE: DMA alloc {} pages phys=0x{:x} virt=0x{:x}",
        page_count,
        phys_base,
        virt_addr
    );

    Ok(virt_addr as usize)
}

/// Get physical address of a DMA allocation
pub fn sys_device_dma_phys(virt_addr: usize) -> SysResult<usize> {
    let hhdm_offset = crate::boot_info::get().map(|i| i.hhdm_offset).unwrap_or(0);
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
