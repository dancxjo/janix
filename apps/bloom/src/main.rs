#![no_std]
#![no_main]

extern crate alloc;

use thing_std::console_write;
use thing_std::abi::ids::ThingId;
use alloc::format;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    console_write("BLOOM PANIC\n");
    loop {}
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DisplayInfo {
    pub bytespace: u128,
    pub byte_len: u64,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub format: u32,
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    console_write("BLOOM: initialized\n");

    let mut info = DisplayInfo {
        bytespace: 0,
        byte_len: 0,
        width: 0,
        height: 0,
        pitch: 0,
        format: 0,
    };

    console_write("BLOOM: requesting primary display...\n");
    let res = unsafe {
        // thing_std::syscall is at root
        thing_std::syscall(
            160, 
            &mut info as *mut DisplayInfo as u64, 
            core::mem::size_of::<DisplayInfo>() as u64,
            0, 0, 0, 0
        )
    };

    if res.status != 0 {
        console_write("BLOOM: failed to get display info\n");
        thing_std::sys_exit(1);
    }
    
    console_write(&format!("BLOOM: got display info: {:?}\n", info));

    // Map it
    let map_len = ((info.byte_len + 4095) / 4096) * 4096;
    let vaddr = 0x8000_0000; // 2GB
    
    console_write(&format!("BLOOM: mapping bytespace {:?} at {:#x}, len {:#x}\n", ThingId(info.bytespace), vaddr, map_len));

    match thing_std::space_map(ThingId(info.bytespace), vaddr, 0, map_len) {
        Ok(ptr) => {
             console_write(&format!("BLOOM: mapped display at {:#x}\n", ptr));
        },
        Err(e) => {
             console_write(&format!("BLOOM: failed to map bytespace: {}\n", e));
             thing_std::sys_exit(1);
        }
    }

    let buf = unsafe { core::slice::from_raw_parts_mut(vaddr as *mut u32, (info.byte_len / 4) as usize) };
    
    console_write("BLOOM: painting gradient...\n");
    
    let mut offset = 0;
    loop {
        for y in 0..info.height {
            for x in 0..info.width {
                let r = (x + offset) % 255;
                let g = (y + offset) % 255;
                let b = (x + y) % 255;
                let color = (r << 16) | (g << 8) | b;
                
                let idx = (y as u64 * (info.pitch as u64 / 4) + x as u64) as usize;
                
                if idx < buf.len() {
                    buf[idx] = color;
                }
            }
        }
        offset = (offset + 1) % 255;
    }
}
