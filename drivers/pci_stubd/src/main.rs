#![feature(restricted_std)]
#![no_main]

use abi::schema::{keys, kinds, source};
use stem::thing::sys as thingsys;
use stem::thing::ThingId;
use stem::{info, warn};

const MAX_FUNCTIONS: usize = 256;
const MAX_TRACKED: usize = 128;

const CLASS_DISPLAY: u8 = 0x03;
const CLASS_NETWORK: u8 = 0x02;
const CLASS_SERIAL_BUS: u8 = 0x0c;
const CLASS_STORAGE: u8 = 0x01;

const SUBCLASS_NETWORK_OTHER: u8 = 0x80;
const SUBCLASS_USB: u8 = 0x03;
const SUBCLASS_NVME: u8 = 0x08;

const PROGIF_XHCI: u8 = 0x30;
const PROGIF_NVME: u8 = 0x02;

#[derive(Copy, Clone)]
struct PciRule {
    name: &'static str,
    vendor_id: u16,
    device_id: Option<u16>,
    class_code: u8,
    subclass: Option<u8>,
    prog_if: Option<u8>,
    role_kind: &'static str,
}

#[derive(Copy, Clone, Default)]
struct ClaimedDevice {
    id: ThingId,
}

const RULES: [PciRule; 9] = [
    // Discrete NVIDIA mobile GPUs (exact GA107M id + class fallback for this vendor/class)
    PciRule {
        name: "nvidia-ga107m-gpu",
        vendor_id: 0x10de,
        device_id: Some(0x25a2),
        class_code: CLASS_DISPLAY,
        subclass: None,
        prog_if: None,
        role_kind: "drv.display.nvidia",
    },
    PciRule {
        name: "nvidia-display-fallback",
        vendor_id: 0x10de,
        device_id: None,
        class_code: CLASS_DISPLAY,
        subclass: None,
        prog_if: None,
        role_kind: "drv.display.nvidia",
    },
    // AMD iGPU (Rembrandt class)
    PciRule {
        name: "amd-rembrandt-igpu",
        vendor_id: 0x1002,
        device_id: Some(0x1681),
        class_code: CLASS_DISPLAY,
        subclass: None,
        prog_if: None,
        role_kind: "drv.display.amd",
    },
    // AMD Rembrandt USB4 XHCI families (known IDs + class fallback)
    PciRule {
        name: "amd-rembrandt-xhci-161d",
        vendor_id: 0x1022,
        device_id: Some(0x161d),
        class_code: CLASS_SERIAL_BUS,
        subclass: Some(SUBCLASS_USB),
        prog_if: Some(PROGIF_XHCI),
        role_kind: "drv.usb.xhci.amd",
    },
    PciRule {
        name: "amd-rembrandt-xhci-161e",
        vendor_id: 0x1022,
        device_id: Some(0x161e),
        class_code: CLASS_SERIAL_BUS,
        subclass: Some(SUBCLASS_USB),
        prog_if: Some(PROGIF_XHCI),
        role_kind: "drv.usb.xhci.amd",
    },
    PciRule {
        name: "amd-rembrandt-xhci-fallback",
        vendor_id: 0x1022,
        device_id: None,
        class_code: CLASS_SERIAL_BUS,
        subclass: Some(SUBCLASS_USB),
        prog_if: Some(PROGIF_XHCI),
        role_kind: "drv.usb.xhci.amd",
    },
    PciRule {
        name: "realtek-rtl8852be",
        vendor_id: 0x10ec,
        device_id: Some(0xb852),
        class_code: CLASS_NETWORK,
        subclass: Some(SUBCLASS_NETWORK_OTHER),
        prog_if: None,
        role_kind: "drv.net.rtl8852be",
    },
    // WD/SanDisk NVMe families from your lspci tree
    PciRule {
        name: "wd-sn740-nvme",
        vendor_id: 0x15b7,
        device_id: Some(0x5003),
        class_code: CLASS_STORAGE,
        subclass: Some(SUBCLASS_NVME),
        prog_if: Some(PROGIF_NVME),
        role_kind: "drv.storage.nvme.wd",
    },
    PciRule {
        name: "wd-nvme-fallback",
        vendor_id: 0x15b7,
        device_id: None,
        class_code: CLASS_STORAGE,
        subclass: Some(SUBCLASS_NVME),
        prog_if: Some(PROGIF_NVME),
        role_kind: "drv.storage.nvme.wd",
    },
];

fn has_device_id(device_id: u16, rule_device_id: Option<u16>) -> bool {
    match rule_device_id {
        Some(expected) => expected == device_id,
        None => true,
    }
}

fn has_subclass(subclass: u8, rule_subclass: Option<u8>) -> bool {
    match rule_subclass {
        Some(expected) => expected == subclass,
        None => true,
    }
}

