use alloc::vec::Vec;
use core::ptr;
use core::mem::transmute;
use hw::MmioMapper;
use thing_os::println;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XhciCapabilityRegs {
    length: u8,
    reserved: u8,
    hci_version: u16,
    hcs_params1: u32,
    hcs_params2: u32,
    hcs_params3: u32,
    hcc_params1: u32,
    db_off: u32,
    rts_off: u32,
    hcc_params2: u32,
}

pub struct XhciController<'a> {
    mapper: &'a dyn MmioMapper,
    base: usize,
}

impl<'a> XhciController<'a> {
    pub fn new(mapper: &'a dyn MmioMapper, base: usize) -> Self {
        Self { mapper, base }
    }

    pub fn init(&mut self) {
        println!("XHCI: Init at base {:x}", self.base);
        // Map capability registers
        // Assuming base is physical address, we need to map it.
        // Wait, base passed from PCI bar is physical.
        let virt = unsafe { self.mapper.map_mmio(self.base as u64, 4096) };
        println!("XHCI: Mapped to virt {:p}", virt);
        
        let cap_regs = unsafe { &*(virt as *const XhciCapabilityRegs) };
        println!("XHCI: CapRegs: {:?}", cap_regs);
    }
}
