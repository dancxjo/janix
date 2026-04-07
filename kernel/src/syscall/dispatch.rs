use crate::syscall::handlers;

use abi::syscall::*;

pub fn dispatch(n: usize, args: [usize; 6]) -> isize {
    let syscall_id = n as u32;

    let result = match syscall_id {
        SYS_EXIT => handlers::sys_exit(args[0] as i32),
        SYS_REBOOT => handlers::sys_reboot(),
        SYS_READ => handlers::sys_read(args[0], args[1], args[2]),
        SYS_WRITE => handlers::sys_write(args[0], args[1], args[2]),
        SYS_DEBUG_WRITE => handlers::sys_debug_write(args[0], args[1]),
        SYS_LOG_WRITE => handlers::sys_log_write(args[0], args[1], args[2]),
        SYS_YIELD => handlers::sys_yield(),
        SYS_SLEEP_MS => handlers::sys_sleep_ms(args[0] as u64),
        SYS_SLEEP_NS => handlers::sys_sleep_ns(args[0] as u64),
        SYS_TIME_MONOTONIC => handlers::sys_time_monotonic_ns(),
        SYS_TIME_NOW => handlers::sys_time_now(),
        SYS_TIME_ANCHOR => handlers::sys_time_anchor(args[0] as u64),
        SYS_DEVICE_CALL => handlers::sys_device_call(args[0]),
        SYS_SPAWN_THREAD => handlers::sys_spawn_thread(args[0], args[1]),
        SYS_SPAWN_PROCESS => handlers::sys_spawn_process(args[0], args[1], args[2]),
        SYS_GET_TID => handlers::sys_get_tid(),
        SYS_TASK_POLL => handlers::sys_task_poll(args[0]),
        SYS_TASK_KILL => handlers::sys_task_kill(args[0]),
        SYS_SET_PRIORITY => handlers::sys_set_priority(args[0], args[1]),
        SYS_TASK_DUMP => handlers::sys_task_dump(),
        SYS_GETPID => handlers::sys_getpid(),
        SYS_GETPPID => handlers::sys_getppid(),
        SYS_ARGV_GET => handlers::sys_argv_get(args[0], args[1]),
        SYS_ENV_GET => handlers::sys_env_get(args[0], args[1], args[2], args[3]),
        SYS_ENV_SET => handlers::sys_env_set(args[0], args[1], args[2], args[3]),
        SYS_ENV_UNSET => handlers::sys_env_unset(args[0], args[1]),
        SYS_ENV_LIST => handlers::sys_env_list(args[0], args[1]),
        SYS_SPAWN_PROCESS_EX => handlers::sys_spawn_process_ex(args[0], args[1]),
        SYS_ALLOC_STACK => handlers::sys_alloc_stack(args[0]),
        SYS_FUTEX_WAIT => handlers::sys_futex_wait(args[0], args[1] as u32, args[2] as u64),
        SYS_FUTEX_WAKE => handlers::sys_futex_wake(args[0], args[1] as u32),
        SYS_WAIT_MANY => {
            handlers::sys_wait_many(args[0], args[1], args[2], args[3], args[4] as u64)
        }
        SYS_VM_MAP => handlers::sys_vm_map(args[0], args[1]),
        SYS_VM_UNMAP => handlers::sys_vm_unmap(args[0], args[1]),
        SYS_VM_PROTECT => handlers::sys_vm_protect(args[0]),
        SYS_VM_ADVISE => handlers::sys_vm_advise(args[0]),
        SYS_VM_QUERY => handlers::sys_vm_query(args[0], args[1]),
        SYS_TASK_WAIT => handlers::sys_task_wait(args[0]),

        SYS_PORT_CREATE => handlers::sys_port_create(args[0]),
        SYS_PORT_SEND => handlers::sys_port_send(args[0], args[1], args[2]),
        SYS_PORT_SEND_ALL => handlers::sys_port_send_all(args[0], args[1], args[2]),
        SYS_PORT_RECV => handlers::sys_port_recv(args[0], args[1], args[2]),
        SYS_PORT_CLOSE => handlers::sys_port_close(args[0]),
        SYS_PORT_WAIT => handlers::sys_port_wait(args[0], args[1], args[2]),
        SYS_PORT_INFO => handlers::sys_port_info(args[0]),
        SYS_TOPIC_CREATE => handlers::sys_topic_create(),
        SYS_TOPIC_SUBSCRIBE => handlers::sys_topic_subscribe(args[0], args[1]),
        SYS_TOPIC_PUBLISH => handlers::sys_topic_publish(args[0], args[1], args[2]),
        SYS_PORT_TRY_RECV => handlers::sys_port_try_recv(args[0], args[1], args[2]),

        SYS_TRACE_READ => handlers::sys_trace_read(args[0], args[1]),
        SYS_CONSOLE_DISABLE => handlers::sys_console_disable(),

        SYS_STREAM_LISTEN => handlers::stream::sys_stream_listen(args[0], args[1]),
        SYS_STREAM_OPEN => handlers::stream::sys_stream_open(args[0]),
        SYS_STREAM_READ => handlers::stream::sys_stream_read(args[0], args[1], args[2]),
        SYS_STREAM_POLL => handlers::stream::sys_stream_poll(args[0], args[1], args[2]),

        SYS_DEVICE_CLAIM => handlers::sys_device_claim(args[0]),
        SYS_DEVICE_MAP_MMIO => handlers::sys_device_map_mmio(args[0], args[1]),
        SYS_DEVICE_IRQ_SUBSCRIBE => handlers::sys_device_irq_subscribe(args[0], args[1], args[2]),
        SYS_DEVICE_IOPORT_READ => handlers::sys_device_ioport(args[0], 0, false, args[1]),
        SYS_DEVICE_IOPORT_WRITE => handlers::sys_device_ioport(args[0], args[1], true, args[2]),
        SYS_DEVICE_ALLOC_DMA => handlers::sys_device_alloc_dma(args[0], args[1]),
        SYS_DEVICE_DMA_PHYS => handlers::sys_device_dma_phys(args[0]),
        SYS_DEVICE_IRQ_WAIT => handlers::sys_device_irq_wait(args[0], args[1], args[2]),

        SYS_ROOT_GET_KIND => handlers::sys_root_get_kind(args[0]),
        SYS_ROOT_BYTESPACE_CREATE => handlers::sys_root_bytespace_create(args[0], args[1], args[2]),
        SYS_ROOT_BYTESPACE_READ => {
            handlers::sys_root_bytespace_read(args[0], args[1], args[2], args[3])
        }
        SYS_ROOT_BYTESPACE_WRITE => {
            handlers::sys_root_bytespace_write(args[0], args[1], args[2], args[3])
        }
        SYS_ROOT_BYTESPACE_INFO => handlers::sys_root_bytespace_info(args[0]),
        SYS_ROOT_BYTESPACE_MAP => handlers::sys_root_bytespace_map(args[0]),
        SYS_ROOT_BYTESPACE_UNMAP => handlers::sys_root_bytespace_unmap(args[0], args[1]),
        SYS_ROOT_BYTESPACE_PHYS => handlers::sys_root_bytespace_phys(args[0]),
        SYS_ROOT_WATCH_SUBSCRIBE => handlers::sys_root_watch_subscribe(args[0], args[1]),
        SYS_ROOT_STREAM_POLL => handlers::sys_root_stream_poll(args[0], args[1], args[2]),
        SYS_ROOT_PROP_SET => handlers::sys_root_prop_set(args[0], args[1], args[2]),
        SYS_ROOT_DESCRIBE_THING => handlers::sys_root_describe_thing(args[0], args[1], args[2]),
        SYS_ROOT_DESCRIBE_SYMBOL => handlers::sys_root_describe_symbol(args[0], args[1], args[2]),
        SYS_ROOT_DESCRIBE_EDGE => {
            handlers::sys_root_describe_edge(args[0], args[1], args[2], args[3], args[4])
        }
        SYS_ROOT_LINK => handlers::sys_root_link(args[0], args[1], args[2]),
        SYS_ROOT_DUMP_EDGES => handlers::sys_root_dump_edges(args[0], args[1], args[2]),
        SYS_ROOT_GET_EDGES => handlers::sys_root_get_edges(args[0], args[1], args[2]),
        SYS_ROOT_GET_PROPS => handlers::sys_root_get_props(args[0], args[1], args[2]),
        SYS_ROOT_INTERN => handlers::sys_root_intern(args[0], args[1]),
        SYS_ROOT_PROP_GET => handlers::sys_root_prop_get(args[0], args[1], args[2]),
        SYS_ROOT_FIND => handlers::sys_root_find(args[0], args[1], args[2]),
        SYS_ROOT_CREATE_NODE => handlers::sys_root_create_node(args[0]),
        SYS_ROOT_QUERY => handlers::sys_root_query(args[0], args[1], args[2], args[3]),
        SYS_ROOT_DUMP_GRAPH => handlers::sys_root_dump_graph(args[0]),

        SYS_ROOT_WATCH_OPEN => handlers::sys_root_watch_open(args[0]),
        SYS_ROOT_WATCH_NEXT => handlers::sys_root_watch_next(args[0], args[1], args[2], args[3]),
        SYS_ROOT_WATCH_TRY_NEXT => {
            handlers::sys_root_watch_try_next(args[0], args[1], args[2], args[3])
        }
        SYS_ROOT_WATCH_CLOSE => handlers::sys_root_watch_close(args[0]),
        SYS_ROOT_APPLY_BATCH => handlers::sys_root_apply_batch(args[0], args[1]),
        SYS_ROOT_PROPS_GET_MANY => {
            handlers::sys_root_props_get_many(args[0], args[1], args[2], args[3])
        }

        SYS_ROOT_BYTESPACE_TRUNCATE => handlers::sys_root_bytespace_truncate(args[0], args[1]),
        SYS_ROOT_RESOLVE_PATH => handlers::sys_root_resolve_path(args[0], args[1]),
        SYS_ROOT_UNLINK => handlers::sys_root_unlink(args[0], args[1], args[2]),
        SYS_ROOT_DIR_LIST => handlers::sys_root_dir_list(args[0], args[1], args[2]),
        SYS_ROOT_ORPHAN_THING => handlers::sys_root_orphan_thing(args[0]),

        SYS_ROOT_ASYNC_PROP_SET => handlers::sys_root_async_prop_set(args[0], args[1], args[2]),
        SYS_ROOT_ASYNC_LINK => handlers::sys_root_async_link(args[0], args[1], args[2]),
        SYS_ROOT_ASYNC_CREATE_NODE => handlers::sys_root_async_create_node(args[0]),
        SYS_ROOT_ASYNC_WAIT => handlers::sys_root_async_wait(args[0]),
        SYS_ROOT_ASYNC_DROP => handlers::sys_root_async_drop(args[0]),
        SYS_ROOT_ASYNC_STATUS => handlers::sys_root_async_status(args[0]),

        SYS_NIC_MAC => handlers::sys_nic_mac(args[0]),
        SYS_NIC_LINK_UP => handlers::sys_nic_link_up(),
        SYS_NIC_POLL_RX => handlers::sys_nic_poll_rx(args[0], args[1]),
        SYS_NIC_TX => handlers::sys_nic_tx(args[0], args[1]),

        SYS_PIPE_CREATE => handlers::sys_pipe_create(args[0], args[1]),
        SYS_PIPE_READ => handlers::sys_pipe_read(args[0], args[1], args[2]),
        SYS_PIPE_WRITE => handlers::sys_pipe_write(args[0], args[1], args[2]),
        SYS_PIPE_CLOSE => handlers::sys_pipe_close(args[0], args[1]),

        SYS_GETRANDOM => handlers::sys_getrandom(args[0], args[1]),

        // ── VFS (janix) ───────────────────────────────────────────────────
        SYS_VFS_OPEN => handlers::vfs::sys_vfs_open(args[0], args[1], args[2]),
        SYS_VFS_CLOSE => handlers::vfs::sys_vfs_close(args[0]),
        SYS_VFS_READ => handlers::vfs::sys_vfs_read(args[0], args[1], args[2]),
        SYS_VFS_WRITE => handlers::vfs::sys_vfs_write(args[0], args[1], args[2]),
        SYS_DUP => handlers::vfs::sys_dup(args[0]),
        SYS_DUP2 => handlers::vfs::sys_dup2(args[0], args[1]),
        SYS_PIPE => handlers::vfs::sys_pipe(args[0]),
        SYS_VFS_UNLINK => handlers::vfs::sys_vfs_unlink(args[0], args[1]),
        SYS_VFS_MKDIR => handlers::vfs::sys_vfs_mkdir(args[0], args[1]),

        _ => {
            crate::kprintln!("SYSCALL: Unknown syscall #{}", syscall_id);
            Err(abi::errors::Errno::ENOSYS)
        }
    };

    match result {
        Ok(val) => val as isize,
        Err(e) => -(e as isize),
    }
}
