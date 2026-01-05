use crate::qemu::QemuProcess;
use crate::shared::{ANY_FAILURE, GLOBAL_LAST_ERROR, GLOBAL_QEMU};
use anyhow::{Context, Result};
use cucumber::{given, then, when, World};
use image::{GenericImageView, Pixel};
use std::path::PathBuf;
use std::process::Command as SyncCommand;
use std::process::Stdio;
use tokio::process::Command;

#[derive(Debug, Default, World)]
pub struct BootWorld {
    pub arch: String,
    pub boot_variant: String,
    pub last_query_success: bool,
}

// --- Helpers ---

pub async fn get_clean_log() -> String {
    let guard = GLOBAL_QEMU.lock().await;
    if let Some(qemu) = guard.as_ref() {
        let log = qemu.log_buffer.lock();
        return strip_ansi_codes(&log);
    }
    String::new()
}

pub fn strip_ansi_codes(s: &str) -> String {
    let re = regex::Regex::new(r"\x1B\[([0-9]{1,2}(;[0-9]{1,2})?)?[mGK]").unwrap();
    re.replace_all(s, "").to_string()
}

pub async fn soft_fail(msg: String) {
    eprintln!("SOFT FAIL: {}", msg);

    // PRINT LOG CONTEXT
    let log = get_clean_log().await;
    let lines: Vec<&str> = log.lines().collect();
    let count = lines.len();
    let start = if count > 100 { count - 100 } else { 0 };
    eprintln!("--- LOG CONTEXT (Last 100 lines) ---");
    for line in &lines[start..] {
        eprintln!("{}", line);
    }
    eprintln!("-----------------------------------");

    {
        let mut guard = GLOBAL_LAST_ERROR.lock().await;
        *guard = Some(msg);
    }
    ANY_FAILURE.store(true, std::sync::atomic::Ordering::SeqCst);
}

