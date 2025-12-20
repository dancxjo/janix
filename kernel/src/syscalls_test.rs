use abi::syscalls::*;

#[test]
fn test_syscall_constants_hygiene() {
    // 1. Enumerate all syscall numbers from abi/src/syscalls.rs
    // Since we cannot iterate module constants automatically without macros, we manually list them here.
    // This list must be kept in sync with abi/src/syscalls.rs.
    // The test ensures they are distinct.

    let syscalls = [
        ("SYSCALL_YIELD", SYSCALL_YIELD),
        ("SYSCALL_SLEEP_FOR_NS", SYSCALL_SLEEP_FOR_NS),
        ("SYSCALL_SLEEP_UNTIL", SYSCALL_SLEEP_UNTIL),
        ("SYSCALL_TIME_MONOTONIC_NS", SYSCALL_TIME_MONOTONIC_NS),
        ("SYSCALL_TIME_SYSTEM_NS", SYSCALL_TIME_SYSTEM_NS),
        ("SYSCALL_TIME_NOW", SYSCALL_TIME_NOW),
        ("SYSCALL_LOG", SYSCALL_LOG),
        ("SYSCALL_EXIT_THREAD", SYSCALL_EXIT_THREAD),
        ("SYSCALL_CREATE_PROCESS", SYSCALL_CREATE_PROCESS),
        ("SYSCALL_CREATE_THREAD", SYSCALL_CREATE_THREAD),
        ("SYSCALL_SPAWN_PROGRAM", SYSCALL_SPAWN_PROGRAM),
        ("SYSCALL_ALLOC_FRAME", SYSCALL_ALLOC_FRAME),
        ("SYSCALL_FREE_FRAME", SYSCALL_FREE_FRAME),
        ("SYSCALL_THING_CREATE", SYSCALL_THING_CREATE),
        ("SYSCALL_THING_GET", SYSCALL_THING_GET),
        ("SYSCALL_THING_UPDATE", SYSCALL_THING_UPDATE),
        ("SYSCALL_THING_LIST", SYSCALL_THING_LIST),
        ("SYSCALL_ADD_LINK", SYSCALL_ADD_LINK),
        ("SYSCALL_LINK_AT", SYSCALL_LINK_AT),
        ("SYSCALL_GRAPH_QUERY", SYSCALL_GRAPH_QUERY),
        ("SYSCALL_CREATE_TRANSACTION", SYSCALL_CREATE_TRANSACTION),
        ("SYSCALL_COMMIT_TRANSACTION", SYSCALL_COMMIT_TRANSACTION),
        ("SYSCALL_SCHEMA_REGISTER_PACKAGE", SYSCALL_SCHEMA_REGISTER_PACKAGE),
        ("SYSCALL_SCHEMA_GET", SYSCALL_SCHEMA_GET),
        ("SYSCALL_MAP_SHARED_BUFFER", SYSCALL_MAP_SHARED_BUFFER),
        ("SYSCALL_CREATE_SHARED_BUFFER", SYSCALL_CREATE_SHARED_BUFFER),
        ("SYSCALL_GET_SHARED_BUFFER_INFO", SYSCALL_GET_SHARED_BUFFER_INFO),
        ("SYSCALL_RESIDENT_ALLOC", SYSCALL_RESIDENT_ALLOC),
        ("SYSCALL_RESIDENT_MAP", SYSCALL_RESIDENT_MAP),
        ("SYSCALL_RESIDENT_UNMAP", SYSCALL_RESIDENT_UNMAP),
        ("SYSCALL_THING_REST", SYSCALL_THING_REST),
        ("SYSCALL_SYMBOL_INTERN", SYSCALL_SYMBOL_INTERN),
        ("SYSCALL_SYMBOL_RESOLVE", SYSCALL_SYMBOL_RESOLVE),
        ("SYSCALL_THING_BATCH_UPDATE", SYSCALL_THING_BATCH_UPDATE),
        ("SYSCALL_DEV_OPEN", SYSCALL_DEV_OPEN),
        ("SYSCALL_DEV_READ", SYSCALL_DEV_READ),
    ];

    // Check for duplicates
    for i in 0..syscalls.len() {
        for j in (i + 1)..syscalls.len() {
            if syscalls[i].1 == syscalls[j].1 {
                panic!(
                    "Duplicate syscall number detected: {} and {} both use {}",
                    syscalls[i].0, syscalls[j].0, syscalls[i].1
                );
            }
        }
    }

    // Check specific required constants
    assert_eq!(SYSCALL_SCHEMA_REGISTER_PACKAGE, 19, "SYSCALL_SCHEMA_REGISTER_PACKAGE must be 19");
    assert_eq!(SYSCALL_DEV_OPEN, 64, "SYSCALL_DEV_OPEN must be 64");
}
