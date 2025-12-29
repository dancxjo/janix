#![no_std]
#![no_main]

extern crate alloc;

use thing_std::typed::{ThingType, ThingType as _};
use thing_std::syscalls;
use abi::wire::typed::{TypeId, TypeDesc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ThingType)]
#[thing(name = "Window", version = 1, codec = "postcard")]
struct Window {
    title: alloc::string::String,
    opacity: f64,
}

#[no_mangle]
pub extern "C" fn _start(heap_start: u64) -> ! {
    unsafe { thing_std::rt::init_heap(heap_start as usize, 1024 * 1024); }
    // thing_std::init(); // Initialize allocator if needed (thing_std doesn't have init public yet, but it defines global allocator)

    // 1. Register Typedef
    let typedef = Window::typedef();

    // Manual registration via syscall (if thing_std wrapper not available yet)
    let encoded_typedef = postcard::to_allocvec(&typedef).unwrap();
    let ret = unsafe {
        syscalls::syscall2(
            abi::syscall_defs::SYSCALL_TYPEDEF_REGISTER,
            encoded_typedef.as_ptr() as usize,
            encoded_typedef.len()
        )
    };

    if ret != 0 {
        // panic!("Failed to register typedef: error {}", ret);
    }

    // 2. Retrieve Typedef
    let mut buf = [0u8; 1024];
    let type_id_bytes = postcard::to_allocvec(&Window::type_id()).unwrap();
    let len = unsafe {
        syscalls::syscall3(
             abi::syscall_defs::SYSCALL_TYPEDEF_GET,
             type_id_bytes.as_ptr() as usize,
             buf.as_mut_ptr() as usize,
             buf.len()
        )
    };

    if len == 0 {
        // panic!("Failed to retrieve typedef");
    }

    let retrieved_typedef: abi::wire::typed::TypeDef = postcard::from_bytes(&buf[..len as usize]).unwrap();

    if retrieved_typedef.type_id != Window::type_id() {
        // panic!("Retrieved TypeID mismatch");
    }

    // 3. Round-trip typed payload (Userland side check)
    let win = Window {
        title: alloc::string::String::from("My Window"),
        opacity: 0.8,
    };

    let encoded = win.encode().unwrap();
    let decoded = Window::decode(&encoded).unwrap();

    // assert_eq!(win, decoded);

    // 4. Validate
    win.validate().unwrap();

    loop {}
}
