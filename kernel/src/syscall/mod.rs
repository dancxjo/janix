// use crate::trap::x86_64::TrapFrame; // Removed specific import
use crate::boot::ArchTrapFrame;
use crate::user::abi::syscall::{
    SYSCALL_PUTCHAR, SYSCALL_TICKS, SYSCALL_YIELD, SYSCALL_EXIT, SYSCALL_SPAWN_MODULE, 
    SYSCALL_RTC_CMOS_READ, SYSCALL_GRAPH_APPEND, SYSCALL_WATCH_CREATE, SYSCALL_WATCH_NEXT
};


#[unsafe(no_mangle)]
pub fn syscall_dispatch(tf: &mut dyn ArchTrapFrame) {
    let nr = tf.syscall_num();
    // crate::kinfo!("Syscall dispatch: nr={}", nr);
    // Only log potentially problematic ones or all?
    if nr >= 6 {
         crate::kinfo!("Syscall dispatch: nr={} (Root/Complex)", nr);
    }
    let ret = match nr {
        SYSCALL_PUTCHAR => {
            let c = tf.syscall_arg(0) as u8;
            crate::runtime().putchar(c);
            0
        },
        SYSCALL_TICKS => {
            crate::runtime().mono_ticks()
        },
        SYSCALL_YIELD => {
            crate::task::yield_now();
            0
        },
        SYSCALL_EXIT => {
            let _code = tf.syscall_arg(0) as i32;
            crate::kinfo!("User task exited with code {}", _code);
            // Mark task dead or just halt for now since we don't have task destruction
            loop {
                crate::task::yield_now();
            }
        },
        SYSCALL_SPAWN_MODULE => {
            let path_ptr = tf.syscall_arg(0) as *const u8;
            let path_len = tf.syscall_arg(1) as usize;
            
            // Validate user pointer (rudimentary)
            if path_ptr as u64 >= 0x8000_0000_0000_0000 {
                 crate::kwarn!("SpawnModule: invalid pointer {:p}", path_ptr);
                 u64::MAX // error
            } else {
                 match crate::user::sys_spawn_module(path_ptr, path_len) {
                     Ok(tid) => tid as u64,
                     Err(e) => {
                         crate::kwarn!("SpawnModule failed: {}", e);
                         match e {
                             // Map some errors to negative numbers if needed
                             _ => u64::MAX // -1 mostly
                         }
                     }
                 }
            }
        },
        SYSCALL_RTC_CMOS_READ => {
            let reg = tf.syscall_arg(0) as u8;
            match crate::user::sys_rtc_cmos_read(reg) {
                Ok(val) => val as u64,
                Err(_) => u64::MAX,
            }
        },
        SYSCALL_GRAPH_APPEND => {
            // crate::kinfo!("Dispatch v4: MATCH GRAPH_APPEND (const={})", SYSCALL_GRAPH_APPEND);
            let op_ptr = tf.syscall_arg(0) as *const crate::user::abi::root::JournalOp;
            // Validate pointer
            if op_ptr as u64 >= 0x8000_0000_0000_0000 {
                u64::MAX
            } else {
                let op = unsafe { *op_ptr };
                crate::global::root().append(op)
            }
        },
        SYSCALL_WATCH_CREATE => {
            // crate::kinfo!("Dispatch: MATCH WATCH_CREATE (const={})", SYSCALL_WATCH_CREATE);
            crate::global::root().watch_create()
        },
        SYSCALL_WATCH_NEXT => {
            let watch_id = tf.syscall_arg(0);
            let out_ptr = tf.syscall_arg(1) as *mut crate::user::abi::root::WatchEvent;
            
             // Validate pointer
            if out_ptr as u64 >= 0x8000_0000_0000_0000 {
                // -EFAULT ideally, but using i64::MAX or similar error convention
                 u64::MAX 
            } else {
                match crate::global::root().watch_next(watch_id) {
                    Ok(event) => {
                        unsafe { *out_ptr = event };
                        0 // Success
                    }
                    Err(e) => e as u64, // Pass through error code (e.g. -EAGAIN)
                }
            }
        },
        _ => {
            crate::kinfo!("Unknown syscall: {}", nr);
            u64::MAX
        }
    };
    tf.syscall_ret(ret);
}
