//! ATA Disk Driver (Read-only v0)
//!
//! Userspace driver that detects ATA devices on legacy ports and registers
//! them in the System Graph. Uses ioport_read/write syscalls for PIO access.

#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use core::time::Duration;
use stem::abi::module_manifest::{ManifestHeader, ModuleKind, MANIFEST_MAGIC};
use stem::syscall::{ioport_read, ioport_write};
use stem::thing::sys as thingsys;
use stem::{error, info, warn};

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Service,
    device_kind: *b"dev.storage.ata\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    version: 1,
    _reserved: 0,
};

// ATA port bases (legacy)
const ATA_PRIMARY_IO: u16 = 0x1F0;
const ATA_PRIMARY_CTRL: u16 = 0x3F6;
const ATA_SECONDARY_IO: u16 = 0x170;
const ATA_SECONDARY_CTRL: u16 = 0x376;

// ATA register offsets from IO base
const ATA_REG_DATA: u16 = 0;
const ATA_REG_SECCOUNT: u16 = 2;
const ATA_REG_LBA_LO: u16 = 3;
const ATA_REG_LBA_MID: u16 = 4;
const ATA_REG_LBA_HI: u16 = 5;
const ATA_REG_DRIVE: u16 = 6;
const ATA_REG_STATUS: u16 = 7;
const ATA_REG_COMMAND: u16 = 7;

// ATA commands
const ATA_CMD_IDENTIFY: u8 = 0xEC;
const ATA_CMD_READ_SECTORS: u8 = 0x20; // LBA28
const ATA_CMD_READ_SECTORS_EXT: u8 = 0x24; // LBA48

// Status bits
const ATA_SR_BSY: u8 = 0x80;
const ATA_SR_DRQ: u8 = 0x08;
const ATA_SR_ERR: u8 = 0x01;

struct AtaDisk {
    graph_id: u64,
    io_base: u16,
    is_slave: bool,
    sector_count: u64,
    supports_lba48: bool,
    model: [u8; 40],
}

fn ata_inb(port: u16) -> u8 {
    ioport_read(port as usize, 1) as u8
}

fn ata_inw(port: u16) -> u16 {
    ioport_read(port as usize, 2) as u16
}

fn ata_outb(port: u16, val: u8) {
    ioport_write(port as usize, val as usize, 1);
}

fn wait_bsy_clear(io_base: u16) -> bool {
    for _ in 0..100000 {
        let status = ata_inb(io_base + ATA_REG_STATUS);
        if status & ATA_SR_BSY == 0 {
            return true;
        }
    }
    false
}

fn identify_drive(io_base: u16, ctrl_base: u16, is_slave: bool) -> Option<AtaDisk> {
    // Select drive
    let drive_sel = if is_slave { 0xB0 } else { 0xA0 };
    ata_outb(io_base + ATA_REG_DRIVE, drive_sel);

    // Small delay (read alternate status 4 times)
    for _ in 0..4 {
        ata_inb(ctrl_base);
    }

    // Clear sector count and LBA registers
    ata_outb(io_base + ATA_REG_SECCOUNT, 0);
    ata_outb(io_base + ATA_REG_LBA_LO, 0);
    ata_outb(io_base + ATA_REG_LBA_MID, 0);
    ata_outb(io_base + ATA_REG_LBA_HI, 0);

    // Send IDENTIFY command
    ata_outb(io_base + ATA_REG_COMMAND, ATA_CMD_IDENTIFY);

    // Check if drive exists
    let status = ata_inb(io_base + ATA_REG_STATUS);
    if status == 0 || status == 0xFF {
        return None; // No drive
    }

    // Wait for BSY to clear
    if !wait_bsy_clear(io_base) {
        return None;
    }

    // Check for ATAPI (different signature in LBA mid/hi)
    let lba_mid = ata_inb(io_base + ATA_REG_LBA_MID);
    let lba_hi = ata_inb(io_base + ATA_REG_LBA_HI);
    if lba_mid != 0 || lba_hi != 0 {
        return None; // ATAPI or SATA, not supported
    }

    // Wait for DRQ or ERR
    loop {
        let status = ata_inb(io_base + ATA_REG_STATUS);
        if status & ATA_SR_DRQ != 0 {
            break;
        }
        if status & ATA_SR_ERR != 0 {
            return None;
        }
        if status == 0 {
            return None;
        }
    }

    // Read 256 words of identification data
    let mut ident = [0u16; 256];
    for i in 0..256 {
        ident[i] = ata_inw(io_base + ATA_REG_DATA);
    }

    // Parse identification data
    let supports_lba48 = (ident[83] & (1 << 10)) != 0;

    let sector_count = if supports_lba48 {
        (ident[100] as u64)
            | ((ident[101] as u64) << 16)
            | ((ident[102] as u64) << 32)
            | ((ident[103] as u64) << 48)
    } else {
        (ident[60] as u64) | ((ident[61] as u64) << 16)
    };

    // Extract model string (words 27-46, byte-swapped)
    let mut model = [0u8; 40];
    for i in 0..20 {
        let word = ident[27 + i];
        model[i * 2] = (word >> 8) as u8;
        model[i * 2 + 1] = (word & 0xFF) as u8;
    }

    Some(AtaDisk {
        graph_id: 0,
        io_base,
        is_slave,
        sector_count,
        supports_lba48,
        model,
    })
}