fn has_prog_if(prog_if: u8, rule_prog_if: Option<u8>) -> bool {
    match rule_prog_if {
        Some(expected) => expected == prog_if,
        None => true,
    }
}

fn find_rule(
    vendor_id: u16,
    device_id: u16,
    class_code: u8,
    subclass: u8,
    prog_if: u8,
) -> Option<PciRule> {
    for rule in RULES {
        if vendor_id != rule.vendor_id {
            continue;
        }
        if class_code != rule.class_code {
            continue;
        }
        if !has_device_id(device_id, rule.device_id) {
            continue;
        }
        if !has_subclass(subclass, rule.subclass) {
            continue;
        }
        if !has_prog_if(prog_if, rule.prog_if) {
            continue;
        }
        return Some(rule);
    }
    None
}

fn already_claimed(
    id: ThingId,
    tracked: &[ClaimedDevice; MAX_TRACKED],
    tracked_len: usize,
) -> bool {
    tracked[..tracked_len].iter().any(|entry| entry.id == id)
}

fn push_claim(tracked: &mut [ClaimedDevice; MAX_TRACKED], tracked_len: &mut usize, id: ThingId) {
    if *tracked_len >= MAX_TRACKED {
        return;
    }
    tracked[*tracked_len] = ClaimedDevice { id };
    *tracked_len += 1;
}

fn publish_binding(id: ThingId, rule: PciRule, claim: usize) {
    let drv_id = match thingsys::create_node("drv.pci.Bind") {
        Ok(id) => id,
        Err(_) => return,
    };
    let _ = thingsys::prop_set(drv_id, "drv.claim_handle", claim as u64);
    let _ = thingsys::prop_set(drv_id, keys::SOURCE, source::PCI as u64);
    if let Ok(sym) = thingsys::intern(rule.name) {
        let _ = thingsys::prop_set(drv_id, keys::NAME, sym as u64);
    }
    if let Ok(sym) = thingsys::intern(rule.role_kind) {
        let _ = thingsys::prop_set(drv_id, keys::KIND, sym as u64);
    }
    let _ = thingsys::link(drv_id, "DRIVES", id);
    let _ = thingsys::link(id, "MANAGED_BY", drv_id);
}

fn scan_once(
    tracked: &mut [ClaimedDevice; MAX_TRACKED],
    tracked_len: &mut usize,
    funcs: &mut [ThingId; MAX_FUNCTIONS],
) {
    let count = thingsys::find(kinds::DEV_PCI_FUNCTION, funcs).unwrap_or(0);
    for &id in funcs.iter().take(count) {
        if already_claimed(id, tracked, *tracked_len) {
            continue;
        }

        let vendor_id = thingsys::prop_get(id, keys::VENDOR_ID).unwrap_or(0) as u16;
        let device_id = thingsys::prop_get(id, keys::DEVICE_ID).unwrap_or(0) as u16;
        let class_code = thingsys::prop_get(id, keys::CLASS_CODE).unwrap_or(0) as u8;
        let subclass = thingsys::prop_get(id, keys::SUBCLASS_CODE).unwrap_or(0) as u8;
        let prog_if = thingsys::prop_get(id, keys::PROG_IF).unwrap_or(0) as u8;
        let bus = thingsys::prop_get(id, keys::BUS).unwrap_or(0);
        let dev = thingsys::prop_get(id, keys::DEVICE).unwrap_or(0);
        let func = thingsys::prop_get(id, keys::FUNCTION).unwrap_or(0);

        let Some(rule) = find_rule(vendor_id, device_id, class_code, subclass, prog_if) else {
            continue;
        };

        match stem::syscall::device_claim(id.to_u64_lossy()) {
            Ok(claim) => {
                info!(
                    "pci_stubd: bound {} to {:02x}:{:02x}.{} {:04x}:{:04x} class {:02x}:{:02x}:{:02x} claim={}",
                    rule.name,
                    bus as u8,
                    dev as u8,
                    func as u8,
                    vendor_id,
                    device_id,
                    class_code,
                    subclass,
                    prog_if,
                    claim
                );
                publish_binding(id, rule, claim);
                push_claim(tracked, tracked_len, id);
            }
            Err(e) => {
                warn!(
                    "pci_stubd: failed bind {} {:04x}:{:04x} at {:02x}:{:02x}.{}: {:?}",
                    rule.name, vendor_id, device_id, bus as u8, dev as u8, func as u8, e
                );
            }
        }
    }
}

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("pci_stubd: starting pci-id matcher");
    let mut tracked = [ClaimedDevice::default(); MAX_TRACKED];
    let mut tracked_len = 0usize;
    let mut funcs = [ThingId::default(); MAX_FUNCTIONS];

    loop {
        scan_once(&mut tracked, &mut tracked_len, &mut funcs);
        stem::time::sleep_ms(1000);
    }
}
