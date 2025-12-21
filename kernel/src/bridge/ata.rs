#[cfg(target_arch = "x86_64")]
mod inner {
    use x86_64::instructions::port::{Port, PortReadOnly, PortWriteOnly};
    use thing_models::io::BlockDevice;
    use thing_models::{PropValue, Thing};
    use crate::graph;
    use alloc::string::{String, ToString};
    use alloc::vec;

    const ATA_PRIMARY_DATA: u16 = 0x1F0;
    const ATA_PRIMARY_ERR: u16 = 0x1F1;
    const ATA_PRIMARY_SEC_COUNT: u16 = 0x1F2;
    const ATA_PRIMARY_LBA_LO: u16 = 0x1F3;
    const ATA_PRIMARY_LBA_MID: u16 = 0x1F4;
    const ATA_PRIMARY_LBA_HI: u16 = 0x1F5;
    const ATA_PRIMARY_DRIVE_HEAD: u16 = 0x1F6;
    const ATA_PRIMARY_STATUS: u16 = 0x1F7;
    const ATA_PRIMARY_COMMAND: u16 = 0x1F7;

    const STATUS_BSY: u8 = 0x80;
    const STATUS_DRQ: u8 = 0x08;
    const STATUS_ERR: u8 = 0x01;

    pub fn init() {
        crate::log("ATA: Initializing...");

        unsafe {
            identify();
        }
    }

    unsafe fn wait_busy() {
        let mut port = PortReadOnly::<u8>::new(ATA_PRIMARY_STATUS);
        while port.read() & STATUS_BSY != 0 {}
    }

    unsafe fn wait_drq() {
        let mut port = PortReadOnly::<u8>::new(ATA_PRIMARY_STATUS);
        while port.read() & STATUS_DRQ == 0 {}
    }

    unsafe fn identify() {
        // Select master drive
        let mut drive_head_port = PortWriteOnly::<u8>::new(ATA_PRIMARY_DRIVE_HEAD);
        drive_head_port.write(0xA0);

        // Zero sector count and LBA registers
        let mut sec_count_port = PortWriteOnly::<u8>::new(ATA_PRIMARY_SEC_COUNT);
        let mut lba_lo_port = PortWriteOnly::<u8>::new(ATA_PRIMARY_LBA_LO);
        let mut lba_mid_port = PortWriteOnly::<u8>::new(ATA_PRIMARY_LBA_MID);
        let mut lba_hi_port = PortWriteOnly::<u8>::new(ATA_PRIMARY_LBA_HI);

        sec_count_port.write(0);
        lba_lo_port.write(0);
        lba_mid_port.write(0);
        lba_hi_port.write(0);

        // Send IDENTIFY command
        let mut cmd_port = PortWriteOnly::<u8>::new(ATA_PRIMARY_COMMAND);
        cmd_port.write(0xEC);

        let mut status_port = PortReadOnly::<u8>::new(ATA_PRIMARY_STATUS);
        let status = status_port.read();

        if status == 0 {
            crate::log("ATA: Primary master not found (status=0)");
            return;
        }

        wait_busy();

        // Check for non-ATA devices (e.g. ATAPI)
        // Actually 0x1F4/1F5 are RW. We need ReadOnly wrappers to read.

        let lba_mid_read = PortReadOnly::<u8>::new(ATA_PRIMARY_LBA_MID).read();
        let lba_hi_read = PortReadOnly::<u8>::new(ATA_PRIMARY_LBA_HI).read();

        if lba_mid_read != 0 || lba_hi_read != 0 {
             crate::log("ATA: Device is not ATA (possibly ATAPI)");
             return;
        }

        let status = status_port.read();
        while status & STATUS_ERR != 0 {
             crate::log("ATA: Error during IDENTIFY");
             return;
        }

        while status_port.read() & STATUS_DRQ == 0 {}

        // Read 256 words (512 bytes)
        let mut data_port = Port::<u16>::new(ATA_PRIMARY_DATA);
        let mut buffer = [0u16; 256];
        for i in 0..256 {
            buffer[i] = data_port.read();
        }

        // Extract sector count (words 60-61 for LBA28, 100-103 for LBA48)
        let sectors_28 = (buffer[60] as u32) | ((buffer[61] as u32) << 16);
        let sectors_48 = (buffer[100] as u64) | ((buffer[101] as u64) << 16) | ((buffer[102] as u64) << 32) | ((buffer[103] as u64) << 48);

        let sector_count = if sectors_48 > 0 { sectors_48 } else { sectors_28 as u64 };

        // Extract model string (words 27-46)
        let mut model = String::new();
        for i in 27..47 {
            let word = buffer[i];
            // ATA strings are big-endian pairs
            let b1 = (word >> 8) as u8;
            let b2 = (word & 0xFF) as u8;
            if b1 != 0 { model.push(b1 as char); }
            if b2 != 0 { model.push(b2 as char); }
        }
        let model = model.trim().to_string();

        let msg = alloc::format!("ATA: Found drive '{}', {} sectors", model, sector_count);
        crate::log(&msg);

        // Register BlockDevice Thing
        let bd_kind = crate::symbols::intern(BlockDevice::KIND);
        let props = vec![
            (crate::symbols::intern("sector_size"), PropValue::U64(512)),
            (crate::symbols::intern("sector_count"), PropValue::U64(sector_count)),
            (crate::symbols::intern("transport"), PropValue::Str("ata_pio".to_string())),
            (crate::symbols::intern("model"), PropValue::Str(model)),
        ];

        let id = graph::create_thing(bd_kind, props);
        let msg = alloc::format!("ATA: Created BlockDevice Thing({})", id.0);
        crate::log(&msg);

        // READ SECTOR 0 TEST
        read_sector(0);
    }

