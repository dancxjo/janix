//! ISO Reader Service
//!
//! Scans boot CD-ROM for ISO9660 filesystem and publishes discovered files
//! to the System Graph with the same schema as Limine modules.

#![feature(restricted_std)]
#![no_main]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::time::Duration;
use iso9660::{IsoFs, ISO_SECTOR_SIZE};
use stem::abi::module_manifest::{ManifestHeader, ModuleKind, MANIFEST_MAGIC};
use stem::abi::schema::{keys, kinds, rels};
use stem::block::{BlockDevice, BlockError};
use stem::syscall::{ioport_read, ioport_write};
use stem::thing::sys as thingsys;
use stem::thing::ThingId;
use stem::{info, warn};

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Service,
    device_kind: *b"svc.iso.Reader\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    version: 1,
    _reserved: 0,
};

// ATA port bases
const ATA_PRIMARY_IO: u16 = 0x1F0;
const ATA_PRIMARY_CTRL: u16 = 0x3F6;
const ATA_SECONDARY_IO: u16 = 0x170;
const ATA_SECONDARY_CTRL: u16 = 0x376;

// ATA register offsets
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
const ATA_CMD_IDENTIFY_PACKET: u8 = 0xA1;
const ATA_CMD_PACKET: u8 = 0xA0;

// ATAPI signatures
const ATAPI_SIG_MID: u8 = 0x14;
const ATAPI_SIG_HI: u8 = 0xEB;

// Status bits
const ATA_SR_BSY: u8 = 0x80;
const ATA_SR_DRQ: u8 = 0x08;
const ATA_SR_ERR: u8 = 0x01;

// -----------------------------------------------------------------------------
// Performance Tracking & Indexing
// -----------------------------------------------------------------------------

/// Performance counters for ISO scanning
#[derive(Default)]
struct ScanStats {
    files_visited: usize,
    dirs_visited: usize,
    metadata_published: usize,
    content_materialized: usize,
    bytes_read: u64,
    time_scan_start_ns: u64,
    time_metadata_ns: u64,
    time_materialization_ns: u64,
}

/// Index key for published files to avoid O(files²) behavior
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
struct FileKey {
    source_id: u64, // ThingId as u64
    name_sym: u64,  // Interned string ID
}

/// In-memory index of published files
struct PublishIndex {
    by_key: BTreeMap<FileKey, ThingId>,
}

impl PublishIndex {
    fn new() -> Self {
        Self {
            by_key: BTreeMap::new(),
        }
    }

    fn insert(&mut self, key: FileKey, thing_id: ThingId) {
        self.by_key.insert(key, thing_id);
    }

    fn get(&self, key: &FileKey) -> Option<ThingId> {
        self.by_key.get(key).copied()
    }
}

/// File metadata for lazy materialization
struct FileMetadata {
    extent_lba: u32,
    size: u32,
    path: String,
}

/// Get current monotonic time in nanoseconds (best effort)
fn get_monotonic_ns() -> u64 {
    // Try to read monotonic clock from graph if available
    // Otherwise return 0 (timing will show as 0 but won't crash)
    let mut clocks = [ThingId::default(); 4];
    if let Ok(count) = thingsys::find("dev.time.Clock", &mut clocks) {
        if count > 0 {
            let mono_ns = thingsys::prop_get(clocks[0], "monotonic_ns").unwrap_or(0);
            return mono_ns;
        }
    }
    0
}

/// ATAPI device that implements BlockDevice.
struct AtapiDevice {
    io_base: u16,
    ctrl_base: u16,
    is_slave: bool,
}

impl AtapiDevice {
    fn ata_inb(&self, port: u16) -> u8 {
        ioport_read(port as usize, 1) as u8
    }

    fn ata_inw(&self, port: u16) -> u16 {
        ioport_read(port as usize, 2) as u16
    }

    fn ata_outb(&self, port: u16, val: u8) {
        ioport_write(port as usize, val as usize, 1);
    }

    fn ata_outw(&self, port: u16, val: u16) {
        ioport_write(port as usize, val as usize, 2);
    }

    fn wait_bsy_clear(&self) -> bool {
        for _ in 0..100000 {
            if self.ata_inb(self.io_base + ATA_REG_STATUS) & ATA_SR_BSY == 0 {
                return true;
            }
        }
        false
    }
}

