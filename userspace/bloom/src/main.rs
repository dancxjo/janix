#![no_std]
#![no_main]

use stem::info;
use stem::{thread, time};

#[stem::main]
fn main(arg: usize) -> ! {
    let disp_req_write = ((arg >> 0) & 0xFFFF) as u16;
    let disp_resp_read = ((arg >> 16) & 0xFFFF) as u16;

    info!(
        "bloom: starting ui stub (disp_req_w={}, disp_resp_r={})",
        disp_req_write, disp_resp_read
    );

    let mut tick = 0u32;
    loop {
        let _ = bloom::demo_drawlist(640, 480);
        tick = tick.wrapping_add(1);
        if tick % 300 == 0 {
            info!("bloom: ui tick {}", tick);
        }
        thread::yield_now();
        time::sleep_ms(16);
    }
}