    unsafe fn read_sector(lba: u64) {
        crate::log("ATA: Reading sector 0...");

        // wait_busy(); // already checked before

        let mut drive_head_port = PortWriteOnly::<u8>::new(ATA_PRIMARY_DRIVE_HEAD);
        // LBA28 mode, Master (0xE0) | ((lba >> 24) & 0x0F)
        drive_head_port.write(0xE0 | ((lba >> 24) as u8 & 0x0F));

        let mut sec_count_port = PortWriteOnly::<u8>::new(ATA_PRIMARY_SEC_COUNT);
        sec_count_port.write(1);

        let mut lba_lo_port = PortWriteOnly::<u8>::new(ATA_PRIMARY_LBA_LO);
        let mut lba_mid_port = PortWriteOnly::<u8>::new(ATA_PRIMARY_LBA_MID);
        let mut lba_hi_port = PortWriteOnly::<u8>::new(ATA_PRIMARY_LBA_HI);

        lba_lo_port.write(lba as u8);
        lba_mid_port.write((lba >> 8) as u8);
        lba_hi_port.write((lba >> 16) as u8);

        let mut cmd_port = PortWriteOnly::<u8>::new(ATA_PRIMARY_COMMAND);
        cmd_port.write(0x20); // READ SECTORS (with retry)

        wait_busy();
        wait_drq();

        let mut data_port = Port::<u16>::new(ATA_PRIMARY_DATA);
        let mut sector_data = [0u8; 512];

        for i in 0..256 {
            let word = data_port.read();
            sector_data[i*2] = (word & 0xFF) as u8;
            sector_data[i*2+1] = (word >> 8) as u8;
        }

        // Hex dump first 32 bytes
        let mut hex = String::from("Sector 0: ");
        for i in 0..32 {
            hex.push_str(&alloc::format!("{:02X} ", sector_data[i]));
        }
        crate::log(&hex);
    }
}

#[cfg(target_arch = "x86_64")]
pub use inner::*;

#[cfg(not(target_arch = "x86_64"))]
pub fn init() {}
