#[cfg(feature = "fill-framebuffer")]
pub fn fill_framebuffer_with_color() {
    const COLOR: u32 = 0x00_00_FF_00;

    let response = match crate::FRAMEBUFFER_REQUEST.get_response() {
        Some(resp) => resp,
        None => {
                    kernel_core::log("Framebuffer fill skipped: no Limine framebuffer response");
            return;
        }
    };

    let framebuffer = match response.framebuffers().next() {
        Some(fb) => fb,
        None => {
            kernel_core::log("Framebuffer fill skipped: response has no framebuffers");
            return;
        }
    };

    let width = framebuffer.width() as usize;
    let height = framebuffer.height() as usize;
    let pitch = framebuffer.pitch() as usize;
    if width == 0 || height == 0 {
        kernel_core::log("Framebuffer fill skipped: zero dimensions");
        return;
    }

    let stride = pitch / 4;
    if stride < width {
        kernel_core::log("Framebuffer fill skipped: pitch less than width");
        return;
    }

    let hhdm_offset = crate::boot_model::HHDM_REQUEST
        .get_response()
        .map(|resp| resp.offset())
        .unwrap_or(0);
    let fb_phys = framebuffer.addr() as u64;
    let fb_virt = match virtual_framebuffer_address(fb_phys, hhdm_offset) {
        Some(addr) => addr,
        None => {
            kernel_core::log("Framebuffer fill skipped: could not determine virtual pointer");
            return;
        }
    };

    kernel_core::println!(
        "Framebuffer fill: phys={:#x} hhdm={:#x} virt={:#x} pitch={} width={} height={}",
        fb_phys,
        hhdm_offset,
        fb_virt,
        pitch,
        width,
        height
    );

    let ptr = fb_virt as *mut u32;

    unsafe {
        for y in 0..height {
            for x in 0..width {
                let pixel = ptr.add(y * stride + x);
                core::ptr::write_volatile(pixel, COLOR);
            }
        }
    }
    kernel_core::log("Framebuffer filled with solid green temporarily");
}

pub fn virtual_framebuffer_address(guest_addr: u64, hhdm_offset: u64) -> Option<u64> {
    if is_canonical_address(guest_addr) {
        return Some(guest_addr);
    }

    if hhdm_offset == 0 {
        return None;
    }

    let translated = guest_addr.wrapping_add(hhdm_offset);
    if is_canonical_address(translated) {
        Some(translated)
    } else {
        None
    }
}

pub fn is_canonical_address(addr: u64) -> bool {
    addr <= 0x0000_7fff_ffff_ffff || addr >= 0xffff_8000_0000_0000
}
