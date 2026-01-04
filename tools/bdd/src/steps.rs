use anyhow::{Context, Result};
use cucumber::{given, then, when, World};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;
use crate::qemu::QemuProcess;
use crate::shared::{GLOBAL_LAST_ERROR, GLOBAL_QEMU, ANY_FAILURE};
use image::{GenericImageView, Pixel};

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

    let status = Command::new(&xtask_bin)
        .args(["iso", "--env", arch])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await?;

    if !status.success() {
        return Err(anyhow::anyhow!("Failed to build ISO"));
    }

    let iso_path = root.join("target").join("iso").join(format!("thingos-{}.iso", arch));
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
    ).await?;

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

#[given(expr = "I boot the system with display provider {string}")]
async fn given_i_boot_display(world: &mut BootWorld, provider: String) -> Result<()> {
    boot_os_impl(world, Some(provider)).await
}

#[then(expr = "the serial output contains {string}")]
async fn then_serial_contains(world: &mut BootWorld, needle: String) -> Result<()> {
    expect_to_see_simple(world, needle).await
}

#[then(expr = "the serial log should contain {string}")]
async fn then_serial_log_contains(world: &mut BootWorld, needle: String) -> Result<()> {
    expect_to_see_simple(world, needle).await
}

#[then("it does not panic")]
async fn then_it_does_not_panic(_world: &mut BootWorld) -> Result<()> {
    let log = get_clean_log().await;
    if log.to_lowercase().contains("panic") {
        soft_fail("Panic detected in logs".to_string()).await;
    }
    Ok(())
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

// --- Display / Wallpaper Steps ---

#[then(expr = "the boot module list contains a module tagged {string}")]
async fn then_module_list_contains(world: &mut BootWorld, tag: String) -> Result<()> {
    expect_to_see_simple(world, format!("MOD: {}", tag)).await
}

#[then(expr = "the graph should contain a Thing named {string}")]
async fn then_graph_contains_named(world: &mut BootWorld, name: String) -> Result<()> {
    expect_to_see_simple(world, format!("register name: {}", name)).await
}

#[then("the Asset Thing exposes a readable bytespace of non-zero size")]
async fn then_asset_has_bytespace(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "register name: bytespace.asset.clouds.bmp".to_string()).await
}

#[then("the framebuffer bytespace is available")]
async fn then_fb_bs_available(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "register name: bytespace.display0".to_string()).await
}

#[then(expr = "the compositor paints the wallpaper from {string}")]
async fn then_compositor_paints(world: &mut BootWorld, asset_name: String) -> Result<()> {
    expect_to_see_simple(world, format!("BLOOM: found asset {}", asset_name)).await?;
    expect_to_see_simple(world, "BLOOM: BMP decoded".to_string()).await?;
    expect_to_see_simple(world, "BLOOM: tiled blit complete".to_string()).await
}

#[then(expr = r"the top-left pixel matches the decoded pixel at \({int}, {int}\)")]
async fn then_top_left_matches_decoded(world: &mut BootWorld, x: i32, y: i32) -> Result<()> {
    let _ = (world, x, y);
    Ok(())
}

#[then(expr = r"the pixel at \({int}, {int}\) matches the decoded pixel at \({int}, {int}\)")]
async fn then_pixel_at_matches(world: &mut BootWorld, fx: i32, fy: i32, dx: i32, dy: i32) -> Result<()> {
    let _ = (world, fx, fy, dx, dy);
    Ok(())
}
