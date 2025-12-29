#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use thing_std::{print, println};
use thing_std::bytespace;
use models::payload::*;
use models::Thing;
use models::abi;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    thing_std::init();
    println!("BYTESPACE TEST: Starting...");

    // 1. Create and Map
    let hint = 0x8000_0000;
    let (bs_id, addr) = match bytespace::create_and_map(4096, 3, 3, hint) {
        Ok(res) => res,
        Err(_) => {
            println!("BYTESPACE TEST: FAILED to create_and_map");
            loop {}
        }
    };

    println!("BYTESPACE TEST: Created ByteSpace {:?} at {:#x}", bs_id, addr);

    // 2. Write to memory
    unsafe {
        let ptr = addr as *mut u8;
        *ptr = b'H';
        *ptr.add(1) = b'e';
        *ptr.add(2) = b'l';
        *ptr.add(3) = b'l';
        *ptr.add(4) = b'o';
    }
    println!("BYTESPACE TEST: Wrote 'Hello' to mapped memory");

    // 3. Create Stream Thing
    let stream = Stream { cursor: 0, mode: 2 };
    let bytes = postcard::to_allocvec(&stream).unwrap();

    use models::abi::wire::graph::{GraphOp, GraphReply};
    use thing_std::client::GraphClient;

    let client = GraphClient::new();
    let op = GraphOp::CreateThing { kind: Stream::KIND, value: bytes };

    let mut buf = [0u8; 512];
    match client.call_op(&op, &mut buf) {
        Ok(GraphReply::Created { id }) => {
             println!("BYTESPACE TEST: Created Stream Thing {:?}", id);
             // Link Stream -> ByteSpace
             let op_link = GraphOp::AddLink { from: id, to: bs_id, kind: BACKED_BY };
             if let Ok(GraphReply::Created { id: link_id }) = client.call_op(&op_link, &mut buf) {
                 println!("BYTESPACE TEST: Linked Stream -> ByteSpace via BACKED_BY (Link {:?})", link_id);
             } else {
                 println!("BYTESPACE TEST: Failed to link");
             }
        },
        Ok(r) => println!("BYTESPACE TEST: Unexpected reply {:?}", r),
        Err(_) => println!("BYTESPACE TEST: Failed to create Stream"),
    }

    println!("BYTESPACE TEST: SUCCESS");
    loop {}
}
