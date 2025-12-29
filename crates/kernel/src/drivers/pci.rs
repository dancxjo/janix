use crate::bridge::HardwareBridge;
use alloc::vec::Vec;
use thing_models::core::pci::PciDeviceBody;

const CONFIG_ADDRESS: u16 = 0xCF8;
const CONFIG_DATA: u16 = 0xCFC;

pub fn read_config_32(
    bridge: &impl HardwareBridge,
    bus: u8,
    device: u8,
    func: u8,
    offset: u8,
) -> u32 {
    let address = 0x80000000
        | ((bus as u32) << 16)
        | ((device as u32) << 11)
        | ((func as u32) << 8)
        | ((offset as u32) & 0xFC);

    bridge.port_outd(CONFIG_ADDRESS, address);
    bridge.port_ind(CONFIG_DATA)
}

pub fn read_config_16(
    bridge: &impl HardwareBridge,
    bus: u8,
    device: u8,
    func: u8,
    offset: u8,
) -> u16 {
    let val = read_config_32(bridge, bus, device, func, offset);
    if (offset & 2) != 0 {
        (val >> 16) as u16
    } else {
        (val & 0xFFFF) as u16
    }
}

pub fn read_config_8(
    bridge: &impl HardwareBridge,
    bus: u8,
    device: u8,
    func: u8,
    offset: u8,
) -> u8 {
    let val = read_config_32(bridge, bus, device, func, offset);
    let shift = (offset & 3) * 8;
    ((val >> shift) & 0xFF) as u8
}

pub fn scan_pci(bridge: &impl HardwareBridge) -> Vec<PciDeviceBody> {
    let mut devices = Vec::new();

    for bus in 0..=255 {
        for device in 0..32 {
            // Check Function 0 to see if device exists
            let vendor_id = read_config_16(bridge, bus, device, 0, 0);
            if vendor_id == 0xFFFF {
                continue;
            }

            // Check for Multi-function
            let header_type = read_config_8(bridge, bus, device, 0, 0x0E);
            let func_count = if (header_type & 0x80) != 0 { 8 } else { 1 };

            for func in 0..func_count {
                let vid = read_config_16(bridge, bus, device, func, 0);
                if vid == 0xFFFF {
                    continue;
                }

                let did = read_config_16(bridge, bus, device, func, 2);
                let class_id = read_config_8(bridge, bus, device, func, 0x0B);
                let subclass_id = read_config_8(bridge, bus, device, func, 0x0A);
                let prog_if = read_config_8(bridge, bus, device, func, 0x09);
                let rev_id = read_config_8(bridge, bus, device, func, 0x08);
                let irq = read_config_8(bridge, bus, device, func, 0x3C);

                let mut bars = [0u32; 6];
                // Only Type 0 (Endpoint) has 6 BARs. Type 1 (Bridge) has 2.
                // We crudely read 6 for now, but mask based on header type?
                // Actually header type 0x00 is device, 0x01 is pci-to-pci bridge.
                let htype = read_config_8(bridge, bus, device, func, 0x0E) & 0x7F;

                if htype == 0x00 {
                    for i in 0..6 {
                        bars[i] = read_config_32(bridge, bus, device, func, 0x10 + (i as u8) * 4);
                    }
                }

                devices.push(PciDeviceBody {
                    bus,
                    device,
                    function: func,
                    vendor_id: vid,
                    device_id: did,
                    class_id,
                    subclass_id,
                    prog_if,
                    revision_id: rev_id,
                    bars,
                    irq_line: irq,
                });
            }
        }
    }

    devices
}
