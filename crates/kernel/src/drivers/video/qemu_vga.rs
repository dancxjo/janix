use crate::bridge::HardwareBridge;
use crate::Kernel;

// Bochs VBE ports
const VBE_DISPI_IOPORT_INDEX: u16 = 0x01CE;
const VBE_DISPI_IOPORT_DATA: u16 = 0x01CF;

// Indexes
const VBE_DISPI_INDEX_ID: u16 = 0x0;
const VBE_DISPI_INDEX_XRES: u16 = 0x1;
const VBE_DISPI_INDEX_YRES: u16 = 0x2;
const VBE_DISPI_INDEX_BPP: u16 = 0x3;
const VBE_DISPI_INDEX_ENABLE: u16 = 0x4;
const VBE_DISPI_INDEX_VIRT_WIDTH: u16 = 0x6;
const VBE_DISPI_INDEX_VIRT_HEIGHT: u16 = 0x7;
const VBE_DISPI_INDEX_X_OFFSET: u16 = 0x8;
const VBE_DISPI_INDEX_Y_OFFSET: u16 = 0x9;

// Values
const VBE_DISPI_DISABLED: u16 = 0x00;
const VBE_DISPI_ENABLED: u16 = 0x01;
const VBE_DISPI_LFB_ENABLED: u16 = 0x40;

use thing_models::core::pci::PciDeviceBody;

fn write_reg(bridge: &impl HardwareBridge, index: u16, val: u16) {
    bridge.port_outw(VBE_DISPI_IOPORT_INDEX, index);
    bridge.port_outw(VBE_DISPI_IOPORT_DATA, val);
}

fn read_reg(bridge: &impl HardwareBridge, index: u16) -> u16 {
    bridge.port_outw(VBE_DISPI_IOPORT_INDEX, index);
    bridge.port_inw(VBE_DISPI_IOPORT_DATA)
}

pub fn init<B: HardwareBridge>(
    k: &mut Kernel<B>,
    pci_devices: &[PciDeviceBody],
) -> Option<(u64, u64)> {
    // 1. Check if supported (Bochs VBE ID)
    let id = read_reg(&k.bridge, VBE_DISPI_INDEX_ID);
    if id < 0xB0C0 || id > 0xB0C5 {
        k.bridge.log("DRIVER(qemu_vga): Bochs VBE not detected.\n");
        return None;
    }

    k.bridge.log("DRIVER(qemu_vga): Bochs VBE detected (ID: ");
    // crate::print_hex(id as u64); // Removed dep
    k.bridge.log(")\n");

    // 2. Find LFB address from PCI (Vendor 0x1234, Device 0x1111 for QEMU Standard VGA)
    let mut lfb_phys: u64 = 0;

    for dev in pci_devices {
        // QEMU Standard VGA
        if dev.vendor_id == 0x1234 && dev.device_id == 0x1111 {
            // BAR 0 is usually the VRAM (FrameBuffer)
            let bar0 = dev.bars[0];
            if bar0 != 0 && (bar0 & 1) == 0 {
                // Memory mapped
                // Mask bits 0-3
                lfb_phys = (bar0 & 0xFFFFFFF0) as u64;
                k.bridge.log("DRIVER(qemu_vga): Found LFB via PCI\n");
            }
            break;
        }
    }

    if lfb_phys == 0 {
        k.bridge
            .log("DRIVER(qemu_vga): Could not find LFB address via PCI. Aborting.\n");
        return None;
    }

    let width = 1024;
    let height = 768;
    let bpp = 32;

    let fb_size = (width as u64) * (height as u64) * ((bpp / 8) as u64);

    // We publish the Thing.

    use abi::wire::typed::{CodecId, TypeId, TypedBytes};
    use thing_models::builtins::core_kinds::DisplayFramebufferBody;
    use thing_models::builtins::ids::THING_LINK_KIND;
    use thing_models::builtins::ids::{
        THING_BOOT_ROOT, THING_DISPLAY_FRAMEBUFFER_KIND, THING_HAS_DEVICE_KIND,
    };
    use thing_models::link::LinkBody;
    use thing_models::value::ThingBody;

    // Use standard user address
    let user_virt_addr = 0x1_0000_0000u64;

    let fb_body = DisplayFramebufferBody {
        width: width as u64,
        height: height as u64,
        pitch: (width * (bpp / 8)) as u64,
        format: 32,
        address: user_virt_addr,
    };

    let bytes = postcard::to_allocvec(&fb_body).unwrap();
    let tb = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_DISPLAY_FRAMEBUFFER_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: bytes,
    })
    .unwrap();

    let fb_id = k.graph.create_thing(THING_DISPLAY_FRAMEBUFFER_KIND, tb);

    let link = LinkBody {
        from: THING_BOOT_ROOT,
        to: fb_id,
        predicate: THING_HAS_DEVICE_KIND,
    };
    let lb = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_LINK_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: postcard::to_allocvec(&link).unwrap(),
    })
    .unwrap();
    k.graph.create_thing(THING_LINK_KIND, lb);

    set_mode(&k.bridge, width, height, bpp);

    Some((lfb_phys, fb_size))
}

pub fn set_mode(bridge: &impl HardwareBridge, width: u16, height: u16, bpp: u16) {
    write_reg(bridge, VBE_DISPI_INDEX_ENABLE, VBE_DISPI_DISABLED);
    write_reg(bridge, VBE_DISPI_INDEX_XRES, width);
    write_reg(bridge, VBE_DISPI_INDEX_YRES, height);
    write_reg(bridge, VBE_DISPI_INDEX_BPP, bpp);
    write_reg(bridge, VBE_DISPI_INDEX_VIRT_WIDTH, width);
    write_reg(bridge, VBE_DISPI_INDEX_VIRT_HEIGHT, height * 2);
    write_reg(bridge, VBE_DISPI_INDEX_X_OFFSET, 0);
    write_reg(bridge, VBE_DISPI_INDEX_Y_OFFSET, 0);
    write_reg(
        bridge,
        VBE_DISPI_INDEX_ENABLE,
        VBE_DISPI_ENABLED | VBE_DISPI_LFB_ENABLED,
    );
}
