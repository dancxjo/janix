use core::ptr::NonNull;
use crate::Bridge;
use hw::HardwareBridge;

// Minimal ACPI Table Headers
#[repr(C, packed)]
struct Rsdp {
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_addr: u32,
    length: u32,
    xsdt_addr: u64,
    ext_checksum: u8,
    reserved: [u8; 3],
}

#[repr(C, packed)]
struct SdtHeader {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: u32,
    creator_id: u32,
    creator_revision: u32,
}

static mut HHDM_OFFSET: u64 = 0;

pub unsafe fn init(rsdp_addr: u64, hhdm: u64) {
    HHDM_OFFSET = hhdm;
    let bridge = Bridge;
    
    // 1. Verify RSDP
    let rsdp = &*(to_virt(rsdp_addr).as_ptr() as *const Rsdp);
    if &rsdp.signature != b"RSD PTR " {
        bridge.log("ACPI: Invalid RSDP signature\n");
        return;
    }

    // 2. Use XSDT if revision > 0, else RSDT
    let xsdt_addr = if rsdp.revision > 0 && rsdp.xsdt_addr != 0 {
        Some(rsdp.xsdt_addr)
    } else {
        None
    };

    if let Some(addr) = xsdt_addr {
        parse_xsdt(addr);
    } else {
        parse_rsdt(rsdp.rsdt_addr as u64);
    }
}

pub(crate) unsafe fn to_virt(phys: u64) -> NonNull<u8> {
    let virt = phys + HHDM_OFFSET;
    NonNull::new(virt as *mut u8).unwrap()
}

unsafe fn parse_xsdt(phys: u64) {
    let bridge = Bridge;
    bridge.log("ACPI: Parsing XSDT\n");
    
    let header = &*(to_virt(phys).as_ptr() as *const SdtHeader);
    let entries_len = header.length as usize - core::mem::size_of::<SdtHeader>();
    let entries_count = entries_len / 8;
    
    let entries_ptr = (to_virt(phys).as_ptr() as *const u8).add(core::mem::size_of::<SdtHeader>()) as *const u64;
    
    for i in 0..entries_count {
        let entry_phys = core::ptr::read_unaligned(entries_ptr.add(i));
        check_table(entry_phys);
    }
}

unsafe fn parse_rsdt(phys: u64) {
    let bridge = Bridge;
    bridge.log("ACPI: Parsing RSDT\n");
    
    let header = &*(to_virt(phys).as_ptr() as *const SdtHeader);
    let entries_len = header.length as usize - core::mem::size_of::<SdtHeader>();
    let entries_count = entries_len / 4;
    
    let entries_ptr = (to_virt(phys).as_ptr() as *const u8).add(core::mem::size_of::<SdtHeader>()) as *const u32;
    
    for i in 0..entries_count {
        let entry_phys = core::ptr::read_unaligned(entries_ptr.add(i));
        check_table(entry_phys as u64);
    }
}

unsafe fn check_table(phys: u64) {
    let header = &*(to_virt(phys).as_ptr() as *const SdtHeader);
    match &header.signature {
        b"HPET" => {
            crate::hpet::init_table(header as *const _ as u64);
        },
        _ => {}
    }
}
