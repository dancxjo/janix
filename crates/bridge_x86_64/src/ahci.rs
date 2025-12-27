use core::sync::atomic::{Ordering, AtomicU64};
use alloc::boxed::Box;
use alloc::vec::Vec;
use models::core::pci::PciDeviceBody;
use kernel_core::Kernel;
use models::builtins::ids::{THING_BLOCK_DEVICE_KIND, THING_LINK_KIND, THING_BOOT_ROOT, THING_HAS_DEVICE_KIND, THING_EMITS_KIND};
use models::core::block::{BlockDeviceBody, BlockDeviceType};
use models::Thing;
use models::value::ThingBody;
use abi::wire::typed::{TypedBytes, TypeId, CodecId};
use crate::{Bridge, HHDM_OFFSET};
use hw::HardwareBridge;

// --- AHCI Constants & Structs ---

const HBA_PX_CMD_ST: u32 = 0x0001;
const HBA_PX_CMD_FRE: u32 = 0x0010;
const HBA_PX_CMD_FR: u32 = 0x4000;
const HBA_PX_CMD_CR: u32 = 0x8000;

#[repr(C, packed)]
struct HbaPort {
    clb: u32,
    clbu: u32,
    fb: u32,
    fbu: u32,
    is: u32,
    ie: u32,
    cmd: u32,
    rsv0: u32,
    tfd: u32,
    sig: u32,
    ssts: u32,
    sctl: u32,
    serr: u32,
    sact: u32,
    ci: u32,
    sntf: u32,
    fbs: u32,
    rsv1: [u32; 11],
    vendor: [u32; 4],
}

#[repr(C, packed)]
struct HbaMem {
    cap: u32,
    ghc: u32,
    is: u32,
    pi: u32,
    vs: u32,
    ccc_ctl: u32,
    ccc_ports: u32,
    em_loc: u32,
    em_ctl: u32,
    cap2: u32,
    bohc: u32,
    rsv: [u8; 0xA0 - 0x2C],
    vendor: [u8; 0x100 - 0xA0],
    ports: [HbaPort; 32],
}

// --- Driver Logic ---

pub unsafe fn init(dev: &PciDeviceBody, k: &mut Kernel<Bridge>) {
    let bridge = Bridge;
    bridge.log("AHCI: Init\n");

    // 1. Get ABAR from BAR 5
    let bar5 = dev.bars[5];
    let abar_phys = (bar5 as u64) & 0xFFFFFFF0; // Mask type bits
    
    // 2. Map ABAR
    let hhdm = HHDM_OFFSET.load(Ordering::Relaxed);
    if hhdm == 0 {
        bridge.log("AHCI: HHDM not set!\n");
        return;
    }
    let abar_virt = hhdm + abar_phys;
    let hba = &mut *(abar_virt as *mut HbaMem);

    // 3. Enable AHCI (GHC.AE)
    hba.ghc |= 0x80000000;
    
    // 4. Scan Ports
    let pi = hba.pi;
    for i in 0..32 {
        if (pi & (1 << i)) != 0 {
            let port = &mut hba.ports[i];
            
            let ssts = port.ssts;
            let det = ssts & 0x0F;
            let ipm = (ssts >> 8) & 0x0F;
            
            if det == 3 && ipm == 1 {
                let sig = port.sig;
                let kind = match sig {
                    0x00000101 => BlockDeviceType::IDE, // ATA
                    0xEB140101 => BlockDeviceType::SATA, // ATAPI
                    0xC33C0101 => BlockDeviceType::SATA, // SEMB
                    0x96690101 => BlockDeviceType::SATA, // PM
                    _ => BlockDeviceType::SATA, // Default to SATA
                };
                
                bridge.log("AHCI: Port ");
                crate::print_u64(i as u64);
                bridge.log(" Found Device (Sig ");
                crate::print_hex(sig as u64);
                bridge.log(")\n");
                
                // Publish BlockDevice
                // TODO: Read Identify to get model/capacity
                let serial_s = alloc::format!("AHCI-PORT-{}", i);
                
                let body = BlockDeviceBody {
                    model: alloc::string::String::from("Generic AHCI Drive"),
                    serial: serial_s,
                    capacity_sectors: 0, // Unknown
                    sector_size: 512,
                    device_type: kind,
                };
                
                let body_bytes = postcard::to_allocvec(&body).unwrap();
                let tb = ThingBody::from(&TypedBytes {
                    type_id: TypeId(THING_BLOCK_DEVICE_KIND.0 as u128),
                    codec_id: CodecId::POSTCARD,
                    bytes: body_bytes,
                }).unwrap();
                
                let dev_id = k.graph.create_thing(THING_BLOCK_DEVICE_KIND, tb);
                
                // Link Root -> Device
                let link = models::link::LinkBody {
                    from: THING_BOOT_ROOT,
                    to: dev_id,
                    predicate: THING_HAS_DEVICE_KIND,
                };
                let lb = ThingBody::from(&TypedBytes {
                    type_id: TypeId(THING_LINK_KIND.0 as u128),
                    codec_id: CodecId::POSTCARD,
                    bytes: postcard::to_allocvec(&link).unwrap() 
                }).unwrap();
                k.graph.create_thing(THING_LINK_KIND, lb);
                
                
                // TODO: Init Port Commands and Identify
                // But first verify we see the device.
                if sig == 0x00000101 || sig == 0xEB140101 { // ATA or ATAPI
                     init_port(port, i, hhdm);
                }
            }
        }
    }
}

