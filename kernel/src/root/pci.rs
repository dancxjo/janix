#![allow(dead_code)]

use abi::schema::{confidence, keys, kinds, rels, source};
use alloc::format;
use stem::pci;

// Wrappers for BootRuntime PCI access
// 0xFFFFFFFF is returned on error to simulate "not present"
pub(crate) unsafe fn pci_read_config(bus: u8, dev: u8, func: u8, offset: u8) -> u32 {
    crate::runtime_base()
        .pci_cfg_read32(bus, dev, func, offset)
        .unwrap_or(0xFFFFFFFF)
}

pub(crate) unsafe fn pci_write_config(bus: u8, dev: u8, func: u8, offset: u8, val: u32) {
    let _ = crate::runtime_base().pci_cfg_write32(bus, dev, func, offset, val);
}

#[inline]
fn pci_read_config_u8(bus: u8, dev: u8, func: u8, offset: u8) -> u8 {
    let aligned = offset & !0x3;
    let shift = (offset & 0x3) * 8;
    let val = unsafe { pci_read_config(bus, dev, func, aligned) };
    ((val >> shift) & 0xFF) as u8
}

#[inline]
fn pci_read_config_u16(bus: u8, dev: u8, func: u8, offset: u8) -> u16 {
    let aligned = offset & !0x3;
    let shift = (offset & 0x2) * 8;
    let val = unsafe { pci_read_config(bus, dev, func, aligned) };
    ((val >> shift) & 0xFFFF) as u16
}

fn find_capability(bus: u8, dev: u8, func: u8, cap_id: u8) -> Option<u8> {
    let status = unsafe { pci_read_config(bus, dev, func, 0x04) };
    let status_bits = ((status >> 16) & 0xFFFF) as u16;
    if (status_bits & 0x10) == 0 {
        return None;
    }

    let mut cap_ptr = pci_read_config_u8(bus, dev, func, 0x34) & 0xFC;
    let mut limit = 0;
    while cap_ptr != 0 && limit < 48 {
        let id = pci_read_config_u8(bus, dev, func, cap_ptr);
        if id == cap_id {
            return Some(cap_ptr);
        }
        cap_ptr = pci_read_config_u8(bus, dev, func, cap_ptr + 1) & 0xFC;
        limit += 1;
    }
    None
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
    crate::kinfo!("PCI: Starting enumeration...");

    // Check if PCI legacy config is supported by attempting to read bus 0 dev 0
    // If it returns NotSupported, we skip enumeration.
    if let Err(abi::errors::Errno::NotSupported) = crate::runtime_base().pci_cfg_read32(0, 0, 0, 0)
    {
        crate::kinfo!("PCI: Legacy config space not supported on this platform. Skipping.");
        return;
    }

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

    // Log BARs for debugging
    for i in 0..6 {
        if bar_addrs[i] != 0 {
            crate::kinfo!(
                "PCI:   BAR{}: phys=0x{:x} size=0x{:x}",
                i,
                bar_addrs[i],
                bar_sizes[i]
            );
        }
    }

    // PCI capabilities: MSI/MSI-X
    let msi_cap = find_capability(bus, dev, func, 0x05);
    let msix_cap = find_capability(bus, dev, func, 0x11);

    if msi_cap.is_some() {
        set(node, keys::MSI_CAPABLE, 1);
    }
    if msix_cap.is_some() {
        set(node, keys::MSIX_CAPABLE, 1);
    }

    // Virtio GPU detection (vendor 0x1af4, class 0x03 display controller)
    if vendor_id == 0x1af4 && class_code == 0x03 {
        crate::kinfo!(
            "PCI: Found virtio display controller at {:02x}:{:02x}.{}",
            bus,
            dev,
            func
        );
        register_virtio_gpu(
            node, bus, dev, func, &bar_addrs, &bar_sizes, msi_cap, msix_cap, create, set, link,
        );
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
        crate::kinfo!(
            "PCI: Found LPC/ISA bridge at {:02x}:{:02x}.{}",
            bus,
            dev,
            func
        );
        publish_lpc_bridge(node, create, set, link, intern);
    }

    // AHCI SATA controller detection - class 0x01, subclass 0x06, prog_if 0x01
    if class_code == 0x01 && subclass == 0x06 && prog_if == 0x01 {
        crate::kinfo!(
            "PCI: Found AHCI SATA controller at {:02x}:{:02x}.{}",
            bus,
            dev,
            func
        );
        register_ahci_controller(
            node, bus, dev, func, &bar_addrs, &bar_sizes, msi_cap, msix_cap,
        );
    }

    // Virtio network device detection (vendor 0x1af4, class 0x02 network controller)
    // Device IDs: 0x1000 (transitional), 0x1041 (modern)
    if vendor_id == 0x1af4 && class_code == 0x02 {
        crate::kinfo!(
            "PCI: Found virtio network controller at {:02x}:{:02x}.{}",
            bus,
            dev,
            func
        );
        register_virtio_net(
            node, bus, dev, func, &bar_addrs, &bar_sizes, msi_cap, msix_cap, create, set, link,
        );
    }
}

