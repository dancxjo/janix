//! Image creation tasks - ISO and HDD.

use crate::common::{Result, image_name};
use xshell::{Shell, cmd};

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct ProgramConfig {
    pub name: &'static str,
    pub is_init: bool,
    pub boot_module: bool,
    pub features: Vec<&'static str>,
}

/// Configuration for ISO builds.
#[derive(Default)]
pub struct IsoConfig<'a> {
    /// Display resolution (e.g., "1920x1080"). None = 1920x1080.
    pub resolution: Option<&'a str>,
    /// Explicit ISO output path. None = use timestamped naming.
    pub iso_path: Option<&'a Path>,
}

pub fn default_programs() -> Vec<ProgramConfig> {
    vec![
        ProgramConfig {
            name: "sprout",
            is_init: false,
            boot_module: true,
            features: vec!["diagnostic-apps"],
        },
        ProgramConfig {
            name: "bristle",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig {
            name: "rtc_cmos",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig {
            name: "clock",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig {
            name: "font_explorer",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig {
            name: "ps2_kbd",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig {
            name: "echo",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig {
            name: "bloom",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig {
            name: "ps2_mouse",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig {
            name: "root_batch_bench",
            is_init: false,
            boot_module: false,
            features: vec![],
        },
        ProgramConfig {
            name: "root_watch_tester",
            is_init: false,
            boot_module: false,
            features: vec![],
        },
        ProgramConfig {
            name: "display_bootfb",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig {
            name: "display_virtio_gpu",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig {
            name: "fontd",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig {
            name: "blossom",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig {
            name: "ingestd",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig {
            name: "cambium",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        // Scheduler fairness test apps
        ProgramConfig {
            name: "scheduler_fairness",
            is_init: false,
            boot_module: false,
            features: vec![],
        },
        ProgramConfig {
            name: "drawlist_demo",
            is_init: false,
            boot_module: false,
            features: vec![],
        },
        ProgramConfig {
            name: "tick_printer",
            is_init: false,
            boot_module: false,
            features: vec![],
        },
        ProgramConfig {
            name: "photosynthesis",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
        ProgramConfig {
            name: "ahci_disk",
            is_init: false,
            boot_module: false,
            features: vec![],
        },
        ProgramConfig {
            name: "iso_reader",
            is_init: false,
            boot_module: true,
            features: vec![],
        },
    ]
}

fn generate_limine_config(
    _sh: &Shell,
    programs: &[ProgramConfig],
    assets: &[PathBuf],
    resolution: Option<&str>,
) -> String {
    let res = resolution.unwrap_or("1920x1080");
    let mut conf = String::new();
    conf.push_str("timeout: 0\nquiet: yes\nverbose: no\nserial: yes\n\n");
    conf.push_str("/ThingOS\n");
    conf.push_str("    protocol: limine\n");
    conf.push_str(&format!("    resolution: {}\n", res));
    conf.push_str("    kernel_path: boot():/boot/kernel\n");

    for prog in programs {
        if !prog.boot_module {
            continue;
        }
        conf.push_str(&format!("    module_path: boot():/boot/{}\n", prog.name));
        if prog.is_init {
            conf.push_str("    module_cmdline: init\n");
        }
    }

    for asset in assets {
        let path_str = asset.to_string_lossy();
        let clean_path = path_str.replace("\\", "/");

        // Allowed assets
        let allowed = clean_path.ends_with("NotoSans-Regular.ttf")
            || clean_path.ends_with("future/default.svg")
            || clean_path.ends_with("wallpapers/leather.bmp");

        if allowed {
            conf.push_str(&format!("    module_path: boot():/{}\n", clean_path));
        }
    }

    conf
}

/// Build an ISO image for the target architecture with default settings.
pub fn build_iso(sh: &Shell, arch: &str, programs: &[ProgramConfig]) -> Result<PathBuf> {
    build_iso_with_config(sh, arch, programs, &IsoConfig::default())
}

/// Build an ISO image for the target architecture with custom configuration.
/// Each build produces a unique, self-contained ISO that doesn't affect other sessions.
pub fn build_iso_with_config(
    sh: &Shell,
    arch: &str,
    programs: &[ProgramConfig],
    config: &IsoConfig,
) -> Result<PathBuf> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    // Determine ISO path - always unique, never use fixed names
    let iso_name = if let Some(explicit_path) = config.iso_path {
        explicit_path.to_string_lossy().to_string()
    } else {
        format!("thing-os-{}-{}-{}.iso", arch, timestamp, nanos)
    };

    let iso_root = Path::new("iso_root");

    println!("Building ISO {}...", iso_name);

    if sh.path_exists(iso_root) {
        sh.remove_path(iso_root)?;
    }
    sh.create_dir(iso_root.join("boot"))?;
    sh.create_dir(iso_root.join("boot/limine"))?;
    sh.create_dir(iso_root.join("EFI/BOOT"))?;

    cmd!(sh, "cp -r assets iso_root/").run()?;

    let mut asset_files = Vec::new();
    for entry in WalkDir::new("assets") {
        let entry: walkdir::DirEntry = entry?;
        if entry.file_type().is_file() {
            let path = entry.path();
            if path.to_string_lossy().contains("assets/icons")
                && !path.to_string_lossy().contains("thingos")
            {
                continue;
            }
            asset_files.push(path.to_path_buf());
        }
    }

    asset_files.sort_by(|a, b| {
        let a_str = a.to_string_lossy();
        let b_str = b.to_string_lossy();

        let a_priority = if a_str.ends_with(".cur") || a_str.ends_with(".ani") {
            0
        } else if a_str.ends_with(".bmp") {
            1
        } else if a_str.ends_with(".ttf") {
            2
        } else if a_str.ends_with(".svg") {
            4
        } else {
            3
        };

        let b_priority = if b_str.ends_with(".cur") || b_str.ends_with(".ani") {
            0
        } else if b_str.ends_with(".bmp") {
            1
        } else if b_str.ends_with(".ttf") {
            2
        } else if b_str.ends_with(".svg") {
            4
        } else {
            3
        };

        match a_priority.cmp(&b_priority) {
            std::cmp::Ordering::Equal => a_str.cmp(&b_str),
            other => other,
        }
    });

    let kernel_src = format!("bran/bin-{}/kernel", arch);
    sh.copy_file(&kernel_src, iso_root.join("boot/kernel"))?;

    println!("Building userspace programs...");

    let cwd = std::env::current_dir().unwrap();
    let target_json = if arch == "x86_64" {
        cwd.join("targets/x86_64-unknown-thingos.json")
    } else if arch == "riscv64" {
        cwd.join("targets/riscv64gc-unknown-thingos.json")
    } else {
        cwd.join(format!("targets/{}-unknown-thingos.json", arch))
    };
    let target = target_json.to_str().unwrap();

    for prog in programs {
        build_userspace_app_with_features(sh, prog.name, target, "release", &prog.features)?;
        copy_userspace_binary(
            sh,
            prog.name,
            target,
            "release",
            iso_root
                .join(format!("boot/{}", prog.name))
                .to_str()
                .unwrap(),
        )?;
    }

    let limine_conf_content = generate_limine_config(sh, programs, &asset_files, config.resolution);
    sh.write_file(
        iso_root.join("boot/limine/limine.conf"),
        limine_conf_content,
    )?;

    match arch {
        "x86_64" => {
            sh.copy_file(
                "vendor/limine/limine-bios.sys",
                iso_root.join("boot/limine/limine-bios.sys"),
            )?;
            sh.copy_file(
                "vendor/limine/limine-bios-cd.bin",
                iso_root.join("boot/limine/limine-bios-cd.bin"),
            )?;
            sh.copy_file(
                "vendor/limine/limine-uefi-cd.bin",
                iso_root.join("boot/limine/limine-uefi-cd.bin"),
            )?;
            sh.copy_file(
                "vendor/limine/BOOTX64.EFI",
                iso_root.join("EFI/BOOT/BOOTX64.EFI"),
            )?;
            sh.copy_file(
                "vendor/limine/BOOTIA32.EFI",
                iso_root.join("EFI/BOOT/BOOTIA32.EFI"),
            )?;

            cmd!(sh, "xorriso -as mkisofs -R -J -b boot/limine/limine-bios-cd.bin -no-emul-boot -boot-load-size 4 -boot-info-table --efi-boot boot/limine/limine-uefi-cd.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o {iso_name}").run()?;
            cmd!(sh, "./vendor/limine/limine bios-install {iso_name}").run()?;

            sh.remove_path("iso_root")?;
            println!("ISO created: {}", iso_name);

            Ok(PathBuf::from(iso_name))
        }
        "aarch64" => {
            sh.copy_file(
                "vendor/limine/limine-uefi-cd.bin",
                iso_root.join("boot/limine/limine-uefi-cd.bin"),
            )?;
            sh.copy_file(
                "vendor/limine/BOOTAA64.EFI",
                iso_root.join("EFI/BOOT/BOOTAA64.EFI"),
            )?;

            cmd!(sh, "xorriso -as mkisofs -R -J --efi-boot boot/limine/limine-uefi-cd.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o {iso_name}").run()?;

            sh.remove_path("iso_root")?;
            println!("ISO created: {}", iso_name);

            Ok(PathBuf::from(iso_name))
        }
        "riscv64" => {
            let efi_img = iso_root.join("boot/limine/limine-uefi-riscv64.bin");
            let efi_img_str = efi_img.to_str().unwrap();
            cmd!(
                sh,
                "dd if=/dev/zero of={efi_img_str} bs=1K count=2880 status=none"
            )
            .run()?;
            cmd!(sh, "mformat -i {efi_img_str} -f 2880 ::").run()?;
            cmd!(sh, "mmd -i {efi_img_str} ::/EFI ::/EFI/BOOT").run()?;
            cmd!(
                sh,
                "mcopy -i {efi_img_str} vendor/limine/BOOTRISCV64.EFI ::/EFI/BOOT/BOOTRISCV64.EFI"
            )
            .run()?;
            sh.write_file(
                iso_root.join("startup.nsh"),
                "\\EFI\\BOOT\\BOOTRISCV64.EFI\n",
            )?;
            let startup_nsh = iso_root.join("startup.nsh");
            let startup_nsh_str = startup_nsh.to_str().unwrap();
            cmd!(sh, "mcopy -i {efi_img_str} {startup_nsh_str} ::").run()?;
            sh.copy_file(
                "vendor/limine/BOOTRISCV64.EFI",
                iso_root.join("EFI/BOOT/BOOTRISCV64.EFI"),
            )?;

            cmd!(sh, "xorriso -as mkisofs -R -J --efi-boot boot/limine/limine-uefi-riscv64.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o {iso_name}").run()?;

            sh.remove_path("iso_root")?;
            println!("ISO created: {}", iso_name);

            Ok(PathBuf::from(iso_name))
        }
        "loongarch64" => {
            let efi_img = iso_root.join("boot/limine/limine-uefi-loongarch64.bin");
            let efi_img_str = efi_img.to_str().unwrap();
            cmd!(
                sh,
                "dd if=/dev/zero of={efi_img_str} bs=1K count=2880 status=none"
            )
            .run()?;
            cmd!(sh, "mformat -i {efi_img_str} -f 2880 ::").run()?;
            cmd!(sh, "mmd -i {efi_img_str} ::/EFI ::/EFI/BOOT").run()?;
            cmd!(sh, "mcopy -i {efi_img_str} vendor/limine/BOOTLOONGARCH64.EFI ::/EFI/BOOT/BOOTLOONGARCH64.EFI").run()?;
            sh.write_file(
                iso_root.join("startup.nsh"),
                "\\EFI\\BOOT\\BOOTLOONGARCH64.EFI\n",
            )?;
            let startup_nsh = iso_root.join("startup.nsh");
            let startup_nsh_str = startup_nsh.to_str().unwrap();
            cmd!(sh, "mcopy -i {efi_img_str} {startup_nsh_str} ::").run()?;
            sh.copy_file(
                "vendor/limine/BOOTLOONGARCH64.EFI",
                iso_root.join("EFI/BOOT/BOOTLOONGARCH64.EFI"),
            )?;

            cmd!(sh, "xorriso -as mkisofs -R -J --efi-boot boot/limine/limine-uefi-loongarch64.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o {iso_name}").run()?;

            sh.remove_path("iso_root")?;
            println!("ISO created: {}", iso_name);

            Ok(PathBuf::from(iso_name))
        }
        _ => return Err(format!("Unsupported architecture: {}", arch).into()),
    }
}

fn build_userspace_app_with_features(
    sh: &Shell,
    name: &str,
    target: &str,
    profile: &str,
    features: &[&str],
) -> Result<()> {
    println!("Building {} ...", name);

    let mut cmd = cmd!(
        sh,
        "cargo build --target {target} --profile {profile} -p {name} -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem"
    )
    .env("RUSTFLAGS", "-Awarnings");

    for f in features {
        cmd = cmd.arg("--features").arg(f);
    }

    cmd.run()?;
    Ok(())
}

/// Copy and objcopy a userspace binary
fn copy_userspace_binary(
    sh: &Shell,
    name: &str,
    target: &str,
    profile_dir: &str,
    dst: &str,
) -> Result<()> {
    let target_name = std::path::Path::new(target)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(target);

    let elf = format!("target/{}/{}/{}", target_name, profile_dir, name);

    cmd!(sh, "cp {elf} {dst}").run()?;
    Ok(())
}

/// Build an HDD image for the target architecture.
pub fn build_hdd(sh: &Shell, arch: &str, programs: &[ProgramConfig]) -> Result<PathBuf> {
    let name = image_name(arch);
    let hdd = format!("{}.hdd", name);

    println!("Building HDD image for {}...", arch);

    sh.remove_path(&hdd)?;
    cmd!(sh, "dd if=/dev/zero bs=1M count=0 seek=64 of={hdd}").run()?;
    cmd!(sh, "sgdisk {hdd} -n 1:2048 -t 1:ef00").run()?;

    if arch == "x86_64" {
        cmd!(sh, "./vendor/limine/limine bios-install {hdd}").run()?;
    }

    cmd!(sh, "mformat -i {hdd}@@1M").run()?;
    cmd!(
        sh,
        "mmd -i {hdd}@@1M ::/EFI ::/EFI/BOOT ::/boot ::/boot/limine"
    )
    .run()?;

    cmd!(sh, "mcopy -i {hdd}@@1M -s assets ::").run()?;

    let mut asset_files = Vec::new();
    for entry in WalkDir::new("assets") {
        let entry: walkdir::DirEntry = entry?;
        if entry.file_type().is_file() {
            let path = entry.path();
            if path.to_string_lossy().contains("assets/icons") {
                continue;
            }
            asset_files.push(path.to_path_buf());
        }
    }

    let kernel_src = format!("bran/bin-{}/kernel", arch);
    cmd!(sh, "mcopy -i {hdd}@@1M {kernel_src} ::/boot").run()?;

    let limine_conf_content = generate_limine_config(sh, programs, &asset_files, None);
    let limine_cfg = "limine.generated.conf";
    sh.write_file(limine_cfg, limine_conf_content)?;

    cmd!(
        sh,
        "mcopy -i {hdd}@@1M {limine_cfg} ::/boot/limine/limine.conf"
    )
    .run()?;
    sh.remove_path(limine_cfg)?;

    match arch {
        "x86_64" => {
            cmd!(
                sh,
                "mcopy -i {hdd}@@1M vendor/limine/limine-bios.sys ::/boot/limine"
            )
            .run()?;
            cmd!(
                sh,
                "mcopy -i {hdd}@@1M vendor/limine/BOOTX64.EFI ::/EFI/BOOT"
            )
            .run()?;
            cmd!(
                sh,
                "mcopy -i {hdd}@@1M vendor/limine/BOOTIA32.EFI ::/EFI/BOOT"
            )
            .run()?;
        }
        "aarch64" => {
            cmd!(
                sh,
                "mcopy -i {hdd}@@1M vendor/limine/BOOTAA64.EFI ::/EFI/BOOT"
            )
            .run()?;
        }
        "riscv64" => {
            cmd!(
                sh,
                "mcopy -i {hdd}@@1M vendor/limine/BOOTRISCV64.EFI ::/EFI/BOOT"
            )
            .run()?;
        }
        "loongarch64" => {
            cmd!(
                sh,
                "mcopy -i {hdd}@@1M vendor/limine/BOOTLOONGARCH64.EFI ::/EFI/BOOT"
            )
            .run()?;
        }
        _ => return Err(format!("Unsupported architecture: {}", arch).into()),
    }

    println!("HDD image created: {}", hdd);
    Ok(PathBuf::from(hdd))
}
