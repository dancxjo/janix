//! Image creation tasks - ISO and HDD.

use crate::common::{Result, image_name};
use xshell::{Shell, cmd};

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

    // Build and copy sprout
    println!("Building sprout for {}...", arch);

    // Use absolute path to target JSON file to work around RUST_TARGET_PATH propagation issues
    let cwd = std::env::current_dir().unwrap();
    let target_json = if arch == "x86_64" {
        cwd.join("targets/x86_64-unknown-thingos.json")
    } else {
        cwd.join(format!("targets/{}-unknown-thingos.json", arch))
    };
    let target = target_json.to_str().unwrap();

    build_userspace_app(sh, "sprout", target, "release")?;
    build_userspace_app(sh, "threads_demo", target, "release")?;
    build_userspace_app(sh, "rtc_cmos", target, "release")?;
    build_userspace_app(sh, "clock", target, "dev")?;  // clock uses dev profile
    build_userspace_app(sh, "ps2_kbd", target, "release")?;
    build_userspace_app(sh, "thigmonasty", target, "release")?;
    build_userspace_app(sh, "echo", target, "release")?;
    build_userspace_app(sh, "virtio_gpu", target, "release")?;

    // Copy binaries to iso_root
    copy_userspace_binary(sh, "sprout", target, "release", "iso_root/boot/sprout")?;
    copy_userspace_binary(sh, "threads_demo", target, "release", "iso_root/boot/threads")?;
    copy_userspace_binary(sh, "rtc_cmos", target, "release", "iso_root/boot/rtc_cmos")?;
    copy_userspace_binary(sh, "clock", target, "debug", "iso_root/boot/clock")?;
    copy_userspace_binary(sh, "ps2_kbd", target, "release", "iso_root/boot/ps2_kbd")?;
    copy_userspace_binary(sh, "thigmonasty", target, "release", "iso_root/boot/thigmonasty")?;
    copy_userspace_binary(sh, "echo", target, "release", "iso_root/boot/echo")?;
    copy_userspace_binary(sh, "virtio_gpu", target, "release", "iso_root/boot/virtio_gpu")?;

    // Copy limine config
    sh.copy_file("limine.conf", "iso_root/boot/limine/limine.conf")?;

    match arch {
        "x86_64" => {
            sh.copy_file(
                "limine/limine-bios.sys",
                "iso_root/boot/limine/limine-bios.sys",
            )?;
            sh.copy_file(
                "limine/limine-bios-cd.bin",
                "iso_root/boot/limine/limine-bios-cd.bin",
            )?;
            sh.copy_file(
                "limine/limine-uefi-cd.bin",
                "iso_root/boot/limine/limine-uefi-cd.bin",
            )?;
            sh.copy_file("limine/BOOTX64.EFI", "iso_root/EFI/BOOT/BOOTX64.EFI")?;
            sh.copy_file("limine/BOOTIA32.EFI", "iso_root/EFI/BOOT/BOOTIA32.EFI")?;

            let iso = format!("{}.iso", name);
            cmd!(sh, "xorriso -as mkisofs -R -J -b boot/limine/limine-bios-cd.bin -no-emul-boot -boot-load-size 4 -boot-info-table --efi-boot boot/limine/limine-uefi-cd.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o {iso}").run()?;
            cmd!(sh, "./limine/limine bios-install {iso}").run()?;
        }
        "aarch64" => {
            sh.copy_file(
                "limine/limine-uefi-cd.bin",
                "iso_root/boot/limine/limine-uefi-cd.bin",
            )?;
            sh.copy_file("limine/BOOTAA64.EFI", "iso_root/EFI/BOOT/BOOTAA64.EFI")?;

            let iso = format!("{}.iso", name);
            cmd!(sh, "xorriso -as mkisofs -R -J --efi-boot boot/limine/limine-uefi-cd.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o {iso}").run()?;
        }
        "riscv64" => {
            let efi_img = "iso_root/boot/limine/limine-uefi-riscv64.bin";
            cmd!(sh, "dd if=/dev/zero of={efi_img} bs=1K count=2880 status=none").run()?;
            cmd!(sh, "mformat -i {efi_img} -f 2880 ::").run()?;
            cmd!(sh, "mmd -i {efi_img} ::/EFI ::/EFI/BOOT").run()?;
            cmd!(sh, "mcopy -i {efi_img} limine/BOOTRISCV64.EFI ::/EFI/BOOT/BOOTRISCV64.EFI").run()?;
            sh.write_file("iso_root/startup.nsh", "\\EFI\\BOOT\\BOOTRISCV64.EFI\n")?;
            cmd!(sh, "mcopy -i {efi_img} iso_root/startup.nsh ::").run()?;
            sh.copy_file("limine/BOOTRISCV64.EFI", "iso_root/EFI/BOOT/BOOTRISCV64.EFI")?;

            let iso = format!("{}.iso", name);
            cmd!(sh, "xorriso -as mkisofs -R -J --efi-boot boot/limine/limine-uefi-riscv64.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o {iso}").run()?;
        }
        "loongarch64" => {
            let efi_img = "iso_root/boot/limine/limine-uefi-loongarch64.bin";
            cmd!(sh, "dd if=/dev/zero of={efi_img} bs=1K count=2880 status=none").run()?;
            cmd!(sh, "mformat -i {efi_img} -f 2880 ::").run()?;
            cmd!(sh, "mmd -i {efi_img} ::/EFI ::/EFI/BOOT").run()?;
            cmd!(sh, "mcopy -i {efi_img} limine/BOOTLOONGARCH64.EFI ::/EFI/BOOT/BOOTLOONGARCH64.EFI").run()?;
            sh.write_file("iso_root/startup.nsh", "\\EFI\\BOOT\\BOOTLOONGARCH64.EFI\n")?;
            cmd!(sh, "mcopy -i {efi_img} iso_root/startup.nsh ::").run()?;
            sh.copy_file("limine/BOOTLOONGARCH64.EFI", "iso_root/EFI/BOOT/BOOTLOONGARCH64.EFI")?;

            let iso = format!("{}.iso", name);
            cmd!(sh, "xorriso -as mkisofs -R -J --efi-boot boot/limine/limine-uefi-loongarch64.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o {iso}").run()?;
        }
        _ => return Err(format!("Unsupported architecture: {}", arch).into()),
    }

    sh.remove_path("iso_root")?;
    println!("ISO created: {}.iso", name);
    Ok(())
}

/// Build a userspace application
fn build_userspace_app(sh: &Shell, name: &str, target: &str, profile: &str) -> Result<()> {
    println!("Building {} ...", name);
    cmd!(
        sh,
        "cargo build --target {target} --profile {profile} -p {name} -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem"
    )
    .run()?;
    Ok(())
}

/// Copy and objcopy a userspace binary
fn copy_userspace_binary(sh: &Shell, name: &str, target: &str, profile_dir: &str, dst: &str) -> Result<()> {
    // Extract just the target name from the path for the output directory
    let target_name = std::path::Path::new(target)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(target);
    
    let elf = format!("target/{}/{}/{}", target_name, profile_dir, name);
    let bin = format!("target/{}/{}/{}.bin", target_name, profile_dir, name);
    
    cmd!(sh, "llvm-objcopy -O binary {elf} {bin}").run()?;
    sh.copy_file(&bin, dst)?;
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
    cmd!(
        sh,
        "mmd -i {hdd}@@1M ::/EFI ::/EFI/BOOT ::/boot ::/boot/limine"
    )
    .run()?;

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
