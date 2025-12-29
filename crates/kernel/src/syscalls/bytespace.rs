use crate::bridge::HardwareBridge;
use crate::Kernel;
use abi::wire::bytespace::*;
use abi::{ThingId, SymbolId};
use thing_models::payload::{ByteSpace, ThingPayload};
use thing_models::Thing;

fn create_bytespace_impl<B: HardwareBridge>(
    kernel: &mut Kernel<B>,
    len: u64,
    flags: u32,
) -> Result<ThingId, ()> {
    // 1. Create backing
    let backing_id = kernel.bytespaces.create_backing(len, flags)?;

    // 2. Create Payload
    let payload = ByteSpace {
        len,
        flags,
        backing: abi::symbols::sym("anonymous"),
    };

    // 3. Encode
    let bytes = postcard::to_allocvec(&payload).map_err(|_| ())?;

    // 4. Create Thing
    let thing_id = kernel.graph.create_thing(ByteSpace::KIND, bytes);

    // 5. Bind
    kernel.bytespaces.bind_thing(thing_id, backing_id);

    Ok(thing_id)
}

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

    match create_bytespace_impl(kernel, req.len, req.flags) {
        Ok(id) => {
            let resp = ByteSpaceCreateResp { bytespace_thing_id: id };
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

pub fn sys_bytespace_register<B: HardwareBridge>(
    kernel: &mut Kernel<B>,
    req_ptr: usize,
    req_len: usize,
    out_ptr: usize,
    out_len: usize,
) -> isize {
    let req_slice = unsafe { core::slice::from_raw_parts(req_ptr as *const u8, req_len) };
    let req: ByteSpaceRegisterReq = match postcard::from_bytes(req_slice) {
        Ok(r) => r,
        Err(_) => return -1,
    };

    if let Some(thing) = kernel.graph.get(req.bytespace_thing_id) {
        if thing.kind != ByteSpace::KIND {
            return -1;
        }
    } else {
        return -1;
    }

    if kernel.bytespaces.get_backing_id(req.bytespace_thing_id).is_some() {
        // Already backed
    } else {
        match kernel.bytespaces.create_backing(req.len, req.flags) {
            Ok(backing_id) => {
                kernel.bytespaces.bind_thing(req.bytespace_thing_id, backing_id);
            }
            Err(_) => return -1,
        }
    }

    let resp = ByteSpaceRegisterResp { ok: true };
    match postcard::to_slice(&resp, unsafe {
        core::slice::from_raw_parts_mut(out_ptr as *mut u8, out_len)
    }) {
        Ok(s) => s.len() as isize,
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

    let backing_id = match kernel.bytespaces.get_backing_id(req.bytespace_thing_id) {
        Some(id) => id,
        None => return -1,
    };

    let bs_len = if let Some(bs) = kernel.bytespaces.get(backing_id) {
        bs.len
    } else {
        return -1;
    };

    if req.offset + req.len > bs_len {
        return -1;
    }

    let start_addr = if req.user_va_hint != 0 {
        req.user_va_hint
    } else {
        return -1;
    };

    if start_addr % 4096 != 0 || req.offset % 4096 != 0 {
        return -1;
    }

    let num_pages = (req.len + 4095) / 4096;
    let mut current_addr = start_addr;
    let mut current_offset = req.offset;

    for _ in 0..num_pages {
        if let Err(_) = kernel.bridge.map_new_user_page(current_addr, 2 | 4) {
             return -1;
        }

        let dst_slice = unsafe { core::slice::from_raw_parts_mut(current_addr as *mut u8, 4096) };
        kernel.bytespaces.read(backing_id, current_offset, dst_slice);

        current_addr += 4096;
        current_offset += 4096;
    }

    let resp = ByteSpaceMapResp { user_addr: start_addr };
    match postcard::to_slice(&resp, unsafe {
        core::slice::from_raw_parts_mut(out_ptr as *mut u8, out_len)
    }) {
        Ok(s) => s.len() as isize,
        Err(_) => -1,
    }
}

pub fn sys_bytespace_read<B: HardwareBridge>(
    kernel: &mut Kernel<B>,
    req_ptr: usize,
    req_len: usize,
    out_ptr: usize,
    out_len: usize,
) -> isize {
    let req_slice = unsafe { core::slice::from_raw_parts(req_ptr as *const u8, req_len) };
    let req: ByteSpaceReadReq = match postcard::from_bytes(req_slice) {
        Ok(r) => r,
        Err(_) => return -1,
    };

    let backing_id = match kernel.bytespaces.get_backing_id(req.bytespace_thing_id) {
        Some(id) => id,
        None => return -1,
    };

    if req.user_dst_ptr == 0 { return -1; }

    let dst_slice = unsafe { core::slice::from_raw_parts_mut(req.user_dst_ptr as *mut u8, req.len as usize) };
    let bytes_read = kernel.bytespaces.read(backing_id, req.offset, dst_slice);

    let resp = ByteSpaceReadResp { bytes_read: bytes_read as u64 };
    match postcard::to_slice(&resp, unsafe {
        core::slice::from_raw_parts_mut(out_ptr as *mut u8, out_len)
    }) {
        Ok(s) => s.len() as isize,
        Err(_) => -1,
    }
}

pub fn sys_bytespace_write<B: HardwareBridge>(
    kernel: &mut Kernel<B>,
    req_ptr: usize,
    req_len: usize,
    out_ptr: usize,
    out_len: usize,
) -> isize {
    let req_slice = unsafe { core::slice::from_raw_parts(req_ptr as *const u8, req_len) };
    let req: ByteSpaceWriteReq = match postcard::from_bytes(req_slice) {
        Ok(r) => r,
        Err(_) => return -1,
    };

    let backing_id = match kernel.bytespaces.get_backing_id(req.bytespace_thing_id) {
        Some(id) => id,
        None => return -1,
    };

    if req.user_src_ptr == 0 { return -1; }

    let src_slice = unsafe { core::slice::from_raw_parts(req.user_src_ptr as *const u8, req.len as usize) };
    let bytes_written = kernel.bytespaces.write(backing_id, req.offset, src_slice);

    let resp = ByteSpaceWriteResp { bytes_written: bytes_written as u64 };
    match postcard::to_slice(&resp, unsafe {
        core::slice::from_raw_parts_mut(out_ptr as *mut u8, out_len)
    }) {
        Ok(s) => s.len() as isize,
        Err(_) => -1,
    }
}

pub fn sys_bytespace_create_and_map<B: HardwareBridge>(
    kernel: &mut Kernel<B>,
    req_ptr: usize,
    req_len: usize,
    out_ptr: usize,
    out_len: usize,
) -> isize {
    let req_slice = unsafe { core::slice::from_raw_parts(req_ptr as *const u8, req_len) };
    let req: ByteSpaceCreateAndMapReq = match postcard::from_bytes(req_slice) {
        Ok(r) => r,
        Err(_) => return -1,
    };

    let thing_id = match create_bytespace_impl(kernel, req.len, req.flags) {
        Ok(id) => id,
        Err(_) => return -1,
    };

    let start_addr = if req.user_va_hint != 0 {
        req.user_va_hint
    } else {
        return -1;
    };

    if start_addr % 4096 != 0 { return -1; }

    let num_pages = (req.len + 4095) / 4096;
    let mut current_addr = start_addr;

    for _ in 0..num_pages {
        if let Err(_) = kernel.bridge.map_new_user_page(current_addr, 2 | 4) {
             return -1;
        }
        current_addr += 4096;
    }

    let resp = ByteSpaceCreateAndMapResp { bytespace_thing_id: thing_id, user_addr: start_addr };
    match postcard::to_slice(&resp, unsafe {
        core::slice::from_raw_parts_mut(out_ptr as *mut u8, out_len)
    }) {
        Ok(s) => s.len() as isize,
        Err(_) => -1,
    }
}
