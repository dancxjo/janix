#![allow(unused_macros)]

#[macro_export]
macro_rules! for_each_syscall {
    ($mac:ident) => {
        $mac! {
            // Core & Diagnostics
            SYSCALL_LOG => 0,
            SYSCALL_YIELD => 1,
            SYSCALL_EXIT_THREAD => 2,
            SYSCALL_TIME_NOW => 3,
            SYSCALL_TIME_MONOTONIC_NS => 4,
            SYSCALL_TIME_SYSTEM_NS => 5,
            SYSCALL_SLEEP_FOR_NS => 6,
            SYSCALL_SLEEP_UNTIL => 7,

            // Memory Management
            SYSCALL_ALLOC_FRAME => 10,
            SYSCALL_FREE_FRAME => 11,
            SYSCALL_RESIDENT_ALLOC => 12,
            SYSCALL_RESIDENT_MAP => 13,
            SYSCALL_RESIDENT_UNMAP => 14,
            SYSCALL_THING_REST => 15,

            // Graph: Read & Query
            SYSCALL_SYMBOL_INTERN => 20,
            SYSCALL_SYMBOL_RESOLVE => 21,
            SYSCALL_THING_GET => 22,
            SYSCALL_THING_LIST => 23,
            SYSCALL_LINK_AT => 24,
            SYSCALL_GRAPH_QUERY => 25,
            SYSCALL_SCHEMA_GET => 26,

            // Graph: Write & Modification
            SYSCALL_THING_CREATE => 30,
            SYSCALL_THING_UPDATE => 31,
            SYSCALL_THING_BATCH_UPDATE => 32,
            SYSCALL_ADD_LINK => 33,
            SYSCALL_SCHEMA_REGISTER_PACKAGE => 34,
            SYSCALL_CREATE_TRANSACTION => 35,
            SYSCALL_COMMIT_TRANSACTION => 36,

            // Process & IPC
            SYSCALL_CREATE_THREAD => 40,
            SYSCALL_CREATE_PROCESS => 41,
            SYSCALL_SPAWN_PROGRAM => 42,
            SYSCALL_CREATE_SHARED_BUFFER => 43,
            SYSCALL_MAP_SHARED_BUFFER => 44,
            SYSCALL_GET_SHARED_BUFFER_INFO => 45,

            // Hardware & Drivers
            SYSCALL_DEV_OPEN => 50,
            SYSCALL_DEV_READ => 51,
            SYSCALL_PCI_READ_CONFIG => 52,
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
