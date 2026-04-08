//! Disk Probe Utility
//! 
//! TODO: Convert this to a `vfs_provider` like `vfs_hello`.
//! 
//! Lists all detected disks in the system graph and displays their properties.
//! This is a diagnostic tool to verify disk discovery is working.

#![feature(restricted_std)]
#![no_main]

extern crate alloc;

use stem::info;
use stem::thing::sys as thingsys;
use stem::thing::ThingId;

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("DISK_PROBE: Starting disk probe utility");

    // Small delay to let ata_disk register disks first
    stem::sleep(core::time::Duration::from_millis(500));

    // Find all dev.storage.Disk Things
    let mut disks = [ThingId(0); 8];
    let count = match thingsys::find("dev.storage.Disk", &mut disks) {
        Ok(c) => c,
        Err(e) => {
            info!("DISK_PROBE: Error finding disks: {:?}", e);
            0
        }
    };

    if count == 0 {
        info!("DISK_PROBE: No disks found in system graph");
        info!("DISK_PROBE: (This is expected if no disks are attached to QEMU)");
        // stem::syscall::exit(0);
        loop {
            stem::sleep(core::time::Duration::from_secs(3600));
        }
    }

    info!("DISK_PROBE: Found {} disk(s) in system graph", count);

    for i in 0..count {
        let disk_id = disks[i];
        probe_disk(disk_id, i);
    }

    info!("DISK_PROBE: Probe complete");
    loop {
        stem::sleep(core::time::Duration::from_secs(3600));
    }
}

fn probe_disk(disk_id: ThingId, index: usize) {
    info!("DISK_PROBE: === Disk {} (Thing {}) ===", index, disk_id.0);

    // Get disk properties
    match thingsys::prop_get(disk_id, "sector_count") {
        Ok(sectors) => {
            let size_mb = (sectors * 512) / (1024 * 1024);
            info!("DISK_PROBE:   Sector count: {} ({} MB)", sectors, size_mb);
        }
        Err(_) => info!("DISK_PROBE:   Sector count: (unavailable)"),
    }

    match thingsys::prop_get(disk_id, "sector_size") {
        Ok(size) => info!("DISK_PROBE:   Sector size: {} bytes", size),
        Err(_) => info!("DISK_PROBE:   Sector size: (unavailable)"),
    }

    match thingsys::prop_get(disk_id, "lba48") {
        Ok(lba48) => {
            let support = if lba48 != 0 { "yes" } else { "no" };
            info!("DISK_PROBE:   LBA48 support: {}", support);
        }
        Err(_) => info!("DISK_PROBE:   LBA48 support: (unavailable)"),
    }

    info!("DISK_PROBE:");
}
