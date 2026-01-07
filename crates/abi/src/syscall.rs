//! Syscall Numbers and Error Codes

pub mod nr {
    pub const SYS_LOG: u32 = 1;
    pub const SYS_SCHED_YIELD: u32 = 2;

    pub const SYS_THING_CREATE: u32 = 10;
    pub const SYS_REL_CREATE: u32 = 11;
    pub const SYS_REL_DELETE: u32 = 12;

    pub const SYS_THING_GET: u32 = 20;
    pub const SYS_REL_GET_FROM: u32 = 21;
    pub const SYS_THING_FIND: u32 = 22;
    pub const SYS_THING_REGISTER_NAME: u32 = 23;
    pub const SYS_SYMBOL_RESOLVE: u32 = 24;
    pub const SYS_SYMBOL_INTERN: u32 = 25;
    pub const SYS_THING_SET_BODY: u32 = 26;
    pub const SYS_REL_GET_TARGETS: u32 = 27;

    pub const SYS_BYTESPACE_CREATE: u32 = 30;
    pub const SYS_SPACE_MAP: u32 = 31;
    pub const SYS_SPACE_UNMAP: u32 = 32;
    pub const SYS_HEAP_GROW: u32 = 33;
    pub const SYS_DMA_BYTESPACE_CREATE: u32 = 34;

    pub const SYS_WATCH_CREATE: u32 = 40;
    pub const SYS_WATCH_POLL: u32 = 41;
    pub const SYS_WAIT: u32 = 42;

    pub const SYS_SURFACE_CREATE: u32 = 50;
    pub const SYS_SURFACE_DRAW: u32 = 51;

    pub const SYS_PROC_SPAWN: u32 = 60;
    pub const SYS_PROC_EXIT: u32 = 61;
    pub const SYS_CAP_GRANT: u32 = 70;
    pub const SYS_DISPLAY_PRIMARY: u32 = 160;

    pub const SYS_TIME_MONOTONIC_NOW: u32 = 170;
    pub const SYS_TIME_SYSTEM_NOW: u32 = 171;
    pub const SYS_TIME_SET_SYSTEM: u32 = 172;
    pub const SYS_SLEEP_UNTIL: u32 = 173;

    pub const SYS_MACHINE: u32 = 100;
    pub const SYS_CPU_FEATURES: u32 = 110;

    pub const SYS_IOPORT_READ8: u32 = 130;
    pub const SYS_IOPORT_WRITE8: u32 = 131;

    pub const SYS_PCI_CFG_READ32: u32 = 180;
    pub const SYS_BOOT_PROGRESS: u32 = 190;

    pub const SYS_INPUT_READ: u32 = 200;

    // Thread syscalls
    pub const SYS_THREAD_SPAWN: u32 = 80;
    pub const SYS_THREAD_EXIT: u32 = 81;
    pub const SYS_THREAD_JOIN: u32 = 82;
    pub const SYS_THREAD_BLOCK_ON_WATCH: u32 = 84;
}

pub mod err {
    pub const EFAULT: i32 = 1;
    pub const EINVAL: i32 = 2;
    pub const ENOSYS: i32 = 3;
    pub const EPERM: i32 = 4;
    pub const ENOENT: i32 = 5;
    pub const ENOMEM: i32 = 6;
    pub const EAGAIN: i32 = 7;
    pub const ERR_INVALID_THING_BODY: i32 = 8;
}
