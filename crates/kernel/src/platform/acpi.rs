use crate::bridge::HardwareBridge;
use core::ptr::NonNull;

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

#[derive(Clone, Copy)]
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

pub fn init(bridge: &impl HardwareBridge, rsdp_addr: u64, hhdm: u64) {
    if rsdp_addr == 0 {
        bridge.log("ACPI: No RSDP address provided. Skipping ACPI init.\n");
        return;
    }

    // 1. Verify RSDP
    // SAFETY: We blindly trust rsdp_addr is valid.
    // Check if rsdp_addr is already virtual (Limine standard)
    let rsdp_virt = if rsdp_addr >= hhdm {
        rsdp_addr
    } else {
        to_virt(rsdp_addr, hhdm).as_ptr() as u64
    };

    let rsdp_ptr = rsdp_virt as *const Rsdp;
    let rsdp = unsafe { core::ptr::read_unaligned(rsdp_ptr) };

    if &rsdp.signature != b"RSD PTR " {
        bridge.log("ACPI: Invalid RSDP signature\n");
        return;
    }

    // 2. Use XSDT if revision > 0, else RSDT
    // Use u64 for address checks to avoid unaligned reference hazards
    let xsdt_addr_val = rsdp.xsdt_addr;
    let rsdt_addr_val = rsdp.rsdt_addr;

    let xsdt_addr = if rsdp.revision > 0 && xsdt_addr_val != 0 {
        Some(xsdt_addr_val)
    } else {
        None
    };

    if let Some(addr) = xsdt_addr {
        unsafe {
            parse_xsdt(bridge, addr, hhdm);
        }
    } else {
        unsafe {
            parse_rsdt(bridge, rsdt_addr_val as u64, hhdm);
        }
    }
}

pub fn to_virt(phys: u64, hhdm: u64) -> NonNull<u8> {
    let virt = phys.wrapping_add(hhdm);
    NonNull::new(virt as *mut u8).unwrap()
}

unsafe fn print_hex(bridge: &impl HardwareBridge, val: u64) {
    bridge.log("0x");
    let mut printed = false;
    for i in (0..16).rev() {
        let digit = (val >> (i * 4)) & 0xF;
        if digit != 0 || printed || i == 0 {
            let c = if digit < 10 {
                digit as u8 + b'0'
            } else {
                digit as u8 - 10 + b'a'
            };
            bridge.log(core::str::from_utf8_unchecked(&[c]));
            printed = true;
        }
    }
}

unsafe fn parse_xsdt(bridge: &impl HardwareBridge, phys: u64, hhdm: u64) {
    bridge.log("ACPI: Parsing XSDT at ");
    print_hex(bridge, phys);
    bridge.log("\n");

    let ptr = to_virt(phys, hhdm).as_ptr();
    let header = core::ptr::read_unaligned(ptr as *const SdtHeader);

    bridge.log("ACPI: XSDT Length: ");
    print_hex(bridge, header.length as u64);
    bridge.log("\n");

    if (header.length as usize) < core::mem::size_of::<SdtHeader>() {
        bridge.log("ACPI: Invalid XSDT length!\n");
        return;
    }

    let entries_len = header.length as usize - core::mem::size_of::<SdtHeader>();
    let entries_count = entries_len / 8;

    let entries_ptr = ptr.add(core::mem::size_of::<SdtHeader>()) as *const u64;

    for i in 0..entries_count {
        let entry_phys = core::ptr::read_unaligned(entries_ptr.add(i));
        check_table(bridge, entry_phys, hhdm);
    }
}

unsafe fn parse_rsdt(bridge: &impl HardwareBridge, phys: u64, hhdm: u64) {
    bridge.log("ACPI: Parsing RSDT at ");
    print_hex(bridge, phys);
    bridge.log("\n");

    let ptr = to_virt(phys, hhdm).as_ptr();
    let header = core::ptr::read_unaligned(ptr as *const SdtHeader);

    bridge.log("ACPI: RSDT Length: ");
    print_hex(bridge, header.length as u64);
    bridge.log("\n");

    if (header.length as usize) < core::mem::size_of::<SdtHeader>() {
        bridge.log("ACPI: Invalid RSDT length!\n");
        return;
    }

    let entries_len = header.length as usize - core::mem::size_of::<SdtHeader>();
    let entries_count = entries_len / 4;

    let entries_ptr = ptr.add(core::mem::size_of::<SdtHeader>()) as *const u32;

    for i in 0..entries_count {
        let entry_phys = core::ptr::read_unaligned(entries_ptr.add(i));
        check_table(bridge, entry_phys as u64, hhdm);
    }
}

