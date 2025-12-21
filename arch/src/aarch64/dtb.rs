use limine::request::DeviceTreeBlobRequest;

#[used]
#[unsafe(link_section = ".requests")]
static DTB_REQUEST: DeviceTreeBlobRequest = DeviceTreeBlobRequest::new();

pub fn get_pl031_address() -> Option<u64> {
    let response = DTB_REQUEST.get_response()?;
    let dtb_ptr = response.dtb_ptr();
    if dtb_ptr.is_null() {
        return None;
    }

    // Safety: The pointer is provided by Limine and is valid for the duration of the boot.
    let fdt = unsafe { fdt::Fdt::from_ptr(dtb_ptr as *const u8).ok()? };

    for node in fdt.all_nodes() {
        if let Some(compat) = node.compatible() {
            if compat.all().any(|s| s == "arm,pl031") {
                if let Some(reg) = node.reg()?.next() {
                    return Some(reg.starting_address as u64);
                }
            }
        }
    }
    None
}
