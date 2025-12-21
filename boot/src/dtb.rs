use alloc::boxed::Box;
use alloc::format;
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

    // Pass to arch
    unsafe {
        arch::riscv64::time::init_frequency_from_dtb(dtb_ptr as *const u8);
    }

    // Log the frequency
    let freq = arch::riscv64::time::FREQUENCY.load(core::sync::atomic::Ordering::Relaxed);
    kernel::log(Box::leak(format!("RISC-V Timer Frequency: {} Hz", freq).into_boxed_str()));
}

#[cfg(not(target_arch = "riscv64"))]
pub fn seed_dtb_frequency() {
    // No-op for other architectures
}