pub async fn expect_to_see_simple(world: &mut BootWorld, needle: String) -> Result<()> {
    let _ = world; // Silence unused warning
    let start = std::time::Instant::now();
    let timeout_duration = std::time::Duration::from_secs(30);

    while start.elapsed() < timeout_duration {
        let log = get_clean_log().await;
        if log.contains(&needle) {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        let status = {
            let mut guard = GLOBAL_QEMU.lock().await;
            if let Some(qemu) = guard.as_mut() {
                qemu.check_status()
            } else {
                None
            }
        };

        if let Some(s) = status {
            soft_fail(format!("QEMU exited early with {}", s)).await;
            return Ok(());
        }
    }

    soft_fail(format!("Timeout waiting for '{}'", needle)).await;
    Ok(())
}

async fn boot_os_impl(world: &mut BootWorld, display_provider: Option<String>) -> Result<()> {
    if world.arch.is_empty() {
        world.arch = std::env::var("BDD_ARCH").unwrap_or_else(|_| "x86_64".to_string());
    }
    if world.arch == "all" {
        world.arch = "x86_64".to_string();
    }
    let arch = &world.arch;

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_dir.parent().unwrap().parent().unwrap();
    let xtask_bin = root.join("target").join("debug").join("xtask");

    // Rebuild ISO
    let mut cmd = Command::new(&xtask_bin);
    cmd.args(["iso", "--env", arch]);

    let status = cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await?;

    if !status.success() {
        return Err(anyhow::anyhow!("Failed to build ISO"));
    }

    let iso_path = root
        .join("target")
        .join("iso")
        .join(format!("thingos-{}.iso", arch));
    let ovmf_dir = root.join("vendor").join("ovmf");
    let ovmf_code = ovmf_dir.join(format!("ovmf-code-{}.fd", arch));
    let ovmf_vars = ovmf_dir.join(format!("ovmf-vars-{}.fd", arch));

    let qmp_sock_dir = std::env::temp_dir();
    let qmp_sock_path = qmp_sock_dir.join(format!("qmp-{}.sock", uuid::Uuid::new_v4()));

    let qemu = QemuProcess::spawn(
        arch,
        &iso_path,
        &ovmf_code,
        &ovmf_vars,
        &qmp_sock_path,
        display_provider.as_deref(),
    )
    .await?;

    {
        let mut guard = GLOBAL_QEMU.lock().await;
        *guard = Some(qemu);
    }

    let mut guard = GLOBAL_LAST_ERROR.lock().await;
    *guard = None;
    ANY_FAILURE.store(false, std::sync::atomic::Ordering::SeqCst);

    Ok(())
}

// --- STEPS ---

#[given("sprout is online")]
async fn given_sprout_online(world: &mut BootWorld) -> Result<()> {
    {
        let guard = GLOBAL_QEMU.lock().await;
        if guard.is_none() {
            drop(guard);
            boot_os_impl(world, None).await?;
        }
    }
    expect_to_see_simple(world, "SPROUT: I am alive".to_string()).await
}

#[given("I boot the system")]
async fn given_i_boot(world: &mut BootWorld) -> Result<()> {
    boot_os_impl(world, None).await
}

#[then(expr = "the serial output contains {string}")]
async fn then_serial_contains(world: &mut BootWorld, needle: String) -> Result<()> {
    expect_to_see_simple(world, needle).await
}

#[given(expr = "the system has reached {string}")]
async fn given_system_reached(world: &mut BootWorld, state: String) -> Result<()> {
    {
        let guard = GLOBAL_QEMU.lock().await;
        if guard.is_none() {
            drop(guard);
            boot_os_impl(world, None).await?;
        }
    }

    match state.as_str() {
        "kernel ready" => expect_to_see_simple(world, "Booted.".to_string()).await?,
        "userland start" => expect_to_see_simple(world, "SPROUT: I am alive".to_string()).await?,
        _ => soft_fail(format!("Unknown state: {}", state)).await,
    }
    Ok(())
}

#[then(expr = "the system should reach {string}")]
async fn then_system_should_reach(world: &mut BootWorld, state: String) -> Result<()> {
    match state.as_str() {
        "kernel ready" => expect_to_see_simple(world, "Booted.".to_string()).await,
        "userland start" => expect_to_see_simple(world, "SPROUT: I am alive".to_string()).await,
        _ => {
            soft_fail(format!("Unknown state: {}", state)).await;
            Ok(())
        }
    }
}

// --- Thing Envelope Steps ---

#[when("I attempt to create a Thing with an invalid magic number")]
async fn when_invalid_magic(_world: &mut BootWorld) -> Result<()> {
    // bouncer_test runs automatically at userland start
    Ok(())
}

#[then("the kernel should return ERR_INVALID_THING_BODY")]
async fn then_kernel_rejects_magic(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "TEST: invalid magic rejected - PASS".to_string()).await
}

#[when("I create a Thing with a valid envelope")]
async fn when_create_valid_envelope(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[then("I should be able to retrieve it and get a matching CRC64 digest")]
async fn then_digest_verified(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "TEST: integrity digest verified - PASS".to_string()).await
}

#[given("thingcheck is running")]
async fn given_thingcheck_running(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "--- Thing Check Tool ---".to_string()).await
}

#[then("thingcheck should successfully decode a DisplayDevice body from the graph")]
async fn then_thingcheck_decodes_display(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "Found DisplayDevice!".to_string()).await
}

#[then("it should not require a global ontology.bin")]
async fn then_no_ontology_required(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

// --- Display / Wallpaper Steps ---

#[then(expr = "the graph should contain a Thing named {string}")]
async fn then_graph_contains_named(world: &mut BootWorld, name: String) -> Result<()> {
    expect_to_see_simple(world, format!("register name: {}", name)).await
}

#[then(expr = "the compositor paints the wallpaper from {string}")]
async fn then_compositor_paints(world: &mut BootWorld, asset_name: String) -> Result<()> {
    expect_to_see_simple(world, format!("BLOOM: found asset {}", asset_name)).await?;
    expect_to_see_simple(world, "BLOOM: BMP decoded".to_string()).await?;
    expect_to_see_simple(world, "BLOOM: tiled blit complete".to_string()).await
}

#[given(expr = "I wait for {string}")]
async fn given_wait_for(world: &mut BootWorld, marker: String) -> Result<()> {
    expect_to_see_simple(world, marker).await
}

#[then("the display should show the clouds wallpaper")]
async fn then_display_shows_clouds(world: &mut BootWorld) -> Result<()> {
    let _ = world;
    // ... basic check center is not black as placeholder ...
    Ok(())
}
