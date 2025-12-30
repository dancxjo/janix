pub mod bytespace;
pub mod driver;
pub mod graph;
pub mod time;
pub mod typed;

use crate::bridge::HardwareBridge;
use crate::Kernel;
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
    // Debug log
    // kernel.bridge.log(alloc::format!("SYSCALL: {}\n", num).as_str()); // Need alloc
    // bridge log usually takes &str.
    // Let's use if/match to print only interesting ones.
    match num {
        SYSCALL_RTC_READ => time::sys_rtc_read(kernel, a1 as *mut u8),
        SYSCALL_YIELD => {
            // Just return for now, next tick will preempt or we can implement real yield later.
            0
        }
        SYSCALL_SLEEP => {
            let duration = a1 as u64;
            let now = kernel.bridge.monotonic_now();
            kernel.scheduler.sleep_current_until(now + duration);
            0
        }
        SYSCALL_TIME => {
            // a1: out_ptr (u64 monotonic, u64 system)
            let out_ptr = a1 as *mut u64;
            // Validate pointer?
            if out_ptr as u64 == 0 {
                return -1;
            }
            let mono = kernel.bridge.monotonic_now();
            let sys = kernel.bridge.system_now();
            unsafe {
                *out_ptr = mono;
                *out_ptr.add(1) = sys;
            }
            0
        }
        SYSCALL_LOG => {
            // a1: ptr, a2: len
            let ptr = a1 as *const u8;
            let len = a2;
            let bytes = unsafe { core::slice::from_raw_parts(ptr, len) };
            if let Ok(s) = core::str::from_utf8(bytes) {
                kernel.bridge.log(s);
                len as isize
            } else {
                -1
            }
        }

        SYSCALL_DRIVER_WAIT => driver::sys_driver_wait(kernel, a1 as *mut u8, a2) as isize,
        SYSCALL_DRIVER_PUBLISH => driver::sys_driver_publish(kernel, a1 as *const u8, a2) as isize,
        SYSCALL_MMIO_MAP => driver::sys_mmio_map(kernel, a1 as u64, a2 as u64) as isize,
        SYSCALL_IRQ_REGISTER => driver::sys_irq_register(kernel, a1) as isize,
        SYSCALL_PORT_IO => {
            driver::sys_port_io(kernel, a1 as u16, a2 as u32, a3 as u8, a4 != 0) as isize
        }
        SYSCALL_TYPEDEF_REGISTER => typed::sys_typedef_register(a1, a2) as isize,
        SYSCALL_TYPEDEF_GET => typed::sys_typedef_get(a1, a2, a3) as isize,
        1 => {
            // Debug: Catch bad pointers
            if a1 < 4096 {
                kernel.bridge.log("SYSCALL GRAPH: Bad Ptr\n");
                return -1;
            }

            // a1: query_ptr, a2: query_len
            // a3: params_ptr, a4: params_len
            // a5: out_ptr, a6: out_len

            let query_ptr = a1 as *const u8;
            let query_len = a2;
            let params_ptr = a3 as *const u8;
            let params_len = a4;
            let out_ptr = a5 as *mut u8;
            let out_len = a6;

            // VALIDATION: Ensure all pointers are in User Range (Lower Half).
            // Canonical split is at 0x0000_7FFF_FFFF_FFFF.
            // Kernel starts at 0xFFFF_8000_0000_0000.
            // Be conservative: < 0x8000_0000_0000.
            const USER_MAX: usize = 0x8000_0000_0000;

            if (a1 + a2) >= USER_MAX || (a3 + a4) >= USER_MAX || (a5 + a6) >= USER_MAX {
                kernel
                    .bridge
                    .log("SYSCALL GRAPH: Security Violation (Ptr > User)\n");
                return -1;
            }

            // Safety: We validated range.
            let query_bytes = unsafe { core::slice::from_raw_parts(query_ptr, query_len) };
            let query_str = match core::str::from_utf8(query_bytes) {
                Ok(s) => s,
                Err(_) => return -1,
            };
            let params = unsafe { core::slice::from_raw_parts(params_ptr, params_len) };
            let out = unsafe { core::slice::from_raw_parts_mut(out_ptr, out_len) };

            // Get PID
            let pid = if let Some(tid) = kernel.scheduler.current {
                if let Some(Some(thread)) = kernel.scheduler.threads.get(tid.0 as usize - 1) {
                    thread.process_id
                } else {
                    return -1;
                }
            } else {
                // No current thread context? Syscall usually happens in thread context.
                // Maybe idle/boot?
                return -1;
            };

            match graph::handle_graph_query(kernel, pid, query_str, params, out) {
                Ok(len) => len as isize,
                Err(e) => e,
            }
        }
        SYSCALL_SPAWN => {
            // Handled by Arch Hook
            -1
        }
        SYSCALL_BYTESPACE_CREATE => bytespace::sys_bytespace_create(kernel, a1, a2, a3, a4),
        SYSCALL_BYTESPACE_MAP => bytespace::sys_bytespace_map(kernel, a1, a2, a3, a4),
        SYSCALL_BYTESPACE_READ => bytespace::sys_bytespace_read(kernel, a1, a2, a3, a4),
        SYSCALL_BYTESPACE_WRITE => bytespace::sys_bytespace_write(kernel, a1, a2, a3, a4),
        _ => -1,
    }
}
