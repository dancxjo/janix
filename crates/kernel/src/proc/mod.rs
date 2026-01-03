//! Process and Thread Management

use crate::machine::Context;
use abi::ids::ThingId;



#[derive(Debug)]
pub enum ThreadState {
    Runnable,
    Blocked,
    Exited,
}

#[derive(Debug)]
pub struct Process {
    pub pid: ThingId,
    // AddressSpace will go here
    pub heap_base: u64,
    pub heap_limit: u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct Thread {
    pub tid: ThingId,
    pub process: *mut Process,
    
    // Kernel stack top (for TSS/SYSCALL)
    pub kernel_stack_top: u64,
    
    // Saved context (callee-saved regs + SP)
    pub context: Context,
    
    pub state: ThreadState,
}

pub fn spawn_kernel_module(module: &crate::boot::ModuleInfo) -> Result<(), ()> {
    // Stub
    crate::log::kprintln("PROC: spawn_kernel_module stub called");
    Ok(())
}
