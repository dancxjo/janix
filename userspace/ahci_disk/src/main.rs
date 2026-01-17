//! AHCI (SATA) Disk Driver (Read-only v0)
//!
//! Userspace driver for AHCI SATA controllers. Detects SATA devices via
//! PCI enumeration and MMIO access, registers them in the System Graph.

#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use core::time::Duration;
use stem::abi::module_manifest::{ManifestHeader, ModuleKind, MANIFEST_MAGIC};
use stem::thing::sys as thingsys;
use stem::thing::ThingId;
use stem::{error, info};

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
const PORT_CMD: usize = 0x18;
const PORT_SIG: usize = 0x24;
const PORT_SSTS: usize = 0x28;
const PORT_SERR: usize = 0x30;
#[allow(dead_code)]
const PORT_CI: usize = 0x38;

const SATA_SIG_ATA: u32 = 0x00000101;
const SATA_SIG_ATAPI: u32 = 0xEB140101;
const SATA_SIG_SEMB: u32 = 0xC33C0101;
const SATA_SIG_PM: u32 = 0x96690101;

const PORT_CMD_ST: u32 = 1 << 0;
const PORT_CMD_FRE: u32 = 1 << 4;
const PORT_CMD_FR: u32 = 1 << 14;
const PORT_CMD_CR: u32 = 1 << 15;

const SSTS_DET_MASK: u32 = 0x0F;
const SSTS_DET_PRESENT: u32 = 0x03;
const SSTS_IPM_MASK: u32 = 0x0F00;
const SSTS_IPM_ACTIVE: u32 = 0x0100;

#[allow(dead_code)]
const FIS_TYPE_REG_H2D: u8 = 0x27;
#[allow(dead_code)]
const ATA_CMD_IDENTIFY: u8 = 0xEC;

struct AhciPort {
    port_num: u32,
    sector_count: u64,
    supports_lba48: bool,
    model: [u8; 40],
    graph_id: u64,
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
    if det != SSTS_DET_PRESENT || ipm != SSTS_IPM_ACTIVE { return None; }
    Some(mmio_read32(pb, PORT_SIG))
}

fn stop_port(hba_base: u64, port: u32) {
    let pb = port_base(hba_base, port);
    let mut cmd = mmio_read32(pb, PORT_CMD);
    cmd &= !PORT_CMD_ST;
    mmio_write32(pb, PORT_CMD, cmd);
    for _ in 0..1000 { if mmio_read32(pb, PORT_CMD) & PORT_CMD_CR == 0 { break; } }
    cmd = mmio_read32(pb, PORT_CMD);
    cmd &= !PORT_CMD_FRE;
    mmio_write32(pb, PORT_CMD, cmd);
    for _ in 0..1000 { if mmio_read32(pb, PORT_CMD) & PORT_CMD_FR == 0 { break; } }
}

fn start_port(hba_base: u64, port: u32) {
    let pb = port_base(hba_base, port);
    for _ in 0..1000 { if mmio_read32(pb, PORT_CMD) & PORT_CMD_CR == 0 { break; } }
    let mut cmd = mmio_read32(pb, PORT_CMD);
    cmd |= PORT_CMD_FRE;
    mmio_write32(pb, PORT_CMD, cmd);
    cmd |= PORT_CMD_ST;
    mmio_write32(pb, PORT_CMD, cmd);
}

fn find_ahci_controller() -> Option<(ThingId, u64)> {
    let mut pci_funcs = [ThingId(0); 32];
    let count = match thingsys::find("dev.pci.Function", &mut pci_funcs) {
        Ok(c) => { info!("AHCI: Found {} PCI functions", c); c }
        Err(e) => { info!("AHCI: Error finding PCI functions: {:?}", e); return None; }
    };
    
    for i in 0..count {
        let func_id = pci_funcs[i];
        let class = thingsys::prop_get(func_id, "class_code").unwrap_or(0);
        let subclass = thingsys::prop_get(func_id, "subclass_code").unwrap_or(0);
        let prog_if = thingsys::prop_get(func_id, "prog_if").unwrap_or(0);
        
        if class == PCI_CLASS_STORAGE && subclass == PCI_SUBCLASS_SATA && prog_if == PCI_PROGIF_AHCI {
            info!("AHCI: Found AHCI controller at PCI func {}", func_id.0);
            let bar5 = thingsys::prop_get(func_id, "bar5").unwrap_or(0);
            if bar5 != 0 {
                info!("AHCI: BAR5=0x{:x}", bar5);
                return Some((func_id, bar5));
            }
        }
    }
    None
}

