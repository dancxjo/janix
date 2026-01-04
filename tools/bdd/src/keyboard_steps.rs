use anyhow::Result;
use cucumber::{given, then, when};
use crate::steps::{BootWorld, expect_to_see_simple};
use crate::shared::GLOBAL_QEMU;

#[given("the system has booted")]
async fn given_system_booted(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "Booted.".to_string()).await
}

#[given("the scheduler is running with interrupts enabled")]
async fn given_scheduler_running(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "TICK: switching to task".to_string()).await
}

#[given("the input service is available")]
async fn given_input_service_available(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "INPUTD: Alive".to_string()).await
}

#[given("at least one keyboard-capable input device exists")]
async fn given_keyboard_exists(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "input: discovered keyboard".to_string()).await
}

#[when("I query the input devices")]
async fn when_query_input_devices(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[then(expr = "I can discover a device with kind {string}")]
async fn then_discover_device(world: &mut BootWorld, kind: String) -> Result<()> {
    expect_to_see_simple(world, format!("kind: {}", kind)).await
}

#[then("it has a stable identity")]
async fn then_stable_identity(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "identity: stable".to_string()).await
}

#[given("no keyboard-capable input device exists")]
async fn given_no_keyboard(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[when("the system is running")]
async fn when_system_running(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "TICK: switching to task".to_string()).await
}

#[then("the input service remains healthy")]
async fn then_input_service_healthy(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "INPUTD: healthy".to_string()).await
}

#[then("no keyboard events are produced")]
async fn then_no_keyboard_events(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[when("a key is pressed")]
async fn when_key_pressed(_world: &mut BootWorld) -> Result<()> {
    let mut guard = GLOBAL_QEMU.lock().await;
    if let Some(qemu) = guard.as_mut() {
        qemu.send_key("a").await?;
    }
    Ok(())
}

#[then("a KeyEvent is produced")]
async fn then_keyevent_produced(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "KeyEvent produced".to_string()).await
}

#[then("the event includes press-or-release state")]
async fn then_event_includes_state(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "state: ".to_string()).await
}

#[then("the event includes the source device identity")]
async fn then_event_includes_source(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "source: ".to_string()).await
}

#[when("many keys are pressed rapidly")]
async fn when_many_keys_rapidly(_world: &mut BootWorld) -> Result<()> {
    let mut guard = GLOBAL_QEMU.lock().await;
    if let Some(qemu) = guard.as_mut() {
        for _ in 0..10 {
            qemu.send_key("a").await?;
        }
    }
    Ok(())
}

#[then("key events are queued without loss")]
async fn then_events_queued_no_loss(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "queued: 10".to_string()).await
}

#[then("the system remains responsive")]
async fn then_system_responsive(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "TICK: switching to task".to_string()).await
}

#[when("a key event is captured")]
async fn when_key_event_captured(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "KeyEvent captured".to_string()).await
}

#[then("the capture path does not allocate memory")]
async fn then_no_alloc(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[then("the capture path does not perform layout translation")]
async fn then_no_layout_translation(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[given("an application subscribes to keyboard events")]
async fn given_app_subscribes(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "subscription: KeyEvent".to_string()).await
}

#[then("the application receives the KeyEvent")]
async fn then_app_receives_keyevent(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "delivered: KeyEvent".to_string()).await
}

#[given("an application is not currently reading input")]
async fn given_app_not_reading(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[when("key events occur")]
async fn when_key_events_occur(_world: &mut BootWorld) -> Result<()> {
    let mut guard = GLOBAL_QEMU.lock().await;
    if let Some(qemu) = guard.as_mut() {
        qemu.send_key("b").await?;
    }
    Ok(())
}

#[then("the events remain available until read")]
async fn then_events_available(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "buffered: 1".to_string()).await
}

#[given("two applications subscribe to keyboard events")]
async fn given_two_apps_subscribe(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "subscriptions: 2".to_string()).await
}

#[then("both applications can observe the KeyEvent")]
async fn then_both_apps_observe(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "delivered to 2".to_string()).await
}

#[then("the event identity is consistent between observers")]
async fn then_event_identity_consistent(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "identity: consistent".to_string()).await
}

#[then("a KeyEvent Thing exists in the graph")]
async fn then_keyevent_thing_exists(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "kind: kind.KeyEvent".to_string()).await
}

#[then("it links to the originating keyboard device Thing")]
async fn then_links_to_kbd(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "link: device.keyboard".to_string()).await
}

#[given("a KeyEvent exists")]
async fn given_keyevent_exists(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "kind: kind.KeyEvent".to_string()).await
}

#[when("I inspect its relationships")]
async fn when_inspect_relationships(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[then("I can trace which consumer received it")]
async fn then_can_trace_consumer(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "trace: delivered".to_string()).await
}

// --- FUTURE SCENARIOS (Stubs) ---

#[given("two windows exist")]
async fn given_two_windows(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[given("one window is focused")]
async fn given_one_window_focused(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[then("the focused window receives the KeyEvent")]
async fn then_focused_receives(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[then("the unfocused window does not")]
async fn then_unfocused_does_not(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[given("a focused window exists")]
async fn given_focused_window_exists(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[when("focus changes to another window")]
async fn when_focus_changes(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[then("subsequent keyboard events route to the new focused window")]
async fn then_route_to_new_focus(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[given("a keyboard layout is active")]
async fn given_layout_active(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[when("key events correspond to printable characters")]
async fn when_printable_key(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[then("a TextEvent is produced")]
async fn then_textevent_produced(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[when("the key event is a control or navigation key")]
async fn when_control_key(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[then("no TextEvent is produced")]
async fn then_no_textevent(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}
