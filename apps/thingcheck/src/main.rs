#![no_std]
#![no_main]

extern crate alloc;

use abi::syscall::nr;
use models::{
    BytespaceBody, DisplayDeviceBody, FramebufferBody, MouseStreamBody, PointerStateBody,
    SurfaceBody, Thing,
};
use thing_std::{log_info, syscall};

macro_rules! println {
    ($($arg:tt)*) => {{
        let s = alloc::format!($($arg)*);
        log_info(&s);
    }};
}

#[no_mangle]
pub fn main() {
    thing_std::init(0);
    println!("thingcheck: start");

    if check_ontology() {
        println!("OK: ontology fetched");
    }

    if let Some(place_display) = thing_std::graph::thing_find("place.display") {
        let _ = place_display;
        println!("OK: place.display present");
    } else {
        println!("FAIL: place.display missing");
    }

    let display_ok = find_and_decode(
        &["device.display0", "display.0"],
        "DisplayDevice decodable",
        |bytes| DisplayDeviceBody::decode(bytes),
    );

    let framebuffer_ok = find_and_decode(
        &["framebuffer.display0"],
        "Framebuffer decodable",
        |bytes| FramebufferBody::decode(bytes),
    );

    let surface_ok = if let Some(body) =
        find_and_decode(&["surface.display0"], "Surface decodable", |bytes| {
            SurfaceBody::decode(bytes)
        }) {
        if body.bytespace != abi::ids::ThingId(0) {
            println!("OK: Surface has Bytespace");
        } else {
            println!("FAIL: Surface missing bytespace link");
        }
        true
    } else {
        false
    };

    let _ = framebuffer_ok;
    let _ = display_ok;
    let _ = surface_ok;

    if thing_std::graph::thing_find("place.windows").is_some() {
        println!("OK: place.windows present");
    } else {
        println!("FAIL: place.windows missing");
    }

    let _mouse = find_and_decode(&["mouse.stream.0"], "MouseStream present", |bytes| {
        MouseStreamBody::decode(bytes)
    });

    let _pointer = find_and_decode(
        &["pointer.state", "pointer.0"],
        "PointerState decodable",
        |bytes| PointerStateBody::decode(bytes),
    );
}

fn check_ontology() -> bool {
    let res = unsafe { syscall(nr::SYS_ONTOLOGY_GET, 0, 0, 0, 0, 0, 0) };
    if res.status != 0 {
        println!("FAIL: ontology size status {}", res.status);
        return false;
    }
    let size = res.val0;
    if size == 0 {
        println!("FAIL: ontology empty");
        return false;
    }
    let mut buffer = alloc::vec::Vec::with_capacity(size as usize);
    unsafe { buffer.set_len(size as usize) };
    let res = unsafe {
        syscall(
            nr::SYS_ONTOLOGY_GET,
            buffer.as_mut_ptr() as u64,
            size,
            0,
            0,
            0,
            0,
        )
    };
    if res.status != 0 {
        println!("FAIL: ontology fetch status {}", res.status);
        return false;
    }
    let digest = fnv1a_hash(&buffer);
    println!("ONTOLOGY_DIGEST: 0x{:X}", digest);
    true
}

fn find_and_decode<T, F>(names: &[&str], label: &str, decode: F) -> Option<T>
where
    F: Fn(&[u8]) -> Result<T, ()>,
{
    for name in names {
        if let Some(id) = thing_std::graph::thing_find(name) {
            let mut buf = [0u8; 256];
            let len = thing_std::graph::thing_get_payload(id, &mut buf);
            let payload = &buf[..len];
            match decode(payload) {
                Ok(t) => {
                    println!("OK: {}", label);
                    return Some(t);
                }
                Err(_) => {
                    println!("FAIL: decode {}", label);
                    return None;
                }
            }
        }
    }
    println!("FAIL: missing {}", label);
    None
}

fn fnv1a_hash(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(0x100000001b3);
        i += 1;
    }
    hash
}
