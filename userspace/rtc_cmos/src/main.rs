#![no_std]
#![no_main]

extern crate alloc;
use stem::kprintln;
use stem::abi::module_manifest::{ManifestHeader, ModuleKind, MANIFEST_MAGIC};
use stem::abi::driver_ctx::DriverCtx;
use stem::thing::{sys as thingsys};

#[link_section = ".thing_manifest"]
#[no_mangle]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Driver,
    device_kind: [
        0x64, 0x65, 0x76, 0x2e, 0x72, 0x74, 0x63, 0x2e, 0x63, 0x6d, 0x6f, 0x73, 0x00, 
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ],
    version: 1,
    _reserved: 0,
};

#[no_mangle]
pub extern "C" fn main(arg: usize) {
    kprintln!("[rtc_cmos] Starting... arg={:x}", arg);

    if arg == 0 {
         kprintln!("[rtc_cmos] Error: No context provided (arg=0).");
         stem::syscall::exit(1);
    }
    
    let ctx = DriverCtx::from_raw(arg);
    let dev_id = stem::thing::ThingId(ctx.device_id.0); // Convert abi::ThingId to stem::ThingId

    kprintln!("[rtc_cmos] Serving device ID: {:?}", dev_id);
    
    // 1. Create time.source
    let src = match thingsys::create_node("time.source") {
        Ok(id) => id,
        Err(e) => {
            kprintln!("[rtc_cmos] Failed to create time.source: {:?}", e);
            stem::syscall::exit(1);
            unreachable!();
        }
    };
    
    if let Err(e) = thingsys::link(dev_id, "provides", src) {
        kprintln!("[rtc_cmos] Failed to link provides: {:?}", e);
    }
    
    // 2. Create time.instant
    let instant = match thingsys::create_node("time.instant") {
        Ok(id) => id,
        Err(e) => {
             kprintln!("[rtc_cmos] Failed to create time.instant: {:?}", e);
             return;
        }
    };
    
    if let Err(e) = thingsys::link(src, "current", instant) {
        kprintln!("[rtc_cmos] Failed to link current: {:?}", e);
    }
    
    thingsys::prop_set(instant, "seconds", 0).ok();
    
    kprintln!("[rtc_cmos] Publishing time. ticks started.");
    
    let mut ticks = 0;
    loop {
        stem::sleep(core::time::Duration::from_secs(1));
        ticks += 1;
        thingsys::prop_set(instant, "seconds", ticks).ok();
        // kprintln!("[rtc_cmos] Tick {}", ticks);
    }
}
