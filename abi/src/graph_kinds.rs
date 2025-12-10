//! Shared graph kind and edge identifiers used by both kernel and userland.

pub const KIND_THREAD: &str = "Thread";
pub const KIND_PROCESS: &str = "Process";
pub const KIND_CPU_CORE: &str = "CpuCore";
pub const KIND_SLEEP_EVENT: &str = "SleepEvent";
pub const KIND_BOOT_PROFILE: &str = "BootProfile";
pub const KIND_BOOT_PROGRAM: &str = "BootProgram";
pub const KIND_PROGRAM_IMAGE: &str = "ProgramImage";

pub const EDGE_OWNS_THREAD: &str = "proc.owns_thread";
pub const EDGE_RUNS_ON: &str = "sched.runs_on";
pub const EDGE_SLEEPS_UNTIL: &str = "sched.sleeps_until";
pub const EDGE_LAUNCHES: &str = "boot.launches";
pub const EDGE_SPAWNED: &str = "init.spawned";