use alloc::alloc::{alloc, Layout};

unsafe fn init_port(port: &mut HbaPort, port_no: usize, hhdm: u64) {
    let bridge = Bridge;
    
    // Stop Port
    stop_cmd(port);
    
    // Command List Base (1K aligned)
    let cl_layout = Layout::from_size_align(1024, 1024).unwrap();
    let cl_ptr = alloc(cl_layout);
    // Zero out
    core::ptr::write_bytes(cl_ptr, 0, 1024);
    
    // FIS Base (256 aligned)
    let fis_layout = Layout::from_size_align(256, 256).unwrap();
    let fis_ptr = alloc(fis_layout);
    core::ptr::write_bytes(fis_ptr, 0, 256);
    
    // Command Table (128 bytes + PRDT, 128 aligned)
    // We only use Slot 0 for simplicity right now.
    // Table Size = 0x80 + (PRDT_count * 16)
    // Let's alloc 4KB to be safe and cover PRDTs
    let ct_layout = Layout::from_size_align(4096, 128).unwrap();
    let ct_ptr = alloc(ct_layout);
    core::ptr::write_bytes(ct_ptr, 0, 4096);
    
    // Get Phys Addrs via HHDM reverse lookup?
    // Wait, HHDM is "Virt = Phys + Offset". So "Phys = Virt - Offset".
    // This ONLY works if heap is direct-mapped.
    // Our heap is direct mapped via HHDM!
    
    let cl_phys = (cl_ptr as u64) - hhdm;
    let fis_phys = (fis_ptr as u64) - hhdm;
    let ct_phys = (ct_ptr as u64) - hhdm;
    
    port.clb = cl_phys as u32;
    port.clbu = (cl_phys >> 32) as u32;
    port.fb = fis_phys as u32;
    port.fbu = (fis_phys >> 32) as u32;
    
    // Link Command Header 0 to Command Table
    let cl_slice = core::slice::from_raw_parts_mut(cl_ptr as *mut HbaCmdHeader, 32);
    let header = &mut cl_slice[0];
    header.ctba = ct_phys as u32;
    header.ctbau = (ct_phys >> 32) as u32;
    // Assume 1 PRDT entry max for now
    header.prdtl = 1; 

    bridge.log("AHCI: Port ");
    crate::print_u64(port_no as u64);
    bridge.log(" DMA Initialized. Starting...\n");
    
    start_cmd(port);
}

unsafe fn stop_cmd(port: &mut HbaPort) {
    port.cmd &= !HBA_PX_CMD_ST;
    port.cmd &= !HBA_PX_CMD_FRE;
    
    while (port.cmd & HBA_PX_CMD_FR) != 0 {
         core::hint::spin_loop();
    }
    while (port.cmd & HBA_PX_CMD_CR) != 0 {
         core::hint::spin_loop();
    }
}

unsafe fn start_cmd(port: &mut HbaPort) {
    while (port.cmd & HBA_PX_CMD_CR) != 0 {
         core::hint::spin_loop();
    }
    
    port.cmd |= HBA_PX_CMD_FRE;
    port.cmd |= HBA_PX_CMD_ST;
}

#[repr(C, packed)]
struct HbaCmdHeader {
    // DW0
    cfl: u8, // Command FIS length in DWORDS (2-16)
    pm: u8,  // Port Multiplier + ATAPI(A) + Write(W) + Prefetch(P)
    prdtl: u16, // PRDT Length
    // DW1
    prdbc: u32,
    // DW2,3
    ctba: u32,
    ctbau: u32,
    // DW4-7
    rsv: [u32; 4],
}

#[repr(C, packed)]
struct HbaCmdTable {
    // 0x00
    cfis: [u8; 64],
    // 0x40
    acmd: [u8; 16], // ATAPI command
    rsv: [u8; 48],
    // 0x80
    prdt_entry: [HbaPrdtEntry; 1], // Just 1 for now
}


#[repr(C, packed)]
struct HbaPrdtEntry {
    dba: u32,
    dbau: u32,
    rsv0: u32,
    dbc: u32, // Byte count - 1 (Bit 31 = Interrupt on complete)
}