impl BlockDevice for AtapiDevice {
    fn read_sectors(&self, lba: u64, count: u64, buf: &mut [u8]) -> Result<(), BlockError> {
        if count == 0 || count > 32 {
            return Err(BlockError::InvalidParam);
        }
        let bytes_needed = count as usize * ISO_SECTOR_SIZE as usize;
        if buf.len() < bytes_needed {
            return Err(BlockError::InvalidParam);
        }

        // Select drive
        let drive_sel = if self.is_slave { 0xB0 } else { 0xA0 };
        self.ata_outb(self.io_base + ATA_REG_DRIVE, drive_sel);

        // Delay
        for _ in 0..4 {
            self.ata_inb(self.ctrl_base);
        }

        if !self.wait_bsy_clear() {
            return Err(BlockError::NotReady);
        }

        // Set byte count limit
        let byte_count = bytes_needed as u16;
        self.ata_outb(self.io_base + ATA_REG_LBA_MID, (byte_count & 0xFF) as u8);
        self.ata_outb(
            self.io_base + ATA_REG_LBA_HI,
            ((byte_count >> 8) & 0xFF) as u8,
        );

        // Send PACKET command
        self.ata_outb(self.io_base + ATA_REG_COMMAND, ATA_CMD_PACKET);

        // Wait for DRQ
        for _ in 0..100000 {
            let status = self.ata_inb(self.io_base + ATA_REG_STATUS);
            if status & ATA_SR_ERR != 0 {
                return Err(BlockError::IoError);
            }
            if status & ATA_SR_DRQ != 0 {
                break;
            }
        }

        // Build SCSI READ(12) command
        let lba32 = lba as u32;
        let count32 = count as u32;
        let packet: [u16; 6] = [
            0x00A8, // READ(12) opcode
            ((lba32 >> 24) as u16) << 8 | ((lba32 >> 16) as u16 & 0xFF),
            ((lba32 >> 8) as u16 & 0xFF) << 8 | (lba32 as u16 & 0xFF),
            ((count32 >> 24) as u16) << 8 | ((count32 >> 16) as u16 & 0xFF),
            ((count32 >> 8) as u16 & 0xFF) << 8 | (count32 as u16 & 0xFF),
            0x0000,
        ];

        for word in packet {
            self.ata_outw(self.io_base + ATA_REG_DATA, word);
        }

        // Read data
        let mut offset = 0;
        for _ in 0..count {
            loop {
                let status = self.ata_inb(self.io_base + ATA_REG_STATUS);
                if status & ATA_SR_ERR != 0 {
                    return Err(BlockError::IoError);
                }
                if status & ATA_SR_BSY == 0 && status & ATA_SR_DRQ != 0 {
                    break;
                }
            }

            for _ in 0..1024 {
                let word = self.ata_inw(self.io_base + ATA_REG_DATA);
                buf[offset] = (word & 0xFF) as u8;
                buf[offset + 1] = (word >> 8) as u8;
                offset += 2;
            }
        }

        Ok(())
    }

    fn sector_size(&self) -> u64 {
        ISO_SECTOR_SIZE
    }
}

/// Try to detect ATAPI device on a channel.
fn probe_atapi(io_base: u16, ctrl_base: u16, is_slave: bool) -> Option<AtapiDevice> {
    let dev = AtapiDevice {
        io_base,
        ctrl_base,
        is_slave,
    };

    // Select drive
    let drive_sel = if is_slave { 0xB0 } else { 0xA0 };
    dev.ata_outb(io_base + ATA_REG_DRIVE, drive_sel);

    for _ in 0..4 {
        dev.ata_inb(ctrl_base);
    }

    dev.ata_outb(io_base + ATA_REG_SECCOUNT, 0);
    dev.ata_outb(io_base + ATA_REG_LBA_LO, 0);
    dev.ata_outb(io_base + ATA_REG_LBA_MID, 0);
    dev.ata_outb(io_base + ATA_REG_LBA_HI, 0);

    dev.ata_outb(io_base + ATA_REG_COMMAND, ATA_CMD_IDENTIFY);

    let status = dev.ata_inb(io_base + ATA_REG_STATUS);
    if status == 0 || status == 0xFF {
        return None;
    }

    if !dev.wait_bsy_clear() {
        return None;
    }

    let lba_mid = dev.ata_inb(io_base + ATA_REG_LBA_MID);
    let lba_hi = dev.ata_inb(io_base + ATA_REG_LBA_HI);
    if lba_mid != ATAPI_SIG_MID || lba_hi != ATAPI_SIG_HI {
        return None;
    }

    // Consume IDENTIFY PACKET DEVICE response
    dev.ata_outb(io_base + ATA_REG_COMMAND, ATA_CMD_IDENTIFY_PACKET);
    if !dev.wait_bsy_clear() {
        return None;
    }

    for _ in 0..10000 {
        let status = dev.ata_inb(io_base + ATA_REG_STATUS);
        if status & ATA_SR_ERR != 0 {
            return None;
        }
        if status & ATA_SR_DRQ != 0 {
            break;
        }
    }

    // Read and discard identification data
    for _ in 0..256 {
        dev.ata_inw(io_base + ATA_REG_DATA);
    }

    Some(dev)
}

