// Panic handling is now provided externally:
// - Kernel (target_os = "none"): bran provides its own #[panic_handler]
// - Userspace (target_os = "thingos"): std's panic_abort handles it
//
// stem no longer needs to provide a panic handler in either case.
