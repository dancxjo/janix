use anyhow::{Context, Result};
use image::ImageReader;
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
        //"graph_dump", // Disabled to reduce noise
        "ps2_keyboard",
        "ps2_mouse", // We are implementing kernel mouse, but maybe keep it? Or disable if not found.
        "keylog",
        "syscall_crud_smoke",
        "clock",
        // "sleep_smoke", // Obsolete?
        "sleep_accuracy_smoke",
        "ls_boot",
        "cat_boot",
        "input_service",
        "fb_smoke",
        "compositor",
        "loaded",
    ];

    // Check if ps2_mouse exists in user/drivers (it does)
    // Check if ps2_mouse exists in user/drivers (it does)
    // if root.join("user/drivers/ps2_mouse").exists() {
    //      user_apps.push("ps2_mouse");
    // }

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

    let boot_dir = iso_root.join("boot");
    fs::create_dir_all(&boot_dir)?;

    // Copy Kernel
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

    // Copy Apps and Drivers
    let apps_dir = boot_dir.join("apps");
    fs::create_dir_all(&apps_dir)?;
    let drivers_dir = boot_dir.join("drivers");
    fs::create_dir_all(&drivers_dir)?;

    let driver_names = ["ps2_keyboard", "ps2_mouse", "rtc_x86", "rtc_aarch64"];

    let mut init_whitelist = Vec::new();

    for app in &user_apps {
        let is_driver = driver_names.contains(app);
        let dest_dir = if is_driver { &drivers_dir } else { &apps_dir };

        let app_bin = root.join("user/target/x86_64-unknown-none/debug").join(app);

        let dest_name = format!("{}.elf", app);
        let dest_path = dest_dir.join(&dest_name);

        fs::copy(&app_bin, &dest_path)
            .with_context(|| format!("Failed to copy app/driver {} from {:?}", app, app_bin))?;

        if is_driver {
            init_whitelist.push(dest_name.clone());
        } else {
            init_whitelist.push(dest_name);
        }
    }

    // Write init.txt policy
    let init_txt_content = init_whitelist.join("\n");
    fs::write(boot_dir.join("init.txt"), init_txt_content).context("Failed to write init.txt")?;

    // Copy Fonts
    let fonts_src = root.join("assets/fonts");
    let fonts_dst = boot_dir.join("fonts");
    fs::create_dir_all(&fonts_dst)?;

    let allowed_fonts = ["Hack-Regular.ttf", "NotoSans-Regular.ttf"];
    let mut included_modules = Vec::new();

    if fonts_src.exists() {
        println!("==> Processing fonts...");
        for font_name in allowed_fonts {
            let src_path = fonts_src.join(font_name);
            if src_path.exists() {
                let dest_path = fonts_dst.join(font_name);
                fs::copy(&src_path, &dest_path)?;
                included_modules.push(format!("fonts/{}", font_name));
                println!("    Included: {}", font_name);
            }
        }
    }

    // Process Icons
    let icons_src = root.join("assets/icons");
    let icons_dst = boot_dir.join("icons");
    fs::create_dir_all(&icons_dst)?;

    if icons_src.exists() {
        println!("==> Processing icons...");
        for entry in fs::read_dir(&icons_src)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "png") {
                let name = path.file_stem().unwrap().to_string_lossy();
                println!("    Converting {} to BMP...", name);

                let img = ImageReader::open(&path)?.decode()?;
                let dest_path = icons_dst.join(format!("{}.bmp", name));

                // Save as BMP
                img.save(&dest_path)?;
                included_modules.push(format!("icons/{}.bmp", name));
            }
        }
    }

    // Process Cursors
    let cursors_src = root.join("assets/cursors/plain");
    let cursors_dst = boot_dir.join("cursors");
    fs::create_dir_all(&cursors_dst)?;

    if cursors_src.exists() {
        println!("==> Processing cursors (plain)...");
        for entry in fs::read_dir(&cursors_src)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "ani" || ext == "cur" {
                    let name = path.file_name().unwrap().to_string_lossy();
                    let dest_path = cursors_dst.join(name.as_ref());
                    fs::copy(&path, &dest_path)?;
                    included_modules.push(format!("cursors/{}", name));
                    println!("    Included Cursor: {}", name);
                }
            }
        }
    }

    // Limine Files
    let limine_dest = boot_dir.join("limine");
    fs::create_dir_all(&limine_dest)?;

    // Config
    let conf_src = root.join("arch/limine.conf");
    let mut conf_data = fs::read_to_string(&conf_src)?;

    // Generate Module List
    let mut module_lines = String::new();

    // Always include loaded.elf
    module_lines.push_str("    module_path: boot():/boot/apps/loaded.elf\n");

    /*
    // Other apps/drivers if needed as modules (currently disabled, only loaded.elf is bootstrapped)
    for app in &user_apps {
        if app == &"loaded" { continue; }
        // ...
    }
    */

    for mod_path in &included_modules {
        let line = format!("    module_path: boot():/boot/{}\n", mod_path);
        module_lines.push_str(&line);
    }

    if let Some(cmd) = cmdline {
        let needle = "kernel_path: boot():/boot/kernel";
        let insertion = format!("\n    cmdline: {}", cmd);
        conf_data = conf_data.replacen(needle, &format!("{}{}", needle, insertion), 1);
    }

    let needle = "kernel_path: boot():/boot/kernel";
    let with_modules = format!("{}\n{}", needle, module_lines);
    conf_data = conf_data.replace(needle, &with_modules);

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
    cmd.arg("-full-iso9660-filenames");
    cmd.arg("-o").arg(&iso_path);
    cmd.arg(&iso_root);

    let status = cmd.status().context("Failed to run xorriso")?;
    if !status.success() {
        anyhow::bail!("xorriso failed");
    }

    println!("ISO created at: {}", iso_path.display());

    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
