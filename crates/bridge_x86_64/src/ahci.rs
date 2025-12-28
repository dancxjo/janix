use crate::{Bridge, HHDM_OFFSET};
use abi::wire::typed::{CodecId, TypeId, TypedBytes};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use hw::HardwareBridge;
use kernel_core::Kernel;
use models::builtins::ids::{
    THING_BLOCK_DEVICE_KIND, THING_BOOT_ROOT, THING_EMITS_KIND, THING_HAS_DEVICE_KIND,
    THING_LINK_KIND,
};
use models::core::block::{BlockDeviceBody, BlockDeviceType};
use models::core::pci::PciDeviceBody;
use models::value::ThingBody;
use models::Thing;
use spin::Mutex;

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

// Global locks for ports to allow concurrent slot allocation
static PORT_LOCKS: [Mutex<()>; 32] = [
    Mutex::new(()), Mutex::new(()), Mutex::new(()), Mutex::new(()),
    Mutex::new(()), Mutex::new(()), Mutex::new(()), Mutex::new(()),
    Mutex::new(()), Mutex::new(()), Mutex::new(()), Mutex::new(()),
    Mutex::new(()), Mutex::new(()), Mutex::new(()), Mutex::new(()),
    Mutex::new(()), Mutex::new(()), Mutex::new(()), Mutex::new(()),
    Mutex::new(()), Mutex::new(()), Mutex::new(()), Mutex::new(()),
    Mutex::new(()), Mutex::new(()), Mutex::new(()), Mutex::new(()),
    Mutex::new(()), Mutex::new(()), Mutex::new(()), Mutex::new(()),
];

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
                    0x00000101 => BlockDeviceType::IDE,  // ATA
                    0xEB140101 => BlockDeviceType::SATA, // ATAPI
                    0xC33C0101 => BlockDeviceType::SATA, // SEMB
                    0x96690101 => BlockDeviceType::SATA, // PM
                    _ => BlockDeviceType::SATA,          // Default to SATA
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
                    bus: alloc::string::String::from("ahci"),
                    port: i as u32,
                };

                let body_bytes = postcard::to_allocvec(&body).unwrap();
                let tb = ThingBody::from(&TypedBytes {
                    type_id: TypeId(THING_BLOCK_DEVICE_KIND.0 as u128),
                    codec_id: CodecId::POSTCARD,
                    bytes: body_bytes,
                })
                .unwrap();

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
                    bytes: postcard::to_allocvec(&link).unwrap(),
                })
                .unwrap();
                k.graph.create_thing(THING_LINK_KIND, lb);

                // TODO: Init Port Commands and Identify
                // But first verify we see the device.
                if sig == 0x00000101 || sig == 0xEB140101 {
                    // ATA or ATAPI
                    init_port(port, i, hhdm);
                }
            }
        }
    }
}

use alloc::alloc::{alloc, dealloc, Layout};