pub unsafe fn read_atapi_sector(port: &mut HbaPort, lba: u32, buf: &mut [u8], hhdm: u64) -> bool {
    let bridge = Bridge;
    // 1. Find Slot (We hardcode Slot 0)
    port.ci = 0; // Clear commands? No, check status.
    
    // 2. Setup Command Header (Slot 0)
    let cl_phys = ((port.clbu as u64) << 32) | (port.clb as u64);
    let cl_virt = hhdm + cl_phys;
    let cl_slice = core::slice::from_raw_parts_mut(cl_virt as *mut HbaCmdHeader, 32);
    let header = &mut cl_slice[0];
    
    header.cfl = 5; // 5 DWORDS for Host to Device FIS
    header.pm = 0x20; // ATAPI (Bit 5) | Prefetch?? No, just ATAPI.
    // Length is bytes. Buffer must be mapped.
    // PRDTL = 1
    
    // 3. Setup Command Table
    let ct_phys = ((header.ctbau as u64) << 32) | (header.ctba as u64);
    let ct_virt = hhdm + ct_phys;
    let table = &mut *(ct_virt as *mut HbaCmdTable);
    
    // Clear Table
    core::ptr::write_bytes(table as *mut _ as *mut u8, 0, core::mem::size_of::<HbaCmdTable>());
    
    // 4. Setup FIS (RegH2D) - 0x27
    table.cfis[0] = 0x27; // FIS Type
    table.cfis[1] = 0x80; // Command (Bit 7) - C bit? No, for 0x27, bit 7 is 'C' (Command).
    table.cfis[2] = 0xA0; // Command: PACKET
    // Feature? DMA?
    table.cfis[3] = 0x01; // Features: DMA (Bit 0)
    
    // 5. Setup ATAPI Command (SCSI READ 12 - 0xA8)
    table.acmd[0] = 0xA8;
    table.acmd[2] = (lba >> 24) as u8;
    table.acmd[3] = (lba >> 16) as u8;
    table.acmd[4] = (lba >> 8) as u8;
    table.acmd[5] = lba as u8;
    
    let count: u32 = 1; // 1 sector
    table.acmd[6] = (count >> 24) as u8;
    table.acmd[7] = (count >> 16) as u8;
    table.acmd[8] = (count >> 8) as u8;
    table.acmd[9] = count as u8;
    
    // 6. Setup PRDT
    let buf_phys = (buf.as_ptr() as u64) - hhdm; // Assume buffer is in HHDM heap
    let entry = &mut table.prdt_entry[0];
    entry.dba = buf_phys as u32;
    entry.dbau = (buf_phys >> 32) as u32;
    entry.dbc = (2048 - 1); // 2048 bytes
    entry.rsv0 = 0;
    
    // 7. Issue Command
    // Wait for BSY
    let mut timeout = 1000000;
    while (port.tfd & (0x80 | 0x08)) != 0 && timeout > 0 {
        timeout -= 1;
        core::hint::spin_loop();
    }
    
    port.is = 0xFFFFFFFF; // Clear interrupts
    port.ci = 1; // Issue Slot 0
    
    // 8. Wait for Completion
    timeout = 10000000;
    loop {
         if (port.ci & 1) == 0 { break; } // Done
         if (port.is & (1<<30)) != 0 { // TFES (Task File Error Status)
             bridge.log("AHCI: TFES Error\n");
             break;
         }
         timeout -= 1;
         if timeout == 0 { break; }
         core::hint::spin_loop();
    }
    if timeout == 0 {
         bridge.log("AHCI: Timeout exec. TFD: ");
         crate::print_hex(port.tfd as u64);
         bridge.log(" SERR: ");
         crate::print_hex(port.serr as u64);
         bridge.log("\n");
         return false;
    }
    
    // Check Error
    if (port.tfd & 1) != 0 {
         bridge.log("AHCI: Disk Error. TFD: ");
         crate::print_hex(port.tfd as u64);
         bridge.log(" SERR: ");
         crate::print_hex(port.serr as u64);
         bridge.log("\n");
         return false;
    }
    
    true
}

pub unsafe fn read_sector_at(abar_base: u64, port_idx: usize, lba: u32, buf: &mut [u8], hhdm: u64) -> bool {
    // Reconstruct HbaMem pointer
    let abar_virt = hhdm + abar_base;
    let hba = &mut *(abar_virt as *mut HbaMem);
    let port = &mut hba.ports[port_idx];
    
    // Determine type? For now assume ATAPI if we are asking to read.
    // Ideally we check port.sig.
    if port.sig == 0xEB140101 { // ATAPI
        read_atapi_sector(port, lba, buf, hhdm)
    } else {
        false
    }
}