fn register_disk(port: &mut AhciPort) {
    let disk_id = match thingsys::create_node("dev.storage.Disk") {
        Ok(id) => id,
        Err(e) => { error!("AHCI: Failed to create disk node: {:?}", e); return; }
    };
    port.graph_id = disk_id.0;
    thingsys::prop_set(disk_id, "sector_size", 512u64).ok();
    thingsys::prop_set(disk_id, "sector_count", port.sector_count).ok();
    thingsys::prop_set(disk_id, "lba48", if port.supports_lba48 { 1u64 } else { 0u64 }).ok();
    thingsys::prop_set(disk_id, "interface", 1u64).ok();
    let model_str = core::str::from_utf8(&port.model).unwrap_or("Unknown").trim();
    info!("AHCI: Registered disk {} port={} sectors={} lba48={} model='{}'",
        port.graph_id, port.port_num, port.sector_count, port.supports_lba48, model_str);
}

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("AHCI: Starting AHCI/SATA disk driver v0");
    
    let (pci_id, _bar5_phys) = match find_ahci_controller() {
        Some(c) => c,
        None => { info!("AHCI: No AHCI controller found"); loop { stem::sleep(Duration::from_secs(60)); } }
    };
    
    let claim_handle = match stem::syscall::device_claim(pci_id.0) {
        Ok(h) => { info!("AHCI: Claimed PCI device {} handle={}", pci_id.0, h); h }
        Err(e) => { error!("AHCI: Failed to claim: {:?}", e); loop { stem::sleep(Duration::from_secs(60)); } }
    };
    
    let mapped_base = match stem::syscall::device_map_mmio(claim_handle, 5) {
        Ok(addr) => { info!("AHCI: Mapped ABAR at 0x{:x}", addr); addr }
        Err(e) => { error!("AHCI: Failed to map MMIO: {:?}", e); loop { stem::sleep(Duration::from_secs(60)); } }
    };
    
    let cap = mmio_read32(mapped_base, HBA_CAP);
    let version = mmio_read32(mapped_base, HBA_VS);
    let pi = mmio_read32(mapped_base, HBA_PI);
    info!("AHCI: Version {}.{}, {} ports, {} slots, 64-bit: {}",
        (version >> 16) & 0xFFFF, version & 0xFFFF,
        ((cap >> 0) & 0x1F) + 1, ((cap >> 8) & 0x1F) + 1, (cap & (1 << 31)) != 0);
    info!("AHCI: Ports implemented: 0x{:x}", pi);
    
    // Enable AHCI mode
    let mut ghc = mmio_read32(mapped_base, HBA_GHC);
    ghc |= 1 << 31;
    mmio_write32(mapped_base, HBA_GHC, ghc);
    
    // Allocate DMA buffer - get physical address for AHCI
    let dma_pages = 1;
    let dma_virt = match stem::syscall::device_alloc_dma(claim_handle, dma_pages) {
        Ok(addr) => { info!("AHCI: DMA virt=0x{:x}", addr); addr }
        Err(e) => { error!("AHCI: DMA alloc failed: {:?}", e); loop { stem::sleep(Duration::from_secs(60)); } }
    };
    
    let dma_phys = match stem::syscall::device_dma_phys(dma_virt) {
        Ok(addr) => { info!("AHCI: DMA phys=0x{:x}", addr); addr }
        Err(e) => { error!("AHCI: DMA phys failed: {:?}", e); loop { stem::sleep(Duration::from_secs(60)); } }
    };
    
    // Layout in physical DMA buffer
    let clb = dma_phys;
    let fb = dma_phys + 1024;
    let _ctb = dma_phys + 1280;
    let _data_buf = dma_phys + 1536;
    
    let mut ports: Vec<AhciPort> = Vec::new();
    
    // Probe ports
    for port_num in 0..32u32 {
        if pi & (1 << port_num) == 0 { continue; }
        info!("AHCI: Probing port {}...", port_num);
        
        let sig = match check_port_type(mapped_base, port_num) {
            Some(s) => s,
            None => { info!("AHCI: Port {} - no device", port_num); continue; }
        };
        
        match sig {
            SATA_SIG_ATA => info!("AHCI: Port {} - SATA drive (sig=0x{:x})", port_num, sig),
            SATA_SIG_ATAPI => { info!("AHCI: Port {} - SATAPI (skip)", port_num); continue; }
            SATA_SIG_SEMB => { info!("AHCI: Port {} - Enclosure (skip)", port_num); continue; }
            SATA_SIG_PM => { info!("AHCI: Port {} - PM (skip)", port_num); continue; }
            _ => { info!("AHCI: Port {} - Unknown sig=0x{:x}", port_num, sig); continue; }
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
        
        // Write command header via HHDM (kernel provides this address)
        // Note: dma_virt is HHDM-mapped, accessible via the mapped MMIO
        // For proper userspace access, we'd need the kernel to map it to user address space
        // For now, just use physical addresses which is what AHCI needs anyway
        
        // We can't write to dma_virt from userspace, so we need to use MMIO to set up commands
        // This is a limitation - for now just report the port exists
        info!("AHCI: Port {} - SATA drive detected (IDENTIFY deferred - DMA access limitation)", port_num);
        
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
    loop { stem::sleep(Duration::from_secs(60)); }
}
