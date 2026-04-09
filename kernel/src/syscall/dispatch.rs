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

        SYS_CHANNEL_CREATE => handlers::sys_channel_create(args[0]),
        SYS_CHANNEL_SEND => handlers::sys_channel_send(args[0], args[1], args[2]),
        SYS_CHANNEL_SEND_ALL => handlers::sys_channel_send_all(args[0], args[1], args[2]),
        SYS_CHANNEL_RECV => handlers::sys_channel_recv(args[0], args[1], args[2]),
        SYS_CHANNEL_CLOSE => handlers::sys_channel_close(args[0]),
        SYS_CHANNEL_WAIT => handlers::sys_channel_wait(args[0], args[1], args[2]),
        SYS_CHANNEL_INFO => handlers::sys_channel_info(args[0]),
        SYS_CHANNEL_TRY_RECV => handlers::sys_channel_try_recv(args[0], args[1], args[2]),
        SYS_CHANNEL_SEND_HANDLE => handlers::sys_channel_send_handle(args[0], args[1]),
        SYS_CHANNEL_RECV_HANDLE => handlers::sys_channel_recv_handle(args[0], args[1]),

        SYS_TRACE_READ => handlers::sys_trace_read(args[0], args[1]),
        SYS_CONSOLE_DISABLE => handlers::sys_console_disable(),

        SYS_DEVICE_CLAIM => handlers::sys_device_claim(args[0]),
        SYS_DEVICE_MAP_MMIO => handlers::sys_device_map_mmio(args[0], args[1]),
        SYS_DEVICE_IRQ_SUBSCRIBE => handlers::sys_device_irq_subscribe(args[0], args[1], args[2]),
        SYS_DEVICE_IOPORT_READ => handlers::sys_device_ioport(args[0], 0, false, args[1]),
        SYS_DEVICE_IOPORT_WRITE => handlers::sys_device_ioport(args[0], args[1], true, args[2]),
        SYS_DEVICE_ALLOC_DMA => handlers::sys_device_alloc_dma(args[0], args[1]),
        SYS_DEVICE_DMA_PHYS => handlers::sys_device_dma_phys(args[0]),
        SYS_DEVICE_IRQ_WAIT => handlers::sys_device_irq_wait(args[0], args[1], args[2]),

        SYS_MEMFD_CREATE => handlers::sys_memfd_create(args[0], args[1], args[2]),
        SYS_MEMFD_PHYS => handlers::sys_memfd_phys(args[0]),

        SYS_GETRANDOM => handlers::sys_getrandom(args[0], args[1]),

        // ── VFS (janix) ───────────────────────────────────────────────────
        SYS_FS_OPEN => handlers::vfs::sys_fs_open(args[0], args[1], args[2]),
        SYS_FS_CLOSE => handlers::vfs::sys_fs_close(args[0]),
        SYS_FS_READ => handlers::vfs::sys_fs_read(args[0], args[1], args[2]),
        SYS_FS_WRITE => handlers::vfs::sys_fs_write(args[0], args[1], args[2]),
        SYS_FS_DUP => handlers::vfs::SYS_FS_DUP(args[0]),
        SYS_FS_DUP2 => handlers::vfs::SYS_FS_DUP2(args[0], args[1]),
        SYS_FS_RENAME => {
            handlers::vfs::sys_fs_rename(args[0], args[1], args[2], args[3], args[4], args[5])
        }
        SYS_PIPE => handlers::vfs::sys_pipe(args[0]),
        SYS_FS_UNLINK => handlers::vfs::sys_fs_unlink(args[0], args[1]),
        SYS_FS_MKDIR => handlers::vfs::sys_fs_mkdir(args[0], args[1]),
        SYS_FS_MOUNT => handlers::vfs::sys_fs_mount(args[0], args[1], args[2]),
        SYS_FS_UMOUNT => handlers::vfs::sys_fs_umount(args[0], args[1]),
        SYS_FS_STAT => handlers::vfs::sys_fs_stat(args[0], args[1], args[2], args[3]),
        SYS_FS_READDIR => handlers::vfs::sys_fs_readdir(args[0], args[1], args[2]),
        SYS_FS_POLL => handlers::vfs::sys_fs_poll(args[0], args[1], args[2]),
        SYS_FS_SEEK => handlers::vfs::sys_fs_seek(args[0], args[1], args[2]),
        SYS_FS_WATCH_FD => handlers::vfs::sys_watch_fd(args[0], args[1], args[2]),
        SYS_FS_DEVICE_CALL => handlers::vfs::sys_fs_device_call(args[0], args[1]),
        SYS_FS_WATCH_PATH => handlers::vfs::sys_watch_path(args[0], args[1], args[2], args[3]),

        _ => Err(abi::errors::Errno::ENOSYS),
    };

    match result {
        Ok(val) => val as isize,
        Err(e) => -(e as isize),
    }
}
