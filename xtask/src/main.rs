//! xtask - Build automation for Thing-OS
//!
//! This crate provides Rust-based build automation, replacing shell scripts
//! in the justfile with proper type-safe implementations.

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use xshell::{Shell, cmd};

/// Thing-OS build automation tool
#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "Build automation for Thing-OS", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the kernel for a target architecture
    Build {
        /// Target architecture (x86_64, aarch64, riscv64, loongarch64)
        #[arg(long, default_value = "x86_64")]
        env: String,
        /// Rust profile (dev, release)
        #[arg(long, default_value = "dev")]
        profile: String,
    },
    /// Create an ISO image
    Iso {
        /// Target architecture
        #[arg(long, default_value = "x86_64")]
        env: String,
        /// Rust profile
        #[arg(long, default_value = "dev")]
        profile: String,
    },
    /// Create an HDD image
    Hdd {
        /// Target architecture
        #[arg(long, default_value = "x86_64")]
        env: String,
        /// Rust profile
        #[arg(long, default_value = "dev")]
        profile: String,
    },
    /// Run in QEMU (UEFI mode)
    Run {
        /// Target architecture
        #[arg(long, default_value = "x86_64")]
        env: String,
        /// Rust profile
        #[arg(long, default_value = "dev")]
        profile: String,
        /// Additional QEMU flags
        #[arg(long, default_value = "-m 2G", allow_hyphen_values = true)]
        qemu_flags: String,
    },
    /// Run in QEMU (BIOS mode, x86_64 only)
    RunBios {
        /// Additional QEMU flags
        #[arg(long, default_value = "-m 2G", allow_hyphen_values = true)]
        qemu_flags: String,
    },
    /// Run HDD image in QEMU (UEFI mode)
    RunHdd {
        /// Target architecture
        #[arg(long, default_value = "x86_64")]
        env: String,
        /// Rust profile
        #[arg(long, default_value = "dev")]
        profile: String,
        /// Additional QEMU flags
        #[arg(long, default_value = "-m 2G", allow_hyphen_values = true)]
        qemu_flags: String,
    },
    /// Clone and build Limine bootloader
    Limine,
    /// Download OVMF firmware
    Ovmf {
        /// Target architecture
        #[arg(long, default_value = "x86_64")]
        env: String,
    },
    /// Clean build artifacts
    Clean,
    /// Clean everything including downloaded dependencies
    Distclean,
    /// Run BDD tests
    Bdd {
        /// Run a specific feature file (without .feature extension)
        #[arg(long)]
        feature: Option<String>,
        /// Cucumber tag expression (e.g., @smoke)
        #[arg(long, short = 't')]
        tags: Option<String>,
        /// Target architecture (can be specified multiple times)
        #[arg(long, short = 'a')]
        arch: Vec<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let sh = Shell::new()?;

    // Change to project root (parent of xtask directory)
    let project_root = project_root();
    sh.change_dir(&project_root);

    match cli.command {
        Commands::Build { env, profile } => build(&sh, &env, &profile)?,
        Commands::Iso { env, profile } => {
            limine(&sh)?;
            build(&sh, &env, &profile)?;
            build_iso(&sh, &env)?;
        }
        Commands::Hdd { env, profile } => {
            limine(&sh)?;
            build(&sh, &env, &profile)?;
            build_hdd(&sh, &env)?;
        }
        Commands::Run { env, profile, qemu_flags } => {
            ovmf(&sh, &env)?;
            limine(&sh)?;
            build(&sh, &env, &profile)?;
            build_iso(&sh, &env)?;
            run(&sh, &env, &qemu_flags)?;
        }
        Commands::RunBios { qemu_flags } => {
            limine(&sh)?;
            build(&sh, "x86_64", "dev")?;
            build_iso(&sh, "x86_64")?;
            run_bios(&sh, &qemu_flags)?;
        }
        Commands::RunHdd { env, profile, qemu_flags } => {
            ovmf(&sh, &env)?;
            limine(&sh)?;
            build(&sh, &env, &profile)?;
            build_hdd(&sh, &env)?;
            run_hdd(&sh, &env, &qemu_flags)?;
        }
        Commands::Limine => limine(&sh)?,
        Commands::Ovmf { env } => ovmf(&sh, &env)?,
        Commands::Clean => clean(&sh)?,
        Commands::Distclean => distclean(&sh)?,
        Commands::Bdd { feature, tags, arch } => bdd(&sh, feature, tags, arch)?,
    }

    Ok(())
}

