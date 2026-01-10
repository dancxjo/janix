pub use abi;

pub mod elf;

use crate::BootModuleDesc;
use crate::task::TaskId;

pub fn sys_spawn_module_from_desc(module: &BootModuleDesc) -> Result<TaskId, &'static str> {
     crate::kinfo!("Spawning module: {}", module.name);
     
     // 1. Create AddressSpace
     // AddressSpace::new() copies kernel mappings.
     let mut aspace = crate::memory::paging::AddressSpace::new();
     
     // 2. Load ELF
     let image = elf::load_user_elf(module.bytes, &mut aspace)?;
     
     // 3. Spawn
     let tid = unsafe {
         let ptr = core::ptr::addr_of_mut!(crate::task::SCHEDULER);
         if let Some(sched) = (*ptr).as_mut() {
             sched.spawn_user(image, aspace)
         } else {
             return Err("Scheduler not initialized");
         }
     };
     
     Ok(tid)
}

pub fn sys_spawn_module(path_ptr: *const u8, len: usize) -> Result<TaskId, &'static str> {
    // 1. Read path from user
    if len > 256 { return Err("Path too long"); }
    
    // We assume we are in the user's address space (CR3) via syscall,
    // so we can read directly.
    
    let mut path_buf = [0u8; 256];
    unsafe {
        core::ptr::copy_nonoverlapping(path_ptr, path_buf.as_mut_ptr(), len);
    }
    let path_str = core::str::from_utf8(&path_buf[..len]).map_err(|_| "Invalid UTF-8")?;
    
    // 2. Find Module
    let modules = crate::boot_modules();
    let module = modules.iter().find(|m| m.name == path_str).ok_or("Module not found")?;
    
    // 3. Spawn
    sys_spawn_module_from_desc(module)
}

pub fn sys_rtc_cmos_read(reg: u8) -> Result<u8, &'static str> {
    // Basic RTC read
    if reg > 127 { return Err("Invalid register"); }
    
    #[cfg(target_arch = "x86_64")]
        unsafe {
            let addr_port: u16 = 0x70;
            let data_port: u16 = 0x71;
            
            // outb(0x70, reg)
            core::arch::asm!("out dx, al", in("dx") addr_port, in("al") reg, options(nostack, preserves_flags));
            
            // inb(0x71)
            let val: u8;
            core::arch::asm!("in al, dx", in("dx") data_port, out("al") val, options(nostack, preserves_flags));
            Ok(val)
        }
    #[cfg(not(target_arch = "x86_64"))]
    Err("Not supported")
}

pub fn sys_find_module(name: &str) -> Option<&'static BootModuleDesc> {
     crate::boot_modules().iter().find(|m| m.name == name)
}