fn read_sectors(disk: &AtaDisk, lba: u64, count: u16, buf: &mut [u8]) -> Result<usize, &'static str> {
    if count == 0 || count > 256 {
        return Err("Invalid sector count");
    }

    let bytes_needed = count as usize * 512;
    if buf.len() < bytes_needed {
        return Err("Buffer too small");
    }

    if !disk.supports_lba48 && lba > 0x0FFFFFFF {
        return Err("LBA out of range for LBA28");
    }

    let drive_sel = if disk.is_slave { 0xF0 } else { 0xE0 };

    if disk.supports_lba48 {
        ata_outb(disk.io_base + ATA_REG_DRIVE, drive_sel);
        ata_outb(disk.io_base + ATA_REG_SECCOUNT, ((count >> 8) & 0xFF) as u8);
        ata_outb(disk.io_base + ATA_REG_LBA_LO, ((lba >> 24) & 0xFF) as u8);
        ata_outb(disk.io_base + ATA_REG_LBA_MID, ((lba >> 32) & 0xFF) as u8);
        ata_outb(disk.io_base + ATA_REG_LBA_HI, ((lba >> 40) & 0xFF) as u8);
        ata_outb(disk.io_base + ATA_REG_SECCOUNT, (count & 0xFF) as u8);
        ata_outb(disk.io_base + ATA_REG_LBA_LO, (lba & 0xFF) as u8);
        ata_outb(disk.io_base + ATA_REG_LBA_MID, ((lba >> 8) & 0xFF) as u8);
        ata_outb(disk.io_base + ATA_REG_LBA_HI, ((lba >> 16) & 0xFF) as u8);
        ata_outb(disk.io_base + ATA_REG_COMMAND, ATA_CMD_READ_SECTORS_EXT);
    } else {
        let lba28 = lba as u32;
        ata_outb(disk.io_base + ATA_REG_DRIVE, drive_sel | ((lba28 >> 24) & 0x0F) as u8);
        ata_outb(disk.io_base + ATA_REG_SECCOUNT, count as u8);
        ata_outb(disk.io_base + ATA_REG_LBA_LO, (lba28 & 0xFF) as u8);
        ata_outb(disk.io_base + ATA_REG_LBA_MID, ((lba28 >> 8) & 0xFF) as u8);
        ata_outb(disk.io_base + ATA_REG_LBA_HI, ((lba28 >> 16) & 0xFF) as u8);
        ata_outb(disk.io_base + ATA_REG_COMMAND, ATA_CMD_READ_SECTORS);
    }

    let mut offset = 0;
    for _ in 0..count {
        loop {
            let status = ata_inb(disk.io_base + ATA_REG_STATUS);
            if status & ATA_SR_ERR != 0 {
                return Err("Read error");
            }
            if status & ATA_SR_DRQ != 0 {
                break;
            }
        }

        for _ in 0..256 {
            let word = ata_inw(disk.io_base + ATA_REG_DATA);
            buf[offset] = (word & 0xFF) as u8;
            buf[offset + 1] = (word >> 8) as u8;
            offset += 2;
        }
    }

    Ok(bytes_needed)
}

