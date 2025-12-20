#![no_std]
#![feature(allocator_api)]

extern crate alloc;

use alloc::vec::Vec;
use thing_os::graph_ops::{GraphDriver, GraphSink, GraphOp, GraphEvent, ThingProps};
use abi::{ThingId, syscall_defs::SymbolId};
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
impl GraphSink for UserGraphDriver {
    fn submit(&mut self, _op: GraphOp) -> Result<(), &'static str> {
        Ok(())
    }
}
impl GraphDriver for UserGraphDriver {
    fn subscribe(&mut self, _kind: SymbolId, _cb: fn(&GraphEvent)) {
        // Stub
    }
    fn get_thing(&self, _id: ThingId) -> Option<ThingProps> {
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
