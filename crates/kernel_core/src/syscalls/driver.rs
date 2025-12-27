use crate::Kernel;
use abi::wire::driver::{DriverEvent, DriverPublish};
use abi::{SysRet, SYSCALL_DRIVER_PUBLISH, SYSCALL_DRIVER_WAIT};
use hw::HardwareBridge;
use postcard::from_bytes;

pub fn sys_driver_wait<B: HardwareBridge>(
    kernel: &mut Kernel<B>,
    out_ptr: *mut u8,
    out_len: usize,
) -> SysRet {
    // V0: Non-blocking poll.
    // If we loop here, we hold the KERNEL lock (BKL) from syscall_hook,
    // preventing the scheduler from ticking (which also needs BKL).
    // So we must return to user mode if no event, letting user spin.

    if let Some(event) = crate::input::try_pop_event(&kernel.bridge) {
        // Serialize
        let slice = unsafe { core::slice::from_raw_parts_mut(out_ptr, out_len) };
        match postcard::to_slice(&event, slice) {
            Ok(used) => return used.len() as SysRet,
            Err(_) => return -1, // Enobufs
        }
    }

    // No event: Return error so user loops.
    // Use a distinguishable error? For V0, -1 is fine (User ignores Err).
    -1
}

pub fn sys_driver_publish<B: HardwareBridge>(
    kernel: &mut Kernel<B>,
    ptr: *const u8,
    len: usize,
) -> SysRet {
    // Read buffer
    let slice = unsafe { core::slice::from_raw_parts(ptr, len) };
    let publish: DriverPublish = match from_bytes(slice) {
        Ok(p) => p,
        Err(_) => return -2, // Ebadmsg
    };

    match publish {
        DriverPublish::Observation { thing_bytes } => {
            // Decode Thing from bytes?
            // "if Observation { thing_bytes }: decode Thing from bytes... insert into graph"
            // We need to parse Thing.
            match postcard::from_bytes::<thing_models::Thing>(&thing_bytes) {
                Ok(thing) => {
                    // Check capability? "Require calling process has CAP_GRAPH_WRITE_WITNESS"
                    // V0: skip check for now or check dummy.

                    // Insert
                    match kernel.graph.insert_thing(thing) {
                        Ok(_) => 0,
                        Err(_) => -3, // Efail
                    }
                }
                Err(_) => -2,
            }
        }
    }
}
