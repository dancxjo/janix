import sys
import os

path = '/home/dancxjo/src/thing-os/tools/bdd/src/steps.rs'
with open(path, 'r') as f:
    lines = f.readlines()

def find_func_end(lines, start_idx):
    braces = 0
    found_start = False
    for i in range(start_idx, len(lines)):
        braces += lines[i].count('{')
        if '{' in lines[i]:
            found_start = True
        braces -= lines[i].count('}')
        if found_start and braces == 0:
            return i + 1
    return -1

# find help_impl indices
idx_helper_start = -1
idx_helper_end = -1
for i, line in enumerate(lines):
    if 'fn verify_clock_center_pixels' in line:
        idx_helper_start = i
        idx_helper_end = find_func_end(lines, i)
        break

new_helper_impl = [
    "fn verify_clock_center_pixels(img: &image::RgbImage) -> (u32, u32, u32, &'static str) {\n",
    "    let (width, height) = img.dimensions();\n",
    "    \n",
    "    // Check two likely locations: Center and Bottom-Right\n",
    "    let regions = [\n",
    "        (width / 2, height / 2, \"center\"),\n",
    "        (width.saturating_sub(220), height.saturating_sub(105), \"bottom-right\"),\n",
    "    ];\n",
    "\n",
    "    let mut best_black = 0;\n",
    "    let mut best_red = 0;\n",
    "    let mut best_other = 0;\n",
    "    let mut best_loc = \"none\";\n",
    "\n",
    "    for (cx, cy, loc) in regions {\n",
    "        let mut black = 0u32;\n",
    "        let mut red = 0u32;\n",
    "        let mut other = 0u32;\n",
    "        \n",
    "        let sample_w = 100.min(width / 4);\n",
    "        let sample_h = 50.min(height / 6);\n",
    "        \n",
    "        for y in cy.saturating_sub(sample_h)..=(cy + sample_h).min(height - 1) {\n",
    "            for x in cx.saturating_sub(sample_w)..=(cx + sample_w).min(width - 1) {\n",
    "                let pixel = img.get_pixel(x, y).0;\n",
    "                if pixel[0] < 20 && pixel[1] < 20 && pixel[2] < 20 {\n",
    "                    black += 1;\n",
    "                } else if pixel[0] > 180 && pixel[1] < 80 && pixel[2] < 80 {\n",
    "                    red += 1;\n",
    "                } else {\n",
    "                    other += 1;\n",
    "                }\n",
    "            }\n",
    "        }\n",
    "        \n",
    "        // Prioritize region with most red pixels (clock digits)\n",
    "        if red > best_red || (red == best_red && black > best_black) {\n",
    "            best_black = black;\n",
    "            best_red = red;\n",
    "            best_other = other;\n",
    "            best_loc = loc;\n",
    "        }\n",
    "    }\n",
    "    (best_black, best_red, best_other, best_loc)\n",
    "}\n"
]

new_helper = [
    "\n",
    "async fn wait_for_clock_pixels(world: &mut ThingOsWorld, timeout_secs: f64) -> Result<(), StepError> {\n",
    "    let start = std::time::Instant::now();\n",
    "    let timeout = std::time::Duration::from_secs_f64(timeout_secs);\n",
    "    let mut attempts = 0;\n",
    "\n",
    "    while start.elapsed() < timeout {\n",
    "        attempts += 1;\n",
    "        let screenshot_path = crate::artifacts::global()\n",
    "            .lock()\n",
    "            .await\n",
    "            .screenshot_path(&format!(\"clock_verify_{}\", attempts));\n",
    "\n",
    "        let png_path = match world.take_screenshot(&screenshot_path).await {\n",
    "            Ok(p) => p,\n",
    "            Err(e) => {\n",
    "                tokio::time::sleep(std::time::Duration::from_secs(2)).await;\n",
    "                continue;\n",
    "            }\n",
    "        };\n",
    "\n",
    "        let img = match image::open(&png_path) {\n",
    "            Ok(i) => i.to_rgb8(),\n",
    "            Err(_) => {\n",
    "                tokio::time::sleep(std::time::Duration::from_secs(2)).await;\n",
    "                continue;\n",
    "            }\n",
    "        };\n",
    "        \n",
    "        let (black, red, other, loc) = verify_clock_center_pixels(&img);\n",
    "\n",
    "        let total = black + red + other;\n",
    "        let black_pct = if total > 0 { (black as f64 / total as f64) * 100.0 } else { 0.0 };\n",
    "        let red_pct = if total > 0 { (red as f64 / total as f64) * 100.0 } else { 0.0 };\n",
    "\n",
    "        // [CONTRACT] reporting\n",
    "        eprintln!(\"[CONTRACT] Clock Pixels: red={} ({:.1}%), black={} ({:.1}%), other={} region={} (attempt {})\", \n",
    "                  red, red_pct, black, black_pct, other, loc, attempts);\n",
    "\n",
    "        if red > 50 && black > 500 {\n",
    "            eprintln!(\"│  │  │      ✅ Clock window detected with pixels\");\n",
    "            return Ok(());\n",
    "        }\n",
    "\n",
    "        tokio::time::sleep(std::time::Duration::from_secs(5)).await;\n",
    "    }\n",
    "\n",
    "    Err(StepError(format!(\"Clock window pixels not detected within {}s\", timeout_secs)))\n",
    "}\n"
]

