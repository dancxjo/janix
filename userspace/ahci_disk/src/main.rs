//! AHCI (SATA) Disk Driver (v1)
//!
//! Userspace driver for AHCI SATA controllers. Detects SATA devices via
//! PCI enumeration and MMIO access.
//! 
//! Features:
//! - AHCI Controller Initialization and Port Probing
//! - SATA Disk Registration
//! - SATAPI (CD-ROM) support via PACKET commands
//! - Integrated ISO9660 Reader (scans boot CD for files)

#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;
use core::time::Duration;
use stem::abi::module_manifest::{ManifestHeader, ModuleKind, MANIFEST_MAGIC};
use stem::abi::schema::{keys, kinds, rels};
use stem::block::{BlockDevice, BlockError};
use stem::thing::sys as thingsys;
use stem::thing::ThingId;
use stem::{error, info, warn};
use iso9660::{IsoFs, ISO_SECTOR_SIZE};

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Driver,
    device_kind: *b"dev.storage.Ahci\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    version: 1,
    _reserved: 0,
};

const PCI_CLASS_STORAGE: u64 = 0x01;
const PCI_SUBCLASS_SATA: u64 = 0x06;
const PCI_PROGIF_AHCI: u64 = 0x01;

const HBA_CAP: usize = 0x00;
const HBA_GHC: usize = 0x04;
const HBA_PI: usize = 0x0C;
const HBA_VS: usize = 0x10;
const HBA_PORT_BASE: usize = 0x100;

const PORT_CLB: usize = 0x00;
const PORT_CLBU: usize = 0x04;
const PORT_FB: usize = 0x08;
const PORT_FBU: usize = 0x0C;
const PORT_IS: usize = 0x10;
const PORT_IE: usize = 0x14;
const PORT_CMD: usize = 0x18;
const PORT_TFD: usize = 0x20;
const PORT_SIG: usize = 0x24;
const PORT_SSTS: usize = 0x28;
const PORT_SCTL: usize = 0x2C;
const PORT_SERR: usize = 0x30;
const PORT_CI: usize = 0x38;

const SATA_SIG_ATA: u32 = 0x00000101;
const SATA_SIG_ATAPI: u32 = 0xEB140101;
const SATA_SIG_SEMB: u32 = 0xC33C0101;
const SATA_SIG_PM: u32 = 0x96690101;

const PORT_CMD_ST: u32 = 1 << 0;
const PORT_CMD_FRE: u32 = 1 << 4;
const PORT_CMD_FR: u32 = 1 << 14;
const PORT_CMD_CR: u32 = 1 << 15;
const PORT_CMD_ATAPI: u32 = 1 << 24;

const SSTS_DET_MASK: u32 = 0x0F;
const SSTS_DET_PRESENT: u32 = 0x03;
const SSTS_IPM_MASK: u32 = 0x0F00;
const SSTS_IPM_ACTIVE: u32 = 0x0100;

const FIS_TYPE_REG_H2D: u8 = 0x27;
const ATA_CMD_PACKET: u8 = 0xA0;

#[repr(C, align(4096))]
struct DmaBuffer {
    data: [u8; 4096],
}
static mut DMA_BUFFER: DmaBuffer = DmaBuffer { data: [0; 4096] };

struct AhciPort {
    port_num: u32,
    sector_count: u64,
    supports_lba48: bool,
    model: [u8; 40],
    graph_id: u64,
}

// Memory Layout in DMA Buffer (1 page = 4096 bytes)
// 0x000 - 0x400: Command List (32 * 32 = 1024 bytes)
// 0x400 - 0x500: Received FIS (256 bytes)
// 0x500 - 0x580: Command Table (128 bytes)
// 0x600 - 0xE00: Data Buffer (2048 bytes = 1 CDROM sector)
const OFFSET_CMD_LIST: usize = 0x000;
const OFFSET_FIS: usize = 0x400;
const OFFSET_CMD_TABLE: usize = 0x500;
const OFFSET_DATA: usize = 0x600;
const DATA_SIZE: usize = 2048;

#[repr(C, packed)]
struct CommandHeader {
    // DW0
    cfl: u8, // Command FIS length in DWORDS (5 for H2D)
    pm: u8,  // Port Multiplier
    prdtl: u16, // PRDT Length
    // DW1
    prdbc: u32, // PRD Byte Count
    // DW2, 3
    ctba: u32, // Command Table Base Address
    ctbau: u32, // Upper 32-bits
    // DW4-7
    reserved: [u32; 4],
}

