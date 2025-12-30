use alloc::alloc::{alloc, Layout};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::bridge::FullMachineBridge;
use crate::fs::iso9660::{BlockReader, Iso9660Reader};
use crate::Kernel;
use abi::ThingId;
use thing_models::value::ThingBody;

pub struct FileArgs {
    pub iso: Arc<Iso9660Reader<Box<dyn BlockReader + Send + Sync>>>,
    pub path: String,
    pub dir_id: Option<ThingId>,
    pub should_spawn: bool,
    pub hhdm: u64,
}

// Helper Enums
pub enum ModuleType {
    Elf,
    Psf1,
    Psf2,
    Bmp,
    Png,
    Ttf,
    Otf,
    Woff,
    Woff2,
    Unknown,
    Other(String),
}

pub fn classify_bytes(data: &[u8]) -> ModuleType {
    if data.len() >= 4 && data[0] == 0x7F && data[1] == b'E' && data[2] == b'L' && data[3] == b'F' {
        return ModuleType::Elf;
    }
    if data.len() >= 4 && data[0] == 0x00 && data[1] == 0x01 && data[2] == 0x00 && data[3] == 0x00 {
        return ModuleType::Ttf;
    }
    if data.len() >= 4 && data[0] == b'O' && data[1] == b'T' && data[2] == b'T' && data[3] == b'O' {
        return ModuleType::Otf;
    }
    if data.len() >= 4 && data[0] == b'w' && data[1] == b'O' && data[2] == b'F' && data[3] == b'F' {
        return ModuleType::Woff;
    }
    if data.len() >= 4 && data[0] == b'w' && data[1] == b'O' && data[2] == b'F' && data[3] == b'2' {
        return ModuleType::Woff2;
    }
    if data.len() >= 2 && data[0] == 0x36 && data[1] == 0x04 {
        return ModuleType::Psf1;
    }
    if data.len() >= 4 && data[0] == 0x72 && data[1] == 0xB5 && data[2] == 0x4A && data[3] == 0x86 {
        return ModuleType::Psf2;
    }
    if data.len() >= 2 && data[0] == b'B' && data[1] == b'M' {
        return ModuleType::Bmp;
    }
    if data.len() >= 4 && data[0] == 0x89 && data[1] == b'P' && data[2] == b'N' && data[3] == b'G' {
        return ModuleType::Png;
    }

    // Infer crate relies on std usually? No, there is no_std infer.
    // Assuming infer is available or we skip detailed MIME Check if not available.
    // For now, simple check.
    ModuleType::Unknown
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModuleRole {
    App,
    Driver,
    Debug,
    Asset,
    Ignore,
}

pub fn get_module_role(name: &str, mtype: &ModuleType) -> ModuleRole {
    if name.contains("/drivers/") || name.contains("ps2_") {
        return ModuleRole::Driver;
    }
    match mtype {
        ModuleType::Elf => ModuleRole::App,
        ModuleType::Psf1
        | ModuleType::Psf2
        | ModuleType::Bmp
        | ModuleType::Png
        | ModuleType::Ttf
        | ModuleType::Otf
        | ModuleType::Woff
        | ModuleType::Woff2 => ModuleRole::Asset,
        ModuleType::Other(s) if s.starts_with("image/") || s.starts_with("font/") => {
            ModuleRole::Asset
        }
        _ => {
            if name.contains("font") {
                ModuleRole::Asset
            } else {
                ModuleRole::Ignore
            }
        }
    }
}

pub fn mount_and_scan<B: FullMachineBridge>(
    kernel: &mut Kernel<B>,
    iso: Arc<Iso9660Reader<Box<dyn BlockReader + Send + Sync>>>,
    loader_entry: u64,
    hhdm: u64,
) {
    use abi::wire::typed::{CodecId, TypeId, TypedBytes};
    use thing_models::builtins::ids::*;
    use thing_models::core::fs::{DirBody, MountBody};

    // 1. Mount "/boot"
    let m_body = MountBody {
        path: String::from("/boot"),
        readonly: true,
    };
    let m_tb = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_MOUNT_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: postcard::to_allocvec(&m_body).unwrap(),
    })
    .unwrap();
    let m_id = kernel.graph.create_thing(THING_MOUNT_KIND, m_tb);

    // Link BootRoot -> Mount
    let l_root = thing_models::link::LinkBody {
        from: THING_BOOT_ROOT,
        to: m_id,
        predicate: THING_HAS_MOUNT_KIND,
    };
    let l_root_tb = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_LINK_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: postcard::to_allocvec(&l_root).unwrap(),
    })
    .unwrap();
    kernel.graph.create_thing(THING_LINK_KIND, l_root_tb);

    // 2. Root Dir
    let d_body = DirBody {
        name: String::from("/boot"),
        lba: 0,
        size: 0,
        expanded: true,
    };
    let d_tb = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_DIR_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: postcard::to_allocvec(&d_body).unwrap(),
    })
    .unwrap();
    let root_id = kernel.graph.create_thing(THING_DIR_KIND, d_tb);

    // Link Mount -> Root Dir
    let l_mnt = thing_models::link::LinkBody {
        from: m_id,
        to: root_id,
        predicate: THING_MOUNTS_KIND,
    };
    let l_mnt_tb = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_LINK_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: postcard::to_allocvec(&l_mnt).unwrap(),
    })
    .unwrap();
    kernel.graph.create_thing(THING_LINK_KIND, l_mnt_tb);

    let mut make_dir = |name: &str, parent: ThingId| {
        let body = DirBody {
            name: String::from(name),
            lba: 0,
            size: 0,
            expanded: false,
        };
        let tb = ThingBody::from(&TypedBytes {
            type_id: TypeId(THING_DIR_KIND.0 as u128),
            codec_id: CodecId::POSTCARD,
            bytes: postcard::to_allocvec(&body).unwrap(),
        })
        .unwrap();
        let did = kernel.graph.create_thing(THING_DIR_KIND, tb);
        let l = thing_models::link::LinkBody {
            from: parent,
            to: did,
            predicate: THING_HAS_ENTRY_KIND,
        };
        let l_tb = ThingBody::from(&TypedBytes {
            type_id: TypeId(THING_LINK_KIND.0 as u128),
            codec_id: CodecId::POSTCARD,
            bytes: postcard::to_allocvec(&l).unwrap(),
        })
        .unwrap();
        kernel.graph.create_thing(THING_LINK_KIND, l_tb);
        did
    };

    let apps = make_dir("apps", root_id);
    let drivers = make_dir("drivers", root_id);
    let fonts = make_dir("fonts", root_id);
    let cursors = make_dir("cursors", root_id);
    let icons = make_dir("icons", root_id);

    kernel.bridge.log("loader: Mounts created\n");

    // Read Policy
    let mut whitelist = None;
    if let Some(h) = iso.open("/boot/init.txt") {
        let mut data = alloc::vec![0u8; h.size as usize];
        iso.read(&h, 0, h.size as usize, &mut data);
        if let Ok(s) = core::str::from_utf8(&data) {
            let list: Vec<String> = s
                .lines()
                .map(|l| l.trim().to_ascii_lowercase())
                .filter(|l| !l.is_empty())
                .map(String::from)
                .collect();
            whitelist = Some(list);
            kernel.bridge.log("loader: init.txt loaded\n");
        }
    }

    // apps
    let apps_entries = iso.read_dir("/boot/apps").unwrap_or_default();
    kernel.bridge.log("loader: Spawning app loaders...\n");

    for entry in apps_entries {
        if entry.is_dir {
            continue;
        }
        let should_run = if !entry.name.ends_with(".elf") {
            false
        } else {
            if let Some(wl) = &whitelist {
                wl.contains(&entry.name)
            } else {
                false
            }
        };

        let path = alloc::format!("/boot/apps/{}", entry.name);
        let f_args = FileArgs {
            iso: iso.clone(),
            path,
            dir_id: Some(apps),
            should_spawn: should_run,
            hhdm,
        };

        let args_box = Box::new(f_args);
        let args_ptr = Box::into_raw(args_box) as u64;

        let layout = unsafe { Layout::from_size_align_unchecked(256 * 1024, 16) };
        let stack_ptr = unsafe { alloc(layout) };
        let stack_top = unsafe { stack_ptr.add(layout.size()) as u64 } - 8;

        kernel.scheduler.spawn(
            &kernel.bridge,
            "app_loader",
            loader_entry,
            stack_top,
            args_ptr,
            0,
            0,
        );
    }

    // drivers
    let drv_entries = iso.read_dir("/boot/drivers").unwrap_or_default();
    kernel.bridge.log("loader: Spawning driver loaders...\n");
    for entry in drv_entries {
        if entry.is_dir {
            continue;
        }
        let should_run = if let Some(wl) = &whitelist {
            wl.contains(&entry.name)
        } else {
            false
        };

        let path = alloc::format!("/boot/drivers/{}", entry.name);
        let f_args = FileArgs {
            iso: iso.clone(),
            path,
            dir_id: Some(drivers),
            should_spawn: should_run,
            hhdm,
        };
        let args_box = Box::new(f_args);
        let args_ptr = Box::into_raw(args_box) as u64;

        let layout = unsafe { Layout::from_size_align_unchecked(64 * 1024, 16) };
        let stack_ptr = unsafe { alloc(layout) };
        let stack_top = unsafe { stack_ptr.add(layout.size()) as u64 } - 8;

        kernel.scheduler.spawn(
            &kernel.bridge,
            "driver_loader",
            loader_entry,
            stack_top,
            args_ptr,
            0,
            0,
        );
    }

    // assets
    let asset_dirs = [("/boot/cursors", cursors), ("/boot/icons", icons)];
    for (path, dir_id) in asset_dirs {
        let entries = iso.read_dir(path).unwrap_or_default();
        for entry in entries {
            if entry.is_dir {
                continue;
            }
            let full_path = alloc::format!("{}/{}", path, entry.name);
            let f_args = FileArgs {
                iso: iso.clone(),
                path: full_path,
                dir_id: Some(dir_id),
                should_spawn: false,
                hhdm,
            };
            let args_box = Box::new(f_args);
            let args_ptr = Box::into_raw(args_box) as u64;

            let layout = unsafe { Layout::from_size_align_unchecked(64 * 1024, 16) };
            let stack_ptr = unsafe { alloc(layout) };
            let stack_top = unsafe { stack_ptr.add(layout.size()) as u64 } - 8;

            kernel.scheduler.spawn(
                &kernel.bridge,
                "asset_loader",
                loader_entry,
                stack_top,
                args_ptr,
                0,
                0,
            );
        }
    }

    // Fonts (simplified)
    if let Some(h) = iso.open("/boot/.fontcache") {
        kernel.bridge.log("loader: Loading .fontcache...\n");
        let mut data = alloc::vec![0u8; h.size as usize];
        iso.read(&h, 0, h.size as usize, &mut data);

        // Need FontCache definition? It's in `models`.
        // assuming standard postcard
    }

    kernel.bridge.log("loader: All scan tasks spawned.\n");
}