idx_visible_start = -1
idx_visible_end = -1
idx_impl_start = -1
idx_impl_end = -1
idx_given_start = -1
idx_given_end = -1

for i, line in enumerate(lines):
    if '#[then("the clock window should be visible")]' in line:
        idx_visible_start = i
        idx_visible_end = find_func_end(lines, i)
    if '#[then("I should see a clock window displaying a ticking clock")]' in line:
        idx_impl_start = i
        idx_impl_end = find_func_end(lines, i)
    if 'async fn given_clock_ticking' in line:
        idx_given_start = i
        if i > 0 and '///' in lines[i-1]:
            idx_given_start = i - 1
        if i > 0 and '#[given' in lines[i-1]:
            idx_given_start = i - 1
            if i > 1 and '///' in lines[i-2]:
                idx_given_start = i - 2
        idx_given_end = find_func_end(lines, i)

new_visible_func = [
    "#[then(\"the clock window should be visible\")]\n",
    "async fn clock_window_visible(world: &mut ThingOsWorld) -> Result<(), StepError> {\n",
    "    wait_for_clock_pixels(world, 120.0).await\n",
    "}\n"
]

new_impl_func = [
    "#[then(\"I should see a clock window displaying a ticking clock\")]\n",
    "async fn clock_window_impl(world: &mut ThingOsWorld) -> Result<(), StepError> {\n",
    "    eprintln!(\"│  │  │      🕐 Checking for clock window...\");\n",
    "    \n",
    "    // First wait for clock app to log that it's publishing\n",
    "    let clock_ready = world.wait_for_serial(\"CLOCK PUBLISH:\", 30.0).await;\n",
    "    \n",
    "    if !clock_ready {\n",
    "        capture_failure_diagnostics(world, \"clock publish\").await;\n",
    "        return Err(StepError(\"Clock app did not publish within timeout\".to_string()));\n",
    "    }\n",
    "    \n",
    "    eprintln!(\"│  │  │      ✅ Clock app publishing\");\n",
    "    \n",
    "    // Use robust pixel check\n",
    "    wait_for_clock_pixels(world, 120.0).await\n",
    "}\n"
]

new_given_func = [
    "/// Given steps for keyboard/pointer scenarios - boot machine if needed\n",
    "#[given(\"the clock window is ticking\")]\n",
    "async fn given_clock_ticking(world: &mut ThingOsWorld) -> Result<(), StepError> {\n",
    "    // Boot if not already running\n",
    "    if world.qemu.is_none() {\n",
    "        let arch = std::env::var(\"BDD_ARCH\").unwrap_or_else(|_| \"x86_64\".to_string());\n",
    "        world.boot(&arch).await.map_err(|e| StepError(format!(\"Failed to boot QEMU: {}\", e)))?;\n",
    "    }\n",
    "    \n",
    "    // Wait for system ready\n",
    "    let found = world.wait_for_serial(\"[CONTRACT]\", 120.0).await;\n",
    "    if !found {\n",
    "        return Err(StepError(\"System did not reach ready state\".to_string()));\n",
    "    }\n",
    "    \n",
    "    // Wait for actual clock pixels\n",
    "    // The user suggested waiting \"a good minute\"\n",
    "    wait_for_clock_pixels(world, 150.0).await\n",
    "}\n"
]

new_lines = lines[:idx_helper_start] + new_helper_impl + new_helper + lines[idx_helper_end:idx_visible_start] + new_visible_func + lines[idx_visible_end:idx_impl_start] + new_impl_func + lines[idx_impl_end:idx_given_start] + new_given_func + lines[idx_given_end:]

with open(path, 'w') as f:
    f.writelines(new_lines)
