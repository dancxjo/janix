#![allow(unused_macros)]

#[macro_export]
macro_rules! for_each_syscall {
    ($mac:ident) => {
        $mac! {
            // Time & Scheduling
            SYSCALL_YIELD => 0,
            SYSCALL_SLEEP_FOR_NS => 1,
            SYSCALL_SLEEP_UNTIL => 2,
            SYSCALL_TIME_MONOTONIC_NS => 3,
            SYSCALL_TIME_SYSTEM_NS => 4,
            SYSCALL_TIME_NOW => 5,

            // Diagnostics
            SYSCALL_LOG => 6,

            // Thread & Process
            SYSCALL_EXIT_THREAD => 7,
            SYSCALL_ALLOC_FRAME => 8,
            SYSCALL_FREE_FRAME => 9,
            SYSCALL_CREATE_PROCESS => 10,
            SYSCALL_CREATE_THREAD => 11,
            SYSCALL_SPAWN_PROGRAM => 12,

            // Graph Operations
            SYSCALL_THING_CREATE => 13,
            SYSCALL_THING_GET => 14,
            SYSCALL_THING_UPDATE => 15,
            SYSCALL_THING_LIST => 16,
            SYSCALL_ADD_LINK => 17,
            SYSCALL_LINK_AT => 18,
            SYSCALL_SCHEMA_REGISTER_PACKAGE => 19,
            SYSCALL_GRAPH_QUERY => 20,
            SYSCALL_CREATE_TRANSACTION => 21,
            SYSCALL_COMMIT_TRANSACTION => 22,

            // Shared Buffer
            SYSCALL_MAP_SHARED_BUFFER => 23,
            SYSCALL_CREATE_SHARED_BUFFER => 24,
            SYSCALL_GET_SHARED_BUFFER_INFO => 25,

            // Resident Memory
            SYSCALL_RESIDENT_ALLOC => 26,
            SYSCALL_RESIDENT_MAP => 27,
            SYSCALL_RESIDENT_UNMAP => 28,
            SYSCALL_THING_REST => 29,

            // Symbols
            SYSCALL_SYMBOL_INTERN => 30,
            SYSCALL_SYMBOL_RESOLVE => 31,
            SYSCALL_THING_BATCH_UPDATE => 32,
            SYSCALL_SCHEMA_GET => 33,

            // Devices
            SYSCALL_DEV_OPEN => 64,
            SYSCALL_DEV_READ => 65,
        }
    };
}

macro_rules! define_syscall_const {
    ($($name:ident => $num:expr),* $(,)?) => {
        $( pub const $name: u64 = $num; )*
    };
}

macro_rules! define_syscall_list {
    ($($name:ident => $num:expr),* $(,)?) => {
        pub const ABI_SYSCALL_NUMBERS: &[u64] = &[ $($num),* ];
    };
}

for_each_syscall!(define_syscall_const);
for_each_syscall!(define_syscall_list);