unsafe fn init_port(port: &mut HbaPort, port_no: usize, hhdm: u64) {
    let bridge = Bridge;

    // Stop Port
    stop_cmd(port);

    // Command List Base (1K aligned)
    let cl_layout = Layout::from_size_align(1024, 1024).unwrap();
    let cl_ptr = alloc(cl_layout);
    if cl_ptr.is_null() {
        bridge.log("AHCI: Failed to alloc CL\n");
        return;
    }
    // Zero out
    core::ptr::write_bytes(cl_ptr, 0, 1024);

    // FIS Base (256 aligned)
    let fis_layout = Layout::from_size_align(256, 256).unwrap();
    let fis_ptr = alloc(fis_layout);
    if fis_ptr.is_null() {
        bridge.log("AHCI: Failed to alloc FIS\n");
        // Leak cl_ptr for now, panic is imminent
        return;
    }
    core::ptr::write_bytes(fis_ptr, 0, 256);

    // NOTE: We do NOT allocate Command Table here anymore.
    // read_sector_yielding will allocate it dynamically per slot.

    let cl_phys = (cl_ptr as u64) - hhdm;
    let fis_phys = (fis_ptr as u64) - hhdm;

    port.clb = cl_phys as u32;
    port.clbu = (cl_phys >> 32) as u32;
    port.fb = fis_phys as u32;
    port.fbu = (fis_phys >> 32) as u32;

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
    cfl: u8,    // Command FIS length in DWORDS (2-16)
    pm: u8,     // Port Multiplier + ATAPI(A) + Write(W) + Prefetch(P)
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

pub unsafe fn read_atapi_sector_yielding<F: Fn()>(
    port: &mut HbaPort,
    port_idx: usize,
    lba: u32,
    buf: &mut [u8],
    hhdm: u64,
    yield_fn: F,
) -> bool {
    let bridge = Bridge;
    bridge.log("AHCI: Read LBA ");
    
    // Convert LBA to string manually or use format! if available?
    // Minimal log for now to avoid allocation inside AHCI if possible?
    // But we are in a bridge, maybe we can use alloc.
    // Let's just log start.
    // bridge.log("AHCI: Issue\n"); 
    
    // We can use alloc::format! if we import it? 
    // bridge.log(&alloc::format!("{} Slot {}\n", lba, slot));
    let slot;

    // 1. Find Free Slot (Thread Safe)
    {
        let _guard = PORT_LOCKS[port_idx].lock();
        let slots = port.ci | port.sact;
        let mut found = None;
        for i in 0..32 {
            if (slots & (1 << i)) == 0 {
                found = Some(i);
                break;
            }
        }
        if let Some(s) = found {
            slot = s;
            // Optimistically set bit in CI? No, we set it when we issue.
            // But if we release guard, someone else might take it.
            // Actually, we can't set CI before table is ready.
            // But we must Reserve it.
            // Standard AHCI does not have "reserve" bit.
            // We just have to hope no one else takes it? NO.
            // We must hold the lock UNTIL we write CI.
            // This means setup must happen UNDER LOCK.
            // OR use a software bitmap if we want longer setup without lock.
            // For now, setup is fast (memory write). Hold lock.
            
            // 2. Alloc Command Table (4KB)
            let ct_layout = Layout::from_size_align(4096, 128).unwrap();
            let ct_ptr = alloc(ct_layout);
            if ct_ptr.is_null() {
                 bridge.log("AHCI: Failed to alloc CT\n");
                 return false;
            }
            core::ptr::write_bytes(ct_ptr, 0, 4096);
            
            let ct_phys = (ct_ptr as u64) - hhdm;

            // 3. Setup Command Header (Slot 'slot')
            let cl_phys = ((port.clbu as u64) << 32) | (port.clb as u64);
            let cl_virt = hhdm + cl_phys;
            let cl_slice = core::slice::from_raw_parts_mut(cl_virt as *mut HbaCmdHeader, 32);
            let header = &mut cl_slice[slot];

            header.cfl = 5 | 0x20; // 5 DWORDS | ATAPI
            header.pm = 0;
            header.ctba = ct_phys as u32;
            header.ctbau = (ct_phys >> 32) as u32;
            header.prdtl = 1;

            // 4. Setup Command Table
            let ct_virt = hhdm + ct_phys;
            let table = &mut *(ct_virt as *mut HbaCmdTable);

            // FIS
            table.cfis[0] = 0x27; 
            table.cfis[1] = 0x80; 
            table.cfis[2] = 0xA0; 
            table.cfis[3] = 0x01; 

            // ATAPI Packet
            table.acmd[0] = 0xA8;
            table.acmd[2] = (lba >> 24) as u8;
            table.acmd[3] = (lba >> 16) as u8;
            table.acmd[4] = (lba >> 8) as u8;
            table.acmd[5] = lba as u8;
            let count: u32 = 1; 
            table.acmd[6] = (count >> 24) as u8;
            table.acmd[7] = (count >> 16) as u8;
            table.acmd[8] = (count >> 8) as u8;
            table.acmd[9] = count as u8;

            // PRDT
            let buf_phys = (buf.as_ptr() as u64) - hhdm; 
            let entry = &mut table.prdt_entry[0];
            entry.dba = buf_phys as u32;
            entry.dbau = (buf_phys >> 32) as u32;
            entry.dbc = (2048 - 1); 
            entry.rsv0 = 0;

            // 5. Issue Command
            // Use volatile to ensure immediate write to MMIO
            let ci_ptr = core::ptr::addr_of_mut!(port.ci);
            let mut ci = unsafe { core::ptr::read_volatile(ci_ptr) };
            ci |= 1 << slot;
            unsafe { core::ptr::write_volatile(ci_ptr, ci) };
            
            // Now we can release lock! The slot is marked busy in HW.
        } else {
            bridge.log("AHCI: No free slots!\n");
            return false;
        }
    } // Unlock

    // 6. Spin Wait (Yielding)
    let mut timeout = 100_000_000; // 100M spins ~ 0.5-1s?
    loop {
        let ci_ptr = core::ptr::addr_of!(port.ci);
        let ci = unsafe { core::ptr::read_volatile(ci_ptr) };
        if (ci & (1 << slot)) == 0 {
            break;
        } 
        if (port.is & (1 << 30)) != 0 {
             bridge.log("AHCI: TFES Error\n");
             break;
        }
        if timeout == 0 { break; }
        
        yield_fn();
        core::hint::spin_loop();
    }
    
    // 7. Cleanup (Get lock to modify Header? No, Header is owned by HW/Us? CTBA is ours?)
    // We allocated CT. We must free it.
    // Use lock? No, slot is done (CI bit cleared). We own the slot cleanup.
    // However, if we want to be super safe?
    // Accessing Header[slot] is ours since CI bit is 0.

    let cl_phys = ((port.clbu as u64) << 32) | (port.clb as u64);
    let cl_virt = hhdm + cl_phys;
    let cl_slice = core::slice::from_raw_parts_mut(cl_virt as *mut HbaCmdHeader, 32);
    let header = &mut cl_slice[slot];
    let ct_phys = ((header.ctbau as u64) << 32) | (header.ctba as u64);
    
    // Free CT
    let ct_ptr = (hhdm + ct_phys) as *mut u8;
    let ct_layout = Layout::from_size_align(4096, 128).unwrap();
    dealloc(ct_ptr, ct_layout);

    if timeout == 0 {
        bridge.log("AHCI: Timeout exec.\n");
        return false;
    }
    if (port.tfd & 1) != 0 {
        bridge.log("AHCI: Disk Error.\n");
        return false;
    }

    true
}

// Wrapper for existing "unsafe" blocking call
pub unsafe fn read_sector_at(
    abar_base: u64,
    port_idx: usize,
    lba: u32,
    buf: &mut [u8],
    hhdm: u64,
) -> bool {
    let abar_virt = hhdm + abar_base;
    let hba = &mut *(abar_virt as *mut HbaMem);
    let port = &mut hba.ports[port_idx];

    if port.sig == 0xEB140101 {
        // Use spin loop for default behavior
        read_atapi_sector_yielding(port, port_idx, lba, buf, hhdm, || core::hint::spin_loop())
    } else {
        false
    }
}

pub unsafe fn read_sector_yielding<F: Fn()>(
    abar_base: u64,
    port_idx: usize,
    lba: u32,
    buf: &mut [u8],
    hhdm: u64,
    yield_fn: F,
) -> bool {
    let abar_virt = hhdm + abar_base;
    let hba = &mut *(abar_virt as *mut HbaMem);
    let port = &mut hba.ports[port_idx];

    if port.sig == 0xEB140101 {
        read_atapi_sector_yielding(port, port_idx, lba, buf, hhdm, yield_fn)
    } else {
        false
    }
}
