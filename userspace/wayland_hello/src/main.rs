#![no_main]
#![feature(restricted_std)]
extern crate alloc;

use abi::syscall::vfs_flags::O_RDWR;
use alloc::vec::Vec;
use stem::syscall::{sleep_ms, vfs_open, vfs_write};

#[stem::main]
fn main(_arg: usize) -> ! {
    stem::info!("wayland_hello: Starting up...");

    // Wait for wayland socket to appear
    let mut fd_opt = None;
    for _ in 0..50 {
        if let Ok(fd) = vfs_open("/run/wayland-0", O_RDWR) {
            fd_opt = Some(fd);
            break;
        }
        stem::info!("wayland_hello: Waiting for socket /run/wayland-0...");
        sleep_ms(100);
    }

    let fd = fd_opt.expect("Failed to connect to wayland server");
    stem::info!("wayland_hello: Connected to wayland server! fd={}", fd);

    // Get registry
    // object_id=1, opcode=1 (get_registry), arg=new_id (2)
    let mut buf = Vec::new();
    encode_header(1, 1, 12, &mut buf);
    buf.extend_from_slice(&2u32.to_ne_bytes());
    let _ = vfs_write(fd, &buf);

    // Bind compositor
    // object_id=2 (registry), opcode=0 (bind), name=1(compositor), (string), version=1, new_id=3
    let mut buf = Vec::new();
    let name = "wl_compositor";
    let len = name.len() as u32 + 1;
    let mut str_bytes = name.as_bytes().to_vec();
    str_bytes.push(0);
    let padding = (4 - (str_bytes.len() % 4)) % 4;
    for _ in 0..padding {
        str_bytes.push(0);
    }

    let size = 8 + 4 + 4 + str_bytes.len() as u16 + 4 + 4;
    encode_header(2, 0, size, &mut buf);
    buf.extend_from_slice(&1u32.to_ne_bytes()); // name
    buf.extend_from_slice(&len.to_ne_bytes()); // string len
    buf.extend_from_slice(&str_bytes);
    buf.extend_from_slice(&1u32.to_ne_bytes()); // version
    buf.extend_from_slice(&3u32.to_ne_bytes()); // new_id
    let _ = vfs_write(fd, &buf);

    // Bind shm
    // name=2, new_id=4
    let mut buf = Vec::new();
    let name = "wl_shm";
    let len = name.len() as u32 + 1;
    let mut str_bytes = name.as_bytes().to_vec();
    str_bytes.push(0);
    let padding = (4 - (str_bytes.len() % 4)) % 4;
    for _ in 0..padding {
        str_bytes.push(0);
    }
    let size = 8 + 4 + 4 + str_bytes.len() as u16 + 4 + 4;
    encode_header(2, 0, size, &mut buf);
    buf.extend_from_slice(&2u32.to_ne_bytes()); // name
    buf.extend_from_slice(&len.to_ne_bytes()); // string len
    buf.extend_from_slice(&str_bytes);
    buf.extend_from_slice(&1u32.to_ne_bytes()); // version
    buf.extend_from_slice(&4u32.to_ne_bytes()); // new_id
    let _ = vfs_write(fd, &buf);

    // Create surface
    // object_id=3 (compositor), opcode=0 (create_surface), new_id=5
    let mut buf = Vec::new();
    encode_header(3, 0, 12, &mut buf);
    buf.extend_from_slice(&5u32.to_ne_bytes());
    let _ = vfs_write(fd, &buf);

    // Map a shared memory buffer (red color)
    let width: u32 = 200;
    let height: u32 = 200;
    let stride: u32 = width * 4;
    let size: u32 = stride * height;

    let bs_id = stem::thing::sys::bytespace_create(size as usize, 0, 0)
        .expect("Failed to create bytespace");
    let ptr = stem::thing::sys::bytespace_map(bs_id).expect("Failed to map bytespace");

    unsafe {
        let pixels = core::slice::from_raw_parts_mut(ptr as *mut u32, (width * height) as usize);
        for i in 0..pixels.len() {
            pixels[i] = 0xFFFF0088; // Pinkish Red
        }
    }

    // Create pool
    // object_id=4 (shm), opcode=0 (create_pool), new_id=6, fd=bs_id.to_u64, size
    let mut buf = Vec::new();
    encode_header(4, 0, 8 + 12, &mut buf);
    buf.extend_from_slice(&6u32.to_ne_bytes()); // new_id
    buf.extend_from_slice(&(bs_id.to_u64_lossy() as u32).to_ne_bytes()); // fd hack
    buf.extend_from_slice(&size.to_ne_bytes()); // size
    let _ = vfs_write(fd, &buf);

    // Create buffer
    // object_id=6 (pool), opcode=0 (create_buffer), new_id=7, offset=0, width, height, stride, format=0 (ARGB8888)
    let mut buf = Vec::new();
    encode_header(6, 0, 8 + 24, &mut buf);
    buf.extend_from_slice(&7u32.to_ne_bytes());
    buf.extend_from_slice(&0u32.to_ne_bytes());
    buf.extend_from_slice(&width.to_ne_bytes());
    buf.extend_from_slice(&height.to_ne_bytes());
    buf.extend_from_slice(&stride.to_ne_bytes());
    buf.extend_from_slice(&0u32.to_ne_bytes());
    let _ = vfs_write(fd, &buf);

    // Attach buffer to surface
    // object_id=5 (surface), opcode=1 (attach), buffer_id=7, x=0, y=0
    let mut buf = Vec::new();
    encode_header(5, 1, 8 + 12, &mut buf);
    buf.extend_from_slice(&7u32.to_ne_bytes());
    buf.extend_from_slice(&0u32.to_ne_bytes());
    buf.extend_from_slice(&0u32.to_ne_bytes());
    let _ = vfs_write(fd, &buf);

    // Commit
    // object_id=5 (surface), opcode=6 (commit)
    let mut buf = Vec::new();
    encode_header(5, 6, 8, &mut buf);
    let _ = vfs_write(fd, &buf);

    stem::info!("wayland_hello: Sent attach and commit! We should see a 200x200 pink square now.");

    // Now loop and update color
    loop {
        for color in [0xFF00FF00, 0xFF0000FF, 0xFFFFFF00, 0xFFFF00FF] {
            sleep_ms(300);
            unsafe {
                let pixels =
                    core::slice::from_raw_parts_mut(ptr as *mut u32, (width * height) as usize);
                for i in 0..pixels.len() {
                    pixels[i] = color;
                }
            }

            // Re-attach and commit
            let mut buf = Vec::new();
            encode_header(5, 1, 8 + 12, &mut buf);
            buf.extend_from_slice(&7u32.to_ne_bytes());
            buf.extend_from_slice(&0u32.to_ne_bytes());
            buf.extend_from_slice(&0u32.to_ne_bytes());
            let _ = vfs_write(fd, &buf);

            let mut buf = Vec::new();
            encode_header(5, 6, 8, &mut buf);
            let _ = vfs_write(fd, &buf);
        }
    }
}

pub fn encode_header(object_id: u32, opcode: u16, size: u16, buf: &mut Vec<u8>) {
    buf.extend_from_slice(&object_id.to_ne_bytes());
    let size_op = ((size as u32) << 16) | (opcode as u32);
    buf.extend_from_slice(&size_op.to_ne_bytes());
}