/// Initialize the ISO9660 ContentSource node.
fn initialize_iso_content_source() -> Option<ThingId> {
    // Check if ContentSource already exists (buffer size: max 16 sources is sufficient for boot-time sources)
    let mut sources = [ThingId::default(); 16];
    if let Ok(count) = thingsys::find(kinds::CONTENT_SOURCE, &mut sources) {
        for &source_id in &sources[..count] {
            let kind_sym = thingsys::prop_get(source_id, keys::CONTENT_SOURCE_KIND).unwrap_or(0);
            if kind_sym != 0 {
                let mut buf = [0u8; 64];
                if let Ok(len) = thingsys::describe_symbol(kind_sym as u32, &mut buf) {
                    let kind_str = core::str::from_utf8(&buf[..len]).unwrap_or("");
                    if kind_str == "iso9660_disk" {
                        info!("ISO_READER: Found existing ISO ContentSource");
                        return Some(source_id);
                    }
                }
            }
        }
    }

    // Create new ContentSource for ISO9660
    match thingsys::create_node(kinds::CONTENT_SOURCE) {
        Ok(source_id) => {
            let kind_sym = thingsys::intern("iso9660_disk").unwrap_or(0);
            let name_sym = thingsys::intern("cdrom0").unwrap_or(0);
            let state_sym = thingsys::intern("ready").unwrap_or(0);

            let _ = thingsys::prop_set(source_id, keys::CONTENT_SOURCE_KIND, kind_sym as u64);
            let _ = thingsys::prop_set(source_id, keys::CONTENT_SOURCE_NAME, name_sym as u64);
            let _ = thingsys::prop_set(source_id, keys::CONTENT_SOURCE_PRIORITY, 50u64); // Lower priority than Limine (100)
            let _ = thingsys::prop_set(source_id, keys::CONTENT_SOURCE_STATE, state_sym as u64);
            let _ = thingsys::prop_set(source_id, keys::CONTENT_SOURCE_GEN, 1u64);

            info!("ISO_READER: Created ISO ContentSource node");
            Some(source_id)
        }
        Err(_) => {
            warn!("ISO_READER: Failed to create ContentSource");
            None
        }
    }
}

/// Find the host node to attach ISO modules to.
fn find_host_node() -> Option<ThingId> {
    let mut hosts = [ThingId::default(); 4];
    match thingsys::find("dev.Host", &mut hosts) {
        Ok(count) if count > 0 => Some(hosts[0]),
        _ => None,
    }
}

