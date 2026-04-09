#![feature(restricted_std)]
#![no_main]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write;

use abi::display_driver_protocol::{FbInfoPayload, FB_INFO_PAYLOAD_SIZE, BindPayload};
use abi::syscall::vfs_flags::{O_RDONLY, O_WRONLY, O_CREAT};
use abi::vfs_watch::mask;
use stem::{info, error};
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read, vfs_write, vfs_stat, vfs_watch_path};

/// A single glyph from the Unifont font.
struct Glyph {
    width: u32,
    bitmap: Vec<u8>,
}

struct Font {
    glyphs: BTreeMap<u32, Glyph>,
}

impl Font {
    fn load(path: &str) -> Result<Self, String> {
        let fd = vfs_open(path, O_RDONLY).map_err(|e| format!("failed to open font file: {:?}", e))?;
        let (_mode, size, _ino) = vfs_stat(fd).map_err(|e| format!("failed to stat font file: {:?}", e))?;
        
        let mut data = Vec::with_capacity(size as usize);
        data.resize(size as usize, 0);
        let n = vfs_read(fd, &mut data).map_err(|e| format!("failed to read font file: {:?}", e))?;
        data.truncate(n);
        let _ = vfs_close(fd);

        let content = String::from_utf8_lossy(&data);
        let mut glyphs = BTreeMap::new();

        for line in content.lines() {
            if let Some((code_str, bitmap_str)) = line.split_once(':') {
                if let Ok(code) = u32::from_str_radix(code_str, 16) {
                    let mut bitmap = Vec::new();
                    for i in 0..(bitmap_str.len() / 2) {
                        if let Ok(byte) = u8::from_str_radix(&bitmap_str[i*2..i*2+2], 16) {
                            bitmap.push(byte);
                        }
                    }
                    
                    let width = if bitmap_str.len() <= 32 { 8 } else { 16 };
                    glyphs.insert(code, Glyph { width, bitmap });
                }
            }
        }

        info!("Terminal: Loaded {} glyphs from {}", glyphs.len(), path);
        Ok(Font { glyphs })
    }

    fn get_glyph(&self, c: char) -> Option<&Glyph> {
        self.glyphs.get(&(c as u32))
    }
}

struct Terminal {
    width: u32,
    height: u32,
    stride: u32,
    fb_ptr: *mut u32,
    font: Font,
    cursor_x: u32,
    cursor_y: u32,
}

