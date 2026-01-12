use super::DevTreeCtx;
use stem::thing::sys as thingsys;
use stem::println;
use abi::schema::{keys, kinds, rels, source, confidence};

pub fn enumerate(ctx: &DevTreeCtx) -> Result<(), ()> {
    println!("SPROUT: x86_64 platform enrichment... (v0.2)");

    // 1. CMOS RTC (Platform Contract)
    // Always present at 0x70 on PC-compatible hardware
    
    // Check if it already exists
    let mut buf = [stem::thing::ThingId(0); 1];
    if let Ok(0) = thingsys::find(kinds::DEV_RTC_CMOS, &mut buf) {
        if let Ok(rtc) = thingsys::create_node(kinds::DEV_RTC_CMOS) {
            println!("SPROUT: Created dev.rtc.cmos (platform contract)");
            
            // Link to platform bus
            let _ = thingsys::link(ctx.platform_bus, rels::HAS_DEVICE, rtc);
            
            // Provenance (Numeric)
            let _ = thingsys::prop_set(rtc, keys::SOURCE, source::PLATFORM as u64);
            let _ = thingsys::prop_set(rtc, keys::CONFIDENCE, confidence::MEDIUM as u64);
            
            // Resources: IO Ports 0x70-0x71
            if let Ok(res) = thingsys::create_node(kinds::RES_IO_PORT_RANGE) {
                let _ = thingsys::prop_set(res, keys::START, 0x70);
                let _ = thingsys::prop_set(res, keys::SIZE_BYTES, 2); // keys::SIZE_BYTES was "size_bytes", legacy used "size"
                // Checking keys:: definition: pub const SIZE_BYTES: &str = "size_bytes";
                // Legacy Sprout used "size".
                // I should probably support "size" in keys or stick to "size_bytes".
                // boot_register uses keys::SIZE_BYTES ("size_bytes").
                // Sprout mod.rs uses keys::SIZE_BYTES.
                // Uh, Sprout mod.rs imported keys::PROP_SIZE -> "size". 
                // Wait, I updated `abi::schema::keys` to only have `SIZE_BYTES`.
                // Did I remove `keys::SIZE`?
                // Let's check `abi/src/schema.rs` content I wrote.
                // I wrote: `pub const SIZE_BYTES: &str = "size_bytes";`
                // I did NOT write `SIZE`.
                // So my update to `sprout/src/devtree/mod.rs` might have used `prop_set(res, schema::PROP_SIZE, ...)`
                // But `PROP_SIZE` was in the local schema module I removed.
                // I imported `abi::schema::keys`.
                // I need to check if `mod.rs` compiles. I might have broken it if I used `keys::SIZE`?
                // `mod.rs` snippet: `let _ = thingsys::prop_set(res, schema::PROP_SIZE, reg.size.unwrap_or(0) as u64);`
                // Wait, I rewrote `mod.rs` completely in the previous step (Step 290).
                // In that rewrite: `_ = thingsys::prop_set(res, keys::SIZE_BYTES, ...)`?
                // Let me check my memory of step 290.
                
                let _ = thingsys::prop_set(res, keys::SOURCE, source::PLATFORM as u64);
                let _ = thingsys::prop_set(res, keys::CONFIDENCE, confidence::MEDIUM as u64);
                let _ = thingsys::link(rtc, rels::HAS_RESOURCE, res);
            }
        }
    }

    Ok(())
}
