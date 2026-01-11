use super::DevTreeCtx;
use stem::thing::sys as thingsys;
use stem::kprintln;
use super::{schema, set_str_prop};

pub fn enumerate(ctx: &DevTreeCtx) -> Result<(), ()> {
    kprintln!("SPROUT: x86_64 platform enrichment...");

    // 1. CMOS RTC (Platform Contract)
    // Always present at 0x70 on PC-compatible hardware
    
    // Check if it already exists
    let mut buf = [stem::thing::ThingId(0); 1];
    if let Ok(0) = thingsys::find(schema::DEV_RTC_CMOS, &mut buf) {
        if let Ok(rtc) = thingsys::create_node(schema::DEV_RTC_CMOS) {
            kprintln!("SPROUT: Created dev.rtc.cmos (platform contract)");
            
            // Link to platform bus
            let _ = thingsys::link(ctx.platform_bus, schema::HAS_DEVICE, rtc);
            
            // Provenance
            let _ = set_str_prop(rtc, schema::PROP_SOURCE, schema::SRC_PLATFORM);
            let _ = set_str_prop(rtc, schema::PROP_CONFIDENCE, schema::CONFIDENCE_MEDIUM);
            
            // Resources: IO Ports 0x70-0x71
            if let Ok(res) = thingsys::create_node(schema::RES_IO_PORT_RANGE) {
                let _ = thingsys::prop_set(res, schema::PROP_START, 0x70);
                let _ = thingsys::prop_set(res, schema::PROP_SIZE, 2);
                let _ = set_str_prop(res, schema::PROP_SOURCE, schema::SRC_PLATFORM);
                let _ = set_str_prop(res, schema::PROP_CONFIDENCE, schema::CONFIDENCE_MEDIUM);
                let _ = thingsys::link(rtc, schema::HAS_RESOURCE, res);
            }
        }
    }

    Ok(())
}
