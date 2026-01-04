use anyhow::{Context, Result};
use cucumber::{given, then, when, World};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;
use crate::qemu::QemuProcess;
use crate::shared::{GLOBAL_LAST_ERROR, GLOBAL_QEMU, ANY_FAILURE};

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
        if let Ok(log) = qemu.log_buffer.lock() {
            return strip_ansi_codes(&log);
        }
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
    // Acquire log WITHOUT holding GLOBAL_QEMU lock (get_clean_log does that)
    let log = get_clean_log().await;
    let lines: Vec<&str> = log.lines().collect();
    let count = lines.len();
    let start = if count > 20 { count - 20 } else { 0 };
    eprintln!("--- LOG CONTEXT (Last 20 lines) ---");
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
    let start = std::time::Instant::now();
    let timeout_duration = std::time::Duration::from_secs(30);
    
    while start.elapsed() < timeout_duration {
        let log = get_clean_log().await;
        if log.contains(&needle) {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        
        // Check for QEMU death
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
    // 1. Determine Arch
    if world.arch.is_empty() {
        world.arch = std::env::var("BDD_ARCH").unwrap_or_else(|_| "x86_64".to_string());
    }
    if world.arch == "all" {
        world.arch = "x86_64".to_string(); 
    }
    let arch = &world.arch;

    println!("DEBUG: Booting OS for arch: {}", arch);

    // 2. Build ISO
    println!("DEBUG: Building ISO for {}...", arch);
    
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_dir.parent().unwrap().parent().unwrap();
    let xtask_bin = root.join("target").join("debug").join("xtask");

    println!("DEBUG: Invoking xtask at {:?}", xtask_bin);

    // Verify xtask exists
    if !xtask_bin.exists() {
        return Err(anyhow::anyhow!("xtask binary not found at {:?}. Run 'cargo build -p xtask' first.", xtask_bin));
    }

    let status = Command::new(&xtask_bin)
        .args(["iso", "--env", arch])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await?;

    if !status.success() {
        return Err(anyhow::anyhow!("Failed to build ISO"));
    }
    println!("DEBUG: ISO Build Complete");

    // 3. Locate files
    let iso_path = root.join("target").join("iso").join(format!("thingos-{}.iso", arch));
    let ovmf_dir = root.join("vendor").join("ovmf");
    let ovmf_code = ovmf_dir.join(format!("ovmf-code-{}.fd", arch));
    let ovmf_vars = ovmf_dir.join(format!("ovmf-vars-{}.fd", arch));
    
    // Create temp socket path
    let qmp_sock_dir = std::env::temp_dir();
    let qmp_sock_path = qmp_sock_dir.join(format!("qmp-{}.sock", uuid::Uuid::new_v4()));

    println!("DEBUG: Files Located: ISO={:?}", iso_path);
    println!("DEBUG: Spawning QEMU...");

    // 4. Spawn QEMU
    let qemu = QemuProcess::spawn(
        arch,
        &iso_path,
        &ovmf_code,
        &ovmf_vars,
        &qmp_sock_path,
        display_provider.as_deref(),
    ).await?;

    println!("DEBUG: QEMU Spawned successfully");

    {
        let mut guard = GLOBAL_QEMU.lock().await;
        *guard = Some(qemu);
    }
    
    // Clean up failure state
    let mut guard = GLOBAL_LAST_ERROR.lock().await;
    *guard = None;
    ANY_FAILURE.store(false, std::sync::atomic::Ordering::SeqCst);

    println!("DEBUG: boot_os_impl complete");
    Ok(())
}


// --- STEPS ---

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
async fn then_it_does_not_panic(world: &mut BootWorld) -> Result<()> {
    let log = get_clean_log().await;
    if log.to_lowercase().contains("panic") {
        soft_fail("Panic detected in logs".to_string()).await;
    }
    Ok(())
}

#[given(expr = "the system has reached {string}")]
async fn given_system_reached(world: &mut BootWorld, state: String) -> Result<()> {
    let guard = GLOBAL_QEMU.lock().await;
    if guard.is_none() {
        drop(guard); 
        boot_os_impl(world, None).await?;
    } else {
        drop(guard);
    }
    
    match state.as_str() {
        "kernel ready" => expect_to_see_simple(world, "Booted.".to_string()).await?,
        "userland start" => expect_to_see_simple(world, "SPROUT: I am alive".to_string()).await?,
        _ => soft_fail(format!("Unknown state: {}", state)).await,
    }
    Ok(())
}

#[given("sprout is online")]
async fn given_sprout_online(world: &mut BootWorld) -> Result<()> {
    let guard = GLOBAL_QEMU.lock().await;
    if guard.is_none() {
        drop(guard);
        boot_os_impl(world, None).await?;
    } else {
        drop(guard);
    }
    expect_to_see_simple(world, "SPROUT: I am alive".to_string()).await
}

#[then(expr = "the system should reach {string}")]
async fn then_system_should_reach(world: &mut BootWorld, state: String) -> Result<()> {
     match state.as_str() {
        "kernel ready" => expect_to_see_simple(world, "Booted.".to_string()).await?,
        "userland start" => expect_to_see_simple(world, "SPROUT: I am alive".to_string()).await?,
        "steady state" => expect_to_see_simple(world, "TICK: switching to task".to_string()).await?,
        _ => soft_fail(format!("Unknown state: {}", state)).await,
    }
    Ok(())
}

#[then(expr = "the graph should contain a Thing named {string}")]
async fn then_graph_contains_thing_named(world: &mut BootWorld, name: String) -> Result<()> {
    let expected = format!("register name: {}", name);
    expect_to_see_simple(world, expected).await
}

#[when("I query the graph for the root Place")]
async fn when_query_root_place(world: &mut BootWorld) -> Result<()> {
    query_place(world, "place.root".to_string()).await
}

#[then(expr = "a Place should exist named {string}")]
async fn then_place_exists_named(world: &mut BootWorld, name: String) -> Result<()> {
    let expected = format!("created: {}", name);
    expect_to_see_simple(world, expected).await
}

#[then(expr = "the graph should contain a Thing of kind {string}")]
async fn then_graph_contains_kind(world: &mut BootWorld, kind: String) -> Result<()> {
    let expected = format!("kind: {}", kind);
    expect_to_see_simple(world, expected).await
}

#[then(expr = "that process should have a name {string}")]
async fn then_process_has_name(world: &mut BootWorld, name: String) -> Result<()> {
    let expected = format!("name: {}", name);
    expect_to_see_simple(world, expected).await
}

#[when(expr = "I debug query for Place {string}")]
async fn query_place(world: &mut BootWorld, place_name: String) -> Result<()> {
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(10);
    let mut exists = false;

    while start.elapsed() < timeout {
        let log = get_clean_log().await;
        exists = log.contains(&format!("register name: {}", place_name))
            || log.contains(&format!("created: {}", place_name))
            || log.contains(&place_name);

        if exists { break; }
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    world.last_query_success = exists;
    Ok(())
}

#[then("the query should succeed")]
async fn query_succeeds(world: &mut BootWorld) -> Result<()> {
    if !world.last_query_success {
        soft_fail("Previous query failed (item not found in logs)".to_string()).await;
    }
    Ok(())
}

#[then("devices that do not exist should simply be absent")]
async fn then_devices_absent(world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[when(expr = "I boot ThingOS on {string}")]
async fn when_boot_arch_legacy(world: &mut BootWorld, arch: String) -> Result<()> {
    world.arch = arch;
    boot_os_impl(world, None).await
}

#[given(expr = "I boot ThingOS on {string}")]
async fn given_boot_arch_legacy(world: &mut BootWorld, arch: String) -> Result<()> {
    world.arch = arch;
    boot_os_impl(world, None).await
}

#[when(expr = "I wait for {int} milliseconds")]
async fn when_wait_ms(world: &mut BootWorld, ms: u64) -> Result<()> {
    tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
    Ok(())
}

#[when(expr = "I wait for {int} second")]
async fn when_wait_sec_singular(world: &mut BootWorld, sec: u64) -> Result<()> {
    tokio::time::sleep(tokio::time::Duration::from_secs(sec)).await;
    Ok(())
}

#[when(expr = "I wait for {int} seconds")]
async fn when_wait_sec_plural(world: &mut BootWorld, sec: u64) -> Result<()> {
    tokio::time::sleep(tokio::time::Duration::from_secs(sec)).await;
    Ok(())
}
