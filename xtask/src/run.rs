//! QEMU run tasks.

use crate::common::{Result, image_name};
use xshell::{Shell, cmd};

use std::path::Path;

/// Run ISO image in QEMU (UEFI mode).
pub fn run(sh: &Shell, arch: &str, qemu_flags: &str, iso_path: &Path) -> Result<()> {
    let name = image_name(arch);
    let iso = iso_path.to_str().unwrap();
    let ovmf_code = format!("vendor/ovmf/ovmf-code-{}.fd", arch);
    let ovmf_vars = format!("vendor/ovmf/ovmf-vars-{}.fd", arch);

    println!("Running {} in QEMU...", name);

    // Split QEMU flags on whitespace to pass as separate args
    let qemu_args: Vec<&str> = qemu_flags.split_whitespace().collect();

    match arch {
        "x86_64" => {
            cmd!(sh, "qemu-system-x86_64 -M q35 -device virtio-vga  -serial stdio -drive if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on -drive if=pflash,unit=1,format=raw,file={ovmf_vars} -cdrom {iso} -no-reboot -d int,cpu_reset -D qemu.log")
                .args(&qemu_args)
                .run()?;
        }
        "aarch64" => {
            cmd!(sh, "qemu-system-aarch64 -M virt -cpu cortex-a72 -serial stdio -semihosting -device ramfb -device qemu-xhci -device usb-kbd -device usb-mouse -drive if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on -drive if=pflash,unit=1,format=raw,file={ovmf_vars} -cdrom {iso}")
                .args(&qemu_args)
                .run()?;
        }
        "riscv64" => {
            // riscv64 virt requires blockdev syntax with machine-level pflash assignment
            // Also uses virtio-blk instead of -cdrom since riscv64 virt doesn't expose cdrom to UEFI properly
            cmd!(sh, "qemu-system-riscv64 -blockdev node-name=pflash0,driver=file,read-only=on,filename={ovmf_code} -blockdev node-name=pflash1,driver=file,filename={ovmf_vars} -M virt,pflash0=pflash0,pflash1=pflash1 -cpu rv64 -m 2G -serial stdio -device ramfb -device qemu-xhci -device usb-kbd -device usb-mouse -drive file={iso},format=raw,if=none,id=drive0,readonly=on -device virtio-blk-device,drive=drive0")
                .args(&qemu_args)
                .run()?;
        }
        "loongarch64" => {
            cmd!(sh, "qemu-system-loongarch64 -M virt -cpu la464 -serial stdio -device ramfb -device qemu-xhci -device usb-kbd -device usb-mouse -drive if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on -drive if=pflash,unit=1,format=raw,file={ovmf_vars} -cdrom {iso}")
                .args(&qemu_args)
                .run()?;
        }
        _ => return Err(format!("Unsupported architecture: {}", arch).into()),
    }

    Ok(())
}

/// Run in QEMU BIOS mode (x86_64 only).
pub fn run_bios(sh: &Shell, qemu_flags: &str, iso_path: &Path) -> Result<()> {
    let iso = iso_path.to_str().unwrap();
    let qemu_args: Vec<&str> = qemu_flags.split_whitespace().collect();

    println!("Running in QEMU BIOS mode...");
    cmd!(
        sh,
        "qemu-system-x86_64 -M q35 -device virtio-vga  -serial stdio -cdrom {iso} -boot d"
    )
    .args(&qemu_args)
    .run()?;
    Ok(())
}

/// Run HDD image in QEMU (UEFI mode).
pub fn run_hdd(sh: &Shell, arch: &str, qemu_flags: &str, hdd_path: &Path) -> Result<()> {
    let name = image_name(arch);
    let hdd = hdd_path.to_str().unwrap();
    let ovmf_code = format!("vendor/ovmf/ovmf-code-{}.fd", arch);
    let ovmf_vars = format!("vendor/ovmf/ovmf-vars-{}.fd", arch);

    println!("Running {} HDD in QEMU...", name);

    let qemu_args: Vec<&str> = qemu_flags.split_whitespace().collect();

    match arch {
        "x86_64" => {
            cmd!(sh, "qemu-system-x86_64 -M q35 -device virtio-vga  -serial stdio -drive if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on -drive if=pflash,unit=1,format=raw,file={ovmf_vars} -hda {hdd}")
                .args(&qemu_args)
                .run()?;
        }
        "aarch64" => {
            cmd!(sh, "qemu-system-aarch64 -M virt -cpu cortex-a72 -serial stdio -device ramfb -device qemu-xhci -device usb-kbd -device usb-mouse -drive if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on -drive if=pflash,unit=1,format=raw,file={ovmf_vars} -hda {hdd}")
                .args(&qemu_args)
                .run()?;
        }
        "riscv64" => {
            // riscv64 virt requires blockdev syntax with machine-level pflash assignment
            // Also uses virtio-blk instead of -hda since riscv64 virt doesn't expose IDE to UEFI properly
            cmd!(sh, "qemu-system-riscv64 -blockdev node-name=pflash0,driver=file,read-only=on,filename={ovmf_code} -blockdev node-name=pflash1,driver=file,filename={ovmf_vars} -M virt,pflash0=pflash0,pflash1=pflash1 -cpu rv64 -m 2G -serial stdio -device ramfb -device qemu-xhci -device usb-kbd -device usb-mouse -drive file={hdd},format=raw,if=none,id=drive0 -device virtio-blk-device,drive=drive0")
                .args(&qemu_args)
                .run()?;
        }
        "loongarch64" => {
            cmd!(sh, "qemu-system-loongarch64 -M virt -cpu la464 -serial stdio -device ramfb -device qemu-xhci -device usb-kbd -device usb-mouse -drive if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on -drive if=pflash,unit=1,format=raw,file={ovmf_vars} -hda {hdd}")
                .args(&qemu_args)
                .run()?;
        }
        _ => return Err(format!("Unsupported architecture: {}", arch).into()),
    }

    Ok(())
}
