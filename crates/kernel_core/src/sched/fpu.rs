#[repr(align(16))]
#[derive(Copy, Clone, Debug)]
pub struct FpuContext {
    pub data: [u8; 512],
}

impl Default for FpuContext {
    fn default() -> Self {
        let mut data = [0u8; 512];
        // FCW = 0x037F
        data[0] = 0x7F;
        data[1] = 0x03;
        // MXCSR = 0x1F80
        data[24] = 0x80;
        data[25] = 0x1F;
        Self { data }
    }
}
