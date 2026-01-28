//! Memory and stack allocation syscalls

use abi::errors::{Errno, SysResult};
use abi::vm::{
    VmBacking, VmBackingKind, VmMapReq, VmMapResp, VmProt, VmRegionInfo, VmUnmapReq, VmUnmapResp,
};
use core::sync::atomic::{AtomicU64, Ordering};

const USER_VM_BASE: u64 = 0x1000_0000;
static NEXT_USER_MAP: AtomicU64 = AtomicU64::new(USER_VM_BASE);

pub fn sys_alloc_stack(pages: usize) -> SysResult<usize> {
    let top =
        unsafe { crate::task::scheduler::alloc_user_stack_current(pages) }.ok_or(Errno::ENOMEM)?;
    Ok(top)
}

pub fn sys_vm_map(req_ptr: usize, resp_ptr: usize) -> SysResult<usize> {
    use crate::syscall::validate::{copyin, copyout, validate_user_range};

    let req_size = core::mem::size_of::<VmMapReq>();
    let resp_size = core::mem::size_of::<VmMapResp>();
    validate_user_range(req_ptr, req_size, false)?;
    validate_user_range(resp_ptr, resp_size, true)?;

    let mut req: VmMapReq = unsafe { core::mem::zeroed() };
    let req_slice =
        unsafe { core::slice::from_raw_parts_mut(&mut req as *mut _ as *mut u8, req_size) };
    unsafe {
        copyin(req_slice, req_ptr)?;
    }

    let page_size = 4096usize;
    let len = align_up(req.len, page_size);
    if len == 0 {
        return Err(Errno::EINVAL);
    }

    let fixed = req.flags.contains(abi::vm::VmMapFlags::FIXED);
    let guard = req.flags.contains(abi::vm::VmMapFlags::GUARD);

    let mut addr = req.addr_hint;
    if fixed {
        if addr == 0 || addr % page_size != 0 {
            return Err(Errno::EINVAL);
        }
    } else {
        if addr == 0 {
            addr = NEXT_USER_MAP.fetch_add(len as u64, Ordering::SeqCst) as usize;
        }
        addr = align_up(addr, page_size);
    }

    if addr % page_size != 0 {
        return Err(Errno::EINVAL);
    }

    if !guard {
        let perms = map_perms_from_prot(req.prot);
        let hhdm = crate::boot_info::get().map(|i| i.hhdm_offset).unwrap_or(0);
        let mut virt = addr as u64;
        let end = virt + len as u64;
        while virt < end {
            let phys = crate::memory::alloc_frame().ok_or(Errno::ENOMEM)?;
            let VmBacking::Anonymous { zeroed } = req.backing;
            if zeroed {
                let hhdm_virt = phys + hhdm;
                unsafe {
                    core::ptr::write_bytes(hhdm_virt as *mut u8, 0, page_size);
                }
            }
            unsafe {
                crate::memory::map_user_page_with_perms(virt, phys, perms)?;
            }
            virt += page_size as u64;
        }
    }

    let region = VmRegionInfo {
        start: addr,
        end: addr + len,
        prot: req.prot,
        flags: req.flags,
        backing_kind: match req.backing {
            VmBacking::Anonymous { .. } => VmBackingKind::Anonymous,
        },
        _reserved: [0; 7],
    };

    // Only register regions that are actually accessible (not guard-only)
    // Guard regions are virtual address reservations without actual page mappings
    let is_accessible = req.prot.contains(VmProt::READ) || req.prot.contains(VmProt::WRITE);

    if is_accessible {
        // For FIXED mappings, we may be replacing part of an existing region
        // Remove the overlap first to avoid permission conflicts
        if fixed {
            unsafe {
                let _ = crate::task::scheduler::remove_user_mappings_current(addr, len);
            }
        }

        unsafe {
            crate::task::scheduler::add_user_mapping_current(region)?;
        }
    }

    let resp = VmMapResp { addr, len };
    let resp_slice =
        unsafe { core::slice::from_raw_parts(&resp as *const _ as *const u8, resp_size) };
    unsafe {
        copyout(resp_ptr, resp_slice)?;
    }
    Ok(0)
}

pub fn sys_vm_unmap(req_ptr: usize, resp_ptr: usize) -> SysResult<usize> {
    use crate::syscall::validate::{copyin, copyout, validate_user_range};

    let req_size = core::mem::size_of::<VmUnmapReq>();
    let resp_size = core::mem::size_of::<VmUnmapResp>();
    validate_user_range(req_ptr, req_size, false)?;
    validate_user_range(resp_ptr, resp_size, true)?;

    let mut req: VmUnmapReq = unsafe { core::mem::zeroed() };
    let req_slice =
        unsafe { core::slice::from_raw_parts_mut(&mut req as *mut _ as *mut u8, req_size) };
    unsafe {
        copyin(req_slice, req_ptr)?;
    }

    let page_size = 4096usize;
    let len = align_up(req.len, page_size);
    if len == 0 || req.addr % page_size != 0 {
        return Err(Errno::EINVAL);
    }

    // Update mappings
    let removed_ranges =
        unsafe { crate::task::scheduler::remove_user_mappings_current(req.addr, len)? };

    // Unmap pages
    for (start, end) in removed_ranges {
        let mut virt = start as u64;
        let end_virt = end as u64;
        while virt < end_virt {
            unsafe {
                let _ = crate::memory::unmap_user_page(virt);
            }
            virt += page_size as u64;
        }
    }

    let resp = VmUnmapResp { unmapped_len: len };
    let resp_slice =
        unsafe { core::slice::from_raw_parts(&resp as *const _ as *const u8, resp_size) };
    unsafe {
        copyout(resp_ptr, resp_slice)?;
    }

    Ok(0)
}

pub fn sys_vm_protect(req_ptr: usize) -> SysResult<usize> {
    let _ = req_ptr;
    Err(Errno::ENOSYS)
}

pub fn sys_vm_advise(_req_ptr: usize) -> SysResult<usize> {
    Ok(0)
}

pub fn sys_vm_query(req_ptr: usize, resp_ptr: usize) -> SysResult<usize> {
    let _ = (req_ptr, resp_ptr);
    Err(Errno::ENOSYS)
}

fn align_up(value: usize, align: usize) -> usize {
    if align == 0 {
        return value;
    }
    (value + align - 1) & !(align - 1)
}

fn map_perms_from_prot(prot: VmProt) -> crate::MapPerms {
    crate::MapPerms {
        user: prot.contains(VmProt::USER),
        read: prot.contains(VmProt::READ),
        write: prot.contains(VmProt::WRITE),
        exec: prot.contains(VmProt::EXEC),
    }
}
