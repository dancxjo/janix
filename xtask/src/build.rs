//! Build task - compiles the kernel for target architectures.

use xshell::{Shell, cmd};
use crate::common::{rust_target, profile_subdir, Result};

/// Build the kernel for a target architecture.
pub fn build(sh: &Shell, arch: &str, profile: &str) -> Result<()> {
    let target = rust_target(arch);
    let subdir = profile_subdir(profile);

    println!("Building bran kernel for {} ({} profile)...", arch, profile);

    cmd!(sh, "cargo build --target {target} --profile {profile} -p bran")
        .env("RUSTFLAGS", "-C relocation-model=static -C panic=abort")
        .run()?;

    // Copy kernel binary to bran/bin-{arch}/
    let bin_dir = format!("bran/bin-{}", arch);
    sh.create_dir(&bin_dir)?;

    let src = format!("target/{}/{}/bran", target, subdir);
    let dst = format!("{}/kernel", bin_dir);
    sh.copy_file(&src, &dst)?;

    println!("Kernel built: {}", dst);

    // Build Userspace
    println!("Building userspace modules for {}...", arch);
    {
        let _p = sh.push_dir("userspace");
        cmd!(sh, "cargo build --workspace --target {target} --profile {profile}")
            .env("RUSTFLAGS", "-C relocation-model=static -C panic=abort")
            .run()?;
    }

    // Copy userspace binaries
    // We expect them in userspace/target/{target}/{profile}/
    // We want to verify them and copy them to a staging area or directly to where image builder needs them.
    // For now, let's copy them to bran/bin-{arch}/modules/ so image builder can pick them up.
    let modules_dir = format!("{}/modules", bin_dir);
    sh.create_dir(&modules_dir)?;

    let user_profile_dir = if profile == "dev" { "debug" } else { profile };
    let user_target_dir = format!("userspace/target/{}/{}", target, user_profile_dir);

    for app in ["sprout", "clock", "rtc_cmos"] {
        let src = format!("{}/{}", user_target_dir, app);
        let dst = format!("{}/{}", modules_dir, app);
        // On windows it might be app.exe but we are on linux
        if sh.path_exists(&src) {
             sh.copy_file(&src, &dst)?;
             println!("Module built: {}", dst);
        } else {
             eprintln!("Warning: module binary not found at {}", src);
        }
    }

    Ok(())
}
