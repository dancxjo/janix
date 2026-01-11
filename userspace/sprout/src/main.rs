#![no_std]
#![no_main]

extern crate alloc;
use stem::kprintln;
use stem::thing::sys as thingsys;
use stem::abi::driver_ctx::DriverCtx;

mod devtree;
mod registry;

#[no_mangle]
pub extern "C" fn main(_arg0: usize) {
    kprintln!("SPROUT: v0.3 starting (Driver Matchmaking)...");
    
    match devtree::init() {
        Ok(ctx) => {
             if let Err(_) = devtree::build(&ctx) {
                 kprintln!("SPROUT: Failed to build device tree!");
             }
        },
        Err(_) => {
            kprintln!("SPROUT: Failed to initialize devtree context! (continuing)");
        }
    }

    let mut reg = registry::Registry::new();
    reg.scan();

    let mut buf = [stem::thing::ThingId(0); 1];
    if let Ok(1) = thingsys::find(stem::abi::schema::kinds::DEV_RTC_CMOS, &mut buf) {
        let rtc_id = buf[0];
        kprintln!("SPROUT: Found RTC device: {:?}", rtc_id);
        
        if let Some(driver_name) = reg.find_driver("dev.rtc.cmos") {
             kprintln!("SPROUT: Launching driver '{}' for dev.rtc.cmos...", driver_name);
             
             let ctx = DriverCtx { device_id: stem::abi::types::ThingId(rtc_id.0) };
             let arg = ctx.to_raw();
             
             match stem::syscall::spawn_process(driver_name, arg) {
                 Ok(pid) => kprintln!("SPROUT: Driver launched (PID={})", pid),
                 Err(e) => kprintln!("SPROUT: Failed to launch driver: {:?}", e),
             }
        } else {
             kprintln!("SPROUT: No matched driver for RTC.");
        }
    } else {
        kprintln!("SPROUT: RTC device not found in graph!");
    }

    kprintln!("SPROUT: Launching clock demo...");
    if let Err(e) = stem::syscall::spawn_process("clock", 0) {
        kprintln!("SPROUT: Failed to launch clock: {:?}", e);
    }

    // kprintln!("SPROUT: Dumping Root graph...");
    // let _ = thingsys::dump_graph(4096);
    
    kprintln!("SPROUT: Done.");
    stem::syscall::exit(0);
}
