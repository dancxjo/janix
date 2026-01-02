use crate::machine::input;

pub fn sys_input_read(buf: *mut u8, len: usize) -> Result<usize, ()> {
    // 1. Validate permissions
    // TODO: Verify task has capability to read input.
    // For now, allow all (Sprout/Inputd is trusted).
    
    // 2. Validate pointer
    if buf.is_null() || len == 0 {
        return Ok(0);
    }
    
    // TODO: Validate user pointer range (ensure it's in user space)
    
    // 3. Read
    // We need to map user pointer to kernel slice or copy_to_user.
    // Since we are in the context of the user task, the virtual address `buf` is valid 
    // IF we are running on the user's page table.
    // Assuming we are (traditional syscall).
    
    // However, Rust slices need safety.
    let user_slice = unsafe { core::slice::from_raw_parts_mut(buf, len) };
    
    Ok(input::read_scancodes(user_slice))
}
