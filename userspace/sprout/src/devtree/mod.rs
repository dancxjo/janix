pub mod x86_64;
pub mod aarch64;
pub mod riscv64;
pub mod loongarch64;

use stem::thing::{ThingId, sys as thingsys};
// use alloc::vec::Vec;
use alloc::vec;
use stem::kprintln;
use abi::schema::{keys, kinds, rels, source, confidence};

pub fn set_str_prop(id: ThingId, key: &str, val: &str) -> Result<(), ()> {
    let sym = thingsys::intern(val).map_err(|_| ())?;
    thingsys::prop_set(id, key, sym as u64).map_err(|_| ())
}

pub struct DevTreeCtx {
    pub host: ThingId,
    pub platform_bus: ThingId,
    pub hhdm: usize,
    pub acpi_rsdp: Option<usize>,
    pub dtb_ptr: Option<usize>,
    pub dtb_bytespace: Option<ThingId>,
    pub dtb_node_id: Option<ThingId>,
}

pub fn init() -> Result<DevTreeCtx, ()> {
    kprintln!("SPROUT: devtree::init entry (v0.2)");

    // 1. Find Host
    let mut hosts = [ThingId(0); 1];
    // Cast u64 -> usize is implicit in find logic or needed? 
    // root::find syscall takes SymbolShell. kinds::DEV_HOST is &str. fits.
    let count = thingsys::find(kinds::DEV_HOST, &mut hosts).map_err(|e| {
        kprintln!("SPROUT: find(dev.host) failed: {:?}", e);
        ()
    })?;
    if count == 0 { 
        kprintln!("SPROUT: No dev.host node found!");
        return Err(()); 
    }
    let host = hosts[0];
    
    // 2. Get HHDM Offset
    let hhdm = thingsys::prop_get(host, keys::HHDM_OFFSET).map_err(|e| {
        kprintln!("SPROUT: prop_get(hhdm_offset) failed: {:?}", e);
        ()
    })? as usize;
    
    // 3. Find/Create Platform Bus
    let mut buses = [ThingId(0); 1];
    let bcount = thingsys::find(kinds::DEV_BUS_PLATFORM, &mut buses).unwrap_or(0);
    let platform_bus = if bcount > 0 {
        buses[0]
    } else {
        kprintln!("SPROUT: Creating dev.bus.platform...");
        let bus = thingsys::create_node(kinds::DEV_BUS_PLATFORM).map_err(|_| ())?;
        thingsys::link(host, rels::HAS_BUS, bus).map_err(|_| ())?;
        
        thingsys::prop_set(bus, keys::SOURCE, source::PLATFORM as u64).ok();
        thingsys::prop_set(bus, keys::CONFIDENCE, confidence::HIGH as u64).ok();
        
        bus
    };
    
    // 4. Check for Firmware
    let mut acpi_rsdp = None;
    let mut dtb_ptr = None;
    let mut dtb_bytespace = None;
    let mut dtb_node_id = None;
    
    let mut fw_buf = [ThingId(0); 4];
    if let Ok(count) = thingsys::find(kinds::FW_TABLE_ACPI, &mut fw_buf) {
        if count > 0 {
             if let Ok(val) = thingsys::prop_get(fw_buf[0], keys::PHYS_BASE) {
                 acpi_rsdp = Some(val as usize);
                 kprintln!("SPROUT: ACPI RSDP = 0x{:x}", val);
             }
        }
    }
    
    if let Ok(count) = thingsys::find(kinds::FW_TABLE_DTB, &mut fw_buf) {
        if count > 0 {
             dtb_node_id = Some(fw_buf[0]);
             if let Ok(val) = thingsys::prop_get(fw_buf[0], keys::PHYS_BASE) {
                 dtb_ptr = Some(val as usize);
             }
             // For v0.2, DTB is backed by bytespace. Find the edge or property?
             // tasks says: fw.table.* --BACKED_BY--> bytespace
             // But boot_register v0.2 logic sets 'props::BYTESPACE' legacy?
             // No, my new boot_register sets 'link(dtb_node, rels::BACKED_BY, bs)'.
             // Wait, I should traverse the edge to find bytespace ID.
             // OR, for now, if boot_register still sets a legacy prop or I rely on finding the edge.
             // thingsys lib doesn't have convenient "get unique target of relation" yet?
             // It does have dump_edges etc.
             // For v0.2 simplification, let's assume if we need bytespace we find it via edge.
             // BUT, `build` function below relies on `dtb_bytespace` being set here.
             
             // Hack: For now, I'll rely on traversal or finding legacy prop IF I set it.
             // I didn't set legacy prop in boot_register v0.2. I removed it.
             // So I MUST find the bytespace node via relation.
             // thingsys::dump_edges(id) -> list of (rel, dst).
             // Since I need to find BACKED_BY.
             
             // This is hard with current thingsys query API?
             // thingsys::dump_edges returns formatted string? No, syscall `DumpEdges` returns text description.
             // thingsys::describe_edge returns text.
             // Wait, I need raw graph access. RootOp::DumpEdges returns text to buffer.
             // RootOp::DescribeEdge...
             
             // There is `SYS_ROOT_DUMP_EDGES`? No, that formats text.
             // `SYS_ROOT_DESCRIBE_EDGE` formats text.
             
             // Is there a "Get Edge Target" syscall?
             // RootOp::Query!
             // query: (id) -> (BACKED_BY) -> ?
             
             // query::execute is supported!
             // plan: [Load(id), Traverse(BACKED_BY), Return 1st row]
             // Actually, simplest is to just assume I can't read DTB in userspace yet via Graph if query API isn't wrapped nicely.
             // But wait, the previous code had `dtb_bytespace = Some(ThingId(val))` from `prop_get`.
             // I should probably RESTORE `set(dtb_node, "bytespace", bs)` in boot_register as a shortcut for now, 
             // alongside the proper relation. That way I don't need complex query logic in Sprout yet.
             
             // Let's modify boot_register to ALSO set the property "bytespace" with the ID.
             // It's cheaper than implementing query here.
             // But I already wrote boot_register.
             // Okay, I will modify boot_register in next step to add back the helper prop.
             // OR I can use `thingsys::get_kind(id)` on neighbors? No.
             
             // Let's assume for a moment I can get the prop. I'll add the prop back in boot_register.
             if let Ok(val) = thingsys::prop_get(fw_buf[0], "bytespace") {
                 dtb_bytespace = Some(ThingId(val));
                 kprintln!("SPROUT: DTB bytespace (prop) = {}", val);
             }
        }
    }
    
    Ok(DevTreeCtx {
        host,
        platform_bus,
        hhdm,
        acpi_rsdp,
        dtb_ptr,
        dtb_bytespace,
        dtb_node_id,
    })
}

