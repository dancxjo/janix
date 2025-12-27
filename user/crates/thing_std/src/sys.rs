#![allow(dead_code)]

// use abi::SysRet; // Unused for now in this signature, but good to have if we expand.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SysErr(pub i64);

pub fn sys_graph(query: &str, params: &[u8], out: &mut [u8]) -> Result<usize, SysErr> {
    let ret: isize;
    unsafe {
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
    }
    
    if ret < 0 {
        Err(SysErr(ret as i64))
    } else {
        Ok(ret as usize)
    }
}

pub fn sys_yield() {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 2, // SYSCALL_YIELD
            out("rcx") _,
            out("r11") _,
        );
    }
}
