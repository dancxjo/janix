use super::DevTreeCtx;
use abi::schema::{confidence, keys, kinds, rels, source};
use stem::info;
use stem::thing::sys as thingsys;

#[allow(dead_code)]
pub fn enumerate(ctx: &DevTreeCtx) -> Result<(), ()> {
    info!("SPROUT: x86_64 platform enrichment... (v0.2)");

    // 1. CMOS RTC (Platform Contract)
    // Always present at 0x70 on PC-compatible hardware

    // Check if it already exists
    let mut buf = [stem::thing::ThingId::default(); 1];
    if let Ok(0) = thingsys::find(kinds::DEV_RTC_CMOS, &mut buf) {
        if let Ok(rtc) = thingsys::create_node(kinds::DEV_RTC_CMOS) {
            info!("SPROUT: Created dev.rtc.cmos (platform contract)");

            // Link to platform bus
            info!("SPROUT: RTC: linking to platform bus...");
            let _ = thingsys::link(ctx.platform_bus, rels::HAS_DEVICE, rtc);
            info!("SPROUT: RTC: link done");

            // Provenance (Numeric)
            info!("SPROUT: RTC: setting props...");
            let _ = thingsys::prop_set(rtc, keys::SOURCE, source::PLATFORM as u64);
            let _ = thingsys::prop_set(rtc, keys::CONFIDENCE, confidence::MEDIUM as u64);
            info!("SPROUT: RTC: props done");

            // Resources: IO Ports 0x70-0x71
            info!("SPROUT: RTC: creating IO port resource...");
            if let Ok(res) = thingsys::create_node(kinds::RES_IO_PORT_RANGE) {
                info!("SPROUT: RTC: IO port resource created");
                let _ = thingsys::prop_set(res, keys::START, 0x70);
                let _ = thingsys::prop_set(res, keys::SIZE_BYTES, 2);
                let _ = thingsys::prop_set(res, keys::SOURCE, source::PLATFORM as u64);
                let _ = thingsys::prop_set(res, keys::CONFIDENCE, confidence::MEDIUM as u64);
                let _ = thingsys::link(rtc, rels::HAS_RESOURCE, res);
                info!("SPROUT: RTC: IO port resource linked");
            }
        }
    }

    info!("SPROUT: x86_64 enumerate done");
    Ok(())
}
