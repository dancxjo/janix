//! QEMU run tasks.

use crate::common::{Result, image_name};
use xshell::Shell;

use std::net::TcpListener;
use std::path::Path;

fn user_netdev_arg() -> String {
    // Optional override:
    // - `THINGOS_HOSTFWD=off` (or empty) disables host forwarding
    // - `THINGOS_HOSTFWD=8899` forwards host 8899 -> guest 80
    if let Ok(v) = std::env::var("THINGOS_HOSTFWD") {
        let v = v.trim();
        if v.is_empty() || v.eq_ignore_ascii_case("off") || v.eq_ignore_ascii_case("none") {
            return "user,id=n0".to_string();
        }
        if let Ok(port) = v.parse::<u16>() {
            return format!("user,id=n0,hostfwd=tcp::{}-:80", port);
        }
        return format!("user,id=n0,hostfwd={}", v);
    }

    let mut netdev = String::from("user,id=n0");

    if TcpListener::bind(("127.0.0.1", 8888)).is_ok() {
        netdev.push_str(",hostfwd=tcp::8888-:80");
    } else {
        eprintln!("xtask: host port 8888 is busy; skipping HTTP forward");
    }

    if TcpListener::bind(("127.0.0.1", 2323)).is_ok() {
        netdev.push_str(",hostfwd=tcp::2323-:2323");
    } else {
        eprintln!("xtask: host port 2323 is busy; skipping telnet forward");
    }

    netdev
}

fn x86_qemu_trace_enabled() -> bool {
    // Keep heavy QEMU tracing opt-in so normal SMP boot timings are not distorted.
    // Any value except "0"/"off"/"false"/"no" enables tracing.
    if let Ok(v) = std::env::var("THINGOS_QEMU_TRACE") {
        let v = v.trim();
        if v.is_empty() {
            return false;
        }
        !matches!(
            v.to_ascii_lowercase().as_str(),
            "0" | "off" | "false" | "no"
        )
    } else {
        false
    }
}

/// Run ISO image in QEMU (UEFI mode).
pub fn run(
    sh: &Shell,
    arch: &str,
    qemu_flags: &str,
    iso_path: &Path,
    interactive: bool,
    monitor: bool,
) -> Result<()> {
    let name = image_name(arch);
    let iso = iso_path.to_str().unwrap();
    let ovmf_code = format!("vendor/ovmf/ovmf-code-{}.fd", arch);
    let ovmf_vars = format!("vendor/ovmf/ovmf-vars-{}.fd", arch);

    println!("Running {} in QEMU...", name);

    // Standard behavior: --nographic (mon:stdio) by default
    // Interactive: GUI + mon:stdio
    let qemu_args: Vec<&str> = qemu_flags.split_whitespace().collect();
    let mut final_args = Vec::new();

    if !interactive {
        final_args.push("-nographic");
    } else {
        if monitor {
            final_args.extend(&["-serial", "mon:stdio"]);
        } else {
            final_args.extend(&["-serial", "stdio"]);
        }
    }

    if monitor && interactive {
        final_args.extend(&["-monitor", "stdio"]);
    }

    let netdev = user_netdev_arg();
    match arch {
        "x86_64" => {
            let pflash0 = format!("if=pflash,unit=0,format=raw,file={},readonly=on", ovmf_code);
            let pflash1 = format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars);
            let netdev_arg = format!("virtio-net-pci,netdev=n0,mac=52:54:00:12:34:56,disable-legacy=on");
            let netdev_val = netdev.to_string();

            let mut args = if x86_qemu_trace_enabled() {
                vec![
                    "-M", "q35",
                    "-drive", &pflash0,
                    "-drive", &pflash1,
                    "-cdrom", iso,
                    "-no-reboot",
                    "-d", "int,cpu_reset", "-D", "qemu.log",
                    "-device", &netdev_arg,
                    "-netdev", &netdev_val,
                ]
            } else {
                vec![
                    "-M", "q35",
                    "-drive", &pflash0,
                    "-drive", &pflash1,
                    "-cdrom", iso,
                    "-no-reboot",
                    "-device", &netdev_arg,
                    "-netdev", &netdev_val,
                ]
            };

            if interactive {
                args.extend_from_slice(&["-device", "virtio-vga", "-M", "q35,usb=off,vmport=off,i8042=on"]);
            }

            args.extend(&final_args);
            run_qemu(sh, "qemu-system-x86_64", &args, &qemu_args)?;
        }
        "aarch64" => {
            let pflash0 = format!("if=pflash,unit=0,format=raw,file={},readonly=on", ovmf_code);
            let pflash1 = format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars);
            let mut args = vec![
                "-M", "virt",
                "-cpu", "cortex-a72",
                "-semihosting",
                "-drive", &pflash0,
                "-drive", &pflash1,
                "-cdrom", iso,
            ];
            if interactive {
                args.extend_from_slice(&["-device", "ramfb", "-device", "qemu-xhci", "-device", "usb-kbd", "-device", "usb-mouse"]);
            }
            args.extend(&final_args);
            run_qemu(sh, "qemu-system-aarch64", &args, &qemu_args)?;
        }
        "riscv64" => {
            // riscv64 virt requires blockdev syntax with machine-level pflash assignment
            // Also uses virtio-blk instead of -cdrom since riscv64 virt doesn't expose cdrom to UEFI properly
            let pflash0 = format!("node-name=pflash0,driver=file,read-only=on,filename={}", ovmf_code);
            let pflash1 = format!("node-name=pflash1,driver=file,filename={}", ovmf_vars);
            let drive0 = format!("file={},format=raw,if=none,id=drive0,readonly=on", iso);
            let mut args = vec![
                "-blockdev", &pflash0,
                "-blockdev", &pflash1,
                "-M", "virt,pflash0=pflash0,pflash1=pflash1",
                "-cpu", "rv64",
                "-m", "2G",
                "-drive", &drive0,
                "-device", "virtio-blk-device,drive=drive0",
            ];
            if interactive {
                args.extend_from_slice(&["-device", "ramfb", "-device", "qemu-xhci", "-device", "usb-kbd", "-device", "usb-mouse"]);
            }
            args.extend(&final_args);
            run_qemu(sh, "qemu-system-riscv64", &args, &qemu_args)?;
        }
        "loongarch64" => {
            let pflash0 = format!("if=pflash,unit=0,format=raw,file={},readonly=on", ovmf_code);
            let pflash1 = format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars);
            let mut args = vec![
                "-M", "virt",
                "-cpu", "la464",
                "-drive", &pflash0,
                "-drive", &pflash1,
                "-cdrom", iso,
            ];
            if interactive {
                args.extend_from_slice(&["-device", "ramfb", "-device", "qemu-xhci", "-device", "usb-kbd", "-device", "usb-mouse"]);
            }
            args.extend(&final_args);
            run_qemu(sh, "qemu-system-loongarch64", &args, &qemu_args)?;
        }
        _ => return Err(format!("Unsupported architecture: {}", arch).into()),
    }

    Ok(())
}

