//! Shared scheduling-related types and helpers.

use core::str::FromStr;

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
    pub fn as_str(&self) -> &'static str {
        match self {
            ThreadState::New => "New",
            ThreadState::Runnable => "Runnable",
            ThreadState::Running => "Running",
            ThreadState::Sleeping => "Sleeping",
            ThreadState::Blocked => "Blocked",
            ThreadState::Exited => "Exited",
        }
    }
}

impl FromStr for ThreadState {
    type Err = ();

    fn from_str(state: &str) -> Result<Self, Self::Err> {
        match state {
            "New" => Ok(ThreadState::New),
            "Runnable" => Ok(ThreadState::Runnable),
            "Running" => Ok(ThreadState::Running),
            "Sleeping" => Ok(ThreadState::Sleeping),
            "Blocked" => Ok(ThreadState::Blocked),
            "Exited" => Ok(ThreadState::Exited),
            _ => Err(()),
        }
    }
}
