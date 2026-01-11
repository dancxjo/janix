use super::DevTreeCtx;
use stem::thing::{sys as thingsys};
use stem::kprintln;

pub fn enumerate(ctx: &DevTreeCtx) -> Result<(), ()> {
    kprintln!("SPROUT: Enumerating x86_64 platform...");
    
    // 1. CMOS RTC (Legacy Port 0x70)
    let rtc = thingsys::create_node("dev.rtc.cmos").map_err(|_| ())?;
    thingsys::link(ctx.platform_bus, "HAS_DEVICE", rtc).map_err(|_| ())?;
    
    // Add Resources
    let res = thingsys::create_node("res.io.port_range").map_err(|_| ())?;
    thingsys::prop_set(res, "base", 0x70).map_err(|_| ())?;
    thingsys::prop_set(res, "size", 2).map_err(|_| ())?;
    thingsys::link(rtc, "HAS_RESOURCE", res).map_err(|_| ())?;
    
    // 2. Platform Profile
    // If ACPI is present, mark as acpi-aware
    if ctx.acpi_rsdp.is_some() {
        // Can't set string prop "platform_profile"="acpi" easily without interning or string props.
        // For v0.1, we only have u64 values.
        // We can intern "acpi" and set it as value? 
        // prop_set takes u64.
        // Let's assume intern returns u32, cast to u64.
        let val = thingsys::intern("acpi").unwrap_or(0);
        thingsys::prop_set(ctx.host, "platform_profile", val as u64).map_err(|_| ())?;
    } else {
        let val = thingsys::intern("legacy_pc").unwrap_or(0);
        thingsys::prop_set(ctx.host, "platform_profile", val as u64).map_err(|_| ())?;
    }
    
    Ok(())
}
