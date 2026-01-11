use super::{enqueue, RootOp};
use super::graph::ThingId;
use super::SymbolShell;
use super::schema::{kinds, rels, props, provenance};
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

    // Pre-intern provenance constants
    let src_boot = intern(provenance::SRC_BOOT);
    let conf_high = intern(provenance::CONF_HIGH);

    // 1. Host
    let host = create(kinds::HOST);
    set(host, props::HHDM_OFFSET, info.hhdm_offset);
    let arch_id = intern(info.arch);
    set(host, props::ARCH, arch_id);
    let platform_id = intern(info.platform_profile);
    set(host, props::PLATFORM_PROFILE, platform_id);

    set(host, props::SOURCE, src_boot);
    set(host, props::CONFIDENCE, conf_high);

    // 2. Platform Bus
    let platform_bus = create(kinds::BUS_PLATFORM);
    // intern name="platform0"
    let pbus_name = intern("platform0");
    set(platform_bus, props::NAME, pbus_name);
    set(platform_bus, props::SOURCE, src_boot);
    set(platform_bus, props::CONFIDENCE, conf_high);
    
    link(host, rels::HAS_BUS, platform_bus);

    // 3. Kernel
    let kernel = create("proc.kernel");
    set(kernel, props::VERSION, 1);
    link(kernel, rels::RUNS_ON, host);
    
    // 4. Root Service
    let root_svc = create("svc.root");
    link(kernel, rels::PROVIDES, root_svc);
    
    // 5. CPUs
    for i in 0..info.cpu_count {
        let cpu = create(kinds::CPU);
        set(cpu, props::ID, i as u64);
        set(cpu, props::SOURCE, src_boot);
        set(cpu, props::CONFIDENCE, conf_high);
        link(host, rels::HAS_CPU, cpu);
    }
    
    // 6. Memory Ranges
    for range in info.memory_map {
        let mem = create(kinds::MEM_RANGE);
        set(mem, props::START, range.start);
        set(mem, props::END, range.end);
        set(mem, props::KIND, range.kind as u64);
        set(mem, props::SOURCE, src_boot);
        set(mem, props::CONFIDENCE, conf_high);
        link(host, rels::HAS_MEMORY_RANGE, mem);
    }
    
    // 7. Modules
    for (i, m) in info.modules.iter().enumerate() {
        let mod_node = create(kinds::BOOT_MODULE);
        set(mod_node, props::PHYS_BASE, m.phys_start);
        set(mod_node, props::SIZE_BYTES, m.phys_end - m.phys_start);
        set(mod_node, props::INDEX, i as u64);
        let name_id = intern(m.name);
        set(mod_node, props::NAME, name_id);
        
        set(mod_node, props::SOURCE, src_boot);
        set(mod_node, props::CONFIDENCE, conf_high);
        
        link(host, rels::HAS_MODULE, mod_node);
    }
    
    // 8. Framebuffer
    if let Some(fb) = info.framebuffer.as_ref() {
        let fb_node = create(kinds::DISPLAY_FRAMEBUFFER);
        set(fb_node, props::PHYS_BASE, fb.addr);
        set(fb_node, props::WIDTH, fb.width as u64);
        set(fb_node, props::HEIGHT, fb.height as u64);
        set(fb_node, props::STRIDE, fb.pitch as u64);
        set(fb_node, props::BPP, fb.bpp as u64);
        set(fb_node, props::SIZE_BYTES, fb.byte_len as u64);
        
        let fmt = match fb.format {
             crate::PixelFormat::Xrgb8888 => 1,
             crate::PixelFormat::Argb8888 => 2,
             crate::PixelFormat::Rgb565 => 3,
             _ => 0,
        };
        set(fb_node, props::FORMAT, fmt);
        
        set(fb_node, props::SOURCE, src_boot);
        set(fb_node, props::CONFIDENCE, conf_high);
        
        link(host, rels::HAS_DEVICE, fb_node);
    }

    // 9. Firmware Tables
    if info.acpi_rsdp.is_some() || info.dtb_ptr.is_some() {
        let fw_boot = create(kinds::FW_BOOT);
        set(fw_boot, props::SOURCE, src_boot);
        set(fw_boot, props::CONFIDENCE, conf_high);
        link(host, rels::HAS_FIRMWARE, fw_boot);

        if let Some(rsdp) = info.acpi_rsdp {
            let acpi = create(kinds::FW_TABLE_ACPI);
            set(acpi, props::PHYS_BASE, rsdp);
            set(acpi, props::SOURCE, src_boot);
            set(acpi, props::CONFIDENCE, conf_high);
            link(fw_boot, rels::PROVIDES_TABLE, acpi);
        }

        if let Some(dtb_ptr) = info.dtb_ptr {
            let dtb_node = create(kinds::FW_TABLE_DTB);
            // Parse FDT header
            let header = unsafe { core::slice::from_raw_parts(dtb_ptr as *const u8, 8) };
            let size = u32::from_be_bytes([header[4], header[5], header[6], header[7]]) as u64;
            
            crate::kinfo!("ROOT: Absorbing DTB (ptr={:x}, size={})", dtb_ptr, size);
            
            let bs = bytespace_create(size);
            bytespace_write(bs, 0, dtb_ptr, size);
            set(dtb_node, props::BYTESPACE, bs);
            
            set(dtb_node, props::SOURCE, src_boot);
            set(dtb_node, props::CONFIDENCE, conf_high);
            
            link(fw_boot, rels::PROVIDES_TABLE, dtb_node);
        }
    }

    // 10. Tasking
    let scheduler = create("svc.scheduler"); 
    link(kernel, rels::PROVIDES, scheduler);
    
    crate::kinfo!("ROOT: registered items. host={:x} kernel={:x}", host, kernel);
    
    BootInventory { host, kernel, root: root_svc }
}
