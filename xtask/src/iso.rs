use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn run(env: String, cmdline: Option<String>) -> Result<()> {
    let root = project_root();
    
    // 1. Build artifacts if not already built (ensure invocation)
    // We assume build is called before iso usually, or we can call it here.
    // For now, let's assume the user runs `cargo run -p xtask build` then `iso`.
    // Or we can call crate::build::run()?; but let's decouple to follow typical pattern.
    // Actually, `iso` usually implies ensuring fresh build.
    crate::build::run()?;

    // 2. Assemble ISO Root
    println!("==> Assembling ISO root for {}...", env);
    let target_dir = root.join("target");
    let iso_root = target_dir.join("iso_root").join(&env); // e.g. target/iso_root/x86_64

    if iso_root.exists() {
        fs::remove_dir_all(&iso_root)?;
    }
    fs::create_dir_all(&iso_root)?;

    let boot_dir = iso_root.join("boot");
    fs::create_dir_all(&boot_dir)?;
    
    let modules_dir = boot_dir.join("modules");
    fs::create_dir_all(&modules_dir)?;

    // Copy Bran (Kernel) -> /boot/kernel
    // Source: crates/bran/target/x86_64-unknown-none/debug/bran (Wait, Makefile copied to bin-x86_64/kernel)
    // xtask build runs cargo build. The output is in target/x86_64-unknown-none/debug/...
    // Note: Cargo workspace uses shared `target` dir in root.
    // Path: target/x86_64-unknown-none/debug/bran
    let target_triple = "x86_64-unknown-none";
    let profile = "debug";
    
    let bin_src = root.join("target").join(target_triple).join(profile);

    let bran_src = bin_src.join("bran");
    fs::copy(&bran_src, boot_dir.join("kernel"))
        .with_context(|| format!("Failed to copy kernel from {:?}", bran_src))?;

    // Copy Sprout -> /boot/modules/sprout.elf
    let sprout_src = bin_src.join("sprout");
    fs::copy(&sprout_src, modules_dir.join("sprout.elf"))
         .with_context(|| format!("Failed to copy sprout from {:?}", sprout_src))?;

    // Copy Bloom -> /boot/modules/bloom.elf
    let bloom_src = bin_src.join("bloom");
    fs::copy(&bloom_src, modules_dir.join("bloom.elf"))
         .with_context(|| format!("Failed to copy bloom from {:?}", bloom_src))?;

    // Limine Config
    // Source: root/limine.conf (in current branch)
    let conf_src = root.join("limine.conf");
    let limine_dest = boot_dir.join("limine");
    fs::create_dir_all(&limine_dest)?;

    // We can copy it directly, but maybe we want to modify cmdline?
    let mut conf_data = fs::read_to_string(&conf_src)?;

    if let Some(cmd) = cmdline {
        let needle = "kernel_path: boot():/boot/kernel";
        let insertion = format!("\n    cmdline: {}", cmd);
        conf_data = conf_data.replacen(needle, &format!("{}{}", needle, insertion), 1);
    }
    
    // Update module paths if they differ from limine.conf defaults
    // Current limine.conf likely points to /boot/modules/sprout.elf etc.
    // If not, we might need to sed it. Assuming limine.conf is correct for standard layout.

    fs::write(limine_dest.join("limine.conf"), conf_data)?;

    // Vendor bins (Limine)
    // My fetch.rs puts them in root/vendor/limine
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
    
    // For x86_64
    let uefi_boot_name = "BOOTX64.EFI";
    let src_efi = vendor_limine.join(uefi_boot_name);
    if src_efi.exists() {
        fs::copy(&src_efi, efi_boot_dir.join(uefi_boot_name))?;
        // Also BOOTIA32.EFI?
        let src_ia32 = vendor_limine.join("BOOTIA32.EFI");
         if src_ia32.exists() {
             fs::copy(&src_ia32, efi_boot_dir.join("BOOTIA32.EFI"))?; // Optional
         }
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
    // cmd.arg("-full-iso9660-filenames"); // Optional, helps with long names
    cmd.arg("-o").arg(&iso_path);
    cmd.arg(&iso_root);

    let status = cmd.status().context("Failed to run xorriso")?;
    if !status.success() {
        anyhow::bail!("xorriso failed");
    }

    // BIOS Install (Limine 5+)
    // ./limine/limine bios-install image.iso
    // We should use the limine tool from vendor if compiled? Or system installed?
    // Trunk Fetch uses git clone binary branch which has PREBUILT binary? No, "v10.x-binary" branch usually has `limine` executable?
    // The `limine` executable is usually built or fetched.
    // The trunk fetch.rs clone command creates `limine` dir.
    // We might need to build `limine` tool or assume `limine` is in the fetched dir.
    // Wait, trunk `fetch_limine` cloned it. Trunk `GNUmakefile` ran `$(MAKE) -C limine`.
    // So we need to build the `limine` utility if it doesn't exist.
    
    let _limine_tool_src = vendor_limine.join("limine"); // Source dir
    // We need the executable `limine`.
    // In `limine` repo, `make` produces `limine` binary.
    
    let limine_tool_path = vendor_limine.join("limine"); // Binary (if compiled)
    if !limine_tool_path.exists() || !limine_tool_path.is_file() {
         println!("==> Building Limine tool...");
         let status = Command::new("make")
             .current_dir(&vendor_limine)
             .status()?;
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

    println!("ISO created at: {}", iso_path.display());

    Ok(())
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
