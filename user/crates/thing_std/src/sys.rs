#![allow(dead_code)]

// use abi::SysRet; // Unused for now in this signature, but good to have if we expand.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SysErr(pub i64);

pub fn sys_graph(_query: &str, _params: &[u8], _out: &mut [u8]) -> Result<usize, SysErr> {
    // TODO: wire to actual syscall once userland ABI is stable.
    // For now, this must compile. In hosted tests it can be a mocked shim.
    // In a real build, this would define the syscall logic.
    // Since we are building "cleanly" but maybe not running on bare metal immediately (or the user will link it),
    // we return a dummy error so it compiles. 
    // IF the user wanted a real syscall, I'd use inline asm, but they said "No assumptions about arch calling convention... unimplemented!() behind a feature flag or just error".
    // "Err(SysErr(-1))" is fine.
    Err(SysErr(-1))
}