#[repr(C, packed)]
struct CommandTable {
    // 0x00
    cfis: [u8; 64], // Command FIS
    // 0x40
    acmd: [u8; 16], // ATAPI Command (SCSI CDB)
    // 0x50
    reserved: [u8; 48],
    // 0x80
    prdt: [PrdtEntry; 1], // 1 entry for our simple needs
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct PrdtEntry {
    dba: u32, // Data Base Address
    dbau: u32, // Upper
    reserved: u32,
    dbc: u32, // Data Byte Count (bit 31 = Interrupt on Completion)
}

fn mmio_read32(base: u64, offset: usize) -> u32 {
    unsafe { core::ptr::read_volatile((base as usize + offset) as *const u32) }
}

fn mmio_write32(base: u64, offset: usize, val: u32) {
    unsafe { core::ptr::write_volatile((base as usize + offset) as *mut u32, val) }
}

fn port_base(hba_base: u64, port: u32) -> u64 {
    hba_base + HBA_PORT_BASE as u64 + (port as u64 * 0x80)
}

fn check_port_type(hba_base: u64, port: u32) -> Option<u32> {
    let pb = port_base(hba_base, port);
    let ssts = mmio_read32(pb, PORT_SSTS);
    let det = ssts & SSTS_DET_MASK;
    let ipm = ssts & SSTS_IPM_MASK;
    if det != SSTS_DET_PRESENT || ipm != SSTS_IPM_ACTIVE {
        return None;
    }
    Some(mmio_read32(pb, PORT_SIG))
}

fn stop_port(hba_base: u64, port: u32) {
    let pb = port_base(hba_base, port);
    let mut cmd = mmio_read32(pb, PORT_CMD);
    cmd &= !PORT_CMD_ST;
    cmd &= !PORT_CMD_FRE;
    mmio_write32(pb, PORT_CMD, cmd);
    
    // Wait for bits to clear
    for _ in 0..1000 {
        let cmd = mmio_read32(pb, PORT_CMD);
        if cmd & PORT_CMD_CR == 0 && cmd & PORT_CMD_FR == 0 {
            break;
        }
    }
}

fn start_port(hba_base: u64, port: u32) {
    let pb = port_base(hba_base, port);
    // Loop wait logic omitted for brevity, assuming stopped
    let mut cmd = mmio_read32(pb, PORT_CMD);
    cmd |= PORT_CMD_FRE;
    mmio_write32(pb, PORT_CMD, cmd);
    cmd |= PORT_CMD_ST;
    mmio_write32(pb, PORT_CMD, cmd);
}

fn find_ahci_controller() -> Option<(ThingId, u64)> {
    let mut pci_funcs = [ThingId([0; 16]); 32];
    let count = match thingsys::find("dev.pci.Function", &mut pci_funcs) {
        Ok(c) => {
            info!("AHCI: Found {} PCI functions", c);
            c
        }
        Err(e) => {
            info!("AHCI: Error finding PCI functions: {:?}", e);
            return None;
        }
    };

    for i in 0..count {
        let func_id = pci_funcs[i];
        let class = thingsys::prop_get(func_id, "class_code").unwrap_or(0);
        let subclass = thingsys::prop_get(func_id, "subclass_code").unwrap_or(0);
        let prog_if = thingsys::prop_get(func_id, "prog_if").unwrap_or(0);

        if class == PCI_CLASS_STORAGE && subclass == PCI_SUBCLASS_SATA && prog_if == PCI_PROGIF_AHCI
        {
            info!("AHCI: Found AHCI controller at PCI func {:?}", func_id);
            let bar5 = thingsys::prop_get(func_id, "bar5").unwrap_or(0);
            if bar5 != 0 {
                info!("AHCI: BAR5=0x{:x}", bar5);
                return Some((func_id, bar5));
            }
        }
    }
    None
}

// -----------------------------------------------------------------------------
// ATAPI Block Device Implementation
// -----------------------------------------------------------------------------

struct AhciAtapiDevice {
    mmio_base: u64,
    port: u32,
    dma_virt: u64,
    dma_phys: u64,
}

impl AhciAtapiDevice {
    fn read_packet(&self, lba: u64, count: u64, buf: &mut [u8]) -> Result<(), BlockError> {
        let pb = port_base(self.mmio_base, self.port);
        
        // Clear interrupt status
        mmio_write32(pb, PORT_IS, 0xFFFFFFFF);

        // Slot 0
        let slot = 0;
        
        // Prepare Command Table
        let cmd_tbl_addr = (self.dma_virt as usize + OFFSET_CMD_TABLE) as *mut CommandTable;
        let ds = 2048; // Reading 1 sector
        let prdt_entry = PrdtEntry {
            dba: (self.dma_phys as usize + OFFSET_DATA) as u32,
            dbau: ((self.dma_phys as usize + OFFSET_DATA) >> 32) as u32,
            reserved: 0,
            dbc: (ds - 1) as u32, // Byte count - 1
        };
        
        unsafe {
            let tbl = &mut *cmd_tbl_addr;
            // Setup FIS (RegH2D)
            tbl.cfis[0] = FIS_TYPE_REG_H2D;
            tbl.cfis[1] = 0x80; // Command (Bit 7)
            tbl.cfis[2] = ATA_CMD_PACKET;
            tbl.cfis[3] = 1; // Feature (Bit 0 = DMA)
            tbl.cfis[4] = 0; // LBA Low
            tbl.cfis[5] = (ds as u32 & 0xFF) as u8;        // LBA Mid (Byte Count Low)
            tbl.cfis[6] = ((ds as u32 >> 8) & 0xFF) as u8; // LBA High (Byte Count High)
            tbl.cfis[7] = 0; // Device
            // Rest 0
            
            // Setup ATAPI CDB (SCSI READ10)
            // Clear ACDB first to avoid garbage
            tbl.acmd = [0; 16];
            
            let lba32 = lba as u32;
            let count32 = 1u32; // Always read 1 sector here
            tbl.acmd[0] = 0x28; // READ(10)
            tbl.acmd[1] = 0;
            tbl.acmd[2] = (lba32 >> 24) as u8;
            tbl.acmd[3] = (lba32 >> 16) as u8;
            tbl.acmd[4] = (lba32 >> 8) as u8;
            tbl.acmd[5] = lba32 as u8;
            tbl.acmd[6] = 0; // Reserved
            tbl.acmd[7] = (count32 >> 8) as u8; // Length MSB
            tbl.acmd[8] = count32 as u8;        // Length LSB
            tbl.acmd[9] = 0; // Control
            
            tbl.prdt[0] = prdt_entry;
        }

        // Prepare Command Header
        let cmd_hdr_addr = (self.dma_virt as usize + OFFSET_CMD_LIST) as *mut CommandHeader;
        unsafe {
            let hdr = &mut *cmd_hdr_addr.add(slot);
            hdr.cfl = 5; // 5 Dwords for FIS
            hdr.pm = 0;
            hdr.prdtl = 1; // 1 PRDT entry
            
            // Set Atapi bit (Bit 5 of Byte 0)
            let mut opts = 5u8;
            opts |= 0x20; // ATAPI
            hdr.cfl = opts;
            
            hdr.prdbc = 0;
            hdr.ctba = (self.dma_phys as usize + OFFSET_CMD_TABLE) as u32;
            hdr.ctbau = ((self.dma_phys as usize + OFFSET_CMD_TABLE) >> 32) as u32;
        }

        // Issue Command via CI (Command Issue)
        // Wait for port to be idle?
        let mut timeout = 100000;
        while (mmio_read32(pb, PORT_TFD) & 0x88) != 0 && timeout > 0 {
            timeout -= 1;
        }
        if timeout == 0 { return Err(BlockError::NotReady); }

        mmio_write32(pb, PORT_CI, 1 << slot);

        // Wait for completion
        loop {
            let ci = mmio_read32(pb, PORT_CI);
            if (ci & (1 << slot)) == 0 {
                break; // Done
            }
            if (mmio_read32(pb, PORT_IS) & (1 << 30)) != 0 {
                let tfd = mmio_read32(pb, PORT_TFD);
                let serr = mmio_read32(pb, PORT_SERR);
                info!("AHCI: IoError on port {}. TFD=0x{:x} (STS=0x{:x} ERR=0x{:x}) SERR=0x{:x}", 
                    self.port, tfd, tfd & 0xFF, (tfd >> 8) & 0xFF, serr);
                
                // Clear error
                mmio_write32(pb, PORT_SERR, serr);
                
                return Err(BlockError::IoError);
            }
        }
        
        // Copy data to user buffer
        let src = unsafe { 
            core::slice::from_raw_parts((self.dma_virt as usize + OFFSET_DATA) as *const u8, 2048) 
        };
        buf[0..2048].copy_from_slice(src);
        
        Ok(())
    }
}

impl BlockDevice for AhciAtapiDevice {
    fn read_sectors(&self, lba: u64, count: u64, buf: &mut [u8]) -> Result<(), BlockError> {
        if count == 0 { return Ok(()); }
        let mut current_lba = lba;
        let mut buf_offset = 0;
        
        for _ in 0..count {
            if buf_offset + 2048 > buf.len() {
                return Err(BlockError::InvalidParam);
            }
            // Read 1 sector at a time
            self.read_packet(current_lba, 1, &mut buf[buf_offset..buf_offset+2048])?;
            current_lba += 1;
            buf_offset += 2048;
        }
        Ok(())
    }

