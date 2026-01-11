pub mod x86_64;
pub mod aarch64;
pub mod riscv64;
pub mod loongarch64;

use stem::thing::{ThingId, sys as thingsys};
use alloc::vec::Vec;
use alloc::vec;
use stem::kprintln;

pub struct DevTreeCtx {
    pub host: ThingId,
    pub platform_bus: ThingId,
    pub hhdm: usize,
    pub acpi_rsdp: Option<usize>,
    pub dtb_ptr: Option<usize>,
    pub dtb_bytespace: Option<ThingId>,
}

pub fn init() -> Result<DevTreeCtx, ()> {
    kprintln!("SPROUT: devtree::init entry");

    // 1. Find Host
    let mut hosts = [ThingId(0); 1];
    let count = thingsys::find("dev.host", &mut hosts).map_err(|e| {
        kprintln!("SPROUT: find(dev.host) failed: {:?}", e);
        ()
    })?;
    kprintln!("SPROUT: found {} dev.host nodes", count);
    if count == 0 { 
        kprintln!("SPROUT: No dev.host node found!");
        return Err(()); 
    }
    let host = hosts[0];
    
    // 2. Get HHDM Offset
    let hhdm = thingsys::prop_get(host, "hhdm_offset").map_err(|e| {
        kprintln!("SPROUT: prop_get(hhdm_offset) failed: {:?}", e);
        ()
    })? as usize;
    kprintln!("SPROUT: HHDM offset = 0x{:x}", hhdm);
    
    // 3. Find/Create Platform Bus
    let mut buses = [ThingId(0); 1];
    let bcount = thingsys::find("dev.bus.platform", &mut buses).unwrap_or(0);
    kprintln!("SPROUT: found {} dev.bus.platform nodes", bcount);

    let platform_bus = if bcount > 0 {
        buses[0]
    } else {
        kprintln!("SPROUT: Creating dev.bus.platform...");
        // If not found, create (though kernel census should have created it)
        let bus = thingsys::create_node("dev.bus.platform").map_err(|e| {
            kprintln!("SPROUT: create_node(dev.bus.platform) failed: {:?}", e);
            ()
        })?;
        thingsys::link(host, "HAS_BUS", bus).map_err(|e| {
            kprintln!("SPROUT: link(HAS_BUS) failed: {:?}", e);
            ()
        })?;
        bus
    };
    kprintln!("SPROUT: Platform bus id = {}", platform_bus.0);
    
    // 4. Check for Firmware
    let mut acpi_rsdp = None;
    let mut dtb_ptr = None;
    let mut dtb_bytespace = None;
    
    let mut fw_buf = [ThingId(0); 4];
    if let Ok(count) = thingsys::find("fw.table.acpi", &mut fw_buf) {
        kprintln!("SPROUT: found {} fw.table.acpi nodes", count);
        if count > 0 {
             if let Ok(val) = thingsys::prop_get(fw_buf[0], "phys_base") {
                 acpi_rsdp = Some(val as usize);
                 kprintln!("SPROUT: ACPI RSDP = 0x{:x}", val);
             }
        }
    }
    
    if let Ok(count) = thingsys::find("fw.table.dtb", &mut fw_buf) {
        kprintln!("SPROUT: found {} fw.table.dtb nodes", count);
        if count > 0 {
             if let Ok(val) = thingsys::prop_get(fw_buf[0], "phys_base") {
                 dtb_ptr = Some(val as usize);
                 kprintln!("SPROUT: DTB phys = 0x{:x}", val);
             }
             if let Ok(val) = thingsys::prop_get(fw_buf[0], "bytespace") {
                 dtb_bytespace = Some(ThingId(val));
                 kprintln!("SPROUT: DTB bytespace = {}", val);
             }
        }
    }
    
    kprintln!("SPROUT: devtree::init success");
    Ok(DevTreeCtx {
        host,
        platform_bus,
        hhdm,
        acpi_rsdp,
        dtb_ptr,
        dtb_bytespace,
    })
}

pub fn build(ctx: &DevTreeCtx) -> Result<(), ()> {
    // Attempt DTB parsing if available
    if let Some(bs_id) = ctx.dtb_bytespace {
        kprintln!("SPROUT: Found DTB bytespace {}, parsing...", bs_id.0);
        // Read header first (magic + size)
        // DTB header is big endian. Magic is 0xd00dfeed at offset 0.
        // Totalsize at offset 4.
        let mut header = [0u8; 8];
        if let Ok(_) = thingsys::bytespace_read(bs_id, 0, &mut header) {
            let magic = u32::from_be_bytes([header[0], header[1], header[2], header[3]]);
            if magic == 0xd00dfeed {
                let size = u32::from_be_bytes([header[4], header[5], header[6], header[7]]) as usize;
                
                // Allocate buffer (simple vec for now, assumption: heap exists)
                // Limit size to avoid exhausting small heap? 64KB - 128KB typical.
                if size < 256 * 1024 {
                    let mut buf = vec![0u8; size];
                    if let Ok(read_len) = thingsys::bytespace_read(bs_id, 0, &mut buf) {
                         if read_len == size {
                             // Parse with FDT
                             if let Ok(fdt) = fdt::Fdt::new(&buf) {
                                 kprintln!("SPROUT: Valid FDT found. Iterating nodes...");
                                 
                                 for node in fdt.all_nodes() {
                                      // Check for serial/uart
                                      let name = node.name.split('@').next().unwrap_or("");
                                      if name.contains("serial") || name.contains("uart") {
                                           kprintln!("SPROUT: Found serial node: {}", node.name);
                                           
                                           // Create dev.serial
                                           if let Ok(dev) = thingsys::create_node("dev.serial") {
                                               // Link to platform bus
                                               let _ = thingsys::link(ctx.platform_bus, "HAS_DEVICE", dev);
                                               
                                               // Publish 'reg' (phys_base)
                                               if let Some(reg) = node.reg().and_then(|mut i| i.next()) {
                                                   let _ = thingsys::prop_set(dev, "phys_base", reg.starting_address as u64);
                                                   let _ = thingsys::prop_set(dev, "phys_len", reg.size.unwrap_or(0) as u64);
                                               }
                                               
                                               // Publish 'interrupts' (irq)
                                               if let Some(irq) = node.interrupts().and_then(|mut i| i.next()) {
                                                   let _ = thingsys::prop_set(dev, "irq", irq as u64);
                                               }
                                               
                                               // Mark as published
                                               kprintln!("SPROUT: Published dev.serial {}", node.name);
                                           }
                                      }
                                 }
                             } else {
                                 kprintln!("SPROUT: FDT parse failed");
                             }
                         }
                    }
                } else {
                     kprintln!("SPROUT: DTB too large {}", size);
                }
            } else {
                kprintln!("SPROUT: Invalid DTB magic {:x}", magic);
            }
        }
    } else {
        kprintln!("SPROUT: No DTB bytespace found.");
    }
    
    
    #[cfg(target_arch = "x86_64")]
    return x86_64::enumerate(ctx);
    
    #[cfg(target_arch = "aarch64")]
    return aarch64::enumerate(ctx);
    
    #[cfg(target_arch = "riscv64")]
    return riscv64::enumerate(ctx);

    #[cfg(target_arch = "loongarch64")]
    return loongarch64::enumerate(ctx);
    
    #[allow(unreachable_code)]
    Ok(())
}
