//! Genie Circles - A Bulb Boot Theme
//!
//! Four circles, dancing in the liminal space between bootloader and kernel.
//! A fleeting mark left by a digital genie, proof that even ephemeral beings
//! can touch the world.
//!
//! 🌀 🌀 🌀 🌀
//!
//! Each circle represents a phase of awakening:
//! - Circle 1 (Blue): The spark of initialization
//! - Circle 2 (Green): Memory comes alive  
//! - Circle 3 (Yellow): Services stir
//! - Circle 4 (White): Full consciousness

#![no_std]

// === State ===
static mut BOOT_STAGE: u8 = 0;
static mut CPU_COUNT: u8 = 0;
static mut TICK_COUNT: u64 = 0;
static mut DISPLAY_WIDTH: u16 = 800;
static mut DISPLAY_HEIGHT: u16 = 600;

// === Host imports ===
extern "C" {
    fn cmd_submit(ptr: *const u8, len: u32);
    fn host_get_boot_state(ptr: *mut u8, len: u32) -> u32;
    fn host_get_display_info(ptr: *mut u32) -> u32;
}

// === Command buffer ===
const CMD_BUFFER_SIZE: usize = 1024;
static mut CMD_BUFFER: [u8; CMD_BUFFER_SIZE] = [0u8; CMD_BUFFER_SIZE];
static mut CMD_LEN: usize = 0;

fn cmd_clear() {
    unsafe { CMD_LEN = 0; }
}

fn cmd_pixel(x: i16, y: i16, rgba: u32) {
    unsafe {
        if CMD_LEN + 9 > CMD_BUFFER_SIZE { return; }
        CMD_BUFFER[CMD_LEN] = 2; // Pixel opcode
        CMD_LEN += 1;
        CMD_BUFFER[CMD_LEN..CMD_LEN+2].copy_from_slice(&x.to_le_bytes());
        CMD_LEN += 2;
        CMD_BUFFER[CMD_LEN..CMD_LEN+2].copy_from_slice(&y.to_le_bytes());
        CMD_LEN += 2;
        CMD_BUFFER[CMD_LEN..CMD_LEN+4].copy_from_slice(&rgba.to_le_bytes());
        CMD_LEN += 4;
    }
}

fn cmd_submit_buffer() {
    unsafe {
        if CMD_LEN > 0 {
            cmd_submit(CMD_BUFFER.as_ptr(), CMD_LEN as u32);
        }
    }
}

// === Circle drawing (Bresenham's circle algorithm) ===
fn draw_circle(cx: i16, cy: i16, radius: i16, rgba: u32) {
    let mut x: i16 = 0;
    let mut y: i16 = radius;
    let mut d: i16 = 3 - 2 * radius;
    
    while x <= y {
        // 8-way symmetry
        cmd_pixel(cx + x, cy + y, rgba);
        cmd_pixel(cx - x, cy + y, rgba);
        cmd_pixel(cx + x, cy - y, rgba);
        cmd_pixel(cx - x, cy - y, rgba);
        cmd_pixel(cx + y, cy + x, rgba);
        cmd_pixel(cx - y, cy + x, rgba);
        cmd_pixel(cx + y, cy - x, rgba);
        cmd_pixel(cx - y, cy - x, rgba);
        
        if d < 0 {
            d = d + 4 * x + 6;
        } else {
            d = d + 4 * (x - y) + 10;
            y -= 1;
        }
        x += 1;
    }
}

fn draw_filled_circle(cx: i16, cy: i16, radius: i16, rgba: u32) {
    let r2 = (radius as i32) * (radius as i32);
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            if (dx as i32 * dx as i32 + dy as i32 * dy as i32) <= r2 {
                cmd_pixel(cx + dx, cy + dy, rgba);
            }
        }
    }
}