    fn sector_size(&self) -> u64 {
        2048
    }
}

// -----------------------------------------------------------------------------
// ISO Logic (Ported from iso_reader)
// -----------------------------------------------------------------------------

fn initialize_iso_content_source() -> Option<ThingId> {
    let mut sources = [ThingId::default(); 16];
    if let Ok(count) = thingsys::find(kinds::CONTENT_SOURCE, &mut sources) {
        for &source_id in &sources[..count] {
            let kind_sym = thingsys::prop_get(source_id, keys::CONTENT_SOURCE_KIND).unwrap_or(0);
            if kind_sym != 0 {
                let mut buf = [0u8; 64];
                if let Ok(len) = thingsys::describe_symbol(kind_sym as u32, &mut buf) {
                    if core::str::from_utf8(&buf[..len]).unwrap_or("") == "iso9660_disk" {
                        return Some(source_id);
                    }
                }
            }
        }
    }

    match thingsys::create_node(kinds::CONTENT_SOURCE) {
        Ok(source_id) => {
            let kind_sym = thingsys::intern("iso9660_disk").unwrap_or(0);
            let name_sym = thingsys::intern("cdrom0").unwrap_or(0);
            let state_sym = thingsys::intern("ready").unwrap_or(0);
            
            let _ = thingsys::prop_set(source_id, keys::CONTENT_SOURCE_KIND, kind_sym as u64);
            let _ = thingsys::prop_set(source_id, keys::CONTENT_SOURCE_NAME, name_sym as u64);
            let _ = thingsys::prop_set(source_id, keys::CONTENT_SOURCE_PRIORITY, 50u64);
            let _ = thingsys::prop_set(source_id, keys::CONTENT_SOURCE_STATE, state_sym as u64);
            let _ = thingsys::prop_set(source_id, keys::CONTENT_SOURCE_GEN, 1u64);
            
            info!("AHCI: Created ISO ContentSource node");
            Some(source_id)
        }
        Err(_) => None,
    }
}

fn find_host_node() -> Option<ThingId> {
    let mut hosts = [ThingId::default(); 4];
    match thingsys::find("dev.Host", &mut hosts) {
        Ok(count) if count > 0 => Some(hosts[0]),
        _ => None,
    }
}

fn publish_content_file(source_id: ThingId, path: &str, data: &[u8], bs_id: ThingId, size: usize) -> Option<ThingId> {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash_bytes = hasher.finalize();
    let hash = u64::from_le_bytes([
        hash_bytes[0], hash_bytes[1], hash_bytes[2], hash_bytes[3],
        hash_bytes[4], hash_bytes[5], hash_bytes[6], hash_bytes[7],
    ]);

    let name = path.rsplit('/').next().unwrap_or(path);
    // let name = name_raw.to_lowercase(); // Rock Ridge provides correct casing
    // let name = name.as_str();

    let name_lower = name.to_lowercase();
    let mime = if name_lower.ends_with(".svg") { Some("image/svg+xml") }
    else if name_lower.ends_with(".ttf") || name_lower.ends_with(".otf") { Some("application/font-sfnt") }
    else if name_lower.ends_with(".bmp") { Some("image/bmp") }
    else if name_lower.ends_with(".png") { Some("image/png") }
    else if name_lower.ends_with(".am") || name_lower.ends_with(".in") { Some("text/plain") }
    else { None };

    let mut files = [ThingId::default(); 512];
    if let Ok(count) = thingsys::find(kinds::CONTENT_FILE, &mut files) {
        let name_sym = thingsys::intern(name).unwrap_or(0) as u64;
        for &file_id in &files[..count] {
            let existing_name = thingsys::prop_get(file_id, keys::FILE_NAME).unwrap_or(0);
            let existing_source = thingsys::prop_get(file_id, keys::FILE_SOURCE).unwrap_or(0);
            
            if existing_name == name_sym && existing_source == source_id.to_u64_lossy() {
                let old_hash = thingsys::prop_get(file_id, keys::FILE_HASH).unwrap_or(0);
                if old_hash != hash {
                    let _ = thingsys::prop_set(file_id, keys::FILE_BYTESPACE, bs_id.to_u64_lossy());
                    let _ = thingsys::prop_set(file_id, keys::FILE_HASH, hash);
                    let _ = thingsys::prop_set(file_id, keys::FILE_SIZE, size as u64);
                }
                return Some(file_id);
            }
        }
    }

    match thingsys::create_node(kinds::CONTENT_FILE) {
        Ok(file_id) => {
            let name_sym = thingsys::intern(name).unwrap_or(0) as u64;
            let _ = thingsys::prop_set(file_id, keys::FILE_NAME, name_sym);
            let _ = thingsys::prop_set(file_id, keys::FILE_SIZE, size as u64);
            let _ = thingsys::prop_set(file_id, keys::FILE_HASH, hash);
            let _ = thingsys::prop_set(file_id, keys::FILE_BYTESPACE, bs_id.to_u64_lossy());
            let _ = thingsys::prop_set(file_id, keys::FILE_SOURCE, source_id.to_u64_lossy());
            
            if let Some(mime_str) = mime {
                if let Ok(mime_sym) = thingsys::intern(mime_str) {
                    let _ = thingsys::prop_set(file_id, keys::FILE_MIME, mime_sym as u64);
                }
            }
            Some(file_id)
        }
        Err(_) => None,
    }
}

fn publish_iso_file(host: ThingId, source_id: ThingId, path: &str, data: Vec<u8>, index: usize) -> Result<ThingId, &'static str> {
    let size = data.len() as u64;
    let node = thingsys::create_node(kinds::BOOT_MODULE).map_err(|_| "create_node failed")?;
    let name_id = thingsys::intern(path).map_err(|_| "intern name failed")?;
    thingsys::prop_set(node, keys::NAME, name_id as u64).ok();
    thingsys::prop_set(node, keys::SIZE_BYTES, size).ok();
    thingsys::prop_set(node, "index", index as u64).ok();
    thingsys::prop_set(node, keys::SOURCE, 10u64).ok();

