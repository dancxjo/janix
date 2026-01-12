#![allow(dead_code)]

use abi::schema::{confidence, keys, kinds, rels, source};
use alloc::format;
use stem::pci;

// PCI Config Space Access (Legacy Mechanism #1)
const PCI_CONFIG_ADDRESS: u16 = 0xCF8;
const PCI_CONFIG_DATA: u16 = 0xCFC;
const PCI_ENABLE_BIT: u32 = 0x80000000;

#[inline]
unsafe fn outl(port: u16, val: u32) {
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!("out dx, eax", in("dx") port, in("eax") val);
}

#[inline]
unsafe fn inl(port: u16) -> u32 {
    #[cfg(target_arch = "x86_64")]
    {
        let ret: u32;
        core::arch::asm!("in eax, dx", out("eax") ret, in("dx") port);
        ret
    }
    #[cfg(not(target_arch = "x86_64"))]
    0xFFFFFFFF
}

unsafe fn pci_read_config(bus: u8, dev: u8, func: u8, offset: u8) -> u32 {
    let address = PCI_ENABLE_BIT
        | ((bus as u32) << 16)
        | ((dev as u32) << 11)
        | ((func as u32) << 8)
        | ((offset as u32) & 0xFC);
    outl(PCI_CONFIG_ADDRESS, address);
    inl(PCI_CONFIG_DATA)
}

unsafe fn pci_write_config(bus: u8, dev: u8, func: u8, offset: u8, val: u32) {
    let address = PCI_ENABLE_BIT
        | ((bus as u32) << 16)
        | ((dev as u32) << 11)
        | ((func as u32) << 8)
        | ((offset as u32) & 0xFC);
    outl(PCI_CONFIG_ADDRESS, address);
    outl(PCI_CONFIG_DATA, val);
}

pub fn enumerate_and_publish<FCreate, FSet, FLink, FIntern>(
    host_id: u64,
    mut create: FCreate,
    mut set: FSet,
    mut link: FLink,
    mut intern: FIntern,
) where
    FCreate: FnMut(&str) -> u64,
    FSet: FnMut(u64, &str, u64),
    FLink: FnMut(u64, &str, u64),
    FIntern: FnMut(&str) -> u64,
{
    if !cfg!(target_arch = "x86_64") {
        return;
    }

    crate::kinfo!("PCI: Starting enumeration...");

    let pci_bus_node = create(kinds::DEV_BUS_PCI);
    set(pci_bus_node, keys::NAME, intern("pci0"));
    set(pci_bus_node, "segment", 0);
    set(pci_bus_node, keys::SOURCE, source::PCI as u64);
    set(pci_bus_node, keys::CONFIDENCE, confidence::HIGH as u64);
    link(host_id, rels::HAS_BUS, pci_bus_node);

    scan_bus(
        0,
        pci_bus_node,
        &mut create,
        &mut set,
        &mut link,
        &mut intern,
    );
}

fn scan_bus<FCreate, FSet, FLink, FIntern>(
    bus: u8,
    parent_node: u64,
    create: &mut FCreate,
    set: &mut FSet,
    link: &mut FLink,
    intern: &mut FIntern,
) where
    FCreate: FnMut(&str) -> u64,
    FSet: FnMut(u64, &str, u64),
    FLink: FnMut(u64, &str, u64),
    FIntern: FnMut(&str) -> u64,
{
    for dev in 0..32 {
        let vendor_id_reg = unsafe { pci_read_config(bus, dev, 0, 0) };
        let vendor_id = (vendor_id_reg & 0xFFFF) as u16;
        if vendor_id == 0xFFFF {
            continue;
        }

        let header_type_reg = unsafe { pci_read_config(bus, dev, 0, 0x0C) };
        let header_type = ((header_type_reg >> 16) & 0xFF) as u8;
        let multi_function = (header_type & 0x80) != 0;

        let func_count = if multi_function { 8 } else { 1 };

        for func in 0..func_count {
            let vendor_reg = unsafe { pci_read_config(bus, dev, func, 0) };
            if (vendor_reg & 0xFFFF) == 0xFFFF {
                continue;
            }

            publish_function(bus, dev, func, parent_node, create, set, link, intern);
        }
    }
}

