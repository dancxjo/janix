use crate::Kernel;
use hw::HardwareBridge;
use abi::{SysRet, SYSCALL_DRIVER_WAIT, SYSCALL_DRIVER_PUBLISH};
use abi::wire::driver::{DriverEvent, DriverPublish};
use postcard::from_bytes;

pub fn sys_driver_wait<B: HardwareBridge>(kernel: &mut Kernel<B>, out_ptr: *mut u8, out_len: usize) -> SysRet {
    // V0: Busy-wait / yield loop
    // In a real OS, we would put thread to sleep and register a waker.
    loop {
        if let Some(event) = crate::input::try_pop_event() {
            // Serialize
            let slice = unsafe { core::slice::from_raw_parts_mut(out_ptr, out_len) };
            match postcard::to_slice(&event, slice) {
                Ok(used) => return used.len() as SysRet,
                Err(_) => return -1, // Enobufs
            }
        }
        // Yield
        // kernel.scheduler.yield_thread(); // Not easily available via generic B? 
        // We can just hint spin loop.
        core::hint::spin_loop(); 
        // Or better: kernel.bridge.idle(); if appropriate, but inside syscall we are in a thread context.
        // For now, minimal spin.
    }
}

pub fn sys_driver_publish<B: HardwareBridge>(kernel: &mut Kernel<B>, ptr: *const u8, len: usize) -> SysRet {
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