    let bs = thingsys::bytespace_create(size as usize, 0, 0).map_err(|_| "bytespace_create failed")?;
    thingsys::bytespace_write(bs, 0, &data).map_err(|_| "bytespace_write failed")?;

    thingsys::prop_set(node, keys::BYTESPACE, bs.to_u64_lossy()).ok();
    thingsys::link(node, rels::BACKED_BY, bs).ok();
    thingsys::link(host, rels::HAS_MODULE, node).ok();

    publish_content_file(source_id, path, &data, bs, size as usize);
    Ok(node)
}

fn scan_and_publish(dev: &dyn BlockDevice, fs: &IsoFs, host: ThingId, source_id: ThingId, dir_lba: u32, dir_size: u32, prefix: String, index: &mut usize) -> usize {
    let mut published = 0;
    let entries = fs.list_dir(dev, dir_lba, dir_size);

    for entry in entries {
        let full_path = if prefix.is_empty() { entry.name.clone() } else { format!("{}/{}", prefix, entry.name) };

        if entry.is_directory {
            published += scan_and_publish(dev, fs, host, source_id, entry.extent_lba, entry.size, full_path, index);
        } else {
            let file = iso9660::IsoFile { extent_lba: entry.extent_lba, size: entry.size };
            match file.read_all(dev) {
                Ok(data) => {
                    let path_with_slash = format!("/{}", full_path);
                    match publish_iso_file(host, source_id, &path_with_slash, data, *index) {
                        Ok(_) => {
                            info!("AHCI: Published '{}' ({} bytes)", path_with_slash, entry.size);
                            published += 1;
                            *index += 1;
                        }
                        Err(e) => warn!("AHCI: Failed to publish '{}': {}", full_path, e),
                    }
                }
                Err(e) => warn!("AHCI: Failed to read '{}': {:?}", full_path, e),
            }
        }
    }
    published
}