/// Publish a file from the ISO as both BOOT_MODULE (backward compat) and File node.
/// Supports lazy materialization - if data is None, only metadata is published.
fn publish_iso_file(
    host: ThingId,
    source_id: ThingId,
    path: &str,
    data: Option<Vec<u8>>,
    index: usize,
    publish_index: &mut PublishIndex,
    stats: &mut ScanStats,
) -> Result<ThingId, &'static str> {
    let size = data.as_ref().map(|d| d.len() as u64).unwrap_or(0);

    // Create module node (for backward compatibility with existing consumers)
    let node = thingsys::create_node(kinds::BOOT_MODULE).map_err(|_| "create_node failed")?;

    // Set name (intern string, store symbol ID)
    let name_id = thingsys::intern(path).map_err(|_| "intern name failed")?;
    thingsys::prop_set(node, keys::NAME, name_id as u64).map_err(|_| "set name failed")?;

    // Set size
    thingsys::prop_set(node, keys::SIZE_BYTES, size).map_err(|_| "set size failed")?;

    // Set index
    thingsys::prop_set(node, "index", index as u64).map_err(|_| "set index failed")?;

    // Set source to a distinct value for ISO files
    thingsys::prop_set(node, keys::SOURCE, 10u64).map_err(|_| "set source failed")?; // 10 = ISO

    // Create memfd and write data only if we have content
    let bs = if let Some(ref data_vec) = data {
        let fd = thingsys::memfd_create(path, 0)
            .map_err(|_| "memfd_create failed")?;
        thingsys::write(fd, data_vec).map_err(|_| "write failed")?;

        stats.bytes_read += size;

        thingsys::prop_set(node, keys::BYTESPACE, fd as u64).ok();
        // BACKED_BY link might still be useful for generic graph traversals
        // though it now refers to an FD index.
        thingsys::link(node, rels::BACKED_BY, ThingId::from_u64(fd as u64)).ok();
        Some(ThingId::from_u64(fd as u64))
    } else {
        None
    };

    // Link to host
    thingsys::link(host, rels::HAS_MODULE, node).ok();

    // Extract file name from path
    let name = path.rsplit('/').next().unwrap_or(path);
    let mime = get_mime_type(name);

    // Also create/update File node for unified content access
    publish_content_file_indexed(
        source_id,
        name,
        data.as_deref(),
        bs,
        size as usize,
        mime,
        publish_index,
        stats,
    );

    Ok(node)
}

/// Determine MIME type from file extension
fn get_mime_type(name: &str) -> Option<&'static str> {
    let name_lower = name.to_lowercase();
    if name_lower.ends_with(".svg") {
        Some("image/svg+xml")
    } else if name_lower.ends_with(".ttf") || name_lower.ends_with(".otf") {
        Some("application/font-sfnt")
    } else if name_lower.ends_with(".bmp") {
        Some("image/bmp")
    } else if name_lower.ends_with(".png") {
        Some("image/png")
    } else {
        None
    }
}

/// Create or update a File node for ISO content (using index to avoid O(files²))
fn publish_content_file_indexed(
    source_id: ThingId,
    name: &str,
    data: Option<&[u8]>,    // None for metadata-only publish
    bs_id: Option<ThingId>, // None for metadata-only publish
    size: usize,
    mime: Option<&str>,
    index: &mut PublishIndex,
    stats: &mut ScanStats,
) -> Option<ThingId> {
    let name_sym = thingsys::intern(name).unwrap_or(0) as u64;
    let key = FileKey {
        source_id: source_id.to_u64_lossy(),
        name_sym,
    };

    // Check index first (O(log n) instead of O(n) linear scan)
    if let Some(existing_id) = index.get(&key) {
        // File already exists, update if needed
        if let Some(data_bytes) = data {
            // Compute hash for update check
            use sha2::{Digest, Sha256};
            let mut hasher = Sha256::new();
            hasher.update(data_bytes);
            let hash_bytes = hasher.finalize();
            let hash = u64::from_le_bytes([
                hash_bytes[0],
                hash_bytes[1],
                hash_bytes[2],
                hash_bytes[3],
                hash_bytes[4],
                hash_bytes[5],
                hash_bytes[6],
                hash_bytes[7],
            ]);

            let old_hash = thingsys::prop_get(existing_id, keys::FILE_HASH).unwrap_or(0);
            if old_hash != hash {
                if let Some(bs) = bs_id {
                    let _ =
                        thingsys::prop_set(existing_id, keys::FILE_BYTESPACE, bs.to_u64_lossy());
                }
                let _ = thingsys::prop_set(existing_id, keys::FILE_HASH, hash);
                let _ = thingsys::prop_set(existing_id, keys::FILE_SIZE, size as u64);
                stats.content_materialized += 1;
            }
        }
        return Some(existing_id);
    }

    // Create new file node
    match thingsys::create_node(kinds::CONTENT_FILE) {
        Ok(file_id) => {
            let _ = thingsys::prop_set(file_id, keys::FILE_NAME, name_sym);
            let _ = thingsys::prop_set(file_id, keys::FILE_SIZE, size as u64);
            let _ = thingsys::prop_set(file_id, keys::FILE_SOURCE, source_id.to_u64_lossy());

            // Set hash and bytespace only if we have content
            if let Some(data_bytes) = data {
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                hasher.update(data_bytes);
                let hash_bytes = hasher.finalize();
                let hash = u64::from_le_bytes([
                    hash_bytes[0],
                    hash_bytes[1],
                    hash_bytes[2],
                    hash_bytes[3],
                    hash_bytes[4],
                    hash_bytes[5],
                    hash_bytes[6],
                    hash_bytes[7],
                ]);
                let _ = thingsys::prop_set(file_id, keys::FILE_HASH, hash);

                if let Some(bs) = bs_id {
                    let _ = thingsys::prop_set(file_id, keys::FILE_BYTESPACE, bs.to_u64_lossy());
                }
                stats.content_materialized += 1;
            }

            if let Some(mime_str) = mime {
                if let Ok(mime_sym) = thingsys::intern(mime_str) {
                    let _ = thingsys::prop_set(file_id, keys::FILE_MIME, mime_sym as u64);
                }
            }

            // Add to index
            index.insert(key, file_id);
            stats.metadata_published += 1;

            Some(file_id)
        }
        Err(_) => {
            warn!("ISO_READER: Failed to create File node for '{}'", name);
            None
        }
    }
}