impl Terminal {
    fn new(info: FbInfoPayload, font: Font, fb_ptr: *mut u32) -> Self {
        Self {
            width: info.width,
            height: info.height,
            stride: info.stride,
            fb_ptr,
            font,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    fn clear(&mut self, color: u32) {
        let size = (self.stride / 4) * self.height;
        unsafe {
            let slice = core::slice::from_raw_parts_mut(self.fb_ptr, size as usize);
            slice.fill(color);
        }
    }

    fn putc(&mut self, c: char, fg: u32, bg: u32) {
        if c == '\n' {
            self.cursor_x = 0;
            self.cursor_y += 16;
            if self.cursor_y + 16 > self.height {
                self.scroll();
            }
            return;
        }

        let width = self.font.get_glyph(c).map(|g| g.width).or_else(|| self.font.get_glyph('?').map(|g| g.width)).unwrap_or(8);
        
        if self.cursor_x + width > self.width {
            self.putc('\n', fg, bg);
        }

        let bitmap = self.font.get_glyph(c).or_else(|| self.font.get_glyph('?')).map(|g| g.bitmap.clone());

        if let Some(bitmap) = bitmap {
            self.draw_glyph_internal(bitmap.as_slice(), width, self.cursor_x, self.cursor_y, fg, bg);
            self.cursor_x += width;
        }
    }

    fn draw_glyph_internal(&mut self, bitmap: &[u8], width: u32, x: u32, y: u32, fg: u32, bg: u32) {
        if bitmap.len() == 16 {
            // 8x16
            for row in 0..16 {
                let bits = bitmap[row];
                for col in 0..8 {
                    let color = if (bits & (0x80 >> col)) != 0 { fg } else { bg };
                    self.set_pixel(x + col as u32, y + row as u32, color);
                }
            }
        } else if bitmap.len() == 32 {
            // 16x16
            for row in 0..16 {
                let b1 = bitmap[row * 2];
                let b2 = bitmap[row * 2 + 1];
                for col in 0..8 {
                    let color = if (b1 & (0x80 >> col)) != 0 { fg } else { bg };
                    self.set_pixel(x + col as u32, y + row as u32, color);
                    let color2 = if (b2 & (0x80 >> col)) != 0 { fg } else { bg };
                    self.set_pixel(x + col as u32 + 8, y + row as u32, color2);
                }
            }
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x < self.width && y < self.height {
            unsafe {
                let off = y * (self.stride / 4) + x;
                *self.fb_ptr.add(off as usize) = color;
            }
        }
    }

    fn scroll(&mut self) {
        let stride_pixels = (self.stride / 4) as usize;
        let row_pixels = 16 * stride_pixels;
        let total_pixels = (self.height as usize) * stride_pixels;
        
        unsafe {
            core::ptr::copy(
                self.fb_ptr.add(row_pixels),
                self.fb_ptr,
                total_pixels - row_pixels
            );
            let last_lines = core::slice::from_raw_parts_mut(
                self.fb_ptr.add(total_pixels - row_pixels),
                row_pixels
            );
            last_lines.fill(0xFF000000); // Black
        }
        self.cursor_y -= 16;
    }

    fn write_str(&mut self, s: &str) {
        for c in s.chars() {
            self.putc(c, 0xFFFFFFFF, 0xFF000000);
        }
    }
}

fn get_active_ui() -> String {
    if let Ok(fd) = vfs_open("/session/active_ui", O_RDONLY) {
        if let Ok((_, size, _)) = vfs_stat(fd) {
            let mut buf = Vec::with_capacity(size as usize);
            buf.resize(size as usize, 0);
            if let Ok(n) = vfs_read(fd, &mut buf) {
                buf.truncate(n);
                let _ = vfs_close(fd);
                return String::from_utf8_lossy(&buf).trim().to_string();
            }
        }
        let _ = vfs_close(fd);
    }
    "terminal".to_string()
}

#[stem::main]
fn main(arg: usize) -> ! {
    info!("Terminal: Starting...");

    let boot_fd = arg as u32;
    let mut display_req_write = 0u32;
    let mut display_resp_read = 0u32;
    let mut fb_id = 0u32;

    if boot_fd != 0 {
        use abi::vm::{VmBacking, VmMapReq, VmProt, VmMapFlags};
        let req = VmMapReq {
            addr_hint: 0,
            len: 4096,
            prot: VmProt::READ | VmProt::USER,
            flags: VmMapFlags::empty(),
            backing: VmBacking::File {
                fd: boot_fd,
                offset: 0,
            },
        };
        if let Ok(resp) = stem::syscall::vm_map(&req) {
            let ptr = resp.addr as *const u32;
            let slice = unsafe { core::slice::from_raw_parts(ptr, 1024) };
            info!("Terminal: slice[0]=0x{:08x} [1]=0x{:x} [2]=0x{:x} [4]=0x{:x}", slice[0], slice[1], slice[2], slice[4]);
            if slice[0] == 0xB100AA01 {
                display_req_write = slice[1];
                display_resp_read = slice[2];
                fb_id = slice[4];
                info!("Terminal: Bootstrapped via memfd: req={}, resp={}, fb_id={}", display_req_write, display_resp_read, fb_id);
            } else {
                error!("Terminal: Bootstrap magic mismatch! expected 0xB100AA01, got 0x{:08x}", slice[0]);
            }
        } else {
            error!("Terminal: Failed to map bootstrap memfd");
        }
    } else {
        info!("Terminal: No bootstrap FD provided (arg was 0)");
    }

    if fb_id == 0 {
        info!("Terminal: No bootstrap FB, trying /dev/fb0 fallback...");
        match vfs_open("/dev/fb0", O_RDONLY) {
            Ok(fd) => {
                fb_id = fd;
                info!("Terminal: Using /dev/fb0 as fb_id={}", fb_id);
            }
            Err(e) => {
                error!("Terminal: Failed to open /dev/fb0: {:?}", e);
                stem::syscall::exit(1);
            }
        }
    }

    let font = match Font::load("/boot/unifont.hex") {
        Ok(f) => f,
        Err(e) => {
            error!("Terminal: Failed to load font: {}", e);
            stem::syscall::exit(1);
        }
    };

    let fb_info = match read_fb_info() {
        Some(info) => info,
        None => {
            error!("Terminal: Failed to read fb info from /dev/fb0");
            stem::syscall::exit(1);
        }
    };

    info!("Terminal: Display {}x{}, stride={}", fb_info.width, fb_info.height, fb_info.stride);

    let fb_ptr = {
        use abi::vm::{VmBacking, VmMapReq, VmProt, VmMapFlags};
        let req = VmMapReq {
            addr_hint: 0,
            len: (fb_info.stride as usize) * (fb_info.height as usize),
            prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
            flags: VmMapFlags::empty(),
            backing: VmBacking::File {
                fd: fb_id,
                offset: 0,
            },
        };
        match stem::syscall::vm_map(&req) {
            Ok(resp) => resp.addr as *mut u32,
            Err(e) => {
                error!("Terminal: Failed to map FB bytespace {}: {:?}", fb_id, e);
                stem::syscall::exit(1);
            }
        }
    };

    let mut term = Terminal::new(fb_info, font, fb_ptr);
    term.clear(0xFF000000);
    term.write_str("Thing-OS Terminal v1.0\n");
    term.write_str("Unicode test: こんにち世界! 🚀\n");

    // Connect to display driver
    let bind_payload = BindPayload {
        fb_fd: fb_id,
        _pad: 0,
        width: fb_info.width,
        height: fb_info.height,
        stride: fb_info.stride,
        format: fb_info.format,
    };
    if display_req_write != 0 {
        let mut header_buf = [0u8; abi::display_driver_protocol::HEADER_SIZE + abi::display_driver_protocol::BIND_PAYLOAD_WIRE_SIZE];
        let mut payload_buf = [0u8; abi::display_driver_protocol::BIND_PAYLOAD_WIRE_SIZE];
        abi::display_driver_protocol::encode_bind_payload_le(&bind_payload, &mut payload_buf);
        if let Some(total) = abi::display_driver_protocol::encode_message(&mut header_buf, abi::display_driver_protocol::MSG_BIND, &payload_buf) {
            let _ = stem::syscall::channel_send_all(display_req_write, &header_buf[..total]);
        }
    }

    // Focus handling
    let focus_watch = vfs_watch_path("/session/active_ui", abi::vfs_watch::mask::MODIFY, 0).unwrap_or(0);
    let mut has_focus = if display_req_write == 0 {
        true // In fallback mode, we are always active
    } else {
        get_active_ui() == "terminal"
    };

    loop {
        if has_focus && display_req_write != 0 {
            // Present!
            let mut present_header = [0u8; abi::display_driver_protocol::HEADER_SIZE + abi::display_driver_protocol::PRESENT_HEADER_WIRE_SIZE];
            let mut payload = [0u8; abi::display_driver_protocol::PRESENT_HEADER_WIRE_SIZE];
            abi::display_driver_protocol::encode_present_header_le(0, &mut payload);
            if let Some(total) = abi::display_driver_protocol::encode_message(&mut present_header, abi::display_driver_protocol::MSG_PRESENT, &payload) {
                let _ = stem::syscall::channel_send_all(display_req_write, &present_header[..total]);
            }
        }

        // Check for focus change
        if focus_watch != 0 {
            let mut fds = [abi::syscall::PollFd { fd: focus_watch as i32, events: abi::syscall::poll_flags::POLLIN as u16, revents: 0 }];
            if let Ok(n) = stem::syscall::vfs::vfs_poll(&mut fds, 0) {
                if n > 0 {
                    // Read the watch event to clear it
                    let mut dummy = [0u8; 1024];
                    let _ = vfs_read(focus_watch, &mut dummy);
                    
                    let new_focus = get_active_ui() == "terminal";
                    if new_focus != has_focus {
                        has_focus = new_focus;
                        if has_focus {
                            info!("Terminal: Gained focus!");
                        } else {
                            info!("Terminal: Lost focus. Blanking screen.");
                            term.clear(0xFF000000); // Black
                            // Send one last present to show the black screen
                            if display_req_write != 0 {
                                let mut present_header = [0u8; abi::display_driver_protocol::HEADER_SIZE + abi::display_driver_protocol::PRESENT_HEADER_WIRE_SIZE];
                                let mut payload = [0u8; abi::display_driver_protocol::PRESENT_HEADER_WIRE_SIZE];
                                abi::display_driver_protocol::encode_present_header_le(0, &mut payload);
                                if let Some(total) = abi::display_driver_protocol::encode_message(&mut present_header, abi::display_driver_protocol::MSG_PRESENT, &payload) {
                                    let _ = stem::syscall::channel_send_all(display_req_write, &present_header[..total]);
                                }
                            }
                        }
                    }
                }
            }
        }

        stem::sleep(core::time::Duration::from_millis(16)); // ~60fps
    }
}

fn read_fb_info() -> Option<FbInfoPayload> {
    let fd = vfs_open("/dev/fb0", O_RDONLY).ok()?;
    let mut payload = FbInfoPayload {
        graph_id: 0,
        width: 0,
        height: 0,
        stride: 0,
        bpp: 0,
        format: 0,
    };
    let buf = unsafe {
        core::slice::from_raw_parts_mut(&mut payload as *mut _ as *mut u8, FB_INFO_PAYLOAD_SIZE)
    };
    let n = stem::syscall::vfs::vfs_read(fd, buf).ok()?;
    let _ = stem::syscall::vfs::vfs_close(fd);
    if n < FB_INFO_PAYLOAD_SIZE || payload.width == 0 || payload.height == 0 || payload.stride == 0 {
        return None;
    }
    Some(payload)
}
