pub mod graph;
pub mod driver;
pub mod typed;

use crate::Kernel;
use hw::HardwareBridge;
use abi::syscall_defs::*; // e.g. SYSCALL_DRIVER_WAIT, etc.

pub fn syscall_dispatch<B: HardwareBridge>(
    kernel: &mut Kernel<B>,
    num: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> isize {
    match num {
        SYSCALL_DRIVER_WAIT => {
            driver::sys_driver_wait(kernel, a1 as *mut u8, a2) as isize
        },
        SYSCALL_DRIVER_PUBLISH => {
            driver::sys_driver_publish(kernel, a1 as *const u8, a2) as isize
        },
        SYSCALL_TYPEDEF_REGISTER => {
            typed::sys_typedef_register(a1, a2) as isize
        },
        SYSCALL_TYPEDEF_GET => {
            typed::sys_typedef_get(a1, a2, a3) as isize
        },
        // SYSCALL_GRAPH (1)
        1 => {
             // a1: query_ptr, a2: query_len
             // a3: params_ptr, a4: params_len
             // a5: out_ptr, a6: out_len
             
             let query_ptr = a1 as *const u8;
             let query_len = a2;
             let params_ptr = a3 as *const u8;
             let params_len = a4;
             let out_ptr = a5 as *mut u8;
             let out_len = a6;
             
             // Safety: user pointers must be validated. v0: assume valid.
             let query_bytes = unsafe { core::slice::from_raw_parts(query_ptr, query_len) };
             let query_str = match core::str::from_utf8(query_bytes) {
                 Ok(s) => s,
                 Err(_) => return -1,
             };
             let params = unsafe { core::slice::from_raw_parts(params_ptr, params_len) };
             let out = unsafe { core::slice::from_raw_parts_mut(out_ptr, out_len) };
             
             match graph::handle_graph_query(kernel, query_str, params, out) {
                 Ok(len) => len as isize,
                 Err(_) => -1,
             }
        },
        _ => -1,
    }
}