fn register_disk(disk: &mut AtaDisk, channel: &str, drive: &str) {
    let disk_id = match thingsys::create_node("dev.storage.Disk") {
        Ok(id) => id,
        Err(e) => {
            error!("ATA_DISK: Failed to create node: {:?}", e);
            return;
        }
    };

    disk.graph_id = disk_id.0;
    thingsys::prop_set(disk_id, "sector_size", 512u64).ok();
    thingsys::prop_set(disk_id, "sector_count", disk.sector_count).ok();
    thingsys::prop_set(disk_id, "lba48", if disk.supports_lba48 { 1u64 } else { 0u64 }).ok();
    thingsys::prop_set(disk_id, "interface", 0u64).ok(); // 0 = ATA

    let model_str = core::str::from_utf8(&disk.model).unwrap_or("Unknown").trim();
    info!(
        "ATA_DISK: Registered disk {} ch={} drv={} sectors={} lba48={} model='{}'",
        disk.graph_id, channel, drive, disk.sector_count, disk.supports_lba48, model_str
    );
}

fn hexdump_sector(data: &[u8], max_bytes: usize) {
    let len = data.len().min(max_bytes);
    let mut line = [0u8; 64];
    let mut pos = 0;

    for (i, byte) in data[..len].iter().enumerate() {
        if i > 0 && i % 16 == 0 {
            if let Ok(s) = core::str::from_utf8(&line[..pos]) {
                info!("ATA_DISK: {}", s);
            }
            pos = 0;
        }
        let hi = (*byte >> 4) & 0xF;
        let lo = *byte & 0xF;
        let hex_chars = b"0123456789ABCDEF";
        if pos + 3 < line.len() {
            line[pos] = hex_chars[hi as usize];
            line[pos + 1] = hex_chars[lo as usize];
            line[pos + 2] = b' ';
            pos += 3;
        }
    }
    if pos > 0 {
        if let Ok(s) = core::str::from_utf8(&line[..pos]) {
            info!("ATA_DISK: {}", s);
        }
    }
}

fn demo_read_sector0(disk: &AtaDisk) {
    let mut buf = [0u8; 512];
    match read_sectors(disk, 0, 1, &mut buf) {
        Ok(bytes) => {
            info!("ATA_DISK: Read sector 0 ({} bytes)", bytes);
            hexdump_sector(&buf, 64);
            if buf[510] == 0x55 && buf[511] == 0xAA {
                info!("ATA_DISK: MBR signature detected (0x55AA)");
            }
            if buf[0x1C2] == 0xEE {
                info!("ATA_DISK: GPT protective MBR detected (type 0xEE)");
            }
        }
        Err(e) => {
            warn!("ATA_DISK: Failed to read sector 0: {}", e);
        }
    }
}

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("ATA_DISK: Starting ATA disk driver v0");

    let mut disks: Vec<AtaDisk> = Vec::new();

    // Probe primary channel
    info!("ATA_DISK: Probing primary channel (0x1F0)...");
    if let Some(mut disk) = identify_drive(ATA_PRIMARY_IO, ATA_PRIMARY_CTRL, false) {
        info!("ATA_DISK: Found primary master");
        register_disk(&mut disk, "primary", "master");
        disks.push(disk);
    }
    if let Some(mut disk) = identify_drive(ATA_PRIMARY_IO, ATA_PRIMARY_CTRL, true) {
        info!("ATA_DISK: Found primary slave");
        register_disk(&mut disk, "primary", "slave");
        disks.push(disk);
    }

    // Probe secondary channel
    info!("ATA_DISK: Probing secondary channel (0x170)...");
    if let Some(mut disk) = identify_drive(ATA_SECONDARY_IO, ATA_SECONDARY_CTRL, false) {
        info!("ATA_DISK: Found secondary master");
        register_disk(&mut disk, "secondary", "master");
        disks.push(disk);
    }
    if let Some(mut disk) = identify_drive(ATA_SECONDARY_IO, ATA_SECONDARY_CTRL, true) {
        info!("ATA_DISK: Found secondary slave");
        register_disk(&mut disk, "secondary", "slave");
        disks.push(disk);
    }

    if disks.is_empty() {
        info!("ATA_DISK: No legacy ATA disks found (AHCI/SATA may be present)");
    } else {
        info!("ATA_DISK: Found {} disk(s)", disks.len());
        info!("ATA_DISK: Demo - reading sector 0 from first disk");
        demo_read_sector0(&disks[0]);
    }

    info!("ATA_DISK: Entering service loop");

    // Keep alive so disk Things remain in graph
    loop {
        stem::sleep(Duration::from_secs(60));
    }
}
