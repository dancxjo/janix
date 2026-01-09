//! Image creation tasks - ISO and HDD.

use xshell::{Shell, cmd};
use crate::common::{image_name, Result};

/// Build an ISO image for the target architecture.
pub fn build_iso(sh: &Shell, arch: &str) -> Result<()> {
    let name = image_name(arch);

    println!("Building ISO for {}...", arch);

    // Clean and create iso_root
    sh.remove_path("iso_root")?;
    sh.create_dir("iso_root/boot")?;
    sh.create_dir("iso_root/boot/limine")?;
    sh.create_dir("iso_root/EFI/BOOT")?;

    // Copy kernel
    let kernel_src = format!("bran/bin-{}/kernel", arch);
    sh.copy_file(&kernel_src, "iso_root/boot/kernel")?;

    // Copy modules
    sh.create_dir("iso_root/boot/modules")?;
    let modules_dir = format!("bran/bin-{}/modules", arch);
    if sh.path_exists(&modules_dir) {
        for entry in sh.read_dir(&modules_dir)? {
             let path = std::path::PathBuf::from(entry); // it's already a PathBuf from read_dir? No, read_dir returns Vec<PathBuf> in xshell?
             // xshell::read_dir returns Vec<PathBuf>
             if let Some(name) = path.file_name() {
                  let dst = format!("iso_root/boot/modules/{}", name.to_string_lossy());
                  sh.copy_file(&path, &dst)?;
             }
        }
    }

    // Copy limine config
    sh.copy_file("limine.conf", "iso_root/boot/limine/limine.conf")?;

    match arch {
        "x86_64" => {
            sh.copy_file("limine/limine-bios.sys", "iso_root/boot/limine/limine-bios.sys")?;
            sh.copy_file("limine/limine-bios-cd.bin", "iso_root/boot/limine/limine-bios-cd.bin")?;
            sh.copy_file("limine/limine-uefi-cd.bin", "iso_root/boot/limine/limine-uefi-cd.bin")?;
            sh.copy_file("limine/BOOTX64.EFI", "iso_root/EFI/BOOT/BOOTX64.EFI")?;
            sh.copy_file("limine/BOOTIA32.EFI", "iso_root/EFI/BOOT/BOOTIA32.EFI")?;

            let iso = format!("{}.iso", name);
            cmd!(sh, "xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin -no-emul-boot -boot-load-size 4 -boot-info-table --efi-boot boot/limine/limine-uefi-cd.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o {iso}").run()?;
            cmd!(sh, "./limine/limine bios-install {iso}").run()?;
        }
        "aarch64" => {
            sh.copy_file("limine/limine-uefi-cd.bin", "iso_root/boot/limine/limine-uefi-cd.bin")?;
            sh.copy_file("limine/BOOTAA64.EFI", "iso_root/EFI/BOOT/BOOTAA64.EFI")?;

            let iso = format!("{}.iso", name);
            cmd!(sh, "xorriso -as mkisofs --efi-boot boot/limine/limine-uefi-cd.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o {iso}").run()?;
        }
        "riscv64" => {
            // For riscv64, we need to create a custom EFI boot image since limine-uefi-cd.bin
            // only contains x86 bootloaders. We build an arch-specific FAT image.
            let efi_img = "iso_root/boot/limine/limine-uefi-riscv64.bin";
            
            // Create a 3MB FAT12 image (enough for the bootloader plus overhead)
            cmd!(sh, "dd if=/dev/zero of={efi_img} bs=1K count=2880 status=none").run()?;
            cmd!(sh, "mformat -i {efi_img} -f 2880 ::").run()?;
            cmd!(sh, "mmd -i {efi_img} ::/EFI ::/EFI/BOOT").run()?;
            cmd!(sh, "mcopy -i {efi_img} limine/BOOTRISCV64.EFI ::/EFI/BOOT/BOOTRISCV64.EFI").run()?;
            
            // Also add startup.nsh as a fallback inside the EFI image
            sh.write_file("iso_root/startup.nsh", "\\EFI\\BOOT\\BOOTRISCV64.EFI\n")?;
            cmd!(sh, "mcopy -i {efi_img} iso_root/startup.nsh ::").run()?;

            // Copy kernel and limine config to ISO root
            sh.copy_file("limine/BOOTRISCV64.EFI", "iso_root/EFI/BOOT/BOOTRISCV64.EFI")?;

            let iso = format!("{}.iso", name);
            cmd!(sh, "xorriso -as mkisofs --efi-boot boot/limine/limine-uefi-riscv64.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o {iso}").run()?;
        }
        "loongarch64" => {
            // For loongarch64, we need to create a custom EFI boot image since limine-uefi-cd.bin
            // only contains x86 bootloaders. We build an arch-specific FAT image.
            let efi_img = "iso_root/boot/limine/limine-uefi-loongarch64.bin";
            
            // Create a 3MB FAT12 image (enough for the bootloader plus overhead)
            cmd!(sh, "dd if=/dev/zero of={efi_img} bs=1K count=2880 status=none").run()?;
            cmd!(sh, "mformat -i {efi_img} -f 2880 ::").run()?;
            cmd!(sh, "mmd -i {efi_img} ::/EFI ::/EFI/BOOT").run()?;
            cmd!(sh, "mcopy -i {efi_img} limine/BOOTLOONGARCH64.EFI ::/EFI/BOOT/BOOTLOONGARCH64.EFI").run()?;
            
            // Also add startup.nsh as a fallback inside the EFI image
            sh.write_file("iso_root/startup.nsh", "\\EFI\\BOOT\\BOOTLOONGARCH64.EFI\n")?;
            cmd!(sh, "mcopy -i {efi_img} iso_root/startup.nsh ::").run()?;

            // Copy kernel and limine config to ISO root
            sh.copy_file("limine/BOOTLOONGARCH64.EFI", "iso_root/EFI/BOOT/BOOTLOONGARCH64.EFI")?;

            let iso = format!("{}.iso", name);
            cmd!(sh, "xorriso -as mkisofs --efi-boot boot/limine/limine-uefi-loongarch64.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o {iso}").run()?;
        }
        _ => return Err(format!("Unsupported architecture: {}", arch).into()),
    }

    sh.remove_path("iso_root")?;
    println!("ISO created: {}.iso", name);
    Ok(())
}

