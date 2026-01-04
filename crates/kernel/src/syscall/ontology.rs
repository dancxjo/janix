use abi::wire::SyscallResult;
use abi::syscall::err;

// Embed the ontology binary
// We use include_bytes! to bake the ontology into the kernel binary
static ONTOLOGY_BIN: &[u8] = include_bytes!("../../../../artifacts/ontology/ontology.bin");

pub fn sys_ontology_get(dst_ptr: u64, dst_len: u64) -> SyscallResult {
    // If dst_ptr is 0, return the required size
    if dst_ptr == 0 {
        return SyscallResult::new(0, ONTOLOGY_BIN.len() as u64, 0);
    }

    if dst_len == 0 {
        return SyscallResult::new(err::EINVAL, 0, 0);
    }

    // Copy bytes to user buffer
    // Unsafe because we are writing to a raw pointer provided by userspace
    // In a real system, we'd need verify_area / copy_to_user helpers
    
    // Check if the buffer is large enough
    if dst_len < ONTOLOGY_BIN.len() as u64 {
        return SyscallResult::new(err::ENOMEM, ONTOLOGY_BIN.len() as u64, 0);
    }
    
    // Copy
    unsafe {
        let dst = core::slice::from_raw_parts_mut(dst_ptr as *mut u8, ONTOLOGY_BIN.len());
        dst.copy_from_slice(ONTOLOGY_BIN);
    }

    SyscallResult::new(0, ONTOLOGY_BIN.len() as u64, 0)
}
