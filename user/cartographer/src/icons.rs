use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use thing_os::prelude::*;
use abi::{MapFlags, graph_kinds};
use thing_os::RawModule;

pub struct Icon {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u32>, // BGRA8888
}

pub struct IconAtlas {
    pub icons: BTreeMap<String, Icon>,
    pub fallback: Icon,
    pub missing: Icon,
}

impl IconAtlas {
    pub fn new<S: Sys>(sys: &mut S) -> Self {
        let mut icons = BTreeMap::new();
        // Fallback: 32x32 magenta/black checkerboard
        let fallback = create_checkerboard(32, 0xFFFF00FF, 0xFF000000); // Magenta/Black
        let missing = create_checkerboard(32, 0xFFFF0000, 0xFFFFFFFF); // Red/White

        let modules: Vec<RawModule> = thing_os::list_things_by_kind(sys);
        let mut count = 0;
        
        for m in modules {
             // Check if it's an image module
             // We configured limine to set image=filename in cmdline, which sets raw_kind="image"
             if m.raw_kind == "image" {
                 if let Some(icon) = load_icon(sys, &m) {
                     // identifier is filename.bmp e.g. "Process.bmp"
                     let name = m.identifier.strip_suffix(".bmp").unwrap_or(&m.identifier);
                     
                     // Also handle lowercase mapping if needed, but Plataro map uses correct casing.
                     // We store as is.
                     icons.insert(String::from(name), icon);
                     count += 1;
                 }
             }
        }
        
        let msg = alloc::format!("IconAtlas loaded {} icons", count);
        let leaked = alloc::boxed::Box::leak(msg.into_boxed_str());
        thing_os::println(sys, leaked);
        
        Self { icons, fallback, missing }
    }
    
    pub fn get(&self, kind: &str) -> &Icon {
        if let Some(icon) = self.icons.get(kind) {
            return icon;
        }
        
        // If exact match fails, maybe try to be smart or just return missing/fallback?
        // Plataro map should cover known Kinds.
        // If unknown, return missing (Red/White).
        // If we are in fallback mode (count=0), we return fallback.
        
        if self.icons.is_empty() {
             &self.fallback
        } else {
             &self.missing
        }
    }
}

fn load_icon<S: Sys>(sys: &mut S, m: &RawModule) -> Option<Icon> {
    let fid = m.framebuffer_id?;
    
    // Map with user permissions
    let (vaddr, size) = thing_os::shared_buffer_map(sys, fid, MapFlags::READ | MapFlags::USER).ok()?;
    let ptr = vaddr as *const u8;
    
    parse_bmp(ptr, size as usize)
}

fn parse_bmp(ptr: *const u8, size: usize) -> Option<Icon> {
    unsafe {
        if size < 54 { return None; }
        if *ptr != b'B' || *ptr.add(1) != b'M' { return None; }
        
        let read_u32 = |off| {
            let p = ptr.add(off);
            u32::from_le_bytes([*p, *p.add(1), *p.add(2), *p.add(3)])
        };
        let read_i32 = |off| {
            let p = ptr.add(off);
            i32::from_le_bytes([*p, *p.add(1), *p.add(2), *p.add(3)])
        };
        let read_u16 = |off| {
            let p = ptr.add(off);
            u16::from_le_bytes([*p, *p.add(1)])
        };

        let data_offset = read_u32(0x0A) as usize;
        let width = read_i32(0x12);
        let height = read_i32(0x16);
        let bpp = read_u16(0x1C);
        
        // We only support 32bpp (RGBA) for now as that's what build_plataro_icons.sh produces
        if bpp != 32 { return None; }
        if data_offset >= size { return None; }
        
        let w = width.abs() as usize;
        let h = height.abs() as usize;
        let top_down = height < 0;
        
        let mut pixels = Vec::with_capacity(w * h);
        
        // Stride is typically aligned to 4 bytes
        let stride = w * 4; 
        
        let data = ptr.add(data_offset);
        
        // Check bounds
        if data_offset + h * stride > size { return None; }
        
        for y in 0..h {
            let src_y = if top_down { y } else { h - 1 - y };
            let row_start = data.add(src_y * stride);
            for x in 0..w {
                let p = row_start.add(x * 4);
                // Read BGRA (BMP standard for 32bpp)
                // PixelFormat::Bgra8888 is native for compositor
                let val = u32::from_le_bytes([*p, *p.add(1), *p.add(2), *p.add(3)]);
                pixels.push(val);
            }
        }
        
        Some(Icon {
            width: w as u32,
            height: h as u32,
            pixels,
        })
    }
}

fn create_checkerboard(size: u32, color1: u32, color2: u32) -> Icon {
    let mut pixels = Vec::with_capacity((size * size) as usize);
    for y in 0..size {
        for x in 0..size {
            let check = ((x / 8) + (y / 8)) % 2 == 0;
            pixels.push(if check { color1 } else { color2 });
        }
    }
    Icon {
        width: size,
        height: size,
        pixels,
    }
}