/// Register virtio GPU in device registry for userspace claiming
fn register_virtio_gpu(
    graph_id: u64,
    bus: u8,
    dev: u8,
    func: u8,
    bar_addrs: &[u64; 6],
    bar_sizes: &[u64; 6],
    msi_cap: Option<u8>,
    msix_cap: Option<u8>,
    create: &mut impl FnMut(&str) -> u64,
    set: &mut impl FnMut(u64, &str, u64),
    link: &mut impl FnMut(u64, &str, u64),
) {
    use crate::device_registry::{
        DeviceEntry, MsiCapability, MsixCapability, PciLocation, REGISTRY,
    };

    let gpu_node = create(kinds::DEV_DISPLAY_GPU);
    set(gpu_node, keys::SOURCE, source::PCI as u64);
    set(gpu_node, keys::CONFIDENCE, confidence::HIGH as u64);
    link(graph_id, rels::IMPLEMENTS, gpu_node);

    // Parse VirtIO PCI capabilities and publish as graph properties
    parse_virtio_capabilities(bus, dev, func, gpu_node, set);

    let entry = DeviceEntry::new_mmio(kinds::DEV_DISPLAY_GPU, gpu_node, *bar_addrs, *bar_sizes);

    let mut reg = REGISTRY.lock();
    if let Some(idx) = reg.register(entry) {
        let msi_info = msi_cap.map(|offset| {
            let msg_ctrl = pci_read_config_u16(bus, dev, func, offset + 0x2);
            MsiCapability {
                offset,
                is_64bit: (msg_ctrl & (1 << 7)) != 0,
                has_mask: (msg_ctrl & (1 << 8)) != 0,
            }
        });

        let msix_info = msix_cap.map(|offset| {
            let table = unsafe { pci_read_config(bus, dev, func, offset + 0x4) };
            let table_bar = (table & 0x7) as u8;
            let table_offset = table & 0xFFFF_FFF8;
            MsixCapability {
                offset,
                table_bar,
                table_offset,
            }
        });

        let location = PciLocation { bus, dev, func };
        reg.set_pci_info(idx, location, msi_info, msix_info);
        crate::kinfo!(
            "PCI: Registered virtio GPU (graph_id={}, idx={}) BAR0=0x{:x}",
            graph_id,
            idx,
            bar_addrs[0]
        );
    } else {
        crate::kinfo!("PCI: Failed to register virtio GPU - registry full");
    }
}

// VirtIO PCI capability types (vendor-specific cap, 0x09)
const VIRTIO_PCI_CAP_COMMON_CFG: u8 = 1;
const VIRTIO_PCI_CAP_NOTIFY_CFG: u8 = 2;
const VIRTIO_PCI_CAP_ISR_CFG: u8 = 3;
const VIRTIO_PCI_CAP_DEVICE_CFG: u8 = 4;

