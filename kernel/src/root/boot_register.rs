use super::{enqueue, RootOp};
use abi::kinds::*;
use crate::BootRuntime;

pub fn register_all<R: BootRuntime>(runtime: &R) {
    crate::kinfo!("ROOT: boot registration begin");

    let create = |kind: ThingKind| -> u64 {
        let reply = enqueue(RootOp::CreateNode { kind });
        loop {
             let done = reply.done.load(core::sync::atomic::Ordering::Acquire);
             if done != 0 {
                  return reply.value.load(core::sync::atomic::Ordering::Relaxed);
             }
             unsafe { crate::task::scheduler::yield_now_current(); }
        }
    };
    
    let set = |id: u64, key: u64, val: u64| {
        let reply = enqueue(RootOp::PropSet { id, key, value: val });
         loop {
             let done = reply.done.load(core::sync::atomic::Ordering::Acquire);
             if done != 0 { break; }
             unsafe { crate::task::scheduler::yield_now_current(); }
        }
    };

    let link = |src: u64, rel: u64, dst: u64| {
        let reply = enqueue(RootOp::Link { src, rel, dst });
         loop {
             let done = reply.done.load(core::sync::atomic::Ordering::Acquire);
             if done != 0 { break; }
             unsafe { crate::task::scheduler::yield_now_current(); }
        }
    };

    // 1. Host
    let host = create(KIND_HOST);
    // set(host, PROP_ARCH, ...); // No prop symbols yet, skip props for v0.1

    // 2. Kernel
    let kernel = create(KIND_KERNEL);
    link(kernel, REL_RUNS_ON, host);
    
    // 3. Root Service
    let root_svc = create(KIND_SERVICE);
    link(kernel, REL_PROVIDES, root_svc);
    
    // 4. Console
    let console = create(KIND_CONSOLE);
    link(kernel, REL_LOGS_TO, console);
    // Link host?
    link(host, REL_HAS_DEVICE, console);

    // 5. CPUs
    for i in 0..runtime.cpu_count() {
        let cpu = create(KIND_CPU);
        // set(cpu, PROP_ID, i as u64);
        link(host, REL_HAS_CPU, cpu);
    }
    
    // 6. Memory Ranges
    for range in runtime.phys_memory_map() {
        let mem = create(KIND_MEMORY_RANGE);
        // set(mem, PROP_START, range.start);
        // set(mem, PROP_END, range.end);
        set(mem, 0, range.start); // Using 0/1 for start/len as hack? Or just link
        link(host, REL_HAS_MEMORY_RANGE, mem);
    }
    
    // 7. Modules
    for mod_desc in runtime.modules() {
        let m = create(KIND_BOOT_MODULE);
        link(host, REL_HAS_MODULE, m);
    }
    
    // 8. Framebuffer
    if let Some(_fb) = runtime.framebuffer() {
        let fb = create(KIND_FRAMEBUFFER);
        link(host, REL_HAS_DEVICE, fb);
    }
    
    crate::kinfo!("ROOT: registered items. host={:x} kernel={:x}", host, kernel);
}
