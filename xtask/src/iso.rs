use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn run(env: String, cmdline: Option<String>) -> Result<()> {
    // 1. Determine Target
    let (target_triple, kernel_bin_name, uefi_boot_name) = match env.as_str() {
        "x86_64" => ("x86_64-thingos.json", "kernel_x86_64", "BOOTX64.EFI"),
        "aarch64" => ("aarch64-thingos.json", "kernel_aarch64", "BOOTAA64.EFI"),
        _ => anyhow::bail!("Unsupported env for iso: {}. Use x86_64 or aarch64", env),
    };

    let root = project_root();

    // 2. Build Kernel (ensure it exists/is fresh)
    println!("==> Building kernel for {}...", env);
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    // Note: We use check for speed in previous steps, but for ISO we need the binary.
    // Task 2 was implicit, so we do the build here "just in time".
    // We need build-std=core because we are no_std.
    // Target path is relative to repo root for `cargo`.
    // The target.json is in targets/

    // Construct target path relative to project root purely for cargo's --target flag
    let target_flag = format!("targets/{}", target_triple);

    let status = Command::new(&cargo)
        .arg("build")
        .arg("-p")
        .arg(kernel_bin_name)
        .arg("--target")
        .arg(&target_flag)
        .arg("-Z")
        .arg("build-std=core,alloc,compiler_builtins")
        .arg("-Z")
        .arg("build-std-features=compiler-builtins-mem")
        .current_dir(&root)
        .status()
        .context("Failed to run cargo build")?;

    if !status.success() {
        anyhow::bail!("Kernel build failed");
    }

    // 2.5 Build User Apps
    println!("==> Building user apps for {}...", env);
    let mut user_apps = vec![
        "graph_dump",
        "ps2_keyboard",
        "ps2_mouse",
        "keylog",
        "syscall_crud_smoke",
        "clock",
        "sleep_smoke",
        "sleep_accuracy_smoke",
        "ls_boot",
        "cat_boot",
    ];
    if env == "x86_64" {
        user_apps.push("rtc_x86");
    } else if env == "aarch64" {
        user_apps.push("rtc_aarch64");
    }

    for app in &user_apps {
        let status = Command::new(&cargo)
            .arg("build")
            // Use manifest path explicitly since user apps are in a different workspace
            .arg("--manifest-path")
            .arg("user/Cargo.toml")
            .arg("-p")
            .arg(app)
            .arg("--target")
            .arg("x86_64-unknown-none") // User workspace is configured for this target
            .arg("-Z")
            .arg("build-std=core,alloc,compiler_builtins")
            .current_dir(&root)
            .status()
            .context(format!("Failed to build user app {}", app))?;

        if !status.success() {
            anyhow::bail!("User app {} build failed", app);
        }
    }
    // Copied and cleaned up above.

    // 3. Assemble ISO Root
    println!("==> Assembling ISO root for {}...", env);
    let target_dir = root.join("target");
    let iso_root = target_dir.join("iso_root").join(&env);

    if iso_root.exists() {
        fs::remove_dir_all(&iso_root)?;
    }
    fs::create_dir_all(&iso_root)?;

    // Layout:
    // /boot/kernel
    // /boot/modules/clock
    // /boot/modules/graph_dump
    // /boot/limine/limine.conf
    // /boot/limine/limine-bios.sys, etc.
    // /EFI/BOOT/BOOTX64.EFI

    let boot_dir = iso_root.join("boot");
    fs::create_dir_all(&boot_dir)?;

    // Copy Kernel
    // Binary location: target/<triple>/debug/<bin_name>
    // but <triple> might be the filename of the json... cargo puts it under target/x86_64-thingos/debug/ usually
    // if target is "targets/x86_64-thingos.json", the triple dir name is "x86_64-thingos"
    let triple_name = Path::new(target_triple)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();
    let bin_path = target_dir
        .join(triple_name)
        .join("debug")
        .join(kernel_bin_name);

    fs::copy(&bin_path, boot_dir.join("kernel"))
        .with_context(|| format!("Failed to copy kernel from {:?}", bin_path))?;

    // Copy Modules
    let modules_dir = boot_dir.join("modules");
    fs::create_dir_all(&modules_dir)?;
    for app in &user_apps {
        let app_bin = root.join("user/target/x86_64-unknown-none/debug").join(app);
        fs::copy(&app_bin, modules_dir.join(app))
            .with_context(|| format!("Failed to copy app {} from {:?}", app, app_bin))?;
    }

    // Create dummy for the other architecture's RTC driver to satisfy limine.conf
    if env == "x86_64" {
        fs::write(modules_dir.join("rtc_aarch64"), "dummy")?;
    } else {
        fs::write(modules_dir.join("rtc_x86"), "dummy")?;
    }

    // Copy Fonts
    // Copy Fonts
    let fonts_src = root.join("assets/fonts");
    let fonts_dst = boot_dir.join("fonts");
    fs::create_dir_all(&fonts_dst)?;

    // Copy all TTF files from assets/fonts
    if fonts_src.exists() {
        for entry in fs::read_dir(&fonts_src).context("Failed to read fonts directory")? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("ttf") {
                let file_name = path.file_name().unwrap();
                fs::copy(&path, fonts_dst.join(file_name))?;
            }
        }
    } else {
        eprintln!("    [WARNING] fonts directory missing: {:?}", fonts_src);
    }

    // Limine Files
    let limine_dest = boot_dir.join("limine");
    fs::create_dir_all(&limine_dest)?;

    // Config
    // Config
    let conf_src = root.join("kernels/limine.conf");
    let mut conf_data = fs::read_to_string(&conf_src)?;

    if let Some(cmd) = cmdline {
        // Inject cmdline into /ThingOS entry
        // We look for "kernel_path: boot():/boot/kernel"
        // and append the cmdline after it.
        let needle = "kernel_path: boot():/boot/kernel";
        let insertion = format!("\n    cmdline: {}", cmd);
        // We only want to replace the first occurrence (main entry), or maybe all?
        // The second entry is "/ThingOS Smoke Mode", which has a hardcoded cmdline.
        // Let's just replace the first one.
        conf_data = conf_data.replacen(needle, &format!("{}{}", needle, insertion), 1);
    }

    fs::write(limine_dest.join("limine.conf"), conf_data)?;

    // Vendor bins
    let vendor_limine = root.join("vendor/limine");

    let limine_files = [
        "limine-bios.sys",
        "limine-bios-cd.bin",
        "limine-uefi-cd.bin",
    ];

    for f in limine_files {
        let src = vendor_limine.join(f);
        if src.exists() {
            fs::copy(&src, limine_dest.join(f))?;
        } else {
            eprintln!("    [WARNING] Missing Limine file: {:?}", src);
        }
    }

    // EFI Boot
    let efi_boot_dir = iso_root.join("EFI").join("BOOT");
    fs::create_dir_all(&efi_boot_dir)?;

    let src_efi = vendor_limine.join(uefi_boot_name);
    if src_efi.exists() {
        fs::copy(&src_efi, efi_boot_dir.join(uefi_boot_name))?;
    } else {
        eprintln!("    [WARNING] Missing UEFI bootloader: {:?}", src_efi);
    }

    // 4. Generate ISO
    println!("==> Generating ISO via xorriso...");
    let iso_dir = target_dir.join("iso");
    fs::create_dir_all(&iso_dir)?;
    let iso_path = iso_dir.join(format!("thingos-{}.iso", env));

    // xorriso flags for Limine hybrid (BIOS+EFI)
    // Ref: Limine docs or user prompt. "Exact flags".
    // "BIOS El Torito boot: limine-bios-cd.bin"
    // "EFI boot image: limine-uefi-cd.bin"

    let mut cmd = Command::new("xorriso");
    cmd.arg("-as").arg("mkisofs");
    cmd.arg("-b").arg("boot/limine/limine-bios-cd.bin");
    cmd.arg("-no-emul-boot")
        .arg("-boot-load-size")
        .arg("4")
        .arg("-boot-info-table");
    cmd.arg("--efi-boot").arg("boot/limine/limine-uefi-cd.bin");
    cmd.arg("-efi-boot-part").arg("--efi-boot-image");
    cmd.arg("--protective-msdos-label");
    cmd.arg("-o").arg(&iso_path);
    cmd.arg(&iso_root);

    let status = cmd.status().context("Failed to run xorriso")?;
    if !status.success() {
        anyhow::bail!("xorriso failed");
    }

    // 5. Limine Deploy (BIOS)
    // If x86_64, we usually need to run `limine bios-install image.iso` to install stage 1/2 to MBR/gap.
    // The user didn't explicitly ask for `limine bios-install`, but "BIOS El Torito" flags usually cover CD boot?
    // Wait, Limine usually requires post-processing for HDD boot, but for ISO (El Torito), maybe not if just CD?
    // Actually, modern Limine often requires `limine deploy` or `limine-deploy` text/binary patching?
    // The prompt says "Build ISO via xorriso... Done when: outputs ...iso".
    // It does NOT mention running `limine deploy` or `limine binary` patching on the ISO.
    // However, without it, BIOS boot from HDD image works, but CD?
    // Limine docs say: "For CD-ROMs... xorriso ... is enough."
    // BUT usually usage involves `limine bios-install` if targeting hybrid HDD/CD.
    // I will stick strictly to the user's requested instructions: "Run xorriso with the exact flags...".
    // I will NOT add extra steps unless it fails verification.

    println!("ISO created at: {}", iso_path.display());

    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
