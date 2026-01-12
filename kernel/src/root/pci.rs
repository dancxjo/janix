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

    // Simple hex loop
    fn nibble(n: u8) -> u8 {
        if n < 10 { n + b'0' } else { n - 10 + b'a' }
    }
    let mut hex_buf = [0u8; 16];
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

    // BARs - collect MMIO BARs for device registry
    let mut bar_addrs: [u64; 6] = [0; 6];
    let mut bar_sizes: [u64; 6] = [0; 6];
    let mut i = 0usize;
    while i < 6 {
        let offset = (0x10 + (i * 4)) as u8;
        let bar_val = unsafe { pci_read_config(bus, dev, func, offset) };
        if bar_val == 0 || bar_val == 0xFFFFFFFF {
            i += 1;
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

        let base_addr = if is_64 && i < 5 {
            let hi_offset = (0x10 + ((i + 1) * 4)) as u8;
            let hi = unsafe { pci_read_config(bus, dev, func, hi_offset) };
            ((hi as u64) << 32) | ((bar_val & mask) as u64)
        } else {
            (bar_val & mask) as u64
        };

        if !is_io && base_addr != 0 {
            bar_addrs[i] = base_addr;
            bar_sizes[i] = size as u64;
            // Store in graph
            let bar_key = match i {
                0 => keys::BAR0,
                1 => keys::BAR1,
                2 => keys::BAR2,
                3 => keys::BAR3,
                4 => keys::BAR4,
                5 => keys::BAR5,
                _ => keys::BAR0,
            };
            set(node, bar_key, base_addr);
        }

        if is_64 {
            i += 2; // Skip next BAR for 64-bit
        } else {
            i += 1;
        }
    }

    // Virtio GPU detection (vendor 0x1af4, class 0x03 display controller)
    if vendor_id == 0x1af4 && class_code == 0x03 {
        crate::kinfo!("PCI: Found virtio display controller at {:02x}:{:02x}.{}", bus, dev, func);
        register_virtio_gpu(node, &bar_addrs, &bar_sizes);
    }

    // Recursion for PCI-to-PCI Bridge
    if class_code == 0x06 && subclass == 0x04 {
        let secondary_bus = unsafe { pci_read_config(bus, dev, func, 0x18) >> 8 } as u8;
        if secondary_bus > bus {
            scan_bus(secondary_bus, node, create, set, link, intern);
        }
    }

    // LPC/ISA Bridge detection - class 0x06, subclass 0x01
    if class_code == 0x06 && subclass == 0x01 {
        crate::kinfo!("PCI: Found LPC/ISA bridge at {:02x}:{:02x}.{}", bus, dev, func);
        publish_lpc_bridge(node, create, set, link, intern);
    }
}

/// Register virtio GPU in device registry for userspace claiming
fn register_virtio_gpu(graph_id: u64, bar_addrs: &[u64; 6], bar_sizes: &[u64; 6]) {
    use crate::device_registry::{DeviceEntry, REGISTRY};
    
    let entry = DeviceEntry::new_mmio(
        "dev.display.Gpu",
        graph_id,
        *bar_addrs,
        *bar_sizes,
    );
    
    let mut reg = REGISTRY.lock();
    if let Some(idx) = reg.register(entry) {
        crate::kinfo!("PCI: Registered virtio GPU (graph_id={}, idx={}) BAR0=0x{:x}", graph_id, idx, bar_addrs[0]);
    } else {
        crate::kinfo!("PCI: Failed to register virtio GPU - registry full");
    }
}

/// Create the LPC bridge node and legacy IO bus with child devices
fn publish_lpc_bridge<FCreate, FSet, FLink, FIntern>(
    pci_func_node: u64,
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
    // Create LPC bridge node
    let lpc_id = create(kinds::DEV_BRIDGE_LPC);
    set(lpc_id, keys::SOURCE, source::PCI as u64);
    set(lpc_id, keys::CONFIDENCE, confidence::HIGH as u64);
    set(lpc_id, keys::NAME, intern("lpc0"));
    link(pci_func_node, rels::IMPLEMENTS, lpc_id);

    // Create Legacy IO bus
    let lio_id = create(kinds::DEV_BUS_LEGACY_IO);
    set(lio_id, keys::SOURCE, source::PCI as u64);
    set(lio_id, keys::CONFIDENCE, confidence::HIGH as u64);
    set(lio_id, keys::NAME, intern("isa0"));
    link(lpc_id, rels::HAS_BUS, lio_id);

    // Create CMOS device
    let cmos_id = create(kinds::DEV_RTC_CMOS);
    set(cmos_id, keys::SOURCE, source::PCI as u64);
    set(cmos_id, keys::CONFIDENCE, confidence::HIGH as u64);
    set(cmos_id, keys::NAME, intern("rtc0"));
    link(lio_id, rels::HAS_DEVICE, cmos_id);

    // Create CMOS port range descriptor
    let cmos_range = create(kinds::CAP_IOPORT_RANGE);
    set(cmos_range, keys::PORT_START, 0x70);
    set(cmos_range, keys::PORT_END, 0x71);
    link(cmos_id, rels::USES_IOPORTS, cmos_range);

    // Register CMOS in device registry
    {
        use crate::device_registry::{DeviceEntry, REGISTRY, CMOS_IOPORT_RANGES};
        let mut reg = REGISTRY.lock();
        reg.register(DeviceEntry::new_legacy(kinds::DEV_RTC_CMOS, CMOS_IOPORT_RANGES, cmos_id));
    }

    // Create PS/2 Controller device
    let ps2_id = create(kinds::DEV_INPUT_PS2_CONTROLLER);
    set(ps2_id, keys::SOURCE, source::PCI as u64);
    set(ps2_id, keys::CONFIDENCE, confidence::HIGH as u64);
    set(ps2_id, keys::NAME, intern("i8042"));
    link(lio_id, rels::HAS_DEVICE, ps2_id);

    // Create PS/2 port range descriptor
    let ps2_range = create(kinds::CAP_IOPORT_RANGE);
    set(ps2_range, keys::PORT_START, 0x60);
    set(ps2_range, keys::PORT_END, 0x64);
    link(ps2_id, rels::USES_IOPORTS, ps2_range);

    // Register PS/2 in device registry
    {
        use crate::device_registry::{DeviceEntry, REGISTRY, PS2_IOPORT_RANGES};
        let mut reg = REGISTRY.lock();
        reg.register(DeviceEntry::new_legacy(kinds::DEV_INPUT_PS2_CONTROLLER, PS2_IOPORT_RANGES, ps2_id));
    }

    crate::kinfo!("LPC: Created Legacy IO bus with CMOS and PS/2 controller");
}
