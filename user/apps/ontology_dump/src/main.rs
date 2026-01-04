#![no_std]
#![no_main]

extern crate alloc;
use abi::syscall::nr::SYS_ONTOLOGY_GET;
use alloc::vec::Vec;
use thing_std::{log_info, syscall};

// Since we don't have println!
macro_rules! println {
    ($($arg:tt)*) => {{
        let s = alloc::format!($($arg)*);
        thing_std::log_info(&s);
    }};
}

#[no_mangle]
pub fn main() {
    println!("Ontology Dump Tool");

    // 1. Get size
    let res = unsafe { syscall(SYS_ONTOLOGY_GET, 0, 0, 0, 0, 0, 0) };

    if res.status != 0 {
        println!("Error getting ontology size: {}", res.status);
        return;
    }

    let size = res.val0;
    println!("ONTOLOGY_SIZE: {}", size);

    if size == 0 {
        return;
    }

    // 2. Fetch data
    let mut buffer = Vec::with_capacity(size as usize);
    unsafe { buffer.set_len(size as usize) };

    let res = unsafe {
        syscall(
            SYS_ONTOLOGY_GET,
            buffer.as_mut_ptr() as u64,
            size,
            0,
            0,
            0,
            0,
        )
    };

    if res.status != 0 {
        println!("Error fetching ontology: {}", res.status);
        return;
    }

    // 3. Compute digest
    let digest = fnv1a_hash(&buffer);
    println!("ONTOLOGY_DIGEST: 0x{:X}", digest);
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
