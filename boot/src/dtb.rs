use alloc::boxed::Box;
use alloc::format;
use core::sync::atomic::Ordering;
use limine::request::DeviceTreeBlobRequest;

#[used]
#[unsafe(link_section = ".requests")]
static DTB_REQUEST: DeviceTreeBlobRequest = DeviceTreeBlobRequest::new();

#[cfg(target_arch = "riscv64")]
pub fn seed_dtb_frequency() {
    let Some(response) = DTB_REQUEST.get_response() else {
        kernel::log("No DTB provided by Limine");
        return;
    };

    let dtb_ptr = response.dtb_ptr();
    if dtb_ptr.is_null() {
        kernel::log("DTB pointer is null");
        return;
    }

    // Safety: Limine guarantees dtb_ptr is valid if provided.
    // We trust the bootloader.
    let fdt = match unsafe { fdt::Fdt::from_ptr(dtb_ptr as *const u8) } {
        Ok(f) => f,
        Err(e) => {
             kernel::log("Failed to parse DTB");
             return;
        }
    };

    let mut found = false;
    for cpu in fdt.cpus() {
        let freq = cpu.timebase_frequency();
        if freq > 0 {
            let freq = freq as u64;
            kernel::log(Box::leak(format!("DTB: Found timebase-frequency: {}", freq).into_boxed_str()));

            // Set the frequency in arch::riscv64::time::FREQUENCY
            arch::riscv64::time::FREQUENCY.store(freq, Ordering::Relaxed);
            found = true;
            break; // Assuming all CPUs have same timebase
        }
    }

    if !found {
        // Fallback to checking /cpus node directly if iterator misses it or if it is property of /cpus
        // fdt crate's cpus() iterates over /cpus/cpu@* nodes.
        // Sometimes timebase-frequency is in /cpus node itself.
        if let Some(cpus_node) = fdt.find_node("/cpus") {
             if let Some(prop) = cpus_node.property("timebase-frequency") {
                 let freq = prop.as_usize().unwrap_or(0) as u64;
                 if freq > 0 {
                     kernel::log(Box::leak(format!("DTB: Found /cpus/timebase-frequency: {}", freq).into_boxed_str()));
                     arch::riscv64::time::FREQUENCY.store(freq, Ordering::Relaxed);
                     found = true;
                 }
             }
        }
    }

    if !found {
        kernel::log("DTB: timebase-frequency not found, using default");
    }
}

#[cfg(not(target_arch = "riscv64"))]
pub fn seed_dtb_frequency() {
    // No-op for other architectures
}
