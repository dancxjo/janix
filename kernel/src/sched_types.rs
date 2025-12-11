//! Shared scheduling-related types and helpers used across kernel modules.

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
