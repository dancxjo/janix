use abi::schema::{confidence, keys, rels, source};

use crate::device_registry::{DeviceEntry, PciLocation, REGISTRY};

pub struct PciClassInfo {
    pub vendor_id: u16,
    pub class_code: u8,
    pub subclass: u8,
    pub prog_if: u8,
}

pub struct PciStubSpec {
    pub kind: &'static str,
    pub name: &'static str,
    pub log_label: &'static str,
}

pub fn classify_stub(info: PciClassInfo) -> Option<PciStubSpec> {
    match (info.class_code, info.subclass, info.prog_if) {
        // VGA / 3D controllers (non-virtio only)
        (0x03, 0x00 | 0x02, _) if info.vendor_id != 0x1af4 => Some(PciStubSpec {
            kind: abi::schema::kinds::DEV_DISPLAY_GPU_PCI_STUB,
            name: "gpu-pci-stub",
            log_label: "PCI GPU stub",
        }),
        // Ethernet and other wired NIC controllers
        (0x02, 0x00, _) if info.vendor_id != 0x1af4 => Some(PciStubSpec {
            kind: abi::schema::kinds::DEV_NET_PCI_STUB,
            name: "net-pci-stub",
            log_label: "PCI NIC stub",
        }),
        // WLAN / radio network controller
        (0x02, 0x80, _) if info.vendor_id != 0x1af4 => Some(PciStubSpec {
            kind: abi::schema::kinds::DEV_NET_WLAN_PCI_STUB,
            name: "wlan-pci-stub",
            log_label: "PCI WLAN stub",
        }),
        // HD Audio controller
        (0x04, 0x03, _) if info.vendor_id != 0x1af4 => Some(PciStubSpec {
            kind: abi::schema::kinds::DEV_SOUND_HDA_PCI_STUB,
            name: "hda-pci-stub",
            log_label: "PCI HDA stub",
        }),
        // XHCI host controller
        (0x0c, 0x03, 0x30) if info.vendor_id != 0x1af4 => Some(PciStubSpec {
            kind: abi::schema::kinds::DEV_USB_XHCI_PCI_STUB,
            name: "xhci-pci-stub",
            log_label: "PCI XHCI stub",
        }),
        _ => None,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn publish_stub_device(
    graph_id: u64,
    bus: u8,
    dev: u8,
    func: u8,
    vendor_id: u16,
    device_id: u16,
    class_code: u8,
    subclass: u8,
    prog_if: u8,
    spec: PciStubSpec,
    bar_addrs: &[u64; 6],
    bar_sizes: &[u64; 6],
    create: &mut impl FnMut(&str) -> u64,
    set: &mut impl FnMut(u64, &str, u64),
    link: &mut impl FnMut(u64, &str, u64),
    intern: &mut impl FnMut(&str) -> u64,
) {
    let stub_node = create(spec.kind);
    set(stub_node, keys::SOURCE, source::PCI as u64);
    set(stub_node, keys::CONFIDENCE, confidence::MEDIUM as u64);
    set(stub_node, keys::NAME, intern(spec.name));
    set(stub_node, keys::VENDOR_ID, vendor_id as u64);
    set(stub_node, keys::DEVICE_ID, device_id as u64);
    set(stub_node, keys::CLASS_CODE, class_code as u64);
    set(stub_node, keys::SUBCLASS_CODE, subclass as u64);
    set(stub_node, keys::PROG_IF, prog_if as u64);
    set(stub_node, keys::BUS, bus as u64);
    set(stub_node, keys::DEVICE, dev as u64);
    set(stub_node, keys::FUNCTION, func as u64);

    if bar_addrs[0] != 0 {
        set(stub_node, keys::BAR0, bar_addrs[0]);
    }
    if bar_addrs[1] != 0 {
        set(stub_node, keys::BAR1, bar_addrs[1]);
    }
    if bar_addrs[2] != 0 {
        set(stub_node, keys::BAR2, bar_addrs[2]);
    }
    if bar_addrs[3] != 0 {
        set(stub_node, keys::BAR3, bar_addrs[3]);
    }
    if bar_addrs[4] != 0 {
        set(stub_node, keys::BAR4, bar_addrs[4]);
    }
    if bar_addrs[5] != 0 {
        set(stub_node, keys::BAR5, bar_addrs[5]);
    }
    link(graph_id, rels::IMPLEMENTS, stub_node);

    let entry = DeviceEntry::new_mmio(spec.kind, stub_node, *bar_addrs, *bar_sizes);
    let mut reg = REGISTRY.lock();
    if let Some(idx) = reg.register(entry) {
        reg.set_pci_identity(idx, vendor_id, device_id, class_code, subclass, prog_if);
        reg.set_pci_info(idx, PciLocation { bus, dev, func }, None, None);
        crate::kinfo!(
            "PCI: Registered {} (graph_id={}, idx={})",
            spec.log_label,
            graph_id,
            idx
        );
    } else {
        crate::kinfo!("PCI: Failed to register {} - registry full", spec.log_label);
    }
}
