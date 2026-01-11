pub mod x86_64;
pub mod aarch64;
pub mod riscv64;
pub mod loongarch64;

use stem::thing::{ThingId, sys as thingsys};
use alloc::vec::Vec;
use alloc::vec;
use stem::kprintln;

pub fn set_str_prop(id: ThingId, key: &str, val: &str) -> Result<(), ()> {
    let sym = thingsys::intern(val).map_err(|_| ())?;
    thingsys::prop_set(id, key, sym as u64).map_err(|_| ())
}

// Unified Device Graph Schema Constants (v0.1)
pub mod schema {
    pub const DEV_HOST: &str = "dev.host";
    pub const DEV_BUS_PLATFORM: &str = "dev.bus.platform";
    
    // Core
    pub const FW_TABLE_ACPI: &str = "fw.table.acpi";
    pub const FW_TABLE_DTB: &str = "fw.table.dtb";
    
    // Devices
    pub const DEV_SERIAL_UART: &str = "dev.serial.uart";
    pub const DEV_INTERRUPT_CONTROLLER: &str = "dev.interrupt_controller";
    pub const DEV_TIMER: &str = "dev.timer";
    pub const DEV_RTC_CMOS: &str = "dev.rtc.cmos";
    
    // Resources
    pub const RES_MMIO_RANGE: &str = "res.mmio.range";
    pub const RES_IO_PORT_RANGE: &str = "res.io.port_range";
    pub const RES_IRQ: &str = "res.irq";
    
    // Relations
    pub const HAS_BUS: &str = "HAS_BUS";
    pub const HAS_DEVICE: &str = "HAS_DEVICE";
    pub const HAS_RESOURCE: &str = "HAS_RESOURCE";
    pub const DERIVED_FROM: &str = "DERIVED_FROM";
    
    // Properties
    pub const PROP_SOURCE: &str = "source";         
    pub const PROP_CONFIDENCE: &str = "confidence";
    pub const PROP_NAME: &str = "name";
    pub const PROP_COMPATIBLE: &str = "compatible";
    pub const PROP_PATH: &str = "path";
    pub const PROP_HHDM_OFFSET: &str = "hhdm_offset";
    pub const PROP_BYTESPACE: &str = "bytespace";
    pub const PROP_PHYS_BASE: &str = "phys_base";
    pub const PROP_PHYS_LEN: &str = "phys_len"; 
    pub const PROP_SIZE: &str = "size";
    pub const PROP_IRQ: &str = "irq";
    
    pub const PROP_START: &str = "start";
    pub const PROP_END: &str = "end";
    
    // Values
    pub const SRC_DTB: &str = "dtb";
    pub const SRC_PLATFORM: &str = "platform";
    pub const CONFIDENCE_HIGH: &str = "high";
    pub const CONFIDENCE_MEDIUM: &str = "medium";
    pub const CONFIDENCE_LOW: &str = "low";
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
    kprintln!("SPROUT: devtree::init entry");

    // 1. Find Host
    let mut hosts = [ThingId(0); 1];
    let count = thingsys::find(schema::DEV_HOST, &mut hosts).map_err(|e| {
        kprintln!("SPROUT: find(dev.host) failed: {:?}", e);
        ()
    })?;
    if count == 0 { 
        kprintln!("SPROUT: No dev.host node found!");
        return Err(()); 
    }
    let host = hosts[0];
    
    // 2. Get HHDM Offset
    let hhdm = thingsys::prop_get(host, schema::PROP_HHDM_OFFSET).map_err(|e| {
        kprintln!("SPROUT: prop_get(hhdm_offset) failed: {:?}", e);
        ()
    })? as usize;
    
    // 3. Find/Create Platform Bus
    let mut buses = [ThingId(0); 1];
    let bcount = thingsys::find(schema::DEV_BUS_PLATFORM, &mut buses).unwrap_or(0);
    let platform_bus = if bcount > 0 {
        buses[0]
    } else {
        kprintln!("SPROUT: Creating dev.bus.platform...");
        let bus = thingsys::create_node(schema::DEV_BUS_PLATFORM).map_err(|_| ())?;
        thingsys::link(host, schema::HAS_BUS, bus).map_err(|_| ())?;
        bus
    };
    
    // 4. Check for Firmware
    let mut acpi_rsdp = None;
    let mut dtb_ptr = None;
    let mut dtb_bytespace = None;
    let mut dtb_node_id = None;
    
