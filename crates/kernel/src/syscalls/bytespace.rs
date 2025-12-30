use crate::bridge::HardwareBridge;
use crate::Kernel;
use abi::wire::bytespace::*;

pub fn sys_bytespace_create<B: HardwareBridge>(
    kernel: &mut Kernel<B>,
    req_ptr: usize,
    req_len: usize,
    out_ptr: usize,
    out_len: usize,
) -> isize {
    let req_slice = unsafe { core::slice::from_raw_parts(req_ptr as *const u8, req_len) };
    let req: ByteSpaceCreateReq = match postcard::from_bytes(req_slice) {
        Ok(r) => r,
        Err(_) => return -1,
    };

    match kernel.bytespaces.create(req.len, req.flags) {
        Ok(id) => {
            let resp = ByteSpaceCreateResp { id, len: req.len };
            match postcard::to_slice(&resp, unsafe {
                core::slice::from_raw_parts_mut(out_ptr as *mut u8, out_len)
            }) {
                Ok(s) => s.len() as isize,
                Err(_) => -1,
            }
        }
        Err(_) => -1,
    }
}

pub fn sys_bytespace_map<B: HardwareBridge>(
    kernel: &mut Kernel<B>,
    req_ptr: usize,
    req_len: usize,
    out_ptr: usize,
    out_len: usize,
) -> isize {
    let req_slice = unsafe { core::slice::from_raw_parts(req_ptr as *const u8, req_len) };
    let req: ByteSpaceMapReq = match postcard::from_bytes(req_slice) {
        Ok(r) => r,
        Err(_) => return -1,
    };

    let bs = match kernel.bytespaces.get(req.id) {
        Some(b) => b,
        None => return -1,
    };

    if req.offset + req.len > bs.len {
        return -1;
    }

    // Determine target address
    let start_addr = if req.user_va_hint != 0 {
        req.user_va_hint
    } else {
        // Fallback: This is risky without a VMM.
        // For now, return error if no hint. User (loaded) should provide address.
        return -1;
    };

    // Align checks
    if !req.offset.is_multiple_of(4096) {
        return abi::syscall_defs::SYS_EINVAL as isize;
    }

    // Determine number of pages
    let num_pages = req.len.div_ceil(4096);
    let size_bytes = num_pages * 4096;

    // Allocate in ByteSpace
    // (Optimization: We could check if we have enough RAM first,
    // but the allocator will fail if not)

    // Map pages into Current Process
    // For v0.2 MVP we loop. In future we use batch map.
    // Address must be page aligned.
    let mut current_addr = start_addr;
    let mut current_offset = req.offset;

    // Copy-Map: Allocate new user pages and copy data
    for _ in 0..num_pages {
        // 1. Map new user page
        // Flags: 2=Write needed for copy. 4=User.
        // Even if user asked for ReadOnly, we must Write initial data.
        // Real implementation would map ReadWrite, Copy, then Remap ReadOnly.
        // Simplify: Always Map ReadWrite for now (User Flags | 2).
        if kernel
            .bridge
            .map_new_user_page(current_addr, 2 | 4)
            .is_err()
        {
            return -1;
        }

        // 2. Copy data
        // BS pages are stored linearly in `bytespaces.rs`.
        // Page index = current_offset / 4096
        let page_idx = (current_offset / 4096) as usize;
        if let Some(chunk) = bs.pages.get(page_idx) {
            let src = chunk.as_slice();
            let dst = unsafe { core::slice::from_raw_parts_mut(current_addr as *mut u8, 4096) };

            // Calculate how much to copy for this page (handle last partial page)
            let space_rem = bs.len - current_offset;
            let req_rem = (req.offset + req.len) - current_offset;
            let to_copy = core::cmp::min(4096, core::cmp::min(space_rem, req_rem));

            dst[..to_copy as usize].copy_from_slice(&src[..to_copy as usize]);
        }

        current_addr += 4096;
        current_offset += 4096;
    }

    let resp = ByteSpaceMapResp {
        user_addr: start_addr,
    };
    match postcard::to_slice(&resp, unsafe {
        core::slice::from_raw_parts_mut(out_ptr as *mut u8, out_len)
    }) {
        Ok(s) => s.len() as isize,
        Err(_) => -1,
    }
}

pub fn sys_bytespace_read<B: HardwareBridge>(
    _kernel: &mut Kernel<B>,
    _req_ptr: usize,
    _req_len: usize,
    _out_ptr: usize,
    _out_len: usize,
) -> isize {
    // TODO: Implement Read (Stream-like or random access)
    // For now, MAP is sufficient for `loaded`
    -1
}

pub fn sys_bytespace_write<B: HardwareBridge>(
    _kernel: &mut Kernel<B>,
    _req_ptr: usize,
    _req_len: usize,
    _out_ptr: usize,
    _out_len: usize,
) -> isize {
    -1
}
