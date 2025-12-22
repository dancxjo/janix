# Syscall Dispatch Map

| Number | Syscall Name | KernelRequest Variant | Kernel Handler | Notes |
|---|---|---|---|---|
| 0 | `SYSCALL_YIELD` | `N/A` | `kernel::sched::yield_current_thread` |  |
| 1 | `SYSCALL_SLEEP_FOR_NS` | `N/A` | `crate::user::sys_sleep_for_ns` |  |
| 2 | `SYSCALL_SLEEP_UNTIL` | `N/A` | `Inline/Unknown` |  |
| 3 | `SYSCALL_TIME_MONOTONIC_NS` | `N/A` | `Inline/Unknown` |  |
| 4 | `SYSCALL_TIME_SYSTEM_NS` | `N/A` | `Inline/Unknown` |  |
| 5 | `SYSCALL_TIME_NOW` | `N/A` | `Inline/Unknown` | unsafe block, direct user memory |
| 6 | `SYSCALL_LOG` | `N/A` | `Inline/Unknown` | unsafe block, direct user memory |
| 7 | `SYSCALL_EXIT_THREAD` | `N/A` | `kernel::sched::exit_current_thread` |  |
| 8 | `SYSCALL_ALLOC_FRAME` | `N/A` | `Inline/Unknown` | unsafe block, direct user memory |
| 9 | `SYSCALL_FREE_FRAME` | `N/A` | `Inline/Unknown` | unsafe block |
| 10 | `SYSCALL_CREATE_PROCESS` | `N/A` | `Inline/Unknown` | unsafe block, direct user memory |
| 11 | `SYSCALL_CREATE_THREAD` | `N/A` | `Inline/Unknown` | unsafe block, direct user memory, transmute |
| 12 | `SYSCALL_SPAWN_PROGRAM` | `SpawnProgram` | `kernel::handle_request (SpawnProgram)` | unsafe block, direct user memory |
| 13 | `SYSCALL_THING_CREATE` | `N/A` | `kernel::graph::create_thing` | unsafe block, direct user memory |
| 14 | `SYSCALL_THING_GET` | `N/A` | `kernel::symbols::intern` | unsafe block, direct user memory |
| 15 | `SYSCALL_THING_UPDATE` | `N/A` | `kernel::graph::update_thing` | unsafe block, direct user memory |
| 16 | `SYSCALL_THING_LIST` | `N/A` | `Inline/Unknown` |  |
| 17 | `SYSCALL_ADD_LINK` | `N/A` | `kernel::graph::add_link` |  |
| 18 | `SYSCALL_LINK_AT` | `N/A` | `kernel::graph::link_target_at` |  |
| 19 | `SYSCALL_SCHEMA_REGISTER_PACKAGE` | `SchemaRegisterPackage` | `kernel::handle_request (SchemaRegisterPackage)` |  |
| 20 | `SYSCALL_GRAPH_QUERY` | `N/A` | `Inline/Unknown` | unsafe block |
| 21 | `SYSCALL_CREATE_TRANSACTION` | `N/A` | `Inline/Unknown` |  |
| 22 | `SYSCALL_COMMIT_TRANSACTION` | `N/A` | `Inline/Unknown` |  |
| 23 | `SYSCALL_MAP_SHARED_BUFFER` | `N/A` | `Inline/Unknown` | unsafe block, direct user memory |
| 24 | `SYSCALL_CREATE_SHARED_BUFFER` | `N/A` | `kernel::shared_buffer::register_shared_buffer` | unsafe block, transmute |
| 25 | `SYSCALL_GET_SHARED_BUFFER_INFO` | `N/A` | `Inline/Unknown` | unsafe block, direct user memory |
| 26 | `SYSCALL_RESIDENT_ALLOC` | `N/A` | `kernel::resident::manager::sys_resident_alloc` | unsafe block, direct user memory |
| 27 | `SYSCALL_RESIDENT_MAP` | `N/A` | `kernel::resident::manager::sys_resident_map` | unsafe block, direct user memory |
| 28 | `SYSCALL_RESIDENT_UNMAP` | `N/A` | `kernel::resident::manager::sys_resident_unmap` | unsafe block, direct user memory |
| 29 | `SYSCALL_THING_REST` | `N/A` | `kernel::resident::manager::sys_thing_rest` | unsafe block, direct user memory, transmute |
| 30 | `SYSCALL_SYMBOL_INTERN` | `N/A` | `kernel::symbols::intern` | unsafe block, direct user memory |
| 31 | `SYSCALL_SYMBOL_RESOLVE` | `N/A` | `kernel::symbols::resolve` | unsafe block, direct user memory |
| 32 | `SYSCALL_THING_BATCH_UPDATE` | `N/A` | `kernel::graph::update_thing` | unsafe block, direct user memory |
| 33 | `SYSCALL_SCHEMA_GET` | `SchemaGet` | `kernel::handle_request (SchemaGet)` | unsafe block, direct user memory |
| 64 | `SYSCALL_DEV_OPEN` | `N/A` | `kernel::bridge::ps2::dev_open` | unsafe block, direct user memory, transmute |
| 65 | `SYSCALL_DEV_READ` | `N/A` | `kernel::bridge::ps2::dev_read` | unsafe block, direct user memory |
| 66 | `SYSCALL_PCI_READ_CONFIG` | `N/A` | `Inline/Unknown` | unsafe block, direct user memory |
