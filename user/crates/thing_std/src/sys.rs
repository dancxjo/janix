#![allow(dead_code)]

// use abi::SysRet; // Unused for now in this signature, but good to have if we expand.
use abi::SYSCALL_SPAWN;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SysErr(pub i64);

pub fn sys_graph(query: &str, params: &[u8], out: &mut [u8]) -> Result<usize, SysErr> {
    let ret: isize;
    unsafe {
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!(
            "syscall",
            in("rax") 1, // SYSCALL_GRAPH
            in("rdi") query.as_ptr() as usize,
            in("rsi") query.len(),
            in("rdx") params.as_ptr() as usize,
            in("r10") params.len(),
            in("r8") out.as_mut_ptr() as usize,
            in("r9") out.len(),
            lateout("rax") ret,
            out("rcx") _, // Clobbered by syscall
            out("r11") _, // Clobbered by syscall
        );
        #[cfg(target_arch = "aarch64")]
        core::arch::asm!(
            "svc #0",
            in("x8") 1, // SYSCALL_GRAPH
            in("x0") query.as_ptr() as usize,
            in("x1") query.len(),
            in("x2") params.as_ptr() as usize,
            in("x3") params.len(),
            in("x4") out.as_mut_ptr() as usize,
            in("x5") out.len(),
            lateout("x0") ret,
            options(nostack)
        );
    }
    
    if ret < 0 {
        Err(SysErr(ret as i64))
    } else {
        Ok(ret as usize)
    }
}

pub fn sys_yield() {
    unsafe {
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!(
            "syscall",
            in("rax") 2, // SYSCALL_YIELD
            out("rcx") _,
            out("r11") _,
        );
        #[cfg(target_arch = "aarch64")]
        core::arch::asm!(
            "svc #0",
            in("x8") 2, // SYSCALL_YIELD
            lateout("x0") _, 
            options(nostack)
        );
    }
}

pub fn sys_spawn_image(image: &[u8], name: &str) -> Result<isize, isize> {
    let ret: isize;
    unsafe {
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!(
            "syscall",
            in("rax") SYSCALL_SPAWN,
            in("rdi") image.as_ptr() as usize,
            in("rsi") image.len(),
            in("rdx") name.as_ptr() as usize,
            in("r10") name.len(),
            lateout("rax") ret,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
        #[cfg(target_arch = "aarch64")]
        core::arch::asm!(
            "svc #0",
            in("x8") SYSCALL_SPAWN,
            in("x0") image.as_ptr() as usize,
            in("x1") image.len(),
            in("x2") name.as_ptr() as usize,
            in("x3") name.len(),
            lateout("x0") ret,
            options(nostack)
        );
    }

    if ret >= 0 {
        Ok(ret)
    } else {
        Err(ret)
    }
}

pub unsafe fn sys_driver_publish(ptr: *const u8, len: usize) -> isize {
    let ret: isize;
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!(
        "syscall",
        in("rax") 101, // SYSCALL_DRIVER_PUBLISH
        in("rdi") ptr as usize,
        in("rsi") len,
        lateout("rax") ret,
        out("rcx") _,
        out("r11") _,
    );
     #[cfg(target_arch = "aarch64")]
    core::arch::asm!(
        "svc #0",
        in("x8") 101,
        in("x0") ptr as usize,
        in("x1") len,
        lateout("x0") ret,
        options(nostack)
    );
    ret
}
