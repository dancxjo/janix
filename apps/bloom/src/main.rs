#![no_std]
#![no_main]

extern crate alloc;
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

            if let Some(asset_bs_id) = thing_find("bytespace.asset.clouds.bmp") {
                if let Some(wp) = load_bmp(asset_bs_id, 0x8100_0000) {
                    let fb_ptr = fb_base as *mut u32;
                    
                    unsafe {
                        if let Some(ref buf) = BACK_BUFFER {
                            let back = buf.as_ptr() as *mut u32;
                            
                            render_wallpaper(back, width, height, &wp);
                            fast_memcpy(fb_ptr as *mut u8, back as *const u8, buffer_size * 4);
                            
                            log_info("BLOOM: done");
                        }
                    }
                }
            }
            
            loop { sched_yield(); }
        }
        sched_yield();
    }
}

/// Fast memory copy using rep movsb (very efficient on modern x86)
#[cfg(target_arch = "x86_64")]
#[inline(always)]
unsafe fn fast_memcpy(dest: *mut u8, src: *const u8, len: usize) {
    core::arch::asm!(
        "rep movsb",
        inout("rdi") dest => _,
        inout("rsi") src => _,
        inout("rcx") len => _,
        options(nostack, preserves_flags)
    );
}

#[cfg(not(target_arch = "x86_64"))]
#[inline(always)]
unsafe fn fast_memcpy(dest: *mut u8, src: *const u8, len: usize) {
    core::ptr::copy_nonoverlapping(src, dest, len);
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
    let width_i = i32::from_le_bytes([buf[18], buf[19], buf[20], buf[21]]);
    let height_i = i32::from_le_bytes([buf[22], buf[23], buf[24], buf[25]]);
    let bpp = u16::from_le_bytes([buf[28], buf[29]]);
    
    if width_i <= 0 { return None; }
    
    let bytes_per_pixel = (bpp as usize + 7) / 8;
    let row_stride = ((width_i as usize * bytes_per_pixel + 3) / 4) * 4;
    let data_ptr = unsafe { (vaddr as *const u8).add(data_offset) };
    
    Some(Wallpaper {
        data_ptr,
        width: width_i.abs() as u32,
        height: height_i.abs() as u32,
        row_stride,
        bytes_per_pixel,
        bottom_up: height_i > 0,
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
                
                let b = *src_ptr;
                let g = *src_ptr.add(1);
                let r = *src_ptr.add(2);
                
                let pixel = 0xFF000000u32 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                *dest_row.add(x as usize) = pixel;
            }
        }
    }
}
