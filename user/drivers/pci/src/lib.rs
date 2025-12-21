#![no_std]
#![feature(allocator_api)]

extern crate alloc;

use abi::{
    syscall_defs::SymbolId,
    wire::graph::{WirePropValue},
};
use thing_os::thing_models::graph_kinds::{self, KIND_PCI_DEVICE};
use thing_os::graph_ops::GraphOp;
use alloc::vec::Vec;
use hal::{PciConfigAccess};
use thing_os::intern;

pub trait GraphSink {
    fn submit(&mut self, op: GraphOp) -> Result<(), &'static str>;
}

pub struct PciDriver<'a> {
    config: &'a dyn PciConfigAccess,
}

impl<'a> PciDriver<'a> {
    pub fn new(config: &'a dyn PciConfigAccess) -> Self {
        Self { config }
    }

    fn read_u32(&self, bus: u8, slot: u8, func: u8, offset: u16) -> u32 {
        self.config.read_u32(bus, slot, func, offset)
    }
    fn read_u16(&self, bus: u8, slot: u8, func: u8, offset: u16) -> u16 {
        self.config.read_u16(bus, slot, func, offset)
    }
    fn read_u8(&self, bus: u8, slot: u8, func: u8, offset: u16) -> u8 {
        self.config.read_u8(bus, slot, func, offset)
    }

    fn read_bar(&self, bus: u8, slot: u8, func: u8, index: u8) -> u64 {
        let raw = self.read_u32(bus, slot, func, 0x10 + (index as u16) * 4);

        // Very basic BAR parsing
        if raw & 1 == 1 {
            // IO BAR - return as is for now, consumer needs to handle
            (raw & !0x3) as u64
        } else {
            // Memory
            let type_field = (raw >> 1) & 0x3;
            match type_field {
                0 => raw as u64, // 32-bit
                2 => {
                    // 64-bit, need to read next BAR
                    let lower = raw & 0xFFFFFFF0;
                    let upper = self.read_u32(bus, slot, func, 0x10 + ((index + 1) as u16) * 4);
                    ((upper as u64) << 32) | (lower as u64)
                }
                _ => raw as u64, // fallback
            }
        }
    }

    pub fn init(&self) {
    }
    
    pub fn scan_and_publish<S: GraphSink>(&self, sink: &mut S) {
        let bus = 0;
        for slot in 0..32 {
            let vendor_id = self.read_u16(bus, slot, 0, 0);
            if vendor_id == 0xFFFF {
                continue;
            }

            let header_type = self.read_u8(bus, slot, 0, 0x0E);
            let func_count = if header_type & 0x80 != 0 { 8 } else { 1 };

            for func in 0..func_count {
                let vendor_id = self.read_u16(bus, slot, func, 0);
                if vendor_id == 0xFFFF {
                    continue;
                }

                let device_id = self.read_u16(bus, slot, func, 2);
                let class_id = self.read_u8(bus, slot, func, 0x0B);
                let subclass_id = self.read_u8(bus, slot, func, 0x0A);
                let prog_if = self.read_u8(bus, slot, func, 0x09);

                // Read BARs
                let bar0 = self.read_bar(bus, slot, func, 0);
                let bar1 = self.read_bar(bus, slot, func, 1);
                let bar2 = self.read_bar(bus, slot, func, 2);
                let bar3 = self.read_bar(bus, slot, func, 3);
                let bar4 = self.read_bar(bus, slot, func, 4);
                let bar5 = self.read_bar(bus, slot, func, 5);

                let props: Vec<(SymbolId, WirePropValue)> = alloc::vec![
                    (intern(graph_kinds::PROP_BUS), WirePropValue::u64(bus as u64)),
                    (intern(graph_kinds::PROP_SLOT), WirePropValue::u64(slot as u64)),
                    (intern(graph_kinds::PROP_FUNC), WirePropValue::u64(func as u64)),
                    (intern(graph_kinds::PROP_VENDOR_ID), WirePropValue::u64(vendor_id as u64)),
                    (intern(graph_kinds::PROP_DEVICE_ID), WirePropValue::u64(device_id as u64)),
                    (intern(graph_kinds::PROP_CLASS_ID), WirePropValue::u64(class_id as u64)),
                    (intern(graph_kinds::PROP_SUBCLASS_ID), WirePropValue::u64(subclass_id as u64)),
                    (intern(graph_kinds::PROP_PROG_IF), WirePropValue::u64(prog_if as u64)),
                    (intern(graph_kinds::PROP_BAR0), WirePropValue::u64(bar0)),
                    (intern(graph_kinds::PROP_BAR1), WirePropValue::u64(bar1)),
                    (intern(graph_kinds::PROP_BAR2), WirePropValue::u64(bar2)),
                    (intern(graph_kinds::PROP_BAR3), WirePropValue::u64(bar3)),
                    (intern(graph_kinds::PROP_BAR4), WirePropValue::u64(bar4)),
                    (intern(graph_kinds::PROP_BAR5), WirePropValue::u64(bar5)),
                ];
                
                let _ = sink.submit(GraphOp::CreateThing {
                    kind: intern(KIND_PCI_DEVICE),
                    props,
                });
            }
        }
    }
}

use thing_os::println;

struct UserPciConfig;
impl PciConfigAccess for UserPciConfig {
    fn read_u32(&self, bus: u8, slot: u8, func: u8, offset: u16) -> u32 {
        thing_os::syscalls::sys_pci_read_config(bus, slot, func, offset, 4).unwrap_or(0xFFFFFFFF)
    }
    fn read_u16(&self, bus: u8, slot: u8, func: u8, offset: u16) -> u16 {
        thing_os::syscalls::sys_pci_read_config(bus, slot, func, offset, 2).unwrap_or(0xFFFF) as u16
    }
    fn read_u8(&self, bus: u8, slot: u8, func: u8, offset: u16) -> u8 {
        thing_os::syscalls::sys_pci_read_config(bus, slot, func, offset, 1).unwrap_or(0xFF) as u8
    }
}

use abi::wire::graph::WireProp;
use abi::wire::common::UserSlice;

struct UserGraphSink;
impl GraphSink for UserGraphSink {
    fn submit(&mut self, op: GraphOp) -> Result<(), &'static str> {
        match op {
            GraphOp::CreateThing { kind, props } => {
                println!("PCI: Creating thing kind={:?} (sym)", kind);
                
                let mut wire_props = Vec::with_capacity(props.len());
                for (key, value) in props {
                    wire_props.push(WireProp { key, value, _pad: 0 });
                }
                
                let req = thing_os::KernelRequest::ThingCreate {
                    kind,
                    props: UserSlice::from_slice(&wire_props),
                };
                
                match thing_os::syscalls::syscall(req) {
                    thing_os::KernelResponse::ThingCreated { .. } => Ok(()),
                    _ => {
                        println!("PCI: Failed to create thing");
                        Err("Failed to create thing")
                    }
                }
            }
            _ => Ok(()),
        }
    }
}

pub fn driver_main() {
    println!("PCI Driver Starting...");
    let config = UserPciConfig;
    let driver = PciDriver::new(&config);
    let mut sink = UserGraphSink;
    driver.scan_and_publish(&mut sink);
    println!("PCI Driver Finished Scan. Parking.");
    loop {
        // yield
    }
}