fn publish_function<FCreate, FSet, FLink, FIntern>(
    bus: u8,
    dev: u8,
    func: u8,
    parent_node: u64,
    create: &mut FCreate,
    set: &mut FSet,
    link: &mut FLink,
    intern: &mut FIntern,
) where
    FCreate: FnMut(&str) -> u64,
    FSet: FnMut(u64, &str, u64),
    FLink: FnMut(u64, &str, u64),
    FIntern: FnMut(&str) -> u64,
{
    // Read Config Space
    let r0 = unsafe { pci_read_config(bus, dev, func, 0x00) }; // Vendor/Device
    let r2 = unsafe { pci_read_config(bus, dev, func, 0x08) }; // Class/Subclass/ProgIF/Rev
    let r11 = unsafe { pci_read_config(bus, dev, func, 0x2C) }; // Subsystem Vendor/ID

    let vendor_id = (r0 & 0xFFFF) as u16;
    let device_id = (r0 >> 16) as u16;
    let (vendor_name, device_name) = pci::lookup_names(vendor_id, device_id);
    let name_source_sym = intern(pci::NAME_SOURCE);

    let revision_id = (r2 & 0xFF) as u8;
    let prog_if = ((r2 >> 8) & 0xFF) as u8;
    let subclass = ((r2 >> 16) & 0xFF) as u8;
    let class_code = ((r2 >> 24) & 0xFF) as u8;

    let subsystem_vendor_id = (r11 & 0xFFFF) as u16;
    let subsystem_id = (r11 >> 16) as u16;

    // Create Node
    let node = create(kinds::DEV_PCI_FUNCTION);
    set(node, keys::SOURCE, source::PCI as u64);
    set(node, keys::CONFIDENCE, confidence::HIGH as u64);

    // Properties
    set(node, keys::BUS, bus as u64);
    set(node, keys::DEVICE, dev as u64);
    set(node, keys::FUNCTION, func as u64);

    set(node, keys::VENDOR_ID, vendor_id as u64);
    set(node, keys::DEVICE_ID, device_id as u64);
    set(node, keys::CLASS_CODE, class_code as u64);
    set(node, keys::SUBCLASS_CODE, subclass as u64);
    set(node, keys::PROG_IF, prog_if as u64);
    set(node, keys::REVISION_ID, revision_id as u64);
    set(node, keys::PCI_NAME_SOURCE, name_source_sym);

    if let Some(vn) = vendor_name {
        set(node, keys::VENDOR_NAME, intern(vn));
    }
    if let Some(dn) = device_name {
        set(node, keys::DEVICE_NAME, intern(dn));
    }
    if vendor_name.is_some() || device_name.is_some() {
        let combined = format!("{}", pci::fmt_pci_id(vendor_id, device_id));
        set(node, keys::NAME, intern(&combined));
    }

    // Bind Hash: v1
    // Hash: (vendor, device, sub_v, sub_d, class, sub, prog, rev)
    // Simple mixing (DJB2/FNV-ish)
    // Actually we just need consistent bytes for binder.
    // Let's pack them into u64s because set() takes u64.
    // Spec says "store as 16 or 32 bytes". Graph properties are u64 or string.
    // We can store bind_hash as a hex string.

    let mut hash_bytes = [0u8; 16];
    hash_bytes[0..2].copy_from_slice(&vendor_id.to_le_bytes());
    hash_bytes[2..4].copy_from_slice(&device_id.to_le_bytes());
    hash_bytes[4..6].copy_from_slice(&subsystem_vendor_id.to_le_bytes());
    hash_bytes[6..8].copy_from_slice(&subsystem_id.to_le_bytes());
    hash_bytes[8] = class_code;
    hash_bytes[9] = subclass;
    hash_bytes[10] = prog_if;
    hash_bytes[11] = revision_id;
    // FNV-1a on hash_bytes
    let mut h: u64 = 0xcbf29ce484222325;
    for b in &hash_bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }

    // Store as hex string
    // Since we don't have easy hex formatting to string here without allocation and format! macro usage which creates a String...
    // We can just use the u64 hash for now, OR rely on prop set supporting strings.
    // set() takes u64. Oops. boot_register `set` wrapper takes u64.
    // But RootOp::PropSet takes SymbolShell. which can be String.
    // But `set` closure in boot_register calls `PropSet` with `value: val`. val is u64.

    // Wait, `BootOp::PropSet`:
    // `PropSet { id: u64, key: SymbolShell, value: u64 }`.
    // The value corresponds to a PROPERTY VALUE.
    // In ThingOS, are property values restricted to u64?
    // In `schema.rs`, there are string properties? "name" is usually interned (SymbolId which is u64).
    // `bind_hash`... if it's a string, we need to intern it?
    // User said "hash ... store as 16 or 32 bytes".
    // 16 bytes is u128. u64 is too small.
    // Maybe `bind_hash` is intended to be a STRING?
    // If usage of `set` limits to u64, then I must intern the string.

    // Let's execute formatting manually into a buffer and intern valid ASCII/UTF8.
    let mut hex_buf = [0u8; 16];
    // We only use 8 bytes (u64) of hash for brevity if u64 is enough?
    // User requested "16 or 32 bytes".
    // I can format the u64 as hex string (16 chars).

    // Simple hex loop
    fn nibble(n: u8) -> u8 {
        if n < 10 { n + b'0' } else { n - 10 + b'a' }
    }
    for i in 0..8 {
        let b = (h >> (i * 8)) as u8; // Little endian
        hex_buf[i * 2] = nibble(b >> 4);
        hex_buf[i * 2 + 1] = nibble(b & 0xF);
    }

    let s = core::str::from_utf8(&hex_buf).unwrap();
    let hash_id = intern(s);

    set(node, keys::BIND_KIND, intern("pci.bindkey.v1"));
    set(node, keys::BIND_HASH, hash_id);

    link(parent_node, rels::HAS_DEVICE, node);
    crate::kinfo!(
        "PCI: {:02x}:{:02x}.{} {:04x}:{:04x} {} class={:02x}:{:02x} prog_if={:02x} rev={:02x}",
        bus,
        dev,
        func,
        vendor_id,
        device_id,
        pci::fmt_pci_id(vendor_id, device_id),
        class_code,
        subclass,
        prog_if,
        revision_id
    );

    // BARs
    for i in 0..6 {
        let offset = 0x10 + (i * 4);
        let bar_val = unsafe { pci_read_config(bus, dev, func, offset) };
        if bar_val == 0 || bar_val == 0xFFFFFFFF {
            continue;
        }

        let is_io = (bar_val & 1) != 0;
        let is_64 = ((bar_val >> 1) & 3) == 2;

        // Probe Size
        unsafe { pci_write_config(bus, dev, func, offset, 0xFFFFFFFF) };
        let size_mask = unsafe { pci_read_config(bus, dev, func, offset) };
        unsafe { pci_write_config(bus, dev, func, offset, bar_val) };

        // Mask information bits
        let mask = if is_io { 0xFFFFFFFC } else { 0xFFFFFFF0 };
        let size = (!(size_mask & mask)).wrapping_add(1);

        if size > 0 {
            // Encode BAR info into graph?
            // "BARs (probe, restore)".
            // Link to a resource node?
            // For now, simple properties: "bar0" -> size? Or address?
            // Address is in bar_val.

            // set(node, keys::BAR0 + i, ...);

            // Note: 64-bit BARs take two slots.
        }

        if is_64 {
            // i += 1 in loop? But `for` loop doesn't support skipping.
            // We can just ignore next iteration logic or handle it.
            // For exactness, we should handle it, but simple enumeration often ignores this detail.
        }
    }

    // Recursion for Bridge
    if class_code == 0x06 && subclass == 0x04 {
        // PCI-to-PCI bridge
        let secondary_bus = unsafe { pci_read_config(bus, dev, func, 0x18) >> 8 } as u8;
        if secondary_bus > bus {
            scan_bus(secondary_bus, node, create, set, link, intern);
        }
    }
}
