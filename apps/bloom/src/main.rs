#![no_std]
#![no_main]

extern crate alloc;
use alloc::format;
use alloc::vec::Vec;
use thing_std::graph::*;
use thing_std::*;

static mut BACK_BUFFER: Option<Vec<u32>> = None;

#[no_mangle]
pub extern "C" fn main() {
    thing_std::init(0);
    log_info("BLOOM: alive");

    loop {
        if let Some(display_id) = thing_find("device.display0") {
            log_info("BLOOM: found display");

            let mut buf = [0u8; 20];
            let len = thing_std::graph::thing_get_payload(display_id, &mut buf);
            
            let (width, height) = if len >= 8 {
                let w = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
                let h = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
                (w, h)
            } else {
                (1280u32, 720u32)
            };

            let fb_base = 0xA000_0000u64;
            let fb_size: u64 = (width as u64) * (height as u64) * 4;
            
            let bs_id = thing_find("bytespace.display0").expect("bytespace not found");
            let _mapped = thing_std::memory::space_map(bs_id, fb_base, 0, fb_size);

            let buffer_size = (width * height) as usize;
            unsafe {
                BACK_BUFFER = Some(alloc::vec![0u32; buffer_size]);
            }

            let simd_level = detect_simd();

            let mut wallpaper: Option<Wallpaper> = None;
            
            if let Some(asset_bs_id) = thing_find("bytespace.asset.clouds.bmp") {
                if let Some(wp) = load_bmp(asset_bs_id, 0x8100_0000) {
                    // Debug: log first few pixels from BMP
                    unsafe {
                        let p0 = wp.data_ptr;
                        let b0 = *p0;
                        let g0 = *p0.add(1);
                        let r0 = *p0.add(2);
                        let msg = format!("BLOOM: pixel0 BGR: {:02x} {:02x} {:02x}", b0, g0, r0);
                        log_info(&msg);
                    }
                    wallpaper = Some(wp);
                }
            }
            
            let fb_ptr = fb_base as *mut u32;
            
            unsafe {
                if let Some(ref buf) = BACK_BUFFER {
                    let back = buf.as_ptr() as *mut u32;
                    
                    if let Some(ref wp) = wallpaper {
                        render_wallpaper(back, width, height, wp);
                        
                        // Debug: log first pixel after render
                        let p = *back;
                        let msg = format!("BLOOM: rendered pixel0: {:08x}", p);
                        log_info(&msg);
                        
                        simd_blit(fb_ptr, back, buffer_size, simd_level);
                        log_info("BLOOM: done");
                    }
                }
            }
            
            loop { sched_yield(); }
        }
        sched_yield();
    }
}

fn detect_simd() -> u32 {
    #[cfg(target_arch = "x86_64")]
    {
        let mut level = 0u32;
        unsafe {
            let ecx: u32;
            let edx: u32;
            core::arch::asm!(
                "push rbx", "mov eax, 1", "cpuid", "pop rbx",
                out("ecx") ecx, out("edx") edx, out("eax") _,
                options(nostack),
            );
            if edx & (1 << 26) != 0 { level = 1; }
            if ecx & (1 << 28) != 0 && ecx & (1 << 27) != 0 {
                let xcr0_lo: u32;
                core::arch::asm!(
                    "xgetbv", in("ecx") 0u32, out("eax") xcr0_lo, out("edx") _, options(nostack),
                );
                if xcr0_lo & 0x6 == 0x6 { level = 2; }
            }
        }
        level
    }
    #[cfg(not(target_arch = "x86_64"))]
    { 0 }
}

struct Wallpaper {
    data_ptr: *const u8,
    width: u32,
    height: u32,
    row_stride: usize,
    bytes_per_pixel: usize,
    bottom_up: bool,
}

