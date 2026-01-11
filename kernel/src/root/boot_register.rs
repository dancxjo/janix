use super::{enqueue, RootOp};
use super::graph::ThingId;
use abi::kinds::*;

use crate::BootModuleDesc;
use crate::PhysRange;
use crate::FramebufferInfo;

pub struct BootInfo<'a> {
    pub cpu_count: usize,
    pub memory_map: &'a [PhysRange],
    pub modules: &'a [BootModuleDesc],
    pub framebuffer: Option<FramebufferInfo>,
    pub hhdm_offset: u64,
}

pub struct BootInventory {
    pub host: ThingId,
    pub kernel: ThingId,
    pub root: ThingId,
}

pub fn register_all(info: &BootInfo) -> BootInventory {
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
    set(host, PROP_HHDM_OFFSET, info.hhdm_offset);
    // set(host, PROP_ARCH, ...); 

    // 2. Kernel
    let kernel = create(KIND_KERNEL);
    set(kernel, PROP_VERSION, 1);
    link(kernel, REL_RUNS_ON, host);
    
    // 3. Root Service
    let root_svc = create(KIND_SERVICE);
    link(kernel, REL_PROVIDES, root_svc);
    
    // 4. CPUs
    for i in 0..info.cpu_count {
        let cpu = create(KIND_CPU);
        set(cpu, PROP_ID, i as u64);
        link(host, REL_HAS_CPU, cpu);
    }
    
    // 5. Memory Ranges
    for range in info.memory_map {
        let mem = create(KIND_MEMORY_RANGE);
        set(mem, PROP_START, range.start);
        set(mem, PROP_END, range.end);
        set(mem, PROP_KIND, range.kind as u64); // Kind enum maps to u64
        link(host, REL_HAS_MEMORY_RANGE, mem);
    }
    
    // 6. Modules
    for (i, m) in info.modules.iter().enumerate() {
        let mod_node = create(KIND_BOOT_MODULE);
        set(mod_node, PROP_PHYS_BASE, m.phys_start);
        set(mod_node, PROP_SIZE_BYTES, m.phys_end - m.phys_start);
        set(mod_node, PROP_INDEX, i as u64);
        // Path is string, skipping for v0.1 props
        link(host, REL_HAS_MODULE, mod_node);
    }
    
    // 7. Framebuffer
    if let Some(fb) = info.framebuffer.as_ref() {
        let fb_node = create(KIND_FRAMEBUFFER);
        set(fb_node, PROP_PHYS_BASE, fb.addr);
        set(fb_node, PROP_WIDTH, fb.width as u64);
        set(fb_node, PROP_HEIGHT, fb.height as u64);
        set(fb_node, PROP_STRIDE, fb.pitch as u64);
        link(host, REL_HAS_DEVICE, fb_node);
    }

    // 8. Tasking (Scheduler & Boot Task)
    let scheduler = create(KIND_SERVICE); // Reuse service kind? Or generic
    // We don't have KIND_SCHEDULER in kinds.rs, use KIND_SERVICE
    link(kernel, REL_PROVIDES, scheduler);
    
    // Boot task? Maybe simplistic representation
    // let boot_task = create(KIND_TASK); // No KIND_TASK
    
    crate::kinfo!("ROOT: registered items. host={:x} kernel={:x}", host, kernel);
    
    BootInventory { host, kernel, root: root_svc }
}
