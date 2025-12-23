pub fn init() {
    crate::log("Initializing drivers...");

    #[cfg(all(not(test), target_arch = "x86_64"))]
    crate::bridge::ps2::init();

    // USB/PCI userland drivers and ATA bridge are temporarily disabled.
    crate::log("Skipping ATA bridge and PCI/USB drivers (disabled).");
    crate::log("Drivers initialized (Userland transition).");
}
