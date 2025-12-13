use runtime::Sys;

#[cfg(target_os = "none")]
use alloc::boxed::Box;
#[cfg(target_os = "none")]
use alloc::string::ToString;
#[cfg(not(target_os = "none"))]
use std::string::ToString;

/// Create a process with the provided display name.
pub fn create_process(sys: &impl Sys, name: &str) -> Option<u64> {
    let leaked: &'static str = Box::leak(name.to_string().into_boxed_str());
    match sys.syscall(abi::KernelRequest::CreateProcess { name: leaked }) {
        abi::KernelResponse::ProcessCreated { pid } => Some(pid),
        _ => None,
    }
}

/// Create a thread within a process.
pub fn create_thread(
    sys: &impl Sys,
    pid: u64,
    name: &str,
    app_id: u64,
    priority: u64,
) -> Option<u64> {
    let leaked: &'static str = Box::leak(name.to_string().into_boxed_str());
    match sys.syscall(abi::KernelRequest::CreateThread {
        pid,
        name: leaked,
        app_id,
        priority,
    }) {
        abi::KernelResponse::ThreadCreated { tid } => Some(tid),
        _ => None,
    }
}

/// Poll the scheduler for the currently running thread.
pub fn scheduler_tick(sys: &impl Sys) -> Option<abi::ThreadInfo> {
    match sys.syscall(abi::KernelRequest::SchedulerTick) {
        abi::KernelResponse::SchedulerTicked { current } => current,
        _ => None,
    }
}

/// Spawn a boot program and return its generated process/thread IDs.
pub fn spawn_program(sys: &mut impl Sys, boot_program_id: abi::ThingId) -> Option<(abi::ThingId, abi::ThingId)> {
    match sys.syscall(abi::KernelRequest::SpawnProgram { boot_program_id }) {
        abi::KernelResponse::ProgramSpawned {
            process_id,
            thread_id,
        } => Some((process_id, thread_id)),
        abi::KernelResponse::Error { message } => {
            crate::syscalls::println(sys, message);
            None
        }
        _ => None,
    }
}
