use super::{enqueue, RootOp};
use super::graph::ThingId;
use super::SymbolShell;
use crate::{BootModuleDesc, PhysRange, FramebufferInfo};

#[derive(Debug)]
pub struct BootInfo<'a> {
    pub cpu_count: usize,
    pub memory_map: &'a [PhysRange],
    pub modules: &'a [BootModuleDesc],
    pub framebuffer: Option<FramebufferInfo>,
    pub hhdm_offset: u64,
    pub acpi_rsdp: Option<u64>,
    pub dtb_ptr: Option<u64>,
    pub arch: &'static str,
    pub platform_profile: &'static str,
}

pub struct BootInventory {
    pub host: ThingId,
    pub kernel: ThingId,
    pub root: ThingId,
}

pub fn register_all(info: &BootInfo) -> BootInventory {
    crate::kinfo!("ROOT: boot registration begin (Census Phase 1)");

    let create = |kind: &str| -> u64 {
        let reply = enqueue(RootOp::CreateNode { kind: SymbolShell::Str(alloc::string::String::from(kind)) });
        loop {
             let done = reply.done.load(core::sync::atomic::Ordering::Acquire);
             if done != 0 {
                  return reply.value.load(core::sync::atomic::Ordering::Relaxed);
             }
             unsafe { crate::task::scheduler::yield_now_current(); }
        }
    };
    
    let set = |id: u64, key: &str, val: u64| {
        let reply = enqueue(RootOp::PropSet { id, key: SymbolShell::Str(alloc::string::String::from(key)), value: val });
         loop {
             let done = reply.done.load(core::sync::atomic::Ordering::Acquire);
             if done != 0 { break; }
             unsafe { crate::task::scheduler::yield_now_current(); }
        }
    };

    let link = |src: u64, rel: &str, dst: u64| {
        let reply = enqueue(RootOp::Link { src, rel: SymbolShell::Str(alloc::string::String::from(rel)), dst });
         loop {
             let done = reply.done.load(core::sync::atomic::Ordering::Acquire);
             if done != 0 { break; }
             unsafe { crate::task::scheduler::yield_now_current(); }
        }
    };

    let intern = |s: &str| -> u64 {
        let reply = enqueue(RootOp::Intern { name: alloc::string::String::from(s) });
        loop {
             let done = reply.done.load(core::sync::atomic::Ordering::Acquire);
             if done != 0 { return reply.value.load(core::sync::atomic::Ordering::Relaxed); }
             unsafe { crate::task::scheduler::yield_now_current(); }
        }
    };

    let bytespace_create = |len: u64| -> u64 {
        let reply = enqueue(RootOp::BytespaceCreate { len, flags: 0, format: 0 });
        loop {
             let done = reply.done.load(core::sync::atomic::Ordering::Acquire);
             if done != 0 { return reply.value.load(core::sync::atomic::Ordering::Relaxed); }
             unsafe { crate::task::scheduler::yield_now_current(); }
        }
    };

    let bytespace_write = |id: u64, offset: u64, ptr: u64, len: u64| {
        let reply = enqueue(RootOp::BytespaceWrite { id, offset, ptr, len });
        loop {
             let done = reply.done.load(core::sync::atomic::Ordering::Acquire);
             if done != 0 { break; }
             unsafe { crate::task::scheduler::yield_now_current(); }
        }
    };

    // 1. Host
    let host = create("dev.host");
    set(host, "hhdm_offset", info.hhdm_offset);
    let arch_id = intern(info.arch);
    set(host, "arch", arch_id);
    let platform_id = intern(info.platform_profile);
    set(host, "platform_profile", platform_id);

    // 2. Platform Bus
    let platform_bus = create("dev.bus.platform");
    // intern name="platform0"
    let pbus_name = intern("platform0");
    set(platform_bus, "name", pbus_name);
    link(host, "HAS_BUS", platform_bus);

    // 3. Kernel
    let kernel = create("proc.kernel");
    set(kernel, "version", 1);
    link(kernel, "RUNS_ON", host);
    
    // 4. Root Service
    let root_svc = create("svc.root");
    link(kernel, "PROVIDES", root_svc);
    
    // 5. CPUs
    for i in 0..info.cpu_count {
        let cpu = create("dev.cpu");
        set(cpu, "id", i as u64);
        link(host, "HAS_CPU", cpu);
    }
    
    // 6. Memory Ranges
    for range in info.memory_map {
        let mem = create("mem.range");
        set(mem, "start", range.start);
        set(mem, "end", range.end);
        set(mem, "kind", range.kind as u64);
        link(host, "HAS_MEMORY_RANGE", mem);
    }
    
    // 7. Modules
    for (i, m) in info.modules.iter().enumerate() {
        let mod_node = create("boot.module");
        set(mod_node, "phys_base", m.phys_start);
        set(mod_node, "size_bytes", m.phys_end - m.phys_start);
        set(mod_node, "index", i as u64);
        let name_id = intern(m.name);
        set(mod_node, "name", name_id);
        
        link(host, "HAS_MODULE", mod_node);
    }
    
    // 8. Framebuffer
    if let Some(fb) = info.framebuffer.as_ref() {
        let fb_node = create("dev.display.framebuffer");
        set(fb_node, "phys_base", fb.addr);
        set(fb_node, "width", fb.width as u64);
        set(fb_node, "height", fb.height as u64);
        set(fb_node, "stride", fb.pitch as u64);
        set(fb_node, "bpp", fb.bpp as u64);
        set(fb_node, "size_bytes", fb.byte_len as u64);
        
        let fmt = match fb.format {
             crate::PixelFormat::Xrgb8888 => 1,
             crate::PixelFormat::Argb8888 => 2,
             crate::PixelFormat::Rgb565 => 3,
             _ => 0,
        };
        set(fb_node, "format", fmt);
        
        link(host, "HAS_DEVICE", fb_node);
    }

    // 9. Firmware Tables
    if info.acpi_rsdp.is_some() || info.dtb_ptr.is_some() {
        let fw_boot = create("fw.boot");
        link(host, "HAS_FIRMWARE", fw_boot);

        if let Some(rsdp) = info.acpi_rsdp {
            let acpi = create("fw.table.acpi");
            set(acpi, "phys_base", rsdp);
            link(fw_boot, "PROVIDES_TABLE", acpi);
        }

        if let Some(dtb_ptr) = info.dtb_ptr {
            let dtb_node = create("fw.table.dtb");
            // Parse FDT header
            let header = unsafe { core::slice::from_raw_parts(dtb_ptr as *const u8, 8) };
            let size = u32::from_be_bytes([header[4], header[5], header[6], header[7]]) as u64;
            
            crate::kinfo!("ROOT: Absorbing DTB (ptr={:x}, size={})", dtb_ptr, size);
            
            let bs = bytespace_create(size);
            bytespace_write(bs, 0, dtb_ptr, size);
            set(dtb_node, "bytespace", bs);
            
            link(fw_boot, "PROVIDES_TABLE", dtb_node);
        }
    }

    // 10. Tasking
    let scheduler = create("svc.scheduler"); 
    link(kernel, "PROVIDES", scheduler);
    
    crate::kinfo!("ROOT: registered items. host={:x} kernel={:x}", host, kernel);
    
    BootInventory { host, kernel, root: root_svc }
}
