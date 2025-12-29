use abi::wire::graph::GraphOp;
use abi::{SysRet, SYSCALL_DRIVER_PUBLISH, SYSCALL_DRIVER_WAIT, SYSCALL_GRAPH, SYSCALL_RTC_READ};
use core::arch::asm;

// Raw syscall
pub unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> SysRet {
    let ret: SysRet;
    asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        in("rsi") a2,
        lateout("rax") ret,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

pub unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> SysRet {
    let ret: SysRet;
    asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        lateout("rax") ret,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

pub unsafe fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> SysRet {
    let ret: SysRet;
    asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        lateout("rax") ret,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

pub unsafe fn syscall6(
    n: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> SysRet {
    let ret: SysRet;
    asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        in("r8") a5,
        in("r9") a6,
        lateout("rax") ret,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

pub fn rtc_read(out: &mut abi::wire::time::RtcSample) -> Result<(), ()> {
    let ret = unsafe { syscall2(SYSCALL_RTC_READ, out as *mut _ as usize, 0) };
    if ret == 0 {
        Ok(())
    } else {
        Err(())
    }
}

pub fn driver_wait(out_buf: &mut [u8]) -> Result<usize, ()> {
    let ret = unsafe {
        syscall2(
            SYSCALL_DRIVER_WAIT,
            out_buf.as_mut_ptr() as usize,
            out_buf.len(),
        )
    };
    if ret >= 0 {
        Ok(ret as usize)
    } else {
        Err(())
    }
}

pub fn driver_publish(data: &[u8]) -> Result<(), isize> {
    let ret = unsafe { syscall2(SYSCALL_DRIVER_PUBLISH, data.as_ptr() as usize, data.len()) };
    if ret == 0 {
        Ok(())
    } else {
        Err(ret as isize)
    }
}

pub fn graph_op(_op: &GraphOp) -> Result<(), ()> {
    Err(())
}

pub fn graph_query(query: &str, params: &[u8], out: &mut [u8]) -> Result<usize, isize> {
    let ret = unsafe {
        syscall6(
            SYSCALL_GRAPH,
            query.as_ptr() as usize,
            query.len(),
            params.as_ptr() as usize,
            params.len(),
            out.as_mut_ptr() as usize,
            out.len(),
        )
    };
    if ret >= 0 {
        Ok(ret as usize)
    } else {
        Err(ret as isize)
    }
}
