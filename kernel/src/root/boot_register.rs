use super::SymbolShell;
use super::graph::ThingId;
use super::{RootOp, enqueue};
use crate::device_registry::{DeviceEntry, REGISTRY};
use crate::{BootModuleDesc, FramebufferInfo, PhysRange};
use abi::schema::{confidence, keys, kinds, rels, source};

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

pub fn register_all<R: crate::BootRuntime>(runtime: &R, info: &BootInfo) -> BootInventory {
    crate::kinfo!("ROOT: boot registration begin (Census Phase 1 v0.2)");

    let create = |kind: &str| -> u64 {
        let reply = enqueue(RootOp::CreateNode {
            kind: SymbolShell::Str(alloc::string::String::from(kind)),
        });
        loop {
            let done = reply.done.load(core::sync::atomic::Ordering::Acquire);
            if done != 0 {
                return reply.value.load(core::sync::atomic::Ordering::Relaxed);
            }
            unsafe {
                crate::task::scheduler::yield_now_current();
            }
        }
    };

    let set = |id: u64, key: &str, val: u64| {
        let reply = enqueue(RootOp::PropSet {
            id,
            key: SymbolShell::Str(alloc::string::String::from(key)),
            value: val,
        });
        loop {
            let done = reply.done.load(core::sync::atomic::Ordering::Acquire);
            if done != 0 {
                break;
            }
            unsafe {
                crate::task::scheduler::yield_now_current();
            }
        }
    };

    let link = |src: u64, rel: &str, dst: u64| {
        let reply = enqueue(RootOp::Link {
            src,
            rel: SymbolShell::Str(alloc::string::String::from(rel)),
            dst,
        });
        loop {
            let done = reply.done.load(core::sync::atomic::Ordering::Acquire);
            if done != 0 {
                break;
            }
            unsafe {
                crate::task::scheduler::yield_now_current();
            }
        }
    };

    let intern = |s: &str| -> u64 {
        let reply = enqueue(RootOp::Intern {
            name: alloc::string::String::from(s),
        });
        loop {
            let done = reply.done.load(core::sync::atomic::Ordering::Acquire);
            if done != 0 {
                return reply.value.load(core::sync::atomic::Ordering::Relaxed);
            }
            unsafe {
                crate::task::scheduler::yield_now_current();
            }
        }
    };

    let bytespace_create = |len: u64| -> u64 {
        let reply = enqueue(RootOp::BytespaceCreate {
            len,
            flags: 0,
            format: 0,
        });
        loop {
            let done = reply.done.load(core::sync::atomic::Ordering::Acquire);
            if done != 0 {
                return reply.value.load(core::sync::atomic::Ordering::Relaxed);
            }
            unsafe {
                crate::task::scheduler::yield_now_current();
            }
        }
    };

    let bytespace_write = |id: u64, offset: u64, ptr: u64, len: u64| {
        let reply = enqueue(RootOp::BytespaceWrite {
            id,
            offset,
            ptr,
            len,
        });
        loop {
            let done = reply.done.load(core::sync::atomic::Ordering::Acquire);
            if done != 0 {
                break;
            }
            unsafe {
                crate::task::scheduler::yield_now_current();
            }
        }
    };

    // Use consistent numeric provenance (u8 -> u64)
    let src_boot = source::BOOT as u64;
    let conf_high = confidence::HIGH as u64;

    // 1. Host
    crate::contract!("ROOT: registering Host...");
    let host = create(kinds::DEV_HOST);
    set(host, keys::HHDM_OFFSET, info.hhdm_offset);
    let arch_id = intern(info.arch);
    set(host, "arch", arch_id); // "arch" prop key not in keys:: yet, maybe map to ARCH or intern string
    let platform_id = intern(info.platform_profile);
    set(host, "platform_profile", platform_id);

    set(host, keys::SOURCE, src_boot);
    set(host, keys::CONFIDENCE, conf_high);
    // Publish host anchor early so fallback host links can attach during boot census.
    super::graph_anchors::set_host(host);
    crate::contract!("ROOT: Host registered: t{:x}", host);

    // 2. Platform Bus
    let platform_bus = create(kinds::DEV_BUS_PLATFORM);
    let pbus_name = intern("platform0");
    set(platform_bus, keys::NAME, pbus_name);
    set(platform_bus, keys::SOURCE, src_boot);
    set(platform_bus, keys::CONFIDENCE, conf_high);

    link(host, rels::HAS_BUS, platform_bus);

    // 3. Kernel
    let kernel = create(kinds::PROC_KERNEL);
    set(kernel, "version", 1);
    // Initialize TimeState: 0 = Unanchored, 1 = Anchored
    set(kernel, "sys.TimeState", 0);
    link(kernel, rels::RUNS_ON, host);

    // 4. Root Service
    let root_svc = create(kinds::SVC_ROOT);
    let root_name = intern("/");
    set(root_svc, keys::NAME, root_name);
    link(kernel, rels::PROVIDES, root_svc);
    link(root_svc, rels::MONITORS, host);

    // 5. CPUs
    for i in 0..info.cpu_count {
        let cpu = create(kinds::DEV_CPU);
        set(cpu, "id", i as u64);
        set(cpu, keys::SOURCE, src_boot);
        set(cpu, keys::CONFIDENCE, conf_high);
        link(host, rels::HAS_CPU, cpu);
        super::graph_anchors::set_cpu_thing(i, cpu);
    }

    // 6. Memory Ranges
    let mut fb_backing_range: Option<ThingId> = None;
    let fb_phys_start = info
        .framebuffer
        .as_ref()
        .map(|fb| fb.addr.saturating_sub(info.hhdm_offset));

    for range in info.memory_map {
        let mem = create(kinds::MEM_RANGE);
        set(mem, "start", range.start);
        set(mem, "end", range.end);
        set(mem, "kind", range.kind as u64); // "kind" might be legacy, leaving it for now
        set(mem, keys::SOURCE, src_boot);
        set(mem, keys::CONFIDENCE, conf_high);
        link(host, rels::HAS_MEMORY_RANGE, mem);

        // Check if this range creates the backing for the framebuffer
        if let Some(start) = fb_phys_start {
            // Simple containment check: range.start <= fb_phys && range.end > fb_phys
            // Note: Framebuffer usually is its own range or part of a larger Reserved/Framebuffer range.
            if range.start <= start && range.end > start {
                fb_backing_range = Some(mem);
            }
        }
    }

    // 7. Modules
    let bytespace_create_ptr = |ptr: u64, len: u64| -> u64 {
        let reply = enqueue(RootOp::BytespaceCreateFromPtr { ptr, len });
        loop {
            let done = reply.done.load(core::sync::atomic::Ordering::Acquire);
            if done != 0 {
                return reply.value.load(core::sync::atomic::Ordering::Relaxed);
            }
            unsafe {
                crate::task::scheduler::yield_now_current();
            }
        }
    };

    for (i, m) in info.modules.iter().enumerate() {
        let mod_node = create(kinds::BOOT_MODULE);
        set(mod_node, keys::PHYS_BASE, m.phys_start);
        set(mod_node, keys::SIZE_BYTES, m.phys_end - m.phys_start);
        set(mod_node, "index", i as u64);
        let name_id = intern(m.name);
        set(mod_node, keys::NAME, name_id);

        // Zero-copy bytespace wrapper
        let len = m.phys_end - m.phys_start;
        let virt_ptr = m.phys_start.saturating_add(info.hhdm_offset);
        let bs = bytespace_create_ptr(virt_ptr, len);

        link(mod_node, rels::BACKED_BY, bs);
        set(mod_node, "bytespace", bs);

        set(mod_node, keys::SOURCE, src_boot);
        set(mod_node, keys::CONFIDENCE, conf_high);

        link(host, rels::HAS_MODULE, mod_node);
        link(root_svc, rels::HAS_MODULE, mod_node);
    }

    // 8. Framebuffer
    if let Some(fb) = info.framebuffer.as_ref() {
        let fb_node = create(kinds::DEV_DISPLAY_FRAMEBUFFER);

        // Fix: fb.addr is HHDM (virtual). Store as virt_base.
        // Calculate physical by subtracting HHDM offset.
        set(fb_node, "virt_base", fb.addr);
        let phys_base = fb.addr.saturating_sub(info.hhdm_offset);
        set(fb_node, keys::PHYS_BASE, phys_base);

        set(fb_node, "width", fb.width as u64);
        set(fb_node, "height", fb.height as u64);
        set(fb_node, "stride", fb.pitch as u64);
        set(fb_node, "bpp", fb.bpp as u64);
        set(fb_node, keys::SIZE_BYTES, fb.byte_len as u64);

        if let Some(backing_mem) = fb_backing_range {
            link(fb_node, rels::BACKED_BY, backing_mem);
        }

        let fmt = fb.format.to_wire();
        set(fb_node, keys::FORMAT, fmt);

        set(fb_node, keys::SOURCE, src_boot);
        set(fb_node, keys::CONFIDENCE, conf_high);

        link(host, rels::HAS_DEVICE, fb_node);

        let mut bars = [0u64; 6];
        let mut sizes = [0u64; 6];
        bars[0] = phys_base;
        sizes[0] = fb.byte_len as u64;
        let entry = DeviceEntry::new_mmio(kinds::DEV_DISPLAY_FRAMEBUFFER, fb_node, bars, sizes);
        let mut reg = REGISTRY.lock();
        let _ = reg.register(entry);
    }

    // 9. Firmware Tables
    if info.acpi_rsdp.is_some() || info.dtb_ptr.is_some() {
        let fw_boot = create(kinds::FW_BOOT);
        set(fw_boot, keys::SOURCE, src_boot);
        set(fw_boot, keys::CONFIDENCE, conf_high);
        link(host, rels::HAS_FIRMWARE, fw_boot);

        if let Some(rsdp_phys) = info.acpi_rsdp {
            let acpi = create(kinds::FW_TABLE_ACPI);
            set(acpi, keys::PHYS_BASE, rsdp_phys);
            set(acpi, keys::SOURCE, src_boot);
            set(acpi, keys::CONFIDENCE, conf_high);

            // Copy RSDP (36 bytes for v2, 20 for v1; safe to copy 36 if verified)
            // Use temporary mapping to access physical memory
            let size = 36;
            if let Ok(virt) = runtime.map_phys_temp(rsdp_phys, size as usize) {
                let bs = bytespace_create(size);
                bytespace_write(bs, 0, virt, size);
                runtime.unmap_phys_temp(virt, size as usize);

                link(acpi, rels::BACKED_BY, bs);
                link(fw_boot, rels::PROVIDES_TABLE, acpi);
            } else {
                crate::kinfo!("ROOT: Warning: Failed to map ACPI RSDP at {:x}", rsdp_phys);
                // Create a diagnostic node instead
                let diag = create("diagnostic/error");
                let msg = intern("Failed to map ACPI RSDP");
                set(diag, "message", msg);
                link(acpi, "error", diag);
                link(fw_boot, rels::PROVIDES_TABLE, acpi);
            }
        }

        if let Some(dtb_phys) = info.dtb_ptr {
            let dtb_node = create(kinds::FW_TABLE_DTB);

            crate::kinfo!("ROOT: Absorbing DTB (phys={:x})", dtb_phys);

            // 1. Map header to get size
            if let Ok(head_virt) = runtime.map_phys_temp(dtb_phys, 8) {
                let header = unsafe { core::slice::from_raw_parts(head_virt as *const u8, 8) };
                let size_u32 = u32::from_be_bytes([header[4], header[5], header[6], header[7]]);
                let size = size_u32 as u64;
                runtime.unmap_phys_temp(head_virt, 8);

                // 2. Validate size (sane cap: 2MB)
                const DTB_MAX_SIZE: u64 = 2 * 1024 * 1024;
                if size > 0 && size <= DTB_MAX_SIZE {
                    // 3. Map full DTB and copy
                    if let Ok(dtb_virt) = runtime.map_phys_temp(dtb_phys, size as usize) {
                        let bs = bytespace_create(size);
                        bytespace_write(bs, 0, dtb_virt, size);
                        runtime.unmap_phys_temp(dtb_virt, size as usize);

                        link(dtb_node, rels::BACKED_BY, bs);
                        set(dtb_node, "bytespace", bs);
                        set(dtb_node, keys::SOURCE, src_boot);
                        set(dtb_node, keys::CONFIDENCE, conf_high);
                        link(fw_boot, rels::PROVIDES_TABLE, dtb_node);
                    } else {
                        crate::kinfo!(
                            "ROOT: Warning: Failed to map full DTB sized {} at {:x}",
                            size,
                            dtb_phys
                        );
                    }
                } else {
                    crate::kinfo!(
                        "ROOT: Warning: DTB size {} is absurd, capping or skipping.",
                        size
                    );
                    let diag = create("diagnostic/error");
                    let msg = intern("DTB size invalid or too large");
                    set(diag, "message", msg);
                    set(diag, "reported_size", size);
                    link(dtb_node, "error", diag);
                    link(fw_boot, rels::PROVIDES_TABLE, dtb_node);
                }
            } else {
                crate::kinfo!("ROOT: Warning: Failed to map DTB header at {:x}", dtb_phys);
            }
        }
    }

    // 10. Tasking
    let scheduler = create(kinds::SVC_SCHEDULER);
    link(kernel, rels::PROVIDES, scheduler);
    link(root_svc, rels::HAS_SERVICE, scheduler);

    // Store well-known ThingIds for scheduler graphification
    super::graph_anchors::set_scheduler_service(scheduler);
    super::graph_anchors::set_kernel_proc(kernel);
    super::graph_anchors::set_root_service(root_svc);

    // 11. PCI
    crate::contract!("ROOT: Census Phase 2: PCI");
    crate::root::pci::enumerate_and_publish(host, &create, &set, &link, &intern);

    crate::contract!(
        "ROOT: registered items. host=t{:x} kernel=t{:x}",
        host,
        kernel
    );

    BootInventory {
        host,
        kernel,
        root: root_svc,
    }
}
