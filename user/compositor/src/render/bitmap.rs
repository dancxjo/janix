use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub struct Bitmap {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>, // Premultiplied ARGB
}

impl Bitmap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: alloc::vec![0u32; width * height],
        }
    }
}