fn project_root() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).parent().unwrap().to_path_buf()
}

fn rust_target(arch: &str) -> &'static str {
    match arch {
        "riscv64" => "riscv64gc-unknown-none-elf",
        "x86_64" => "x86_64-unknown-none",
        "aarch64" => "aarch64-unknown-none",
        "loongarch64" => "loongarch64-unknown-none",
        _ => panic!("Unsupported architecture: {}", arch),
    }
}

fn profile_subdir(profile: &str) -> &str {
    if profile == "dev" { "debug" } else { profile }
}

fn image_name(arch: &str) -> String {
    format!("thing-os-{}", arch)
}

fn build(sh: &Shell, arch: &str, profile: &str) -> Result<(), Box<dyn std::error::Error>> {
    let target = rust_target(arch);
    let subdir = profile_subdir(profile);

    println!("Building bran kernel for {} ({} profile)...", arch, profile);

    cmd!(sh, "cargo build --target {target} --profile {profile} -p bran")
        .env("RUSTFLAGS", "-C relocation-model=static")
        .run()?;

    // Copy kernel binary to bran/bin-{arch}/
    let bin_dir = format!("bran/bin-{}", arch);
    sh.create_dir(&bin_dir)?;

    let src = format!("target/{}/{}/bran", target, subdir);
    let dst = format!("{}/kernel", bin_dir);
    sh.copy_file(&src, &dst)?;

    println!("Kernel built: {}", dst);
    Ok(())
}

fn limine(sh: &Shell) -> Result<(), Box<dyn std::error::Error>> {
    if sh.path_exists("limine") {
        println!("Limine already present, skipping clone.");
        return Ok(());
    }

    println!("Cloning Limine v10.x...");
    cmd!(sh, "git clone https://github.com/limine-bootloader/limine.git --branch=v10.x-binary --depth=1").run()?;
    cmd!(sh, "make -C limine").run()?;

    Ok(())
}

fn ovmf(sh: &Shell, _arch: &str) -> Result<(), Box<dyn std::error::Error>> {
    sh.create_dir("ovmf")?;

    // Use pinned OVMF release from rust-osdev/ovmf-prebuilt (same as trunk branch)
    let release = std::env::var("THINGOS_OVMF_RELEASE")
        .unwrap_or_else(|_| "edk2-stable202508-r1".to_string());
    let archive_name = format!("{}-bin.tar.xz", release);
    let archive_url = format!(
        "https://github.com/rust-osdev/ovmf-prebuilt/releases/download/{}/{}",
        release, archive_name
    );
    let archive_path = format!("ovmf/{}", archive_name);

    // Check if we already have the required files
    if sh.path_exists("ovmf/ovmf-code-x86_64.fd") && sh.path_exists("ovmf/ovmf-vars-x86_64.fd") {
        println!("OVMF already present, skipping download.");
        return Ok(());
    }

    println!("Using OVMF release: {}", release);

    if !sh.path_exists(&archive_path) {
        println!("Downloading {}...", archive_name);
        cmd!(sh, "curl -fLo {archive_path} {archive_url}").run()?;
    } else {
        println!("Reusing cached {}", archive_name);
    }

    let extract_dir = format!("ovmf/extract-{}", release);
    sh.remove_path(&extract_dir)?;
    sh.create_dir(&extract_dir)?;

    println!("Extracting OVMF archive...");
    cmd!(sh, "tar -xJf {archive_path} -C {extract_dir}").run()?;

    // Map extracted files to expected locations
    let base = format!("{}/{}-bin", extract_dir, release);
    let mappings = [
        ("x64/code.fd", "ovmf/ovmf-code-x86_64.fd"),
        ("x64/vars.fd", "ovmf/ovmf-vars-x86_64.fd"),
        ("aarch64/code.fd", "ovmf/ovmf-code-aarch64.fd"),
        ("aarch64/vars.fd", "ovmf/ovmf-vars-aarch64.fd"),
        ("riscv64/code.fd", "ovmf/ovmf-code-riscv64.fd"),
        ("riscv64/vars.fd", "ovmf/ovmf-vars-riscv64.fd"),
    ];

    for (src_rel, dest) in mappings {
        let src = format!("{}/{}", base, src_rel);
        if sh.path_exists(&src) {
            println!("Installing {} -> {}", src_rel, dest);
            sh.copy_file(&src, dest)?;
        }
    }

    sh.remove_path(&extract_dir)?;
    println!("OVMF installed successfully.");

    Ok(())
}

