#![no_std]
#![no_main]

use stem::syscall::{debug_write, sleep_ms, exit};
use stem::kprintln;

#[no_mangle]
#[repr(C)]
struct ModuleEntry {
     name_ptr: usize,
     name_len: usize,
}

#[no_mangle]
pub fn main(arg: usize) -> i32 {
    kprintln!("SPROUT: Init started.");
    
    if arg == 0 {
        kprintln!("SPROUT: No registry provided (arg == 0)");
        return 1;
    }
    
    let count_ptr = arg as *const usize;
    let count = unsafe { *count_ptr };
    kprintln!("SPROUT: Found {} modules", count);
    
    let entries = unsafe { count_ptr.add(1) as *const ModuleEntry };
    
    for i in 0..count {
        let entry = unsafe { &*entries.add(i) };
        let slice = unsafe { core::slice::from_raw_parts(entry.name_ptr as *const u8, entry.name_len) };
        
        if let Ok(name) = core::str::from_utf8(slice) {
            if name.contains("sprout") { continue; }
            
            kprintln!("SPROUT: Spawning {}", name);
            match stem::syscall::spawn_process(name) {
                Ok(_) => {}, // Kernel logs success usually, or we can log here
                Err(e) => kprintln!("SPROUT: Failed to spawn {}: {:?}", name, e),
            }
        }
    }

    loop { 
        unsafe { stem::syscall::yield_now(); }
    }
}
