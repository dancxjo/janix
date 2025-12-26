use abi::wire::typed::{TypeDef, TypeId};
use crate::types::TYPE_REGISTRY;
use core::slice;

// Syscall handler for registering a TypeDef
// a1: ptr to encoded TypeDef bytes
// a2: len of encoded TypeDef bytes
pub fn sys_typedef_register(ptr: usize, len: usize) -> usize {
    // 1. Copy bytes from user
    // In a real kernel we'd use safe user access. v0.2 relies on identity mapping or assumes access.

    // Safety: Assuming identity mapped user memory for v0 prototype or valid ptr
    let user_slice = unsafe { slice::from_raw_parts(ptr as *const u8, len) };

    // 2. Decode TypeDef using postcard
    let typedef: TypeDef = match postcard::from_bytes(user_slice) {
        Ok(t) => t,
        Err(_) => return 1, // Error code 1: Decode failed
    };

    // 3. Register
    match TYPE_REGISTRY.register_typedef(typedef) {
        Ok(_) => 0, // Success
        Err(_) => 2, // Error code 2: Registration failed (hash mismatch or collision)
    }
}

// Syscall handler for getting a TypeDef
// a1: ptr to TypeId (u128 is 16 bytes)
// a2: ptr to output buffer
// a3: len of output buffer
// returns: bytes written, or error
pub fn sys_typedef_get(id_ptr: usize, buf_ptr: usize, buf_len: usize) -> usize {
    let id_slice = unsafe { slice::from_raw_parts(id_ptr as *const u8, 16) };
    let type_id: TypeId = match postcard::from_bytes(id_slice) {
        Ok(id) => id,
        Err(_) => return 0, // Error
    };

    if let Some(typedef) = TYPE_REGISTRY.get_typedef(type_id) {
         let encoded = match postcard::to_allocvec(&typedef) {
             Ok(v) => v,
             Err(_) => return 0,
         };

         if encoded.len() > buf_len {
             return 0; // Buffer too small
         }

         let out_slice = unsafe { slice::from_raw_parts_mut(buf_ptr as *mut u8, encoded.len()) };
         out_slice.copy_from_slice(&encoded);

         encoded.len()
    } else {
        0 // Not found
    }
}
