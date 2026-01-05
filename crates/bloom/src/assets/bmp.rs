#[derive(Clone, Copy)]
pub struct Wallpaper {
    pub data_ptr: *const u8,
    pub width: u32,
    pub height: u32,
    pub row_stride: usize,
    pub bytes_per_pixel: usize,
    pub bottom_up: bool,
}

pub fn parse_bmp(buf: &[u8]) -> Option<Wallpaper> {
    if buf.len() < 54 || &buf[0..2] != b"BM" {
        return None;
    }
    let data_offset = u32::from_le_bytes([buf[10], buf[11], buf[12], buf[13]]) as usize;
    let width_i = i32::from_le_bytes([buf[18], buf[19], buf[20], buf[21]]);
    let height_i = i32::from_le_bytes([buf[22], buf[23], buf[24], buf[25]]);
    let bpp = u16::from_le_bytes([buf[28], buf[29]]);
    if width_i <= 0 {
        return None;
    }
    let bytes_per_pixel = ((bpp as usize) + 7) / 8;
    let row_stride = (((width_i as usize * bytes_per_pixel) + 3) / 4) * 4;
    
    // Safety check? buf bounds.
    let end_offset = data_offset + (height_i.abs() as usize * row_stride);
    if end_offset > buf.len() {
         return None;
    }

    let data_ptr = unsafe { buf.as_ptr().add(data_offset) };
    Some(Wallpaper {
        data_ptr,
        width: width_i.abs() as u32,
        height: height_i.abs() as u32,
        row_stride,
        bytes_per_pixel,
        bottom_up: height_i > 0,
    })
}
