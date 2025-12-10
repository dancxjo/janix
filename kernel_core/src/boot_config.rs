//! Static boot configuration describing the initial init profile.
//!
//! This data defines the BootProfile → BootProgram graph that the kernel
//! materializes during boot. Init reads these nodes to decide which programs
//! to spawn, so keeping the authoritative list here keeps hosted and bare-metal
//! configurations in sync.

pub struct BootProgramSpec {
    pub name: &'static str,
    pub binary: &'static str,
    pub app_id: u64,
    pub priority: u64,
}

pub const BOOT_PROFILE_VERSION: u64 = 1;

pub const BOOT_PROGRAMS: &[BootProgramSpec] = &[
    BootProgramSpec {
        name: "hello",
        binary: "user_app_hello",
        app_id: 1,
        priority: 1,
    },
    BootProgramSpec {
        name: "heartbeat",
        binary: "user_app_heartbeat",
        app_id: 2,
        priority: 1,
    },
    BootProgramSpec {
        name: "thread_dashboard",
        binary: "user_app_thread_dashboard",
        app_id: 3,
        priority: 1,
    },
];
