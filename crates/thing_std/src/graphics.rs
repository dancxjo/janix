use super::*;

pub fn surface_create(w: u32, h: u32, format: u32) -> ThingId {
    let res = unsafe {
        syscall(
            nr::SYS_SURFACE_CREATE,
            w as u64,
            h as u64,
            format as u64,
            0,
            0,
            0,
        )
    };
    ThingId::from_parts(res.val0, res.val1)
}

pub fn surface_draw(id: ThingId, buf: &[u8], x: u32, y: u32, w: u32, _h: u32) {
    let id_lo = id.low();
    let id_hi = id.high();
    unsafe {
        syscall(
            nr::SYS_SURFACE_DRAW,
            id_lo,
            id_hi,
            buf.as_ptr() as u64,
            x as u64,
            y as u64,
            w as u64,
        )
    };
}