/// Parse VirtIO PCI capabilities and publish offsets as graph properties
fn parse_virtio_capabilities(
    bus: u8,
    dev: u8, 
    func: u8,
    gpu_node: u64,
    set: &mut impl FnMut(u64, &str, u64),
) {
    // Traverse PCI capability list looking for vendor-specific (0x09)
    let status = unsafe { pci_read_config(bus, dev, func, 0x04) };
    let status_bits = ((status >> 16) & 0xFFFF) as u16;
    if (status_bits & 0x10) == 0 {
        crate::kinfo!("PCI: VirtIO device has no capabilities list");
        return;
    }

    let mut cap_ptr = pci_read_config_u8(bus, dev, func, 0x34) & 0xFC;
    let mut limit = 0;
    
    while cap_ptr != 0 && limit < 48 {
        let cap_id = pci_read_config_u8(bus, dev, func, cap_ptr);
        
        // Vendor-specific capability (VirtIO uses this)
        if cap_id == 0x09 {
            // VirtIO PCI capability structure:
            // +0: cap_vndr (0x09)
            // +1: cap_next
            // +2: cap_len
            // +3: cfg_type (1=common, 2=notify, 3=isr, 4=device, 5=pci)
            // +4: bar
            // +5-7: padding
            // +8-11: offset (u32)
            // +12-15: length (u32)
            // For notify cap:
            // +16-19: notify_off_multiplier (u32)
            
            let cfg_type = pci_read_config_u8(bus, dev, func, cap_ptr + 3);
            let bar = pci_read_config_u8(bus, dev, func, cap_ptr + 4);
            let offset = unsafe { pci_read_config(bus, dev, func, cap_ptr + 8) };
            
            match cfg_type {
                VIRTIO_PCI_CAP_COMMON_CFG => {
                    set(gpu_node, keys::VIRTIO_COMMON_BAR, bar as u64);
                    set(gpu_node, keys::VIRTIO_COMMON_OFFSET, offset as u64);
                    crate::kinfo!("PCI: VirtIO common_cfg BAR{} offset=0x{:x}", bar, offset);
                }
                VIRTIO_PCI_CAP_NOTIFY_CFG => {
                    let multiplier = unsafe { pci_read_config(bus, dev, func, cap_ptr + 16) };
                    set(gpu_node, keys::VIRTIO_NOTIFY_BAR, bar as u64);
                    set(gpu_node, keys::VIRTIO_NOTIFY_OFFSET, offset as u64);
                    set(gpu_node, keys::VIRTIO_NOTIFY_MULTIPLIER, multiplier as u64);
                    crate::kinfo!("PCI: VirtIO notify_cfg BAR{} offset=0x{:x} mult={}", bar, offset, multiplier);
                }
                VIRTIO_PCI_CAP_ISR_CFG => {
                    set(gpu_node, keys::VIRTIO_ISR_BAR, bar as u64);
                    set(gpu_node, keys::VIRTIO_ISR_OFFSET, offset as u64);
                }
                VIRTIO_PCI_CAP_DEVICE_CFG => {
                    set(gpu_node, keys::VIRTIO_DEVICE_BAR, bar as u64);
                    set(gpu_node, keys::VIRTIO_DEVICE_OFFSET, offset as u64);
                }
                _ => {} // Ignore other types (5=PCI_CFG)
            }
        }
        
        cap_ptr = pci_read_config_u8(bus, dev, func, cap_ptr + 1) & 0xFC;
        limit += 1;
    }
}

