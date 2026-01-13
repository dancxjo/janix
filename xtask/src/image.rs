//! Image creation tasks - ISO and HDD.

use crate::common::{Result, image_name};
use xshell::{Shell, cmd};

const DIAGNOSTIC_APPS: &[(&str, &str)] = &[
    ("threads_demo", "threads"),
    ("stack_heap_torture", "stack_heap_torture"),
    ("echo_mouse", "echo_mouse"),
];

fn diagnostic_apps_enabled() -> bool {
    cfg!(feature = "diagnostic-apps")
}

fn write_limine_config(sh: &Shell, dst: &str) -> Result<()> {
    let mut contents = sh.read_file("limine.conf")?;
    if diagnostic_apps_enabled() {
        if !contents.ends_with('\n') {
            contents.push('\n');
        }
        for &(_, module_name) in DIAGNOSTIC_APPS {
            contents.push_str(&format!(
                "    module_path: boot():/boot/{}\n",
                module_name
            ));
        }
    }
    sh.write_file(dst, contents)?;
    Ok(())
}

/// Build an ISO image for the target architecture.
pub fn build_iso(sh: &Shell, arch: &str) -> Result<()> {
    let name = image_name(arch);

    println!("Building ISO for {}...", arch);

    // Clean and create iso_root
    sh.remove_path("iso_root")?;
    sh.create_dir("iso_root/boot")?;
    sh.create_dir("iso_root/boot/limine")?;
    sh.create_dir("iso_root/EFI/BOOT")?;

    // Copy assets
    cmd!(sh, "cp -r assets iso_root/").run()?;

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

    build_userspace_app_with_features(
        sh,
        "sprout",
        target,
        "release",
        diagnostic_apps_enabled(),
    )?;
    build_userspace_app_with_features(
        sh,
        "bristle",
        target,
        "release",
        diagnostic_apps_enabled(),
    )?;
    build_userspace_app(sh, "rtc_cmos", target, "release")?;
    build_userspace_app(sh, "clock", target, "release")?;
    build_userspace_app(sh, "ps2_kbd", target, "release")?;
    build_userspace_app(sh, "echo", target, "release")?;
    build_userspace_app(sh, "bloom", target, "release")?;
    build_userspace_app(sh, "ps2_mouse", target, "release")?;
    build_userspace_app(sh, "virtio_gpu", target, "release")?;
    build_userspace_app(sh, "display_bootfb", target, "release")?;
    build_userspace_app(sh, "display_virtio_gpu", target, "release")?;
    if diagnostic_apps_enabled() {
        for &(app, _) in DIAGNOSTIC_APPS {
            build_userspace_app(sh, app, target, "release")?;
        }
    }

    // Copy binaries to iso_root
    copy_userspace_binary(sh, "sprout", target, "release", "iso_root/boot/sprout")?;
    copy_userspace_binary(sh, "rtc_cmos", target, "release", "iso_root/boot/rtc_cmos")?;
    copy_userspace_binary(sh, "clock", target, "release", "iso_root/boot/clock")?;
    copy_userspace_binary(sh, "ps2_kbd", target, "release", "iso_root/boot/ps2_kbd")?;
    copy_userspace_binary(sh, "bristle", target, "release", "iso_root/boot/bristle")?;
    copy_userspace_binary(sh, "echo", target, "release", "iso_root/boot/echo")?;
    copy_userspace_binary(sh, "bloom", target, "release", "iso_root/boot/bloom")?;
    copy_userspace_binary(
        sh,
        "ps2_mouse",
        target,
        "release",
        "iso_root/boot/ps2_mouse",
    )?;
    copy_userspace_binary(
        sh,
        "virtio_gpu",
        target,
        "release",
        "iso_root/boot/virtio_gpu",
    )?;
    copy_userspace_binary(
        sh,
        "display_bootfb",
        target,
        "release",
        "iso_root/boot/display_bootfb",
    )?;
    copy_userspace_binary(
        sh,
        "display_virtio_gpu",
        target,
        "release",
        "iso_root/boot/display_virtio_gpu",
    )?;
    if diagnostic_apps_enabled() {
        for &(app, module_name) in DIAGNOSTIC_APPS {
            let dst = format!("iso_root/boot/{}", module_name);
            copy_userspace_binary(sh, app, target, "release", &dst)?;
        }
    }

    // Copy limine config
    write_limine_config(sh, "iso_root/boot/limine/limine.conf")?;

    match arch {
        "x86_64" => {
            sh.copy_file(
                "vendor/limine/limine-bios.sys",
                "iso_root/boot/limine/limine-bios.sys",
            )?;
            sh.copy_file(
                "vendor/limine/limine-bios-cd.bin",
                "iso_root/boot/limine/limine-bios-cd.bin",
            )?;
            sh.copy_file(
                "vendor/limine/limine-uefi-cd.bin",
                "iso_root/boot/limine/limine-uefi-cd.bin",
            )?;
            sh.copy_file("vendor/limine/BOOTX64.EFI", "iso_root/EFI/BOOT/BOOTX64.EFI")?;
            sh.copy_file(
                "vendor/limine/BOOTIA32.EFI",
                "iso_root/EFI/BOOT/BOOTIA32.EFI",
            )?;

            let iso = format!("{}.iso", name);
            cmd!(sh, "xorriso -as mkisofs -R -J -b boot/limine/limine-bios-cd.bin -no-emul-boot -boot-load-size 4 -boot-info-table --efi-boot boot/limine/limine-uefi-cd.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o {iso}").run()?;
            cmd!(sh, "./vendor/limine/limine bios-install {iso}").run()?;
        }
        "aarch64" => {
            sh.copy_file(
                "vendor/limine/limine-uefi-cd.bin",
                "iso_root/boot/limine/limine-uefi-cd.bin",
            )?;
            sh.copy_file(
                "vendor/limine/BOOTAA64.EFI",
                "iso_root/EFI/BOOT/BOOTAA64.EFI",
            )?;

            let iso = format!("{}.iso", name);
            cmd!(sh, "xorriso -as mkisofs -R -J --efi-boot boot/limine/limine-uefi-cd.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o {iso}").run()?;
        }
        "riscv64" => {
            let efi_img = "iso_root/boot/limine/limine-uefi-riscv64.bin";
            cmd!(
                sh,
                "dd if=/dev/zero of={efi_img} bs=1K count=2880 status=none"
            )
            .run()?;
            cmd!(sh, "mformat -i {efi_img} -f 2880 ::").run()?;
            cmd!(sh, "mmd -i {efi_img} ::/EFI ::/EFI/BOOT").run()?;
            cmd!(
                sh,
                "mcopy -i {efi_img} limine/BOOTRISCV64.EFI ::/EFI/BOOT/BOOTRISCV64.EFI"
            )
            .run()?;
            sh.write_file("iso_root/startup.nsh", "\\EFI\\BOOT\\BOOTRISCV64.EFI\n")?;
            cmd!(sh, "mcopy -i {efi_img} iso_root/startup.nsh ::").run()?;
            sh.copy_file(
                "vendor/limine/BOOTRISCV64.EFI",
                "iso_root/EFI/BOOT/BOOTRISCV64.EFI",
            )?;

            let iso = format!("{}.iso", name);
            cmd!(sh, "xorriso -as mkisofs -R -J --efi-boot boot/limine/limine-uefi-riscv64.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o {iso}").run()?;
        }
        "loongarch64" => {
            let efi_img = "iso_root/boot/limine/limine-uefi-loongarch64.bin";
            cmd!(
                sh,
                "dd if=/dev/zero of={efi_img} bs=1K count=2880 status=none"
            )
            .run()?;
            cmd!(sh, "mformat -i {efi_img} -f 2880 ::").run()?;
            cmd!(sh, "mmd -i {efi_img} ::/EFI ::/EFI/BOOT").run()?;
            cmd!(
                sh,
                "mcopy -i {efi_img} limine/BOOTLOONGARCH64.EFI ::/EFI/BOOT/BOOTLOONGARCH64.EFI"
            )
            .run()?;
            sh.write_file("iso_root/startup.nsh", "\\EFI\\BOOT\\BOOTLOONGARCH64.EFI\n")?;
            cmd!(sh, "mcopy -i {efi_img} iso_root/startup.nsh ::").run()?;
            sh.copy_file(
                "vendor/limine/BOOTLOONGARCH64.EFI",
                "iso_root/EFI/BOOT/BOOTLOONGARCH64.EFI",
            )?;

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

fn build_userspace_app_with_features(
    sh: &Shell,
    name: &str,
    target: &str,
    profile: &str,
    enable_diagnostics: bool,
) -> Result<()> {
    println!("Building {} ...", name);
    let mut cmd = cmd!(
        sh,
        "cargo build --target {target} --profile {profile} -p {name} -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem"
    );
    if enable_diagnostics {
        cmd = cmd.arg("--features").arg("diagnostic-apps");
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
        cmd!(sh, "./vendor/limine/limine bios-install {hdd}").run()?;
    }

    cmd!(sh, "mformat -i {hdd}@@1M").run()?;
    cmd!(
        sh,
        "mmd -i {hdd}@@1M ::/EFI ::/EFI/BOOT ::/boot ::/boot/limine"
    )
    .run()?;

    // Copy assets
    cmd!(sh, "mcopy -i {hdd}@@1M -s assets ::").run()?;

    let kernel_src = format!("bran/bin-{}/kernel", arch);
    cmd!(sh, "mcopy -i {hdd}@@1M {kernel_src} ::/boot").run()?;
    let limine_cfg = "limine.generated.conf";
    write_limine_config(sh, limine_cfg)?;
    cmd!(sh, "mcopy -i {hdd}@@1M {limine_cfg} ::/boot/limine/limine.conf").run()?;
    sh.remove_path(limine_cfg)?;

    match arch {
        "x86_64" => {
            cmd!(
                sh,
                "mcopy -i {hdd}@@1M limine/limine-bios.sys ::/boot/limine"
            )
            .run()?;
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
            cmd!(
                sh,
                "mcopy -i {hdd}@@1M limine/BOOTLOONGARCH64.EFI ::/EFI/BOOT"
            )
            .run()?;
        }
        _ => return Err(format!("Unsupported architecture: {}", arch).into()),
    }

    println!("HDD image created: {}", hdd);
    Ok(())
}
