use crate::syscall::handlers;

use abi::syscall::*;

pub fn dispatch(n: usize, args: [usize; 6]) -> isize {
    let syscall_id = n as u32;

    let result = match syscall_id {
        SYS_EXIT => handlers::sys_exit(args[0] as i32),
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
        SYS_ALLOC_STACK => handlers::sys_alloc_stack(args[0]),
        SYS_TASK_WAIT => handlers::sys_task_wait(args[0]),
        
        SYS_PORT_CREATE => handlers::sys_port_create(args[0]),
        SYS_PORT_SEND => handlers::sys_port_send(args[0], args[1], args[2]),
        SYS_PORT_RECV => handlers::sys_port_recv(args[0], args[1], args[2]),
        SYS_PORT_CLOSE => handlers::sys_port_close(args[0]),

        SYS_DEVICE_CLAIM => handlers::sys_device_claim(args[0]),
        SYS_DEVICE_MAP_MMIO => handlers::sys_device_map_mmio(args[0], args[1]),
        SYS_DEVICE_IRQ_SUBSCRIBE => handlers::sys_device_irq_subscribe(args[0]),
        SYS_DEVICE_IOPORT_READ => handlers::sys_device_ioport(args[0], 0, false, args[1]),
        SYS_DEVICE_IOPORT_WRITE => handlers::sys_device_ioport(args[0], args[1], true, args[2]),

        SYS_ROOT_GET_KIND => handlers::sys_root_get_kind(args[0]),
        SYS_ROOT_BYTESPACE_CREATE => handlers::sys_root_bytespace_create(args[0], args[1], args[2]),
        SYS_ROOT_BYTESPACE_READ => {
            handlers::sys_root_bytespace_read(args[0], args[1], args[2], args[3])
        }
        SYS_ROOT_WATCH_SUBSCRIBE => handlers::sys_root_watch_subscribe(args[0], args[1]),
        SYS_ROOT_STREAM_POLL => handlers::sys_root_stream_poll(args[0], args[1], args[2]),
        SYS_ROOT_PROP_SET => handlers::sys_root_prop_set(args[0], args[1], args[2]),
        SYS_ROOT_DESCRIBE_THING => handlers::sys_root_describe_thing(args[0], args[1], args[2]),
        SYS_ROOT_DESCRIBE_EDGE => {
            handlers::sys_root_describe_edge(args[0], args[1], args[2], args[3], args[4])
        }
        SYS_ROOT_LINK => handlers::sys_root_link(args[0], args[1], args[2]),
        SYS_ROOT_DUMP_EDGES => handlers::sys_root_dump_edges(args[0], args[1], args[2]),
        SYS_ROOT_INTERN => handlers::sys_root_intern(args[0], args[1]),
        SYS_ROOT_PROP_GET => handlers::sys_root_prop_get(args[0], args[1], args[2]),
        SYS_ROOT_FIND => handlers::sys_root_find(args[0], args[1], args[2]),
        SYS_ROOT_CREATE_NODE => handlers::sys_root_create_node(args[0]),
        SYS_ROOT_QUERY => handlers::sys_root_query(args[0], args[1], args[2], args[3]),
        SYS_ROOT_DUMP_GRAPH => handlers::sys_root_dump_graph(args[0]),

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