/// Build an HDD image for the target architecture.
pub fn build_hdd(sh: &Shell, arch: &str) -> Result<()> {
    let name = image_name(arch);
    let hdd = format!("{}.hdd", name);

    println!("Building HDD image for {}...", arch);

    sh.remove_path(&hdd)?;
    cmd!(sh, "dd if=/dev/zero bs=1M count=0 seek=64 of={hdd}").run()?;
    cmd!(sh, "sgdisk {hdd} -n 1:2048 -t 1:ef00").run()?;

    if arch == "x86_64" {
        cmd!(sh, "./limine/limine bios-install {hdd}").run()?;
    }

    cmd!(sh, "mformat -i {hdd}@@1M").run()?;
    cmd!(sh, "mmd -i {hdd}@@1M ::/EFI ::/EFI/BOOT ::/boot ::/boot/limine").run()?;

    let kernel_src = format!("bran/bin-{}/kernel", arch);
    cmd!(sh, "mcopy -i {hdd}@@1M {kernel_src} ::/boot").run()?;
    cmd!(sh, "mcopy -i {hdd}@@1M limine.conf ::/boot/limine").run()?;

    match arch {
        "x86_64" => {
            cmd!(sh, "mcopy -i {hdd}@@1M limine/limine-bios.sys ::/boot/limine").run()?;
            cmd!(sh, "mcopy -i {hdd}@@1M limine/BOOTX64.EFI ::/EFI/BOOT").run()?;
            cmd!(sh, "mcopy -i {hdd}@@1M limine/BOOTIA32.EFI ::/EFI/BOOT").run()?;
        }
        "aarch64" => {
            cmd!(sh, "mcopy -i {hdd}@@1M limine/BOOTAA64.EFI ::/EFI/BOOT").run()?;
        }
        "riscv64" => {
            cmd!(sh, "mcopy -i {hdd}@@1M limine/BOOTRISCV64.EFI ::/EFI/BOOT").run()?;
        }
        "loongarch64" => {
            cmd!(sh, "mcopy -i {hdd}@@1M limine/BOOTLOONGARCH64.EFI ::/EFI/BOOT").run()?;
        }
        _ => return Err(format!("Unsupported architecture: {}", arch).into()),
    }

    println!("HDD image created: {}", hdd);
    Ok(())
}
