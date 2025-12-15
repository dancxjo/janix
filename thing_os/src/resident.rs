use abi::ThingId;
pub use abi::resident::{ResidentAllocResp, ResidentError, ResidentErrorCode, ResidentMapPerms, ResidentMapResp, RestPolicy, RestResp};
use abi::resident_layout::ResidentHeader;
use runtime::Sys;
use core::sync::atomic::{Ordering, compiler_fence};
use alloc::boxed::Box;
use alloc::string::ToString;
use abi::{KernelRequest, KernelResponse};

pub mod mouse;
pub mod keyboard_stream;
mod abi_tests;

/// A wrapper around a resident object mapped in memory.
/// `T` is a phantom type indicating the logical kind of the object (e.g. `MouseStream`).
#[derive(Debug, Clone, Copy)]
pub struct Resident<T> {
    pub id: ThingId,
    pub ptr: *mut u8,
    pub byte_len: usize,
    _marker: core::marker::PhantomData<T>,
}

impl<T> Resident<T> {
    /// Create a ResidentObject from raw parts.
    /// 
    /// # Safety
    /// `ptr` and `byte_len` must describe a valid resident mapping region for `id`.
    pub unsafe fn new(id: ThingId, ptr: *mut u8, byte_len: usize) -> Self {
        Self { id, ptr, byte_len, _marker: core::marker::PhantomData }
    }

    /// Access the resident header.
    pub fn header(&self) -> &ResidentHeader {
        unsafe { &*(self.ptr as *const ResidentHeader) }
    }

    /// Perform a seqlock-protected read of the resident object.
    /// The closure `f` receives the header and the full data slice.
    /// Returns the result of `f` if consistency is observed.
    pub fn with_read<F, R>(&self, mut f: F) -> R
    where
        F: FnMut(&ResidentHeader, &[u8]) -> R,
    {
        unsafe {
            let header_ptr = self.ptr as *const ResidentHeader;
            loop {
                // Read sequence (volatile)
                let seq1 = core::ptr::read_volatile(&(*header_ptr).seq);
                
                // If odd, writing is in progress. Spin/Yield.
                if seq1 % 2 != 0 {
                    core::hint::spin_loop();
                    continue;
                }
                
                // Acquire fence to preventing reading old data before checking seq
                compiler_fence(Ordering::Acquire);
                
                let res = f(&*header_ptr, core::slice::from_raw_parts(self.ptr, self.byte_len));
                
                // Release fence to prevent reading new data before checking seq
                compiler_fence(Ordering::Release);
                
                let seq2 = core::ptr::read_volatile(&(*header_ptr).seq);
                if seq1 == seq2 {
                    return res;
                }
            }
        }
    }

    /// Perform a write operation on the resident object.
    /// The closure `f` receives mutable access to same.
    /// Logic assumes the caller is the Single Writer.
    pub fn with_write<F, R>(&mut self, f: F) -> R
    where F: FnOnce(&mut ResidentHeader, &mut [u8]) -> R
    {
        unsafe {
            let header_ptr = self.ptr as *mut ResidentHeader;
            let seq_ptr = &mut (*header_ptr).seq;
            
            let s = core::ptr::read_volatile(seq_ptr);
            // Increment to odd (start write)
            core::ptr::write_volatile(seq_ptr, s.wrapping_add(1));
            compiler_fence(Ordering::Release);
            
            let res = f(&mut *header_ptr, core::slice::from_raw_parts_mut(self.ptr, self.byte_len));
            
            compiler_fence(Ordering::Release);
            // Increment to even (end write)
            core::ptr::write_volatile(seq_ptr, s.wrapping_add(2));
            
            res
        }
    }
}

pub fn alloc_resident(sys: &impl Sys, kind: &str, byte_len: u32, flags: u32) -> Result<ResidentAllocResp, ResidentError> {
    // Current ABI requires static kind string.
    let kind_static = Box::leak(kind.to_string().into_boxed_str());
    match sys.syscall(KernelRequest::ResidentAlloc { kind: kind_static, byte_len, flags }) {
        KernelResponse::ResidentAllocated { resp } => Ok(resp),
        KernelResponse::ResidentError(e) => Err(e),
        _ => Err(ResidentError { code: ResidentErrorCode::Unknown, aux0: 0, aux1: 0 }),
    }
}

pub fn map_resident(sys: &impl Sys, id: ThingId, perms: ResidentMapPerms) -> Result<ResidentMapResp, ResidentError> {
    match sys.syscall(KernelRequest::ResidentMap { id, perms }) {
        KernelResponse::ResidentMapped { resp } => Ok(resp),
        KernelResponse::ResidentError(e) => Err(e),
        _ => Err(ResidentError { code: ResidentErrorCode::Unknown, aux0: 0, aux1: 0 }),
    }
}

pub fn unmap_resident(sys: &impl Sys, thing_id: ThingId) -> Result<(), ResidentError> {
    match sys.syscall(KernelRequest::ResidentUnmap { thing_id }) {
        KernelResponse::Success { .. } => Ok(()),
        KernelResponse::ResidentError(e) => Err(e),
        _ => Err(ResidentError { code: ResidentErrorCode::Unknown, aux0: 0, aux1: 0 }),
    }
}

pub fn rest_thing(sys: &impl Sys, thing_id: ThingId, policy: RestPolicy) -> Result<RestResp, ResidentError> {
    match sys.syscall(KernelRequest::ThingRest { thing_id, policy }) {
        KernelResponse::ThingRested { resp } => Ok(resp),
        KernelResponse::ResidentError(e) => Err(e),
        _ => Err(ResidentError { code: ResidentErrorCode::Unknown, aux0: 0, aux1: 0 }),
    }
}

pub unsafe fn resident_create_and_map<T>(
    sys: &impl Sys, 
    kind: &str, 
    byte_len: u32, 
    perms: ResidentMapPerms
) -> Result<Resident<T>, ResidentError> {
    // 1. Alloc
    let alloc_resp = alloc_resident(sys, kind, byte_len, 0)?; // default flags
    
    // 2. Map
    // alloc_resp.id is ThingId.
    let map_resp = map_resident(sys, alloc_resp.id, perms)?;
    
    // 3. Wrap
    unsafe {
        Ok(Resident::new(alloc_resp.id, map_resp.user_addr as *mut u8, map_resp.byte_len as usize))
    }
}
