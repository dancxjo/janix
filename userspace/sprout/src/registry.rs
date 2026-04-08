use abi::ids::HandleId;
use abi::module_manifest::{ManifestHeader, ModuleKind, MANIFEST_MAGIC, SECTION_NAME};
use abi::schema::kinds;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use stem::info;
use stem::thing::sys as thingsys;
use stem::thing::ThingId;

pub struct Registry {
    drivers: BTreeMap<String, String>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            drivers: BTreeMap::new(),
        }
    }

    pub fn scan(&mut self) {
        info!("SPROUT: Scanning boot modules...");
        let mut modules = [ThingId::default(); 64];
        let count = thingsys::find(kinds::BOOT_MODULE, &mut modules).unwrap_or(0);
        for i in 0..count {
            self.scan_module(modules[i]);
        }
        info!(
            "SPROUT: Registry scan complete. Found {} drivers.",
            self.drivers.len()
        );
    }

    fn scan_module(&mut self, mod_id: ThingId) {
        let mut buf = [0u8; 1024];
        let mut mod_name = String::new();
        if let Ok(len) = thingsys::describe_thing(mod_id, &mut buf) {
            let s = core::str::from_utf8(&buf[..len]).unwrap_or("");
            if let Some(pos) = s.find("name: \"") {
                let rest = &s[pos + 7..];
                if let Some(end) = rest.find('"') {
                    mod_name = rest[..end].to_string();
                }
            }
        }
        if mod_name.is_empty() {
            return;
        }

        let mut bs_id = ThingId::default();

        let mut registered = false;
        if bs_id.to_u64_lossy() != 0 {
            if let Some(header) = self.read_manifest(bs_id) {
                if let ModuleKind::Driver = header.kind {
                    let raw = &header.device_kind;
                    let end = raw.iter().position(|&c| c == 0).unwrap_or(raw.len());
                    if let Ok(dk_str) = core::str::from_utf8(&raw[..end]) {
                        info!("SPROUT: Registering driver '{}' -> '{}'", dk_str, mod_name);
                        self.drivers.insert(dk_str.to_string(), mod_name.clone());
                        registered = true;
                    }
                }
            }
        }

        // Fallback for v0 if parsing fails
        if !registered {
            if mod_name.contains("rtc_cmos") {
                info!(
                    "SPROUT: Registering driver 'dev.rtc.Cmos' -> '{}' (fallback)",
                    mod_name
                );
                self.drivers.insert("dev.rtc.Cmos".to_string(), mod_name);
            }
        }
    }

    fn read_manifest(&self, bs: ThingId) -> Option<ManifestHeader> {
        let mut hdr_buf = [0u8; 64];
        if thingsys::bytespace_read(bs, 0, &mut hdr_buf).is_err() {
            return None;
        }

        if hdr_buf[0..4] != [0x7f, 0x45, 0x4c, 0x46] {
            return None;
        }

        let shoff = u64::from_le_bytes(hdr_buf[0x28..0x30].try_into().unwrap()) as usize;
        let shentsize = u16::from_le_bytes(hdr_buf[0x3A..0x3C].try_into().unwrap()) as usize;
        let shnum = u16::from_le_bytes(hdr_buf[0x3C..0x3E].try_into().unwrap()) as usize;
        let shstrndx = u16::from_le_bytes(hdr_buf[0x3E..0x40].try_into().unwrap()) as usize;

        let strtab_sh_off = shoff + (shstrndx as usize * shentsize);
        let (strtab_off, _) = self.read_sh_info(bs, strtab_sh_off)?;

        for i in 0..shnum {
            let off = shoff + (i * shentsize);
            if let Some((sh_name_idx, sh_offset, sh_size)) = self.read_sh_entry(bs, off) {
                if let Some(name) = self.read_string(bs, strtab_off, sh_name_idx as usize) {
                    if name == SECTION_NAME {
                        let mut m_buf = [0u8; core::mem::size_of::<ManifestHeader>()];
                        if m_buf.len() > sh_size {
                            return None;
                        }
                        if thingsys::bytespace_read(bs, sh_offset, &mut m_buf).is_ok() {
                            let m: ManifestHeader = unsafe { core::mem::transmute(m_buf) };
                            if m.magic == MANIFEST_MAGIC {
                                return Some(m);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn read_sh_info(&self, bs: ThingId, offset: usize) -> Option<(usize, usize)> {
        let mut buf = [0u8; 64];
        if thingsys::bytespace_read(bs, offset, &mut buf).is_err() {
            return None;
        }
        let sh_offset = u64::from_le_bytes(buf[0x18..0x20].try_into().ok()?) as usize;
        let sh_size = u64::from_le_bytes(buf[0x20..0x28].try_into().ok()?) as usize;
        Some((sh_offset, sh_size))
    }

    fn read_sh_entry(&self, bs: ThingId, offset: usize) -> Option<(u32, usize, usize)> {
        let mut buf = [0u8; 64];
        if thingsys::bytespace_read(bs, offset, &mut buf).is_err() {
            return None;
        }
        let sh_name = u32::from_le_bytes(buf[0..4].try_into().ok()?);
        let sh_offset = u64::from_le_bytes(buf[0x18..0x20].try_into().ok()?) as usize;
        let sh_size = u64::from_le_bytes(buf[0x20..0x28].try_into().ok()?) as usize;
        Some((sh_name, sh_offset, sh_size))
    }

    fn read_string(&self, bs: ThingId, strtab_off: usize, idx: usize) -> Option<String> {
        let mut buf = [0u8; 32];
        let _ = thingsys::bytespace_read(bs, strtab_off + idx, &mut buf);
        let end = buf.iter().position(|&c| c == 0).unwrap_or(0);
        if end == 0 {
            return None;
        }
        core::str::from_utf8(&buf[..end])
            .ok()
            .map(|s| s.to_string())
    }

    pub fn find_driver(&self, device_kind: &str) -> Option<&str> {
        self.drivers.get(device_kind).map(|s| s.as_str())
    }
}
