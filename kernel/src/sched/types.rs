//! Shared scheduling-related types and helpers used across kernel modules.
use abi::{ProcessId, ThingId, ThreadId};

pub type CpuId = u64;
pub type TimeNs = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThreadState {
    New,
    Runnable,
    Running,
    Sleeping,
    Blocked,
    Exited,
}

impl ThreadState {
    pub const fn as_str(&self) -> &'static str {
        match self {
            ThreadState::New => "New",
            ThreadState::Runnable => "Runnable",
            ThreadState::Running => "Running",
            ThreadState::Sleeping => "Sleeping",
            ThreadState::Blocked => "Blocked",
            ThreadState::Exited => "Exited",
        }
    }

    pub fn from_str(state: &str) -> Option<Self> {
        match state {
            "New" => Some(ThreadState::New),
            "Runnable" => Some(ThreadState::Runnable),
            "Running" => Some(ThreadState::Running),
            "Sleeping" => Some(ThreadState::Sleeping),
            "Blocked" => Some(ThreadState::Blocked),
            "Exited" => Some(ThreadState::Exited),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SleepEntry {
    pub thread_id: ThreadId,
    pub wake_at_ns: u64,
}

#[repr(align(16))]
#[derive(Copy, Clone, Debug)]
pub struct FpuContext {
    pub data: [u8; 512],
}

impl Default for FpuContext {
    fn default() -> Self {
        let mut data = [0u8; 512];
        // FCW = 0x037F
        data[0] = 0x7F;
        data[1] = 0x03;
        // MXCSR = 0x00001F80
        data[24] = 0x80;
        data[25] = 0x1F;
        Self { data }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Thread {
    pub id: ThreadId,
    pub process_id: ProcessId,
    pub state: ThreadState,
    pub name: &'static str,
    pub priority: u64,
    pub entry_point: u64,
    pub user_arg: u64,
    pub user_stack_top: u64,
    pub context: [u64; 20],
    pub fpu_context: FpuContext,
    pub started: bool,
    pub thing_id: Option<ThingId>,
    pub sleep_event_id: Option<ThingId>,
    pub sleep_until_ns: u64,
    pub last_run_start_ns: u64,
    pub total_run_ns: u64,
    pub address_space_token: Option<u64>,
    pub pending_wake: bool,
    pub is_idle: bool,
}

pub struct ScheduledThread {
    pub tid: ThreadId,
    pub process_id: ProcessId,
    pub name: &'static str,
    pub started: bool,
    pub entry_point: u64,
    pub user_stack_top: u64,
    pub user_arg: u64,
    pub context: [u64; 20],
    pub fpu_context: FpuContext,
    pub address_space_token: Option<u64>,
    pub is_idle: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct Process {
    pub id: ProcessId,
    pub name: &'static str,
    pub package_id: abi::syscall_defs::SymbolId,
    pub thing_id: Option<ThingId>,
    pub address_space_token: Option<u64>,
    pub heap_base: usize,
    pub heap_limit: usize,
    pub next_map_base: u64,
    pub next_resident_map_base: u64,
}
