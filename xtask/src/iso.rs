use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn run(env: String, cmdline: Option<String>, init_module: Option<String>) -> Result<()> {
    let root = project_root();

    // 1. Build artifacts
    crate::build::run(&env)?;

    let (target_triple, efi_boot_name) = match env.as_str() {
        "x86_64" => ("x86_64-unknown-none", "BOOTX64.EFI"),
        "aarch64" => ("aarch64-unknown-none", "BOOTAA64.EFI"),
        "riscv64" => ("riscv64gc-unknown-none-elf", "BOOTRISCV64.EFI"),
        "loongarch64" => ("loongarch64-unknown-none", "BOOTLOONGARCH64.EFI"),
        _ => anyhow::bail!("Unsupported env for iso: {}", env),
    };

    // 2. Assemble ISO Root
    println!("==> Assembling ISO root for {}...", env);
    let target_dir = root.join("target");
    let iso_root = target_dir.join("iso_root").join(&env);

    if iso_root.exists() {
        fs::remove_dir_all(&iso_root)?;
    }
    fs::create_dir_all(&iso_root)?;

    let boot_dir = iso_root.join("boot");
    fs::create_dir_all(&boot_dir)?;

    let modules_dir = boot_dir.join("modules");
    fs::create_dir_all(&modules_dir)?;

    let bin_src = root.join("target").join(target_triple).join("debug");

    // Copy Bran (Kernel)
    let bran_src = bin_src.join("bran");
    fs::copy(&bran_src, boot_dir.join("kernel"))
        .with_context(|| format!("Failed to copy kernel from {:?}", bran_src))?;

    // Copy Sprout -> /boot/modules/sprout
    let sprout_module_name = init_module.unwrap_or_else(|| "sprout".to_string());
    let sprout_src = bin_src.join(&sprout_module_name);
    // We always name it 'sprout' in the ISO so limine.conf finds it
    fs::copy(&sprout_src, modules_dir.join("sprout")).with_context(|| {
        format!(
            "Failed to copy {} from {:?}",
            sprout_module_name, sprout_src
        )
    })?;

    // Copy Bloom -> /boot/modules/bloom
    let bloom_src = bin_src.join("bloom");
    fs::copy(&bloom_src, modules_dir.join("bloom"))
        .with_context(|| format!("Failed to copy bloom from {:?}", bloom_src))?;

    // Copy Clock -> /boot/modules/clock
    let clock_src = bin_src.join("clock");
    fs::copy(&clock_src, modules_dir.join("clock"))
        .with_context(|| format!("Failed to copy clock from {:?}", clock_src))?;

    // Copy Heap Smoke -> /boot/modules/heap_smoke
    let heap_smoke_src = bin_src.join("heap_smoke");
    if heap_smoke_src.exists() {
        fs::copy(&heap_smoke_src, modules_dir.join("heap_smoke"))
            .with_context(|| format!("Failed to copy heap_smoke from {:?}", heap_smoke_src))?;
    }

    // Copy Apps
    let apps = [
        "graph_smoke",
        "log_smoke",
        "cap_fail",
        "inputd",
        "echo",
        "inspector",
        "logview",
        "simd_check",
        "ontology_dump",
        "ontology_check",
        "thingcheck",
        "usbd",
    ];
    for app in apps {
        let src = bin_src.join(app);
        if src.exists() {
            fs::copy(&src, modules_dir.join(app))
                .with_context(|| format!("Failed to copy {} from {:?}", app, src))?;
        } else {
            eprintln!("    [WARNING] App binary not found: {:?}", src);
        }
    }

    // Copy Assets
    let assets_dir = boot_dir.join("assets");
    fs::create_dir_all(&assets_dir)?;

    // Wallpaper
    let clouds_bmp = root.join("assets/wallpapers/clouds.bmp");
    if clouds_bmp.exists() {
        fs::copy(&clouds_bmp, assets_dir.join("clouds.bmp"))
            .with_context(|| format!("Failed to copy clouds.bmp from {:?}", clouds_bmp))?;
    }

    // Cursor assets (.cur and .ani files)
    let cursors_src = root.join("assets/cursors/plain");
    let cursor_files = [
        "Normal.cur",
        "Working.ani",
        "Busy.cur",
        "Help.cur",
        "Move.cur",
        "Precision.cur",
        "Text.cur",
        "Link.ani",
    ];
    for cursor_file in cursor_files {
        let src = cursors_src.join(cursor_file);
        if src.exists() {
            fs::copy(&src, assets_dir.join(cursor_file))
                .with_context(|| format!("Failed to copy {} from {:?}", cursor_file, src))?;
        }
    }

    // Limine Config
    let conf_src = root.join("limine.conf");
    let limine_dest = boot_dir.join("limine");
    fs::create_dir_all(&limine_dest)?;

    // Read config and remove modules for AArch64 debug (if env == aarch64)
    // Reverted: modules are fine, issue was MAIR.
    let mut conf_data = fs::read_to_string(&conf_src)?;

    if let Some(cmd) = cmdline {
        let needle = "kernel_path: boot():/boot/kernel";
        let insertion = format!("\n    cmdline: {}", cmd);
        conf_data = conf_data.replacen(needle, &format!("{}{}", needle, insertion), 1);
    }
    fs::write(limine_dest.join("limine.conf"), conf_data)?;

    // Vendor bins (Limine)
    let vendor_limine = root.join("vendor/limine");

    // Copy limine-uefi-cd.bin for all arches (generic EFI boot)
    let src_uefi_cd = vendor_limine.join("limine-uefi-cd.bin");
    if src_uefi_cd.exists() {
        fs::copy(&src_uefi_cd, limine_dest.join("limine-uefi-cd.bin"))?;
    } else {
        eprintln!("    [WARNING] Missing limine-uefi-cd.bin");
    }

    if env == "x86_64" {
        // BIOS support files for x86_64
        let bios_files = ["limine-bios.sys", "limine-bios-cd.bin"];
        for f in bios_files {
            let src = vendor_limine.join(f);
            if src.exists() {
                fs::copy(&src, limine_dest.join(f))?;
            }
        }
    }

    // EFI Boot
    let efi_boot_dir = iso_root.join("EFI").join("BOOT");
    fs::create_dir_all(&efi_boot_dir)?;

    let src_efi = vendor_limine.join(efi_boot_name);
    if src_efi.exists() {
        fs::copy(&src_efi, efi_boot_dir.join(efi_boot_name))?;
    } else {
        eprintln!("    [WARNING] Missing UEFI bootloader: {:?}", src_efi);
    }

    // IA32 for x86_64 extra compat?
    if env == "x86_64" {
        let src_ia32 = vendor_limine.join("BOOTIA32.EFI");
        if src_ia32.exists() {
            fs::copy(&src_ia32, efi_boot_dir.join("BOOTIA32.EFI"))?;
        }
    }

    // 4. Generate ISO
    println!("==> Generating ISO via xorriso...");
    let iso_dir = target_dir.join("iso");
    fs::create_dir_all(&iso_dir)?;
    let iso_path = iso_dir.join(format!("thingos-{}.iso", env));

    let mut cmd = Command::new("xorriso");
    cmd.arg("-as").arg("mkisofs");

    // BIOS Boot (only x86_64)
    if env == "x86_64" {
        cmd.arg("-b").arg("boot/limine/limine-bios-cd.bin");
        cmd.arg("-no-emul-boot")
            .arg("-boot-load-size")
            .arg("4")
            .arg("-boot-info-table");
    }

    // EFI Boot (All)
    cmd.arg("--efi-boot").arg("boot/limine/limine-uefi-cd.bin");
    cmd.arg("-efi-boot-part").arg("--efi-boot-image");
    cmd.arg("--protective-msdos-label");
    cmd.arg("-o").arg(&iso_path);
    cmd.arg(&iso_root);

    let status = cmd.status().context("Failed to run xorriso")?;
    if !status.success() {
        anyhow::bail!("xorriso failed");
    }

    // BIOS Install (Limine 5+) - Only x86_64
    if env == "x86_64" {
        let limine_tool_path = vendor_limine.join("limine");
        // Build tool if needed (already handled in previous step ideally, checking existence)
        if !limine_tool_path.exists() || !limine_tool_path.is_file() {
            println!("==> Building Limine tool...");
            let status = Command::new("make").current_dir(&vendor_limine).status()?;
            if !status.success() {
                eprintln!("[WARNING] Failed to build Limine tool. ISO might not be BIOS bootable.");
            }
        }

        if limine_tool_path.exists() {
            let status = Command::new(&limine_tool_path)
                .arg("bios-install")
                .arg(&iso_path)
                .status()?;
            if !status.success() {
                eprintln!("[WARNING] Limine bios-install failed.");
            }
        }
    }

    println!("ISO created at: {}", iso_path.display());

    Ok(())
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