pub fn ingest_module<B: FullMachineBridge>(
    kernel: &mut Kernel<B>,
    name: &str,
    data: &[u8],
    parent_dir_id: Option<ThingId>,
    hhdm_u64: u64,
) -> ThingId {
    use abi::wire::typed::{CodecId, TypeId, TypedBytes};
    use thing_models::builtins::core_kinds::ModuleBody;
    use thing_models::builtins::core_kinds::{ByteSpaceBody, ByteSpaceRef};
    use thing_models::builtins::ids::*;
    use thing_models::builtins::symbols::SYM_BYTESPACE;
    use thing_models::core::fs::FileBody;

    // kernel.bridge.log(alloc::format!("ingest: processing file {}\n", name).as_str());

    let mtype = classify_bytes(data);
    let role_enum = get_module_role(name, &mtype);

    let file_id = {
        let file = FileBody {
            name: String::from(name),
            size: data.len() as u64,
            lba: 0,
            flags: 1, // Ram-backed
        };
        let f_bytes = postcard::to_allocvec(&file).unwrap();
        let f_tb = ThingBody::from(&TypedBytes {
            type_id: TypeId(THING_FILE_KIND.0 as u128),
            codec_id: CodecId::POSTCARD,
            bytes: f_bytes,
        })
        .unwrap();
        kernel.graph.create_thing(THING_FILE_KIND, f_tb)
    };

    if let Some(parent) = parent_dir_id {
        let link = thing_models::link::LinkBody {
            from: parent,
            to: file_id,
            predicate: THING_HAS_ENTRY_KIND,
        };
        let lb = ThingBody::from(&TypedBytes {
            type_id: TypeId(THING_LINK_KIND.0 as u128),
            codec_id: CodecId::POSTCARD,
            bytes: postcard::to_allocvec(&link).unwrap(),
        })
        .unwrap();
        kernel.graph.create_thing(THING_LINK_KIND, lb);
    }

    // --- Module Thing ---
    let role_str = match role_enum {
        ModuleRole::App => "app",
        ModuleRole::Driver => "driver",
        ModuleRole::Debug => "debug",
        ModuleRole::Asset => "asset",
        ModuleRole::Ignore => "ignore",
    };

    // 1. Store Bytes
    let bs_id = kernel.bytespaces.create_from_slice(data, 1).unwrap_or(0);

    // 2. ByteSpace Thing
    let bs_body = ByteSpaceBody {
        store_id: bs_id,
        len: data.len() as u64,
        flags: 1,
        backing: SYM_BYTESPACE,
    };
    let bs_tb = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_BYTESPACE_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: postcard::to_allocvec(&bs_body).unwrap(),
    })
    .unwrap();
    let bs_thing_id = kernel.graph.create_thing(THING_BYTESPACE_KIND, bs_tb);

    // Track physical base if HHDM mapped (Limine modules are usually HHDM)
    let base_ptr = data.as_ptr() as u64;
    let base_phys = if base_ptr >= hhdm_u64 {
        base_ptr - hhdm_u64
    } else {
        0
    };

    let mod_body = ModuleBody {
        path: String::from(name),
        size_bytes: data.len() as u64,
        base_phys,
        index: 0,
        role: String::from(role_str),
        mime: String::from("application/octet-stream"),
        kind: String::from(role_str),
        sniff: 0,
        valid: true,
        bytes: ByteSpaceRef {
            id: bs_id,
            len: data.len() as u64,
        },
    };

    let m_bytes = postcard::to_allocvec(&mod_body).unwrap();
    let m_tb = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_MODULE_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: m_bytes,
    })
    .unwrap();

    let mod_id = kernel.graph.create_thing(THING_MODULE_KIND, m_tb);

    // Link Module -> ByteSpace (HAS_BYTES)
    let link_bytes = thing_models::link::LinkBody {
        from: mod_id,
        to: bs_thing_id,
        predicate: THING_HAS_BYTES_KIND,
    };
    let lb_bytes = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_LINK_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: postcard::to_allocvec(&link_bytes).unwrap(),
    })
    .unwrap();
    kernel.graph.create_thing(THING_LINK_KIND, lb_bytes);

    // Link File -> Module (BACKED_BY)
    let link = thing_models::link::LinkBody {
        from: file_id,
        to: mod_id,
        predicate: THING_BACKED_BY_KIND,
    };
    let lb = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_LINK_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: postcard::to_allocvec(&link).unwrap(),
    })
    .unwrap();
    kernel.graph.create_thing(THING_LINK_KIND, lb);

    // Root -> Module link for discovery
    let root_link = thing_models::link::LinkBody {
        from: THING_BOOT_ROOT,
        to: mod_id,
        predicate: THING_HAS_MODULE_KIND,
    };
    let rl_tb = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_LINK_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: postcard::to_allocvec(&root_link).unwrap(),
    })
    .unwrap();
    kernel.graph.create_thing(THING_LINK_KIND, rl_tb);

    mod_id
}

