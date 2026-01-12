//! Inkwell - Bytespace Memory Mapping Demo
//!
//! This application demonstrates the bytespace mapping primitives:
//! 1. Creates a 1 MiB bytespace
//! 2. Maps it into userspace
//! 3. Writes a pattern via the mapped pointer
//! 4. Reads back via syscall and verifies consistency

#![no_std]
#![no_main]

use stem::thing::sys::{bytespace_create, bytespace_map, bytespace_read, bytespace_phys};
use stem::syscall::yield_now;
use stem::info;

const BYTESPACE_SIZE: usize = 1024 * 1024; // 1 MiB
const TEST_PATTERN_LEN: usize = 256;

#[stem::main]
fn main() -> ! {
    info!("INKWELL: Starting bytespace mapping demo");

    // 1. Create a 1 MiB bytespace
    let bs_id = match bytespace_create(BYTESPACE_SIZE, 0, 0) {
        Ok(id) => {
            info!("INKWELL: Created bytespace id={}", id.0);
            id
        }
        Err(e) => {
            info!("INKWELL: FAIL - bytespace_create error {:?}", e);
            loop { yield_now(); }
        }
    };

    // Check physical address
    if let Ok(phys) = bytespace_phys(bs_id) {
        info!("INKWELL: Physical base = 0x{:x}", phys);
    }

    // 2. Map the bytespace into our address space
    let ptr = match bytespace_map(bs_id) {
        Ok(p) => {
            info!("INKWELL: Mapped at user VA = 0x{:x}", p as usize);
            p
        }
        Err(e) => {
            info!("INKWELL: FAIL - bytespace_map error {:?}", e);
            loop { yield_now(); }
        }
    };

    // 3. Write a test pattern via the mapped pointer
    info!("INKWELL: Writing test pattern via mapped memory...");
    unsafe {
        for i in 0..TEST_PATTERN_LEN {
            *ptr.add(i) = i as u8;
        }
    }
    info!("INKWELL: Pattern written");

    // 4. Read back via syscall and verify
    info!("INKWELL: Reading back via syscall...");
    let mut buf = [0u8; TEST_PATTERN_LEN];
    match bytespace_read(bs_id, 0, &mut buf) {
        Ok(n) => {
            info!("INKWELL: Read {} bytes via syscall", n);
        }
        Err(e) => {
            info!("INKWELL: FAIL - bytespace_read error {:?}", e);
            loop { yield_now(); }
        }
    }

    // 5. Verify consistency
    let mut mismatches = 0;
    for i in 0..TEST_PATTERN_LEN {
        if buf[i] != i as u8 {
            mismatches += 1;
        }
    }

    if mismatches == 0 {
        info!("INKWELL: PASS - All {} bytes verified correctly!", TEST_PATTERN_LEN);
        info!("INKWELL: Bytespace mapping demo successful!");
    } else {
        info!("INKWELL: FAIL - {} byte mismatches detected", mismatches);
    }

    // Done - idle forever
    loop {
        yield_now();
    }
}