fn build_iso(sh: &Shell, arch: &str) -> Result<(), Box<dyn std::error::Error>> {
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
            sh.copy_file("limine/limine-uefi-cd.bin", "iso_root/boot/limine/limine-uefi-cd.bin")?;
            sh.copy_file("limine/BOOTRISCV64.EFI", "iso_root/EFI/BOOT/BOOTRISCV64.EFI")?;

            let iso = format!("{}.iso", name);
            cmd!(sh, "xorriso -as mkisofs --efi-boot boot/limine/limine-uefi-cd.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o {iso}").run()?;
        }
        "loongarch64" => {
            sh.copy_file("limine/limine-uefi-cd.bin", "iso_root/boot/limine/limine-uefi-cd.bin")?;
            sh.copy_file("limine/BOOTLOONGARCH64.EFI", "iso_root/EFI/BOOT/BOOTLOONGARCH64.EFI")?;

            let iso = format!("{}.iso", name);
            cmd!(sh, "xorriso -as mkisofs --efi-boot boot/limine/limine-uefi-cd.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o {iso}").run()?;
        }
        _ => return Err(format!("Unsupported architecture: {}", arch).into()),
    }

    sh.remove_path("iso_root")?;
    println!("ISO created: {}.iso", name);
    Ok(())
}

