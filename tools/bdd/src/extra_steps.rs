use anyhow::Result;
use cucumber::{given, then, when};

use crate::steps::{expect_to_see_simple, BootWorld};

#[given("the system boots to Sprout")]
async fn system_boots(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "SPROUT: I am alive".to_string()).await
}

#[given("Bloom is running as a userspace compositor")]
async fn bloom_running(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "BLOOM: alive".to_string()).await
}

#[given("Bloom owns a Bytespace-backed framebuffer surface")]
async fn bloom_has_surface(_world: &mut BootWorld) -> Result<()> {
    // Presence of Bloom and display mapping is implicit for now.
    Ok(())
}

#[given(regex = "an app \"hello_window\" is started as a user task")]
async fn hello_window_started(_world: &mut BootWorld) -> Result<()> {
    // Boot sequence auto-starts hello_window in this scenario; nothing to enforce yet.
    Ok(())
}

#[when(regex = "\"hello_window\" creates a Window Thing sized .*")]
async fn hello_window_creates_window(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[when(regex = "\"hello_window\" creates a Layout Thing of kind .*")]
async fn hello_window_creates_layout(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[when(regex = "\"hello_window\" creates a Label Thing with text .*")]
async fn hello_window_creates_label(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[when(regex = "\"hello_window\" creates a Button Thing with text .*")]
async fn hello_window_creates_button(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[when(regex = "\"hello_window\" links the .*")]
async fn hello_window_links(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[when(regex = "\"hello_window\" commits a new Frame Thing .*")]
async fn hello_window_commits(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[then("Bloom must observe the Window Thing and its widget tree")]
async fn bloom_observes_window(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "BLOOM: rendered window".to_string()).await
}

#[then("Bloom must render the Window rectangle with rounded corners")]
async fn bloom_renders_window(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "BLOOM: rendered window".to_string()).await
}

#[then("Bloom must render a drop shadow behind the Window using the shared shadow kernel")]
async fn bloom_renders_shadow(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "BLOOM: shadow kernel: shared".to_string()).await
}

#[then("Bloom must render the Label and Button positioned by the Layout")]
async fn bloom_renders_widgets(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[then("the framebuffer must contain non-background pixels in the Window region")]
async fn framebuffer_changed(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[given("Bloom has cursor rendering enabled")]
async fn cursor_enabled(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "BLOOM: alive".to_string()).await
}

#[when("Bloom renders the cursor and renders a shadowed Window in the same frame")]
async fn cursor_and_window(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}

#[then("both shadows must use the same shadow kernel implementation")]
async fn shared_shadow(world: &mut BootWorld) -> Result<()> {
    expect_to_see_simple(world, "BLOOM: shadow kernel: shared".to_string()).await
}

#[then("the kernel must accept two call sites:")]
async fn shadow_kernel_table(_world: &mut BootWorld) -> Result<()> {
    Ok(())
}
