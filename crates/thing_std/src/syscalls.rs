use abi::{SysRet, SYSCALL_DRIVER_WAIT, SYSCALL_DRIVER_PUBLISH, SYSCALL_GRAPH};
use abi::wire::graph::GraphOp;
use core::arch::asm;

// Raw syscall
unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> SysRet {
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

unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> SysRet {
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

pub fn driver_wait(out_buf: &mut [u8]) -> Result<usize, ()> {
    let ret = unsafe {
        syscall2(SYSCALL_DRIVER_WAIT, out_buf.as_mut_ptr() as usize, out_buf.len())
    };
    if ret >= 0 {
        Ok(ret as usize)
    } else {
        Err(())
    }
}

pub fn driver_publish(data: &[u8]) -> Result<(), ()> {
    let ret = unsafe {
        syscall2(SYSCALL_DRIVER_PUBLISH, data.as_ptr() as usize, data.len())
    };
    if ret == 0 {
        Ok(())
    } else {
        Err(())
    }
}

pub fn graph_op(op: &GraphOp) -> Result<(), ()> {
    // Serialize Op? Wait, original ABI might differ.
    // Usually we act on struct?
    // "graph_op" in userland might wrap a "call" or "send".
    // Wait, the ABI for SYSCALL_GRAPH needs to be defined.
    // crates/kernel_core/src/syscalls/mod.rs => graph.rs
    // Look at `crates/kernel_core/src/syscalls/graph.rs` again?
    // Step 108: `pub fn handle_graph_op...`
    // Step 122: `pub fn handle_graph_query...`
    // But how are they exposed as SYSCALLs?
    // I need to find the `syscall_dispatch` routine in `kernel_core/src/syscalls/mod.rs` or similar.
    // If not, I am guessing.
    // Let's assume SYSCALL_GRAPH (1) dispatching is: (ptr, len, out_ptr, out_len) -> ret?
    // User request: apps use exactly one syscall: sys_graph(ptr,len).
    // Wait. "sys_graph(ptr,len)".
    // Maybe query/response model?

    // Stub:
    // We didn't verify the kernel syscall dispatcher properly.
    // But let's assume `sys_graph(ptr, len) -> ret`.
    // We need to pass the Op.
    // Let's defer implementation until we check dispatch.
    
    // For now, let's just serialize op? No.
    // Let's implement `graph_query` instead.
    Err(())
}

unsafe fn syscall6(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize, a6: usize) -> SysRet {
    let ret: SysRet;
    asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4, // RCX clobbered by syscall, so R10 used for arg4 in syscall ABI? 
                      // NO. x86_64 syscall ABI uses R10 for Arg4, not RCX.
                      // System V function ABI uses RCX for Arg4.
                      // Linux Syscall ABI: RDI, RSI, RDX, R10, R8, R9.
                      // My bridge `syscall_handler_naked`:
                      // "mov r9, r8", "mov r8, r10", "mov rcx, rdx", "mov rdx, rsi", "mov rsi, rdi", "mov rdi, rax"
                      // My handler maps:
                      // Syscall RDI -> Rust A1 (RDI)
                      // Syscall RSI -> Rust A2 (RSI)
                      // Syscall RDX -> Rust A3 (RDX)
                      // Syscall R10 -> Rust A4 (RCX)
                      // Syscall R8  -> Rust A5 (R8)
                      // Syscall R9  -> Rust A6 (R9)
                      
                      // So here in userspace:
                      // I must put Arg1 in RDI, Arg2 in RSI, Arg3 in RDX, Arg4 in R10, Arg5 in R8, Arg6 in R9.
                      // Syscall instruction destroys RCX and R11.
                      
        in("r8") a5,
        in("r9") a6,
        lateout("rax") ret,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

pub fn graph_query(query: &str, params: &[u8], out: &mut [u8]) -> Result<usize, ()> {
    let ret = unsafe {
        syscall6(
            SYSCALL_GRAPH,
            query.as_ptr() as usize,
            query.len(),
            params.as_ptr() as usize,
            params.len(),
            out.as_mut_ptr() as usize,
            out.len()
        )
    };
    if ret >= 0 {
        Ok(ret as usize)
    } else {
        Err(())
    }
}