    let mut fw_buf = [ThingId(0); 4];
    if let Ok(count) = thingsys::find(schema::FW_TABLE_ACPI, &mut fw_buf) {
        if count > 0 {
             if let Ok(val) = thingsys::prop_get(fw_buf[0], schema::PROP_PHYS_BASE) {
                 acpi_rsdp = Some(val as usize);
                 kprintln!("SPROUT: ACPI RSDP = 0x{:x}", val);
             }
        }
    }
    
    if let Ok(count) = thingsys::find(schema::FW_TABLE_DTB, &mut fw_buf) {
        if count > 0 {
             dtb_node_id = Some(fw_buf[0]);
             if let Ok(val) = thingsys::prop_get(fw_buf[0], schema::PROP_PHYS_BASE) {
                 dtb_ptr = Some(val as usize);
             }
             if let Ok(val) = thingsys::prop_get(fw_buf[0], schema::PROP_BYTESPACE) {
                 dtb_bytespace = Some(ThingId(val));
                 kprintln!("SPROUT: DTB bytespace = {}", val);
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
                
                if size < 512 * 1024 {
                    let mut buf = vec![0u8; size];
                    if let Ok(read_len) = thingsys::bytespace_read(bs_id, 0, &mut buf) {
                         if read_len == size {
                             if let Ok(fdt) = fdt::Fdt::new(&buf) {
                                 kprintln!("SPROUT: Valid FDT found. Iterating nodes...");
                                 
                                 for node in fdt.all_nodes() {
                                      let name = node.name.split('@').next().unwrap_or("");
                                      let mut kind = "";
                                      
                                      if name.contains("serial") || name.contains("uart") {
                                          kind = schema::DEV_SERIAL_UART;
                                      } else if name.contains("intc") || name.contains("interrupt-controller") || name.contains("plic") || name.contains("clint") || name.contains("gic") {
                                          kind = schema::DEV_INTERRUPT_CONTROLLER;
                                      } else if name.contains("timer") {
                                          kind = schema::DEV_TIMER;
                                      }
                                      
                                      if !kind.is_empty() {
                                           // Create device
                                           if let Ok(dev) = thingsys::create_node(kind) {
                                               // Set provenance
                                               let _ = set_str_prop(dev, schema::PROP_SOURCE, schema::SRC_DTB);
                                               let _ = set_str_prop(dev, schema::PROP_CONFIDENCE, schema::CONFIDENCE_HIGH);
                                               
                                               // Set identity
                                               let _ = set_str_prop(dev, schema::PROP_NAME, node.name);
                                               if let Some(compat) = node.compatible() {
                                                   for c in compat.all() {
                                                       let _ = set_str_prop(dev, schema::PROP_COMPATIBLE, c);
                                                       break; // Only first one for now
                                                   }
                                               }

                                               // Link to platform bus
                                               let _ = thingsys::link(ctx.platform_bus, schema::HAS_DEVICE, dev);
                                               
                                               // Link evidence
                                               if let Some(evidence) = ctx.dtb_node_id {
                                                   let _ = thingsys::link(dev, schema::DERIVED_FROM, evidence);
                                               }
                                               
                                               // Resources: MMIO
                                               if let Some(regs) = node.reg() {
                                                   for reg in regs {
                                                       if let Ok(res) = thingsys::create_node(schema::RES_MMIO_RANGE) {
                                                           let _ = set_str_prop(res, schema::PROP_SOURCE, schema::SRC_DTB);
                                                           let _ = set_str_prop(res, schema::PROP_CONFIDENCE, schema::CONFIDENCE_HIGH);
                                                           let _ = thingsys::prop_set(res, schema::PROP_PHYS_BASE, reg.starting_address as u64);
                                                           let _ = thingsys::prop_set(res, schema::PROP_SIZE, reg.size.unwrap_or(0) as u64);
                                                           let _ = thingsys::link(dev, schema::HAS_RESOURCE, res);
                                                       }
                                                   }
                                               }
                                               
                                               // Resources: IRQ
                                               if let Some(irqs) = node.interrupts() {
                                                   for irq in irqs {
                                                        if let Ok(res) = thingsys::create_node(schema::RES_IRQ) {
                                                            let _ = set_str_prop(res, schema::PROP_SOURCE, schema::SRC_DTB);
                                                            let _ = set_str_prop(res, schema::PROP_CONFIDENCE, schema::CONFIDENCE_HIGH);
                                                            let _ = thingsys::prop_set(res, schema::PROP_IRQ, irq as u64);
                                                            let _ = thingsys::link(dev, schema::HAS_RESOURCE, res);
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