unsafe fn check_table(bridge: &impl HardwareBridge, phys: u64, hhdm: u64) {
    let ptr = to_virt(phys, hhdm).as_ptr();
    let header = core::ptr::read_unaligned(ptr as *const SdtHeader);

    // Log signature
    bridge.log("ACPI: Table ");
    bridge.log(core::str::from_utf8(&header.signature).unwrap_or("????"));
    bridge.log("\n");

    match &header.signature {
        b"HPET" => {
            crate::drivers::hpet::init_table(bridge, phys, hhdm);
        }
        b"APIC" => {
            parse_madt(bridge, phys, hhdm);
        }
        _ => {}
    }
}

// MADT Structures
#[derive(Clone, Copy)]
#[repr(C, packed)]
struct MadtHeader {
    sdt: SdtHeader,
    local_apic_addr: u32,
    flags: u32,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct MadtEntryHeader {
    entry_type: u8,
    length: u8,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct MadtIoApic {
    header: MadtEntryHeader,
    io_apic_id: u8,
    reserved: u8,
    io_apic_addr: u32,
    gsi_base: u32,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct MadtIntOverride {
    header: MadtEntryHeader,
    bus: u8,
    source_irq: u8,
    gsi: u32,
    flags: u16,
}

pub static mut LOCAL_APIC_ADDR: u64 = 0;
pub static mut IO_APIC_ADDR: u64 = 0;
pub static mut IO_APIC_GSI_BASE: u32 = 0;

// Mappings for Legacy IRQ -> GSI
// Index is Legacy IRQ (0..16), Value is GSI
pub static mut ISA_OVERRIDES: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

unsafe fn parse_madt(bridge: &impl HardwareBridge, phys: u64, hhdm: u64) {
    bridge.log("ACPI: Parsing MADT (APIC)\n");

    // READ UNALIGNED
    let sdt_ptr = to_virt(phys, hhdm).as_ptr() as *const MadtHeader;
    let madt = core::ptr::read_unaligned(sdt_ptr);

    LOCAL_APIC_ADDR = madt.local_apic_addr as u64;

    let entries_start =
        (to_virt(phys, hhdm).as_ptr() as *const u8).add(core::mem::size_of::<MadtHeader>());
    let entries_end = (to_virt(phys, hhdm).as_ptr() as *const u8).add(madt.sdt.length as usize);

    let mut ptr = entries_start;
    while ptr < entries_end {
        let entry = core::ptr::read_unaligned(ptr as *const MadtEntryHeader);

        if entry.length == 0 {
            bridge.log("ACPI: Zero length MADT entry! Aborting loop.\n");
            break;
        }

        match entry.entry_type {
            1 => {
                // IO APIC
                let ioapic = core::ptr::read_unaligned(ptr as *const MadtIoApic);
                IO_APIC_ADDR = ioapic.io_apic_addr as u64;
                IO_APIC_GSI_BASE = ioapic.gsi_base;
                bridge.log("ACPI: Found IOAPIC\n");
            }
            2 => {
                // Interrupt Source Override
                let iso = core::ptr::read_unaligned(ptr as *const MadtIntOverride);
                if iso.bus == 0 {
                    // ISA Bus
                    let source = iso.source_irq as usize;
                    if source < 16 {
                        ISA_OVERRIDES[source] = iso.gsi as u8;
                        bridge.log("ACPI: IRQ Override: ");
                        bridge.log("IRQ"); // No formatting
                                           // print_u64(source as u64)
                        bridge.log(" -> GSI ");
                        // print_u64(iso.gsi as u64)
                        bridge.log("\n");
                    }
                }
            }
            _ => {}
        }
        ptr = ptr.add(entry.length as usize);
    }
}
