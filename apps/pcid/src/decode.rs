use models::{PciFunctionV1, PciBar, PciCapability};
use crate::access::ConfigAccess;

pub fn decode_function(
    access: &dyn ConfigAccess,
    seg: u16,
    bus: u8,
    dev: u8,
    fun: u8,
) -> Option<PciFunctionV1> {
    let vendor_id = access.read16(seg, bus, dev, fun, 0x00);
    if vendor_id == 0xFFFF {
        return None;
    }
    let device_id = access.read16(seg, bus, dev, fun, 0x02);
    let command = access.read16(seg, bus, dev, fun, 0x04);
    let status = access.read16(seg, bus, dev, fun, 0x06);
    let rev_prog = access.read16(seg, bus, dev, fun, 0x08);
    let revision_id = (rev_prog & 0xFF) as u8;
    let prog_if = (rev_prog >> 8) as u8;
    let class_sub = access.read16(seg, bus, dev, fun, 0x0A);
    let subclass = (class_sub & 0xFF) as u8;
    let class_code = (class_sub >> 8) as u8;
    
    let bist_hdr = access.read16(seg, bus, dev, fun, 0x0E);
    let header_type = (bist_hdr & 0xFF) as u8;

    let mut bars = [PciBar::Unused; 6];
    
    // Only decode BARs for type 0 (endpoint) headers
    if (header_type & 0x7F) == 0 {
        let mut i = 0;
        while i < 6 {
            let offset = 0x10 + (i as u16) * 4;
            let val = access.read32(seg, bus, dev, fun, offset);
            
            if val == 0 || val == 0xFFFF_FFFF {
                i += 1;
                continue;
            }

            if (val & 1) == 1 {
                // IO
                bars[i] = PciBar::Io { addr: val & !3 };
                i += 1;
            } else {
                // MMIO
                let type_bits = (val >> 1) & 3;
                let prefetch = (val & 8) != 0;
                let addr = val & 0xFFFF_FFF0;
                
                if type_bits == 2 {
                    // 64-bit
                    let offset_high = offset + 4;
                    let high = access.read32(seg, bus, dev, fun, offset_high);
                    let full_addr = (addr as u64) | ((high as u64) << 32);
                    bars[i] = PciBar::Mmio64 { addr: full_addr, prefetch };
                    // 64-bit takes two slots, but our PciFunctionV1 has 6 slots.
                    // Usually the second slot is marked unused or skipped.
                    // We'll leave the next slot Unused (implicit) and skip checks
                    i += 2;
                } else {
                    // 32-bit
                    bars[i] = PciBar::Mmio32 { addr, prefetch };
                    i += 1;
                }
            }
        }
    }

    let interrupt = access.read16(seg, bus, dev, fun, 0x3C);
    let irq_line = (interrupt & 0xFF) as u8;
    let irq_pin = (interrupt >> 8) as u8;

    let capabilities = if (status & 0x10) != 0 {
        1 // Present
    } else {
        0
    };

    Some(PciFunctionV1 {
        seg,
        bus,
        dev,
        fun,
        vendor_id,
        device_id,
        class_code,
        subclass,
        prog_if,
        revision_id,
        header_type,
        bars,
        irq_pin,
        irq_line,
        caps_found: capabilities,
    })
}