/// Hot-set: files that should be loaded eagerly at boot for responsive UI
fn is_in_hot_set(path: &str) -> bool {
    let path_lower = path.to_lowercase();
    // Load fonts, cursors, and critical UI assets eagerly
    path_lower.contains("/cursor")
        || path_lower.ends_with(".ttf")
        || path_lower.ends_with(".otf")
        || path_lower.contains("/font")
        || path_lower == "/boot.svg" // Boot logo if present
}

/// Recursively scan a directory and publish files.
/// Uses lazy loading: only metadata is published by default, content is loaded on-demand
/// or for files in the hot-set.
fn scan_and_publish(
    dev: &dyn BlockDevice,
    fs: &IsoFs,
    host: ThingId,
    source_id: ThingId,
    dir_lba: u32,
    dir_size: u32,
    prefix: String,
    index: &mut usize,
    publish_index: &mut PublishIndex,
    stats: &mut ScanStats,
) -> usize {
    let mut published = 0;
    let entries = fs.list_dir(dev, dir_lba, dir_size);

    for entry in entries {
        let full_path = if prefix.is_empty() {
            entry.name.clone()
        } else {
            alloc::format!("{}/{}", prefix, entry.name)
        };

        if entry.is_directory {
            stats.dirs_visited += 1;
            // Recurse into subdirectory
            published += scan_and_publish(
                dev,
                fs,
                host,
                source_id,
                entry.extent_lba,
                entry.size,
                full_path,
                index,
                publish_index,
                stats,
            );
        } else {
            stats.files_visited += 1;
            let path_with_slash = alloc::format!("/{}", full_path);

            // Check if this file should be in the hot-set (loaded eagerly)
            let in_hot_set = is_in_hot_set(&path_with_slash);

            let file = iso9660::IsoFile {
                extent_lba: entry.extent_lba,
                size: entry.size,
            };

            if in_hot_set {
                // Hot-set: read content immediately
                match file.read_all(dev) {
                    Ok(data) => {
                        match publish_iso_file(
                            host,
                            source_id,
                            &path_with_slash,
                            Some(data),
                            *index,
                            publish_index,
                            stats,
                        ) {
                            Ok(_) => {
                                info!(
                                    "ISO_READER: Published (hot-set) '{}' ({} bytes)",
                                    path_with_slash, entry.size
                                );
                                published += 1;
                                *index += 1;
                            }
                            Err(e) => {
                                warn!("ISO_READER: Failed to publish '{}': {}", full_path, e);
                            }
                        }
                    }
                    Err(e) => {
                        warn!("ISO_READER: Failed to read '{}': {:?}", full_path, e);
                    }
                }
            } else {
                // Metadata-only: don't read file content yet
                match publish_iso_file(
                    host,
                    source_id,
                    &path_with_slash,
                    None, // No data = metadata only
                    *index,
                    publish_index,
                    stats,
                ) {
                    Ok(_) => {
                        published += 1;
                        *index += 1;
                    }
                    Err(e) => {
                        warn!(
                            "ISO_READER: Failed to publish metadata for '{}': {}",
                            full_path, e
                        );
                    }
                }
            }
        }
    }

    published
}

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("ISO_READER: Starting ISO9660 reader service");

    // Wait for graph and other services to stabilize
    stem::sleep(Duration::from_millis(100));

    // Try to find ATAPI CD-ROM
    let mut atapi_dev: Option<AtapiDevice> = None;

    // Probe secondary channel first (common for CD-ROM)
    info!("ISO_READER: Probing for ATAPI CD-ROM...");
    if let Some(dev) = probe_atapi(ATA_SECONDARY_IO, ATA_SECONDARY_CTRL, false) {
        info!("ISO_READER: Found ATAPI device on secondary master");
        atapi_dev = Some(dev);
    } else if let Some(dev) = probe_atapi(ATA_SECONDARY_IO, ATA_SECONDARY_CTRL, true) {
        info!("ISO_READER: Found ATAPI device on secondary slave");
        atapi_dev = Some(dev);
    } else if let Some(dev) = probe_atapi(ATA_PRIMARY_IO, ATA_PRIMARY_CTRL, false) {
        info!("ISO_READER: Found ATAPI device on primary master");
        atapi_dev = Some(dev);
    } else if let Some(dev) = probe_atapi(ATA_PRIMARY_IO, ATA_PRIMARY_CTRL, true) {
        info!("ISO_READER: Found ATAPI device on primary slave");
        atapi_dev = Some(dev);
    }

    let dev = match atapi_dev {
        Some(d) => d,
        None => {
            info!("ISO_READER: No ATAPI CD-ROM found, exiting");
            loop {
                stem::sleep(Duration::from_secs(60));
            }
        }
    };

    // Probe for ISO9660 filesystem
    let fs = match IsoFs::probe(&dev) {
        Some(f) => {
            let vol_id = iso9660::volume_id_str(&f.pvd);
            info!("ISO_READER: Found ISO9660 filesystem, volume='{}'", vol_id);
            f
        }
        None => {
            warn!("ISO_READER: No ISO9660 filesystem found on CD-ROM");
            loop {
                stem::sleep(Duration::from_secs(60));
            }
        }
    };

    // Find host node
    let host = match find_host_node() {
        Some(h) => h,
        None => {
            warn!("ISO_READER: Could not find host node in graph");
            loop {
                stem::sleep(Duration::from_secs(60));
            }
        }
    };

    info!("ISO_READER: Scanning ISO root directory...");

    // Initialize ContentSource
    let source_id = match initialize_iso_content_source() {
        Some(id) => id,
        None => {
            warn!("ISO_READER: Failed to create ContentSource");
            // Note: Could implement retry logic here, but for now we exit gracefully
            // since the service can be restarted by the init system if needed
            loop {
                stem::sleep(Duration::from_secs(60));
            }
        }
    };

    // Initialize performance tracking and index
    let mut stats = ScanStats::default();
    let mut publish_index = PublishIndex::new();
    stats.time_scan_start_ns = get_monotonic_ns();

    let mut index = 1000; // Start at high index to avoid collision with Limine modules

    let start_metadata = get_monotonic_ns();
    let published = scan_and_publish(
        &dev,
        &fs,
        host,
        source_id,
        fs.pvd.root_dir_extent,
        fs.pvd.root_dir_size,
        String::new(),
        &mut index,
        &mut publish_index,
        &mut stats,
    );
    let end_metadata = get_monotonic_ns();
    stats.time_metadata_ns = end_metadata.saturating_sub(start_metadata);

    // Print performance summary
    let total_time_ns = end_metadata.saturating_sub(stats.time_scan_start_ns);
    let total_time_ms = total_time_ns / 1_000_000;
    let metadata_time_ms = stats.time_metadata_ns / 1_000_000;
    let bytes_read_kb = stats.bytes_read / 1024;

    info!(
        "ISO_READER: Scan complete - {} files, {} dirs in {} ms",
        stats.files_visited, stats.dirs_visited, total_time_ms
    );
    info!(
        "ISO_READER: Metadata: {} nodes published in {} ms",
        stats.metadata_published, metadata_time_ms
    );
    info!(
        "ISO_READER: Content: {} files materialized, {} KB read",
        stats.content_materialized, bytes_read_kb
    );
    info!(
        "ISO_READER: Published {} total entries from ISO to graph",
        published
    );

    // Service loop
    info!("ISO_READER: Entering service loop");
    loop {
        stem::sleep(Duration::from_secs(60));
    }
}
