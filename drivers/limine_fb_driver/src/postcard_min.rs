// Minimal Postcard encoder for FbGetInfoResp
// Struct order: width (u32), height (u32), stride (u32), format (u32), addr (u64), size (u64)

pub struct FbGetInfoResp {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
    pub addr: u64,
    pub size: u64,
}

pub fn encode_varint_u64(mut n: u64, out: &mut [u8]) -> usize {
    let mut i = 0;
    loop {
        let mut b = (n & 0x7F) as u8;
        n >>= 7;
        if n != 0 {
            b |= 0x80;
        }
        if i < out.len() {
            out[i] = b;
            i += 1;
        } else {
            return 0; // Error
        }
        if n == 0 {
            break;
        }
    }
    i
}

pub fn encode_varint_u32(n: u32, out: &mut [u8]) -> usize {
    encode_varint_u64(n as u64, out)
}

pub fn encode_fb_info(info: &FbGetInfoResp, out: &mut [u8]) -> usize {
    let mut pos = 0;

    // width
    pos += encode_varint_u32(info.width, &mut out[pos..]);
    // height
    pos += encode_varint_u32(info.height, &mut out[pos..]);
    // stride
    pos += encode_varint_u32(info.stride, &mut out[pos..]);
    // format
    pos += encode_varint_u32(info.format, &mut out[pos..]);
    // addr
    pos += encode_varint_u64(info.addr, &mut out[pos..]);
    // size
    pos += encode_varint_u64(info.size, &mut out[pos..]);

    pos
}