/// Register AHCI controller in device registry for userspace claiming
fn register_ahci_controller(
    graph_id: u64,
    bus: u8,
    dev: u8,
    func: u8,
    bar_addrs: &[u64; 6],
    bar_sizes: &[u64; 6],
    msi_cap: Option<u8>,
    msix_cap: Option<u8>,
) {
    use crate::device_registry::{
        DeviceEntry, MsiCapability, MsixCapability, PciLocation, REGISTRY,
    };

    // AHCI uses BAR5 for ABAR (AHCI Base Address Register)
    // But our DeviceEntry stores all BARs anyway
    let entry = DeviceEntry::new_mmio("dev.storage.Ahci", graph_id, *bar_addrs, *bar_sizes);

    let mut reg = REGISTRY.lock();
    if let Some(idx) = reg.register(entry) {
        let msi_info = msi_cap.map(|offset| {
            let msg_ctrl = pci_read_config_u16(bus, dev, func, offset + 0x2);
            MsiCapability {
                offset,
                is_64bit: (msg_ctrl & (1 << 7)) != 0,
                has_mask: (msg_ctrl & (1 << 8)) != 0,
            }
        });

        let msix_info = msix_cap.map(|offset| {
            let table = unsafe { pci_read_config(bus, dev, func, offset + 0x4) };
            let table_bar = (table & 0x7) as u8;
            let table_offset = table & 0xFFFF_FFF8;
            MsixCapability {
                offset,
                table_bar,
                table_offset,
            }
        });

        let location = PciLocation { bus, dev, func };
        reg.set_pci_info(idx, location, msi_info, msix_info);
        crate::kinfo!(
            "PCI: Registered AHCI controller (graph_id={}, idx={}) BAR5=0x{:x}",
            graph_id,
            idx,
            bar_addrs[5]
        );
    } else {
        crate::kinfo!("PCI: Failed to register AHCI controller - registry full");
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
        use crate::device_registry::{CMOS_IOPORT_RANGES, DeviceEntry, REGISTRY};
        let mut reg = REGISTRY.lock();
        reg.register(DeviceEntry::new_legacy(
            kinds::DEV_RTC_CMOS,
            CMOS_IOPORT_RANGES,
            cmos_id,
        ));
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
        use crate::device_registry::{DeviceEntry, PS2_IOPORT_RANGES, REGISTRY};
        let mut reg = REGISTRY.lock();
        reg.register(DeviceEntry::new_legacy(
            kinds::DEV_INPUT_PS2_CONTROLLER,
            PS2_IOPORT_RANGES,
            ps2_id,
        ));
    }

    crate::kinfo!("LPC: Created Legacy IO bus with CMOS and PS/2 controller");
}

/// Register virtio network controller in device registry for userspace claiming
fn register_virtio_net(
    graph_id: u64,
    bus: u8,
    dev: u8,
    func: u8,
    bar_addrs: &[u64; 6],
    bar_sizes: &[u64; 6],
    msi_cap: Option<u8>,
    msix_cap: Option<u8>,
    create: &mut impl FnMut(&str) -> u64,
    set: &mut impl FnMut(u64, &str, u64),
    link: &mut impl FnMut(u64, &str, u64),
) {
    use crate::device_registry::{
        DeviceEntry, MsiCapability, MsixCapability, PciLocation, REGISTRY,
    };

    let net_node = create(kinds::DEV_NET_NIC);
    set(net_node, keys::SOURCE, source::PCI as u64);
    set(net_node, keys::CONFIDENCE, confidence::HIGH as u64);
    link(graph_id, rels::IMPLEMENTS, net_node);

    // Parse VirtIO PCI capabilities and publish as graph properties
    parse_virtio_capabilities(bus, dev, func, net_node, set);

    let entry = DeviceEntry::new_mmio(kinds::DEV_NET_NIC, net_node, *bar_addrs, *bar_sizes);

    let mut reg = REGISTRY.lock();
    if let Some(idx) = reg.register(entry) {
        let msi_info = msi_cap.map(|offset| {
            let msg_ctrl = pci_read_config_u16(bus, dev, func, offset + 0x2);
            MsiCapability {
                offset,
                is_64bit: (msg_ctrl & (1 << 7)) != 0,
                has_mask: (msg_ctrl & (1 << 8)) != 0,
            }
        });

        let msix_info = msix_cap.map(|offset| {
            let table = unsafe { pci_read_config(bus, dev, func, offset + 0x4) };
            let table_bar = (table & 0x7) as u8;
            let table_offset = table & 0xFFFF_FFF8;
            MsixCapability {
                offset,
                table_bar,
                table_offset,
            }
        });

        let location = PciLocation { bus, dev, func };
        reg.set_pci_info(idx, location, msi_info, msix_info);
        crate::kinfo!(
            "PCI: Registered virtio network (graph_id={}, idx={}) BAR0=0x{:x}",
            graph_id,
            idx,
            bar_addrs[0]
        );
    } else {
        crate::kinfo!("PCI: Failed to register virtio network - registry full");
    }
}
