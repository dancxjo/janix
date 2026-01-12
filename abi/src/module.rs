use crate::device::RootCaps;
use crate::types::ThingId;

/// The section name where the manifest is stored in the ELF binary.
pub const MANIFEST_SECTION: &str = ".thingos.manifest";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ModuleKind {
    Driver = 1,
    App = 2,
    Service = 3,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ManifestMatch {
    pub compatible: [u8; 32],
}

impl ManifestMatch {
    pub const fn new(s: &str) -> Self {
        let mut bytes = [0u8; 32];
        let mut i = 0;
        let s_bytes = s.as_bytes();
        while i < s_bytes.len() && i < 32 {
            bytes[i] = s_bytes[i];
            i += 1;
        }
        Self { compatible: bytes }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ModuleManifest {
    pub magic: u64,
    pub kind: ModuleKind,
    pub name: [u8; 32],
    pub version: u32,
    pub match_count: usize,
    pub matches: [ManifestMatch; 4],
}

impl ModuleManifest {
    pub const fn new_driver(name: &str, matching: &str) -> Self {
        let mut name_bytes = [0u8; 32];
        let mut i = 0;
        let s_bytes = name.as_bytes();
        while i < s_bytes.len() && i < 32 {
            name_bytes[i] = s_bytes[i];
            i += 1;
        }

        Self {
            magic: 0xCAFEBABE,
            kind: ModuleKind::Driver,
            name: name_bytes,
            version: 1,
            match_count: 1,
            matches: [
                ManifestMatch::new(matching),
                ManifestMatch::new(""),
                ManifestMatch::new(""),
                ManifestMatch::new(""),
            ],
        }
    }
}

/// Context passed to a driver when it is spawned.
/// This pointer is passed as `arg` (the second argument to main).
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DriverCtx {
    pub device_id: ThingId,
    pub root_caps: RootCaps,
}
