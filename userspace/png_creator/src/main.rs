#![no_std]
#![no_main]

extern crate alloc;
use stem::syscall;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let _ = main();
    syscall::exit(0)
}

fn main() -> Result<(), abi::errors::Errno> {
    syscall::log_write("PNG_CREATOR: Starting...", 1)?;
    
    // PNG Header
    let png_header = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    
    // Create bytespace
    let len = 4096;
    let id = syscall::root_bytespace_create(len, 0, 0)?;
    syscall::log_write("PNG_CREATOR: Created bytespace", 1)?;
    
    // Write header
    syscall::root_bytespace_write(id, 0, &png_header)?;
    syscall::log_write("PNG_CREATOR: Wrote PNG header", 1)?;
    
    // Sleep to keep process alive if needed, or exit.
    // ingestd monitors GLOBAL bytespaces. It should find it even if creator exits.
    // But creator exiting might clean up resources if they are handle-bound?
    // In ThingOS, bytespaces are usually graph nodes. They persist until explicitly deleted or ref-count/GC.
    // Assuming they persist for now.
    
    Ok(())
}
