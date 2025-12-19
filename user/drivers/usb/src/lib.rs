#![no_std]
#![feature(allocator_api)]

extern crate alloc;

use alloc::vec::Vec;
// use abi::graph_ops::GraphSink; // We don't use implicit GraphSink for init anymore in strict model, 
// though we likely need a way to register the watcher.
// The kernel will call our init.

use abi::graph_ops::GraphDriver;
use hal::MmioMapper;

pub mod xhci;

pub fn init(mmio: &dyn MmioMapper, graph: &mut dyn GraphDriver) {
    xhci::register_watcher(mmio, graph);
}

use thing_os::println;

struct UserMmioMapper;
impl MmioMapper for UserMmioMapper {
    unsafe fn map_mmio(&self, phys: u64, size: u64) -> *mut u8 {
        println!("USB: Requesting MMIO map {:#x} size {}", phys, size);
        core::ptr::null_mut() 
    }
}

struct UserGraphDriver;
impl abi::graph_ops::GraphSink for UserGraphDriver {
    fn submit(&mut self, _op: abi::graph_ops::GraphOp) -> Result<(), &'static str> {
        Ok(())
    }
}
impl GraphDriver for UserGraphDriver {
    fn subscribe(&mut self, _kind: &'static str, _cb: for<'a> fn(&'a abi::graph_ops::GraphEvent)) {
        // Stub
    }
    fn get_thing(&self, _id: abi::ThingId) -> Option<abi::graph_ops::ThingProps> {
        None
    }
}

pub fn driver_main() {
    println!("USB Driver Starting...");
    let mmio = UserMmioMapper;
    let mut graph = UserGraphDriver;
    init(&mmio, &mut graph);
    loop {
        // yield
    }
}
