use crate::syscalls::syscall4;
use abi::wire::bytespace::*;
use abi::syscall_defs::*;
use abi::ThingId;

pub fn create_and_map(len: u64, flags: u32, prot: u32, hint: u64) -> Result<(ThingId, u64), ()> {
    let req = ByteSpaceCreateAndMapReq { len, flags, prot, user_va_hint: hint };
    let mut req_buf = [0u8; 64];
    let req_bytes = postcard::to_slice(&req, &mut req_buf).map_err(|_| ())?;

    let mut resp_buf = [0u8; 64];

    let ret = unsafe { syscall4(SYSCALL_BYTESPACE_CREATE_AND_MAP, req_bytes.as_ptr() as usize, req_bytes.len(), resp_buf.as_mut_ptr() as usize, resp_buf.len()) };

    if ret < 0 { return Err(()); }

    let resp: ByteSpaceCreateAndMapResp = postcard::from_bytes(&resp_buf[..ret as usize]).map_err(|_| ())?;
    Ok((resp.bytespace_thing_id, resp.user_addr))
}

pub fn create(len: u64, flags: u32) -> Result<ThingId, ()> {
    let req = ByteSpaceCreateReq { len, flags };
    let mut req_buf = [0u8; 64];
    let req_bytes = postcard::to_slice(&req, &mut req_buf).map_err(|_| ())?;
    let mut resp_buf = [0u8; 64];
    let ret = unsafe { syscall4(SYSCALL_BYTESPACE_CREATE, req_bytes.as_ptr() as usize, req_bytes.len(), resp_buf.as_mut_ptr() as usize, resp_buf.len()) };
    if ret < 0 { return Err(()); }
    let resp: ByteSpaceCreateResp = postcard::from_bytes(&resp_buf[..ret as usize]).map_err(|_| ())?;
    Ok(resp.bytespace_thing_id)
}

pub fn write(id: ThingId, offset: u64, ptr: u64, len: u64) -> Result<u64, ()> {
    let req = ByteSpaceWriteReq { bytespace_thing_id: id, offset, user_src_ptr: ptr, len };
    let mut req_buf = [0u8; 64];
    let req_bytes = postcard::to_slice(&req, &mut req_buf).map_err(|_| ())?;
    let mut resp_buf = [0u8; 64];
    let ret = unsafe { syscall4(SYSCALL_BYTESPACE_WRITE, req_bytes.as_ptr() as usize, req_bytes.len(), resp_buf.as_mut_ptr() as usize, resp_buf.len()) };
    if ret < 0 { return Err(()); }
    let resp: ByteSpaceWriteResp = postcard::from_bytes(&resp_buf[..ret as usize]).map_err(|_| ())?;
    Ok(resp.bytes_written)
}