fn register_disk(port: &mut AhciPort) {
    let disk_id = match thingsys::create_node("dev.storage.Disk") {
        Ok(id) => id,
        Err(e) => {
            error!("AHCI: Failed to create disk node: {:?}", e);
            return;
        }
    };
    port.graph_id = disk_id.to_u64_lossy();
    thingsys::prop_set(disk_id, "sector_size", 512u64).ok();
    thingsys::prop_set(disk_id, "sector_count", port.sector_count).ok();
    thingsys::prop_set(
        disk_id,
        "lba48",
        if port.supports_lba48 { 1u64 } else { 0u64 },
    )
    .ok();
    thingsys::prop_set(disk_id, "interface", 1u64).ok();
    let model_str = core::str::from_utf8(&port.model)
        .unwrap_or("Unknown")
        .trim();
    info!(
        "AHCI: Registered disk {} port={} sectors={} lba48={} model='{}'",
        port.graph_id, port.port_num, port.sector_count, port.supports_lba48, model_str
    );
}

fn register_atapi_disk(port: &mut AhciPort) {
    let node_id = match thingsys::create_node("dev.storage.Cdrom") {
        Ok(id) => id,
        Err(e) => {
            error!("AHCI: Failed to create CDROM node: {:?}", e);
            return;
        }
    };
    port.graph_id = node_id.to_u64_lossy();
    thingsys::prop_set(node_id, "sector_size", 2048u64).ok();
    thingsys::prop_set(node_id, "interface", 2u64).ok(); // 2 = ATAPI/SATA
    thingsys::prop_set(node_id, "port", port.port_num as u64).ok();
    
    info!("AHCI: Registered CDROM {} port={}", port.graph_id, port.port_num);
}

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("AHCI: Starting AHCI/SATA disk driver v1");

    let (pci_id, _bar5_phys) = match find_ahci_controller() {
        Some(c) => c,
        None => {
            info!("AHCI: No AHCI controller found");
            loop {
                stem::sleep(Duration::from_secs(60));
            }
        }
    };

    let claim_handle = match stem::syscall::device_claim(pci_id.to_u64_lossy()) {
        Ok(h) => {
            info!("AHCI: Claimed PCI device {:?} handle={}", pci_id, h);
            h
        }
        Err(e) => {
            error!("AHCI: Failed to claim: {:?}", e);
            loop {
                stem::sleep(Duration::from_secs(60));
            }
        }
    };

    let mapped_base = match stem::syscall::device_map_mmio(claim_handle, 5) {
        Ok(addr) => {
            info!("AHCI: Mapped ABAR at 0x{:x}", addr);
            addr
        }
        Err(e) => {
            error!("AHCI: Failed to map MMIO: {:?}", e);
            loop {
                stem::sleep(Duration::from_secs(60));
            }
        }
    };

    let cap = mmio_read32(mapped_base, HBA_CAP);
    let version = mmio_read32(mapped_base, HBA_VS);
    let pi = mmio_read32(mapped_base, HBA_PI);
    info!(
        "AHCI: Version {}.{}, {} ports, {} slots, 64-bit: {}",
        (version >> 16) & 0xFFFF,
        version & 0xFFFF,
        ((cap >> 0) & 0x1F) + 1,
        ((cap >> 8) & 0x1F) + 1,
        (cap & (1 << 31)) != 0
    );
    info!("AHCI: Ports implemented: 0x{:x}", pi);

    // Enable AHCI mode
    let mut ghc = mmio_read32(mapped_base, HBA_GHC);
    ghc |= 1 << 31;
    mmio_write32(mapped_base, HBA_GHC, ghc);

    // Use static buffer for DMA to avoid kernel address issues
    let dma_virt = unsafe { DMA_BUFFER.data.as_mut_ptr() as u64 };
    info!("AHCI: DMA virt=0x{:x}", dma_virt);

    let dma_phys = match stem::syscall::device_dma_phys(dma_virt) {
        Ok(addr) => {
            info!("AHCI: DMA phys=0x{:x}", addr);
            addr
        }
        Err(e) => {
            error!("AHCI: DMA phys failed: {:?}", e);
            loop {
                stem::sleep(Duration::from_secs(60));
            }
        }
    };

    // Layout in physical DMA buffer
    let clb = dma_phys + OFFSET_CMD_LIST as u64;
    let fb = dma_phys + OFFSET_FIS as u64;

    let mut ports: Vec<AhciPort> = Vec::new();

    // Probe ports
    for port_num in 0..32u32 {
        if pi & (1 << port_num) == 0 {
            continue;
        }
        info!("AHCI: Probing port {}...", port_num);

        let sig = match check_port_type(mapped_base, port_num) {
            Some(s) => s,
            None => {
                info!("AHCI: Port {} - no device", port_num);
                continue;
            }
        };

        match sig {
            SATA_SIG_ATA => info!("AHCI: Port {} - SATA drive (sig=0x{:x})", port_num, sig),
            SATA_SIG_ATAPI => {
                info!("AHCI: Port {} - SATAPI drive (sig=0x{:x})", port_num, sig);
                
                // Register SATAPI device
                let mut port_info = AhciPort {
                    port_num,
                    sector_count: 0,
                    supports_lba48: false,
                    model: [0u8; 40],
                    graph_id: 0,
                };
                register_atapi_disk(&mut port_info);
                ports.push(port_info);

                // Setup Port for Scanning
                stop_port(mapped_base, port_num);
                let pb = port_base(mapped_base, port_num);
                mmio_write32(pb, PORT_CLB, clb as u32);
                mmio_write32(pb, PORT_CLBU, (clb >> 32) as u32);
                mmio_write32(pb, PORT_FB, fb as u32);
                mmio_write32(pb, PORT_FBU, (fb >> 32) as u32);
                mmio_write32(pb, PORT_IS, 0xFFFFFFFF);
                mmio_write32(pb, PORT_SERR, 0xFFFFFFFF);
                start_port(mapped_base, port_num);

                // Scan ISO
                let atapi_dev = AhciAtapiDevice {
                    mmio_base: mapped_base,
                    port: port_num,
                    dma_virt,
                    dma_phys,
                };

                if let Some(fs) = IsoFs::probe(&atapi_dev) {
                    info!("AHCI: Found ISO9660 filesystem");
                    if let Some(host) = find_host_node() {
                        if let Some(src) = initialize_iso_content_source() {
                            let mut idx = 2000;
                            scan_and_publish(&atapi_dev, &fs, host, src, fs.pvd.root_dir_extent, fs.pvd.root_dir_size, String::new(), &mut idx);
                        }
                    }
                } else {
                    info!("AHCI: No ISO9660 filesystem found.");
                }

                stop_port(mapped_base, port_num);
                continue;
            }
            SATA_SIG_SEMB => {
                info!("AHCI: Port {} - Enclosure (skip)", port_num);
                continue;
            }
            SATA_SIG_PM => {
                info!("AHCI: Port {} - PM (skip)", port_num);
                continue;
            }
            _ => {
                info!("AHCI: Port {} - Unknown sig=0x{:x}", port_num, sig);
                continue;
            }
        }

        stop_port(mapped_base, port_num);

        // Setup port command structures
        let pb = port_base(mapped_base, port_num);
        mmio_write32(pb, PORT_CLB, clb as u32);
        mmio_write32(pb, PORT_CLBU, (clb >> 32) as u32);
        mmio_write32(pb, PORT_FB, fb as u32);
        mmio_write32(pb, PORT_FBU, (fb >> 32) as u32);
        mmio_write32(pb, PORT_IS, 0xFFFFFFFF);
        mmio_write32(pb, PORT_SERR, 0xFFFFFFFF);

        start_port(mapped_base, port_num);

        info!(
            "AHCI: Port {} - SATA drive detected (IDENTIFY deferred - DMA access limitation)",
            port_num
        );

        // Create a placeholder disk entry
        let mut port_info = AhciPort {
            port_num,
            sector_count: 0, // Unknown - would need IDENTIFY
            supports_lba48: false,
            model: [0u8; 40],
            graph_id: 0,
        };
        register_disk(&mut port_info);
        ports.push(port_info);

        stop_port(mapped_base, port_num);
    }

    if ports.is_empty() {
        info!("AHCI: No SATA disks found");
    } else {
        info!("AHCI: Found {} SATA disk(s)", ports.len());
    }

    info!("AHCI: Entering service loop");
    loop {
        stem::sleep(Duration::from_secs(60));
    }
}
