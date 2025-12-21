use alloc::boxed::Box;
use alloc::format;
use limine::request::DeviceTreeBlobRequest;

#[used]
#[unsafe(link_section = ".requests")]
static DTB_REQUEST: DeviceTreeBlobRequest = DeviceTreeBlobRequest::new();

pub fn init(override_freq: Option<u64>) {
    if let Some(freq) = override_freq {
        super::time::set_frequency(freq);
        kernel::log(Box::leak(format!("RISC-V Timer Frequency: {} Hz (forced)", freq).into_boxed_str()));
        // Do not return early, as DTB might contain other info.
        // However, we should ensure we don't overwrite the forced frequency.
    }

    let Some(response) = DTB_REQUEST.get_response() else {
        kernel::log("No DTB provided by Limine");
        return;
    };

    let dtb_ptr = response.dtb_ptr();
    if dtb_ptr.is_null() {
        kernel::log("DTB pointer is null");
        return;
    }

    // Pass to time module to parse and set frequency
    // Only if override was not set?
    if override_freq.is_none() {
        unsafe {
            super::time::init_frequency_from_dtb(dtb_ptr as *const u8);
        }
    }

    // Log the frequency
    let freq = super::time::FREQUENCY.load(core::sync::atomic::Ordering::Relaxed);
    kernel::log(Box::leak(format!("RISC-V Timer Frequency: {} Hz", freq).into_boxed_str()));
}