pub fn register_boot_process<B: FullMachineBridge>(
    kernel: &mut Kernel<B>,
    name: &str,
    entry_point: u64,
) -> ThingId {
    use abi::wire::typed::{CodecId, TypeId, TypedBytes};
    use thing_models::builtins::core_kinds::BootProgramBody;
    use thing_models::builtins::ids::*;
    use thing_models::core::process::{ProcessBody, ProcessState};

    // Create Process Thing
    let p_body = ProcessBody {
        pid: (kernel.scheduler.processes.len()) as u64, // Estimate
        name: kernel
            .symbols
            .intern(name)
            .unwrap_or(thing_models::builtins::symbols::SYM_PROCESS),
        state: ProcessState::Running,
    };
    let tb = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_PROCESS_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: postcard::to_allocvec(&p_body).unwrap(),
    })
    .unwrap();
    let process_id = kernel.graph.create_thing(THING_PROCESS_KIND, tb);

    // Link BootRoot -> Process (SPAWNED)
    let link = thing_models::link::LinkBody {
        from: THING_BOOT_ROOT,
        to: process_id,
        predicate: thing_models::builtins::ids::THING_SPAWNED_KIND,
    };
    let l_tb = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_LINK_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: postcard::to_allocvec(&link).unwrap(),
    })
    .unwrap();
    kernel.graph.create_thing(THING_LINK_KIND, l_tb);

    // Create BootProgram and links
    let bp = BootProgramBody {
        name: alloc::string::String::from(name),
        binary: alloc::string::String::from(name),
        priority: 0,
        entry_point,
    };
    let tb = ThingBody::from(&TypedBytes {
        type_id: TypeId(thing_models::builtins::ids::THING_BOOT_PROGRAM_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: postcard::to_allocvec(&bp).unwrap(),
    })
    .unwrap();
    let prog_id = kernel
        .graph
        .create_thing(thing_models::builtins::ids::THING_BOOT_PROGRAM_KIND, tb);

    // Link Process -> BootProgram (RUNS)
    let link = thing_models::link::LinkBody {
        from: process_id,
        to: prog_id,
        predicate: thing_models::builtins::ids::THING_RUNS_KIND,
    };
    let lb = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_LINK_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: postcard::to_allocvec(&link).unwrap(),
    })
    .unwrap();
    kernel.graph.create_thing(THING_LINK_KIND, lb);

    process_id
}