pub fn build(ctx: &DevTreeCtx) -> Result<(), ()> {
    // Attempt DTB parsing if available
    if let Some(bs_id) = ctx.dtb_bytespace {
        kprintln!("SPROUT: Found DTB bytespace {}, parsing...", bs_id.0);
        let mut header = [0u8; 8];
        if let Ok(_) = thingsys::bytespace_read(bs_id, 0, &mut header) {
            let magic = u32::from_be_bytes([header[0], header[1], header[2], header[3]]);
            if magic == 0xd00dfeed {
                let size = u32::from_be_bytes([header[4], header[5], header[6], header[7]]) as usize;
                
                if size < 2 * 1024 * 1024 { // Up to 2MB DTB
                    let mut buf = vec![0u8; size];
                    if let Ok(read_len) = thingsys::bytespace_read(bs_id, 0, &mut buf) {
                         if read_len == size {
                             if let Ok(fdt) = fdt::Fdt::new(&buf) {
                                 kprintln!("SPROUT: Valid FDT found. Iterating nodes...");
                                 
                                 for node in fdt.all_nodes() {
                                      let name = node.name.split('@').next().unwrap_or("");
                                      let mut kind = "";
                                      
                                      if name.contains("serial") || name.contains("uart") {
                                          kind = "dev.serial.uart"; // Map to schema/kinds if possible, or string literal
                                      } else if name.contains("intc") || name.contains("interrupt-controller") || name.contains("plic") || name.contains("clint") || name.contains("gic") {
                                          kind = "dev.interrupt_controller";
                                      } else if name.contains("timer") {
                                          kind = "dev.timer";
                                      }
                                      
                                      if !kind.is_empty() {
                                           // Create device
                                           if let Ok(dev) = thingsys::create_node(kind) {
                                               // Set provenance (Numeric)
                                               let _ = thingsys::prop_set(dev, keys::SOURCE, source::DTB as u64);
                                               let _ = thingsys::prop_set(dev, keys::CONFIDENCE, confidence::HIGH as u64);
                                               
                                               // Set identity
                                               let _ = set_str_prop(dev, keys::NAME, node.name);
                                               if let Some(compat) = node.compatible() {
                                                   for c in compat.all() {
                                                       let _ = set_str_prop(dev, "compatible", c);
                                                       break; // Only first one for now
                                                   }
                                               }

                                               // Link to platform bus
                                               let _ = thingsys::link(ctx.platform_bus, rels::HAS_DEVICE, dev);
                                               
                                               // Link evidence
                                               if let Some(evidence) = ctx.dtb_node_id {
                                                   let _ = thingsys::link(dev, rels::DERIVED_FROM, evidence);
                                               }
                                               
                                               // Resources: MMIO
                                               if let Some(regs) = node.reg() {
                                                   for reg in regs {
                                                       if let Ok(res) = thingsys::create_node("res.mmio.range") {
                                                           let _ = thingsys::prop_set(res, keys::SOURCE, source::DTB as u64);
                                                           let _ = thingsys::prop_set(res, keys::CONFIDENCE, confidence::HIGH as u64);
                                                           let _ = thingsys::prop_set(res, keys::PHYS_BASE, reg.starting_address as u64);
                                                           let _ = thingsys::prop_set(res, keys::SIZE_BYTES, reg.size.unwrap_or(0) as u64);
                                                           let _ = thingsys::link(dev, rels::HAS_RESOURCE, res);
                                                       }
                                                   }
                                               }
                                               
                                               // Resources: IRQ
                                               if let Some(irqs) = node.interrupts() {
                                                   for irq in irqs {
                                                        if let Ok(res) = thingsys::create_node("res.irq") {
                                                            let _ = thingsys::prop_set(res, keys::SOURCE, source::DTB as u64);
                                                            let _ = thingsys::prop_set(res, keys::CONFIDENCE, confidence::HIGH as u64);
                                                            let _ = thingsys::prop_set(res, "irq", irq as u64);
                                                            let _ = thingsys::link(dev, rels::HAS_RESOURCE, res);
                                                        }
                                                   }
                                               }
                                           }
                                      }
                                 }
                             }
                         }
                    }
                }
            }
        }
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
