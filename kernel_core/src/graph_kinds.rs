//! Canonical graph kind and edge identifiers used by kernel subsystems.
//! Keeping them centralized avoids typos across the kernel/boot boundary.

pub const KIND_THREAD: &str = "Thread";
pub const KIND_PROCESS: &str = "Process";
pub const KIND_CPU_CORE: &str = "CpuCore";
pub const KIND_SLEEP_EVENT: &str = "SleepEvent";

pub const EDGE_OWNS_THREAD: &str = "proc.owns_thread";
pub const EDGE_RUNS_ON: &str = "sched.runs_on";
pub const EDGE_SLEEPS_UNTIL: &str = "sched.sleeps_until";