fn build_hdd(sh: &Shell, arch: &str) -> Result<(), Box<dyn std::error::Error>> {
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

fn run(sh: &Shell, arch: &str, qemu_flags: &str) -> Result<(), Box<dyn std::error::Error>> {
    let name = image_name(arch);
    let iso = format!("{}.iso", name);
    let ovmf_code = format!("ovmf/ovmf-code-{}.fd", arch);
    let ovmf_vars = format!("ovmf/ovmf-vars-{}.fd", arch);

    println!("Running {} in QEMU...", name);

    // Split QEMU flags on whitespace to pass as separate args
    let qemu_args: Vec<&str> = qemu_flags.split_whitespace().collect();

    match arch {
        "x86_64" => {
            cmd!(sh, "qemu-system-x86_64 -M q35 -drive if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on -drive if=pflash,unit=1,format=raw,file={ovmf_vars} -cdrom {iso}")
                .args(&qemu_args)
                .run()?;
        }
        "aarch64" => {
            cmd!(sh, "qemu-system-aarch64 -M virt -cpu cortex-a72 -device ramfb -device qemu-xhci -device usb-kbd -device usb-mouse -drive if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on -drive if=pflash,unit=1,format=raw,file={ovmf_vars} -cdrom {iso}")
                .args(&qemu_args)
                .run()?;
        }
        "riscv64" => {
            cmd!(sh, "qemu-system-riscv64 -M virt -cpu rv64 -device ramfb -device qemu-xhci -device usb-kbd -device usb-mouse -drive if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on -drive if=pflash,unit=1,format=raw,file={ovmf_vars} -cdrom {iso}")
                .args(&qemu_args)
                .run()?;
        }
        "loongarch64" => {
            cmd!(sh, "qemu-system-loongarch64 -M virt -cpu la464 -device ramfb -device qemu-xhci -device usb-kbd -device usb-mouse -drive if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on -drive if=pflash,unit=1,format=raw,file={ovmf_vars} -cdrom {iso}")
                .args(&qemu_args)
                .run()?;
        }
        _ => return Err(format!("Unsupported architecture: {}", arch).into()),
    }

    Ok(())
}

fn run_bios(sh: &Shell, qemu_flags: &str) -> Result<(), Box<dyn std::error::Error>> {
    let iso = "thing-os-x86_64.iso";
    let qemu_args: Vec<&str> = qemu_flags.split_whitespace().collect();
    println!("Running in QEMU BIOS mode...");
    cmd!(sh, "qemu-system-x86_64 -M q35 -cdrom {iso} -boot d")
        .args(&qemu_args)
        .run()?;
    Ok(())
}


fn run_hdd(sh: &Shell, arch: &str, qemu_flags: &str) -> Result<(), Box<dyn std::error::Error>> {
    let name = image_name(arch);
    let hdd = format!("{}.hdd", name);
    let ovmf_code = format!("ovmf/ovmf-code-{}.fd", arch);
    let ovmf_vars = format!("ovmf/ovmf-vars-{}.fd", arch);

    println!("Running {} HDD in QEMU...", name);

    let qemu_args: Vec<&str> = qemu_flags.split_whitespace().collect();

    match arch {
        "x86_64" => {
            cmd!(sh, "qemu-system-x86_64 -M q35 -drive if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on -drive if=pflash,unit=1,format=raw,file={ovmf_vars} -hda {hdd}")
                .args(&qemu_args)
                .run()?;
        }
        "aarch64" => {
            cmd!(sh, "qemu-system-aarch64 -M virt -cpu cortex-a72 -device ramfb -device qemu-xhci -device usb-kbd -device usb-mouse -drive if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on -drive if=pflash,unit=1,format=raw,file={ovmf_vars} -hda {hdd}")
                .args(&qemu_args)
                .run()?;
        }
        "riscv64" => {
            cmd!(sh, "qemu-system-riscv64 -M virt -cpu rv64 -device ramfb -device qemu-xhci -device usb-kbd -device usb-mouse -drive if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on -drive if=pflash,unit=1,format=raw,file={ovmf_vars} -hda {hdd}")
                .args(&qemu_args)
                .run()?;
        }
        "loongarch64" => {
            cmd!(sh, "qemu-system-loongarch64 -M virt -cpu la464 -device ramfb -device qemu-xhci -device usb-kbd -device usb-mouse -drive if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on -drive if=pflash,unit=1,format=raw,file={ovmf_vars} -hda {hdd}")
                .args(&qemu_args)
                .run()?;
        }
        _ => return Err(format!("Unsupported architecture: {}", arch).into()),
    }

    Ok(())
}

fn clean(sh: &Shell) -> Result<(), Box<dyn std::error::Error>> {
    println!("Cleaning build artifacts...");
    cmd!(sh, "cargo clean").run()?;
    sh.remove_path("iso_root")?;
    sh.remove_path("thing-os-x86_64.iso")?;
    sh.remove_path("thing-os-x86_64.hdd")?;
    sh.remove_path("thing-os-aarch64.iso")?;
    sh.remove_path("thing-os-aarch64.hdd")?;
    sh.remove_path("thing-os-riscv64.iso")?;
    sh.remove_path("thing-os-riscv64.hdd")?;
    sh.remove_path("thing-os-loongarch64.iso")?;
    sh.remove_path("thing-os-loongarch64.hdd")?;
    sh.remove_path("bran/bin-x86_64")?;
    sh.remove_path("bran/bin-aarch64")?;
    sh.remove_path("bran/bin-riscv64")?;
    sh.remove_path("bran/bin-loongarch64")?;
    Ok(())
}

fn distclean(sh: &Shell) -> Result<(), Box<dyn std::error::Error>> {
    clean(sh)?;
    println!("Removing downloaded dependencies...");
    sh.remove_path("limine")?;
    sh.remove_path("ovmf")?;
    Ok(())
}

fn bdd(
    sh: &Shell,
    feature: Option<String>,
    _tags: Option<String>,
    arch: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Default to x86_64 if no architectures specified
    let architectures = if arch.is_empty() {
        vec!["x86_64".to_string()]
    } else {
        arch
    };

    // Run tests for each architecture
    for a in &architectures {
        println!("\n=== Building and testing for {} ===\n", a);

        // Build ISO
        ovmf(sh, a)?;
        limine(sh)?;
        build(sh, a, "dev")?;
        build_iso(sh, a)?;

        // Run bdd with environment variables
        let mut run_cmd = cmd!(sh, "cargo run -p bdd");
        run_cmd = run_cmd.env("BDD_ARCH", a);

        if let Some(ref f) = feature {
            run_cmd = run_cmd.env("BDD_FEATURE", f);
        }

        run_cmd.run()?;
    }

    Ok(())
}


