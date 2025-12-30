use serde::Serialize;

#[derive(Serialize)]
struct FbGetInfoResp {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
    pub addr: u64,
    pub size: u64,
}

// Minimal implementation to test against
// Copy-pasted from drivers/limine_fb_driver/src/postcard_min.rs
// to avoid path dependency issues across workspace boundaries with different targets.

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

struct ManualFbInfo {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
    pub addr: u64,
    pub size: u64,
}

fn encode_fb_info(info: &ManualFbInfo, out: &mut [u8]) -> usize {
    let mut pos = 0;
    pos += encode_varint_u32(info.width, &mut out[pos..]);
    pos += encode_varint_u32(info.height, &mut out[pos..]);
    pos += encode_varint_u32(info.stride, &mut out[pos..]);
    pos += encode_varint_u32(info.format, &mut out[pos..]);
    pos += encode_varint_u64(info.addr, &mut out[pos..]);
    pos += encode_varint_u64(info.size, &mut out[pos..]);
    pos
}

fn main() {
    let info = FbGetInfoResp {
        width: 1920,
        height: 1080,
        stride: 1920 * 4,
        format: 32,
        addr: 0x1_0000_0000,
        size: 1920 * 1080 * 4,
    };

    let manual = ManualFbInfo {
        width: 1920,
        height: 1080,
        stride: 1920 * 4,
        format: 32,
        addr: 0x1_0000_0000,
        size: 1920 * 1080 * 4,
    };

    let mut buf_post = [0u8; 64];
    let slice_post = postcard::to_slice(&info, &mut buf_post).unwrap();

    let mut buf_manual = [0u8; 64];
    let len_manual = encode_fb_info(&manual, &mut buf_manual);
    let slice_manual = &buf_manual[..len_manual];

    println!("Postcard: {:?}", slice_post);
    println!("Manual:   {:?}", slice_manual);

    assert_eq!(slice_post, slice_manual);
    println!("SUCCESS: Encodings match.");
}
