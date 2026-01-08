//! QEMU run tasks.

use xshell::{Shell, cmd};
use crate::common::{image_name, Result};

/// Run ISO image in QEMU (UEFI mode).
pub fn run(sh: &Shell, arch: &str, qemu_flags: &str) -> Result<()> {
    let name = image_name(arch);
    let iso = format!("{}.iso", name);
    let ovmf_code = format!("ovmf/ovmf-code-{}.fd", arch);
    let ovmf_vars = format!("ovmf/ovmf-vars-{}.fd", arch);

    println!("Running {} in QEMU...", name);

    // Split QEMU flags on whitespace to pass as separate args
    let qemu_args: Vec<&str> = qemu_flags.split_whitespace().collect();

    match arch {
        "x86_64" => {
            cmd!(sh, "qemu-system-x86_64 -M q35 -serial stdio -drive if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on -drive if=pflash,unit=1,format=raw,file={ovmf_vars} -cdrom {iso}")
                .args(&qemu_args)
                .run()?;
        }
        "aarch64" => {
            cmd!(sh, "qemu-system-aarch64 -M virt -cpu cortex-a72 -serial stdio -device ramfb -device qemu-xhci -device usb-kbd -device usb-mouse -drive if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on -drive if=pflash,unit=1,format=raw,file={ovmf_vars} -cdrom {iso}")
                .args(&qemu_args)
                .run()?;
        }
        "riscv64" => {
            cmd!(sh, "qemu-system-riscv64 -M virt -cpu rv64 -serial stdio -device ramfb -device qemu-xhci -device usb-kbd -device usb-mouse -drive if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on -drive if=pflash,unit=1,format=raw,file={ovmf_vars} -cdrom {iso}")
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
pub fn run_bios(sh: &Shell, qemu_flags: &str) -> Result<()> {
    let iso = "thing-os-x86_64.iso";
    let qemu_args: Vec<&str> = qemu_flags.split_whitespace().collect();
    println!("Running in QEMU BIOS mode...");
    cmd!(sh, "qemu-system-x86_64 -M q35 -serial stdio -cdrom {iso} -boot d")
        .args(&qemu_args)
        .run()?;
    Ok(())
}

/// Run HDD image in QEMU (UEFI mode).
pub fn run_hdd(sh: &Shell, arch: &str, qemu_flags: &str) -> Result<()> {
    let name = image_name(arch);
    let hdd = format!("{}.hdd", name);
    let ovmf_code = format!("ovmf/ovmf-code-{}.fd", arch);
    let ovmf_vars = format!("ovmf/ovmf-vars-{}.fd", arch);

    println!("Running {} HDD in QEMU...", name);

    let qemu_args: Vec<&str> = qemu_flags.split_whitespace().collect();

    match arch {
        "x86_64" => {
            cmd!(sh, "qemu-system-x86_64 -M q35 -serial stdio -drive if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on -drive if=pflash,unit=1,format=raw,file={ovmf_vars} -hda {hdd}")
                .args(&qemu_args)
                .run()?;
        }
        "aarch64" => {
            cmd!(sh, "qemu-system-aarch64 -M virt -cpu cortex-a72 -serial stdio -device ramfb -device qemu-xhci -device usb-kbd -device usb-mouse -drive if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on -drive if=pflash,unit=1,format=raw,file={ovmf_vars} -hda {hdd}")
                .args(&qemu_args)
                .run()?;
        }
        "riscv64" => {
            cmd!(sh, "qemu-system-riscv64 -M virt -cpu rv64 -serial stdio -device ramfb -device qemu-xhci -device usb-kbd -device usb-mouse -drive if=pflash,unit=0,format=raw,file={ovmf_code},readonly=on -drive if=pflash,unit=1,format=raw,file={ovmf_vars} -hda {hdd}")
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