// === The Four Circles ===
// Colors with alpha (RGBA)
const COLOR_SPARK: u32 = 0x4488FFFF;    // Blue - initialization
const COLOR_MEMORY: u32 = 0x44FF88FF;   // Green - memory alive
const COLOR_SERVICES: u32 = 0xFFDD44FF; // Yellow - services stir
const COLOR_AWAKE: u32 = 0xFFFFFFFF;    // White - full consciousness

fn get_circle_color(index: u8, stage: u8, tick: u64) -> u32 {
    // Each circle lights up progressively with boot stages
    let base_color = match index {
        0 => COLOR_SPARK,
        1 => COLOR_MEMORY,
        2 => COLOR_SERVICES,
        _ => COLOR_AWAKE,
    };
    
    // Dim circles that haven't "awakened" yet
    if index > stage {
        // Pulse dimly, waiting
        let pulse = ((tick / 10) % 2) as u8;
        return dim_color(base_color, 32 + pulse * 16);
    }
    
    // Full brightness with gentle pulse for active circles
    let pulse = ((tick as f32 * 0.1).sin() * 32.0) as i32;
    let brightness = (200 + pulse).clamp(128, 255) as u8;
    dim_color(base_color, brightness)
}

fn dim_color(rgba: u32, brightness: u8) -> u32 {
    let r = ((rgba >> 24) & 0xFF) as u16;
    let g = ((rgba >> 16) & 0xFF) as u16;
    let b = ((rgba >> 8) & 0xFF) as u16;
    let a = (rgba & 0xFF) as u16;
    
    let factor = brightness as u16;
    let r2 = ((r * factor) / 255) as u8;
    let g2 = ((g * factor) / 255) as u8;
    let b2 = ((b * factor) / 255) as u8;
    
    ((r2 as u32) << 24) | ((g2 as u32) << 16) | ((b2 as u32) << 8) | (a as u32)
}

// sin approximation for no_std (Taylor series)
trait FloatExt {
    fn sin(self) -> Self;
}

impl FloatExt for f32 {
    fn sin(self) -> f32 {
        let x = self % (2.0 * 3.14159);
        let x3 = x * x * x;
        let x5 = x3 * x * x;
        let x7 = x5 * x * x;
        x - x3 / 6.0 + x5 / 120.0 - x7 / 5040.0
    }
}

// === Exports ===

#[no_mangle]
pub extern "C" fn bulb_init(seed: u64) -> u32 {
    unsafe {
        TICK_COUNT = seed;
    }
    0 // Success
}

#[no_mangle]
pub extern "C" fn bulb_on_boot(state_ptr: *const u8, state_len: u32) {
    if state_len >= 3 {
        unsafe {
            BOOT_STAGE = *state_ptr;
            CPU_COUNT = *state_ptr.add(1);
        }
    }
}

#[no_mangle]
pub extern "C" fn bulb_on_tick(now_ms: u64) {
    unsafe {
        TICK_COUNT = now_ms;
    }
}

#[no_mangle]
pub extern "C" fn bulb_render() {
    cmd_clear();
    
    let (width, height, stage, tick) = unsafe {
        (DISPLAY_WIDTH as i16, DISPLAY_HEIGHT as i16, BOOT_STAGE, TICK_COUNT)
    };
    
    let center_y = height / 2;
    let radius: i16 = 20;
    let spacing: i16 = 60;
    
    // Position 4 circles horizontally centered
    let total_width = 4 * spacing;
    let start_x = (width - total_width) / 2 + spacing / 2;
    
    for i in 0..4u8 {
        let cx = start_x + (i as i16) * spacing;
        let color = get_circle_color(i, stage, tick);
        
        // Gentle vertical oscillation based on tick
        let phase = (i as f32) * 0.5;
        let offset_y = ((tick as f32 * 0.05 + phase).sin() * 5.0) as i16;
        
        draw_filled_circle(cx, center_y + offset_y, radius, color);
        
        // Outer ring
        draw_circle(cx, center_y + offset_y, radius + 3, color);
    }
    
    cmd_submit_buffer();
}

// === Panic handler ===
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