/// Run in QEMU BIOS mode (x86_64 only).
pub fn run_bios(
    sh: &Shell,
    qemu_flags: &str,
    iso_path: &Path,
    interactive: bool,
    monitor: bool,
) -> Result<()> {
    let iso = iso_path.to_str().unwrap();
    let qemu_args: Vec<&str> = qemu_flags.split_whitespace().collect();
    let netdev = user_netdev_arg();

    let mut final_args = Vec::new();
    if !interactive {
        final_args.push("-nographic");
    } else {
        final_args.extend(&["-serial", "mon:stdio"]);
        if monitor {
            final_args.extend(&["-monitor", "stdio"]);
        }
    }

    println!("Running in QEMU BIOS mode...");
    let mut args = vec![
        "-M", "q35",
        "-cdrom", iso,
        "-boot", "d",
        "-device", "virtio-net-pci,netdev=n0,mac=52:54:00:12:34:56,disable-legacy=on",
        "-netdev", &netdev,
    ];

    if interactive {
        args.extend_from_slice(&["-device", "virtio-vga", "-M", "q35,usb=off,vmport=off,i8042=on"]);
    }

    args.extend(&final_args);
    run_qemu(sh, "qemu-system-x86_64", &args, &qemu_args)?;
    Ok(())
}

/// Run HDD image in QEMU (UEFI mode).
pub fn run_hdd(
    sh: &Shell,
    arch: &str,
    qemu_flags: &str,
    hdd_path: &Path,
    interactive: bool,
    monitor: bool,
) -> Result<()> {
    let name = image_name(arch);
    let hdd = hdd_path.to_str().unwrap();
    let ovmf_code = format!("vendor/ovmf/ovmf-code-{}.fd", arch);
    let ovmf_vars = format!("vendor/ovmf/ovmf-vars-{}.fd", arch);

    println!("Running {} HDD in QEMU...", name);

    let mut final_args = Vec::new();
    if !interactive {
        final_args.push("-nographic");
    } else {
        final_args.extend(&["-serial", "mon:stdio"]);
        if monitor {
            final_args.extend(&["-monitor", "stdio"]);
        }
    }

    let qemu_args: Vec<&str> = qemu_flags.split_whitespace().collect();
    let netdev = user_netdev_arg();
    match arch {
        "x86_64" => {
            let pflash0 = format!("if=pflash,unit=0,format=raw,file={},readonly=on", ovmf_code);
            let pflash1 = format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars);
            let netdev_val = netdev.to_string();
            let mut args = vec![
                "-M", "q35",
                "-drive", &pflash0,
                "-drive", &pflash1,
                "-hda", hdd,
                "-device", "virtio-net-pci,netdev=n0,mac=52:54:00:12:34:56,disable-legacy=on",
                "-netdev", &netdev_val,
            ];

            if interactive {
                args.extend_from_slice(&["-device", "virtio-vga", "-M", "q35,usb=off,vmport=off,i8042=on"]);
            }

            args.extend(&final_args);
            run_qemu(sh, "qemu-system-x86_64", &args, &qemu_args)?;
        }
        "aarch64" => {
            let pflash0 = format!("if=pflash,unit=0,format=raw,file={},readonly=on", ovmf_code);
            let pflash1 = format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars);
            let mut args = vec![
                "-M", "virt",
                "-cpu", "cortex-a72",
                "-drive", &pflash0,
                "-drive", &pflash1,
                "-hda", hdd,
            ];
            if interactive {
                args.extend_from_slice(&["-device", "ramfb", "-device", "qemu-xhci", "-device", "usb-kbd", "-device", "usb-mouse"]);
            }
            args.extend(&final_args);
            run_qemu(sh, "qemu-system-aarch64", &args, &qemu_args)?;
        }
        "riscv64" => {
            // riscv64 virt requires blockdev syntax with machine-level pflash assignment
            // Also uses virtio-blk instead of -hda since riscv64 virt doesn't expose IDE to UEFI properly
            let pflash0 = format!("node-name=pflash0,driver=file,read-only=on,filename={}", ovmf_code);
            let pflash1 = format!("node-name=pflash1,driver=file,filename={}", ovmf_vars);
            let hda = format!("file={},format=raw,if=none,id=drive0", hdd);
            let mut args = vec![
                "-blockdev", &pflash0,
                "-blockdev", &pflash1,
                "-M", "virt,pflash0=pflash0,pflash1=pflash1",
                "-cpu", "rv64",
                "-m", "2G",
                "-drive", &hda,
                "-device", "virtio-blk-device,drive=drive0",
            ];
            if interactive {
                args.extend_from_slice(&["-device", "ramfb", "-device", "qemu-xhci", "-device", "usb-kbd", "-device", "usb-mouse"]);
            }
            args.extend(&final_args);
            run_qemu(sh, "qemu-system-riscv64", &args, &qemu_args)?;
        }
        "loongarch64" => {
            let pflash0 = format!("if=pflash,unit=0,format=raw,file={},readonly=on", ovmf_code);
            let pflash1 = format!("if=pflash,unit=1,format=raw,file={}", ovmf_vars);
            let mut args = vec![
                "-M", "virt",
                "-cpu", "la464",
                "-drive", &pflash0,
                "-drive", &pflash1,
                "-hda", hdd,
            ];
            if interactive {
                args.extend_from_slice(&["-device", "ramfb", "-device", "qemu-xhci", "-device", "usb-kbd", "-device", "usb-mouse"]);
            }
            args.extend(&final_args);
            run_qemu(sh, "qemu-system-loongarch64", &args, &qemu_args)?;
        }
        _ => return Err(format!("Unsupported architecture: {}", arch).into()),
    }

    Ok(())
}

fn run_qemu(_sh: &Shell, program: &str, base_args: &[&str], extra_args: &[&str]) -> Result<()> {
    let mut cmd = std::process::Command::new(program);
    cmd.stdout(std::process::Stdio::inherit());
    cmd.stderr(std::process::Stdio::inherit());
    cmd.stdin(std::process::Stdio::inherit());

    for arg in base_args {
        cmd.arg(arg);
    }
    for arg in extra_args {
        cmd.arg(arg);
    }

    // Print command for debugging (similar to xshell)
    println!("$ {} {}", program, cmd.get_args().map(|a| a.to_string_lossy()).collect::<Vec<_>>().join(" "));

    let status = cmd.status().map_err(|e| format!("failed to run qemu: {}", e))?;
    if !status.success() {
        return Err(format!("qemu exited with non-zero code: {}", status).into());
    }
    Ok(())
}
