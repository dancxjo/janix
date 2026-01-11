pub mod x86_64;
pub mod aarch64;
pub mod riscv64;
pub mod loongarch64;

use stem::thing::{ThingId, sys as thingsys};
use alloc::vec::Vec;
use alloc::vec;

pub struct DevTreeCtx {
    pub host: ThingId,
    pub platform_bus: ThingId,
    pub hhdm: usize,
    pub acpi_rsdp: Option<usize>,
    pub dtb_ptr: Option<usize>,
}

pub fn init() -> Result<DevTreeCtx, ()> {
    // 1. Find Host
    let mut hosts = [ThingId(0); 1];
    let count = thingsys::find("dev.host", &mut hosts).map_err(|_| ())?;
    if count == 0 { return Err(()); }
    let host = hosts[0];
    
    // 2. Get HHDM Offset
    let hhdm = thingsys::prop_get(host, "hhdm_offset").map_err(|_| ())? as usize;
    
    // 3. Find/Create Platform Bus
    let mut buses = [ThingId(0); 1];
    let bcount = thingsys::find("dev.bus.platform", &mut buses).unwrap_or(0);
    let platform_bus = if bcount > 0 {
        buses[0]
    } else {
        let bus = thingsys::create_node("dev.bus.platform").map_err(|_| ())?;
        thingsys::link(host, "HAS_BUS", bus).map_err(|_| ())?;
        bus
    };
    
    // 4. Check for Firmware
    let mut acpi_rsdp = None;
    let mut dtb_ptr = None;
    
    let mut fw_buf = [ThingId(0); 4];
    if let Ok(count) = thingsys::find("fw.table.acpi", &mut fw_buf) {
        if count > 0 {
             if let Ok(val) = thingsys::prop_get(fw_buf[0], "phys_base") {
                 acpi_rsdp = Some(val as usize);
             }
        }
    }
    
    if let Ok(count) = thingsys::find("fw.table.dtb", &mut fw_buf) {
        if count > 0 {
             if let Ok(val) = thingsys::prop_get(fw_buf[0], "phys_base") {
                 dtb_ptr = Some(val as usize);
             }
        }
    }
    
    Ok(DevTreeCtx {
        host,
        platform_bus,
        hhdm,
        acpi_rsdp,
        dtb_ptr,
    })
}

pub fn build(ctx: &DevTreeCtx) -> Result<(), ()> {
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