fn load_bmp(bs_id: ThingId, vaddr: u64) -> Option<Wallpaper> {
    let len: u64 = 4 * 1024 * 1024;
    thing_std::memory::space_map(bs_id, vaddr, 0, len);
    let buf = unsafe { core::slice::from_raw_parts(vaddr as *const u8, len as usize) };
    
    if buf.len() < 54 || &buf[0..2] != b"BM" { return None; }
    
    let data_offset = u32::from_le_bytes([buf[10], buf[11], buf[12], buf[13]]) as usize;
    let width = i32::from_le_bytes([buf[18], buf[19], buf[20], buf[21]]);
    let height = i32::from_le_bytes([buf[22], buf[23], buf[24], buf[25]]);
    let bpp = u16::from_le_bytes([buf[28], buf[29]]);
    
    log_info(&format!("BLOOM: BMP {}x{} bpp={} offset={}", width, height, bpp, data_offset));
    
    if width <= 0 { return None; }
    
    let bytes_per_pixel = (bpp as usize + 7) / 8;
    let row_stride = ((width as usize * bytes_per_pixel + 3) / 4) * 4;
    let data_ptr = unsafe { (vaddr as *const u8).add(data_offset) };
    
    Some(Wallpaper {
        data_ptr,
        width: width.abs() as u32,
        height: height.abs() as u32,
        row_stride,
        bytes_per_pixel,
        bottom_up: height > 0,
    })
}

fn render_wallpaper(dest: *mut u32, dest_w: u32, dest_h: u32, wp: &Wallpaper) {
    for y in 0..dest_h {
        let src_y = y % wp.height;
        let actual_src_y = if wp.bottom_up {
            wp.height - 1 - src_y
        } else {
            src_y
        };
        
        let row_ptr = unsafe { wp.data_ptr.add(actual_src_y as usize * wp.row_stride) };
        let dest_row = unsafe { dest.add((y * dest_w) as usize) };
        
        unsafe {
            for x in 0..dest_w {
                let src_x = x % wp.width;
                let src_ptr = row_ptr.add(src_x as usize * wp.bytes_per_pixel);
                
                // BMP 24-bit: B=byte0, G=byte1, R=byte2
                let b = *src_ptr;
                let g = *src_ptr.add(1);
                let r = *src_ptr.add(2);
                
                // XRGB format: 0xFFRRGGBB
                let pixel = 0xFF000000u32 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                
                *dest_row.add(x as usize) = pixel;
            }
        }
    }
}

fn simd_blit(dest: *mut u32, src: *const u32, count: usize, simd_level: u32) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        match simd_level {
            2 => blit_avx(dest, src, count),
            1 => blit_sse2(dest, src, count),
            _ => blit_scalar(dest, src, count),
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    unsafe { blit_scalar(dest, src, count); }
}

#[cfg(target_arch = "x86_64")]
unsafe fn blit_sse2(dest: *mut u32, src: *const u32, count: usize) {
    use core::arch::x86_64::*;
    let dest128 = dest as *mut __m128i;
    let src128 = src as *const __m128i;
    let simd_count = count / 4;
    for i in 0..simd_count {
        _mm_storeu_si128(dest128.add(i), _mm_loadu_si128(src128.add(i)));
    }
    let base = simd_count * 4;
    for i in 0..(count % 4) {
        core::ptr::write_volatile(dest.add(base + i), *src.add(base + i));
    }
}

#[cfg(target_arch = "x86_64")]
unsafe fn blit_avx(dest: *mut u32, src: *const u32, count: usize) {
    use core::arch::x86_64::*;
    let dest256 = dest as *mut __m256i;
    let src256 = src as *const __m256i;
    let simd_count = count / 8;
    for i in 0..simd_count {
        _mm256_storeu_si256(dest256.add(i), _mm256_loadu_si256(src256.add(i)));
    }
    let base = simd_count * 8;
    let rem = count % 8;
    if rem >= 4 {
        let d = dest.add(base) as *mut __m128i;
        let s = src.add(base) as *const __m128i;
        _mm_storeu_si128(d, _mm_loadu_si128(s));
        for i in 4..rem { core::ptr::write_volatile(dest.add(base + i), *src.add(base + i)); }
    } else {
        for i in 0..rem { core::ptr::write_volatile(dest.add(base + i), *src.add(base + i)); }
    }
}

unsafe fn blit_scalar(dest: *mut u32, src: *const u32, count: usize) {
    let d64 = dest as *mut u64;
    let s64 = src as *const u64;
    for i in 0..(count / 2) { core::ptr::write_volatile(d64.add(i), *s64.add(i)); }
    if count % 2 == 1 { core::ptr::write_volatile(dest.add(count - 1), *src.add(count - 1)); }
}
