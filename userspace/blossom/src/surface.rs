pub type MappedSurface = Surface;

pub struct Surface {
    pub ptr: *mut u8,
    pub len: usize,
    pub width: i32,
    pub height: i32,
    pub stride_bytes: usize,
}

impl Surface {
    pub unsafe fn new(
        ptr: *mut u8,
        len: usize,
        width: u32,
        height: u32,
        stride_bytes: u32,
    ) -> Self {
        Self {
            ptr,
            len,
            width: width as i32,
            height: height as i32,
            stride_bytes: stride_bytes as usize,
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    #[allow(dead_code)]
    pub fn stride_bytes(&self) -> usize {
        self.stride_bytes
    }

    pub fn put_px(&mut self, x: i32, y: i32, xrgb: u32) {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return;
        }
        let offset = y as usize * self.stride_bytes + x as usize * 4;
        if offset + 4 > self.len {
            return;
        }
        let bytes = xrgb.to_le_bytes();
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), self.ptr.add(offset), 4);
        }
    }

    pub fn get_px(&self, x: i32, y: i32) -> u32 {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return 0;
        }
        let offset = y as usize * self.stride_bytes + x as usize * 4;
        if offset + 4 > self.len {
            return 0;
        }
        unsafe {
            let mut bytes = [0u8; 4];
            core::ptr::copy_nonoverlapping(self.ptr.add(offset), bytes.as_mut_ptr(), 4);
            u32::from_le_bytes(bytes)
        }
    }

    pub fn clear(&mut self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.put_px(x, y, color);
            }
        }
    }
}
