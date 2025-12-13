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
