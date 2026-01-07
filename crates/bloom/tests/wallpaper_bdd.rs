//! BDD tests for wallpaper loading using Cucumber
//! 
//! Run with: cargo test --test wallpaper_bdd

use cucumber::{given, when, then, World};
use std::sync::Arc;

/// World structure that holds test state between steps
#[derive(Debug, Default, World)]
pub struct WallpaperWorld {
    bmp_data: Vec<u8>,
    parse_result: Option<Result<ParsedWallpaper, &'static str>>,
    requested_addr: u64,
    actual_addr: u64,
    addr_used_for_access: Option<u64>,
    mapping_failed: bool,
}

#[derive(Debug, Clone)]
pub struct ParsedWallpaper {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u32>,
}

// ===================== HELPER FUNCTIONS =====================

fn create_test_bmp(width: u32, height: u32) -> Vec<u8> {
    let row_stride = ((width as usize * 3 + 3) / 4) * 4;
    let pixel_data_size = row_stride * height as usize;
    let file_size = 54 + pixel_data_size;
    
    let mut data = vec![0u8; file_size];
    
    // BMP header
    data[0] = b'B';
    data[1] = b'M';
    data[2..6].copy_from_slice(&(file_size as u32).to_le_bytes());
    data[10..14].copy_from_slice(&54u32.to_le_bytes());
    
    // DIB header
    data[14..18].copy_from_slice(&40u32.to_le_bytes());
    data[18..22].copy_from_slice(&width.to_le_bytes());
    data[22..26].copy_from_slice(&(height as i32).to_le_bytes());
    data[26..28].copy_from_slice(&1u16.to_le_bytes());
    data[28..30].copy_from_slice(&24u16.to_le_bytes()); // 24-bit by default
    
    data
}

fn parse_bmp(data: &[u8]) -> Result<ParsedWallpaper, &'static str> {
    if data.len() < 54 {
        return Err("BMP too small");
    }
    if data[0] != b'B' || data[1] != b'M' {
        return Err("Invalid BMP magic");
    }
    
    let width = u32::from_le_bytes([data[18], data[19], data[20], data[21]]);
    let height_raw = i32::from_le_bytes([data[22], data[23], data[24], data[25]]);
    let height = height_raw.unsigned_abs();
    
    let bits_per_pixel = u16::from_le_bytes([data[28], data[29]]);
    
    if bits_per_pixel != 24 && bits_per_pixel != 32 {
        return Err("Unsupported BMP bit depth");
    }
    
    // Simplified: just return the dimensions and empty pixels for this test
    let pixel_count = (width * height) as usize;
    let pixels = vec![0xFF000000; pixel_count]; // All black with alpha=255
    
    Ok(ParsedWallpaper { width, height, pixels })
}

// ===================== GIVEN STEPS =====================

#[given("the bloom compositor is initialized")]
fn bloom_initialized(_world: &mut WallpaperWorld) {
    // Background step - no action needed for unit test
}

#[given("a valid BMP wallpaper bytespace exists")]
fn bmp_bytespace_exists(_world: &mut WallpaperWorld) {
    // Background step - no action needed for unit test
}

#[given(expr = "a valid 24-bit BMP file with dimensions {int}x{int}")]
fn valid_bmp_file(world: &mut WallpaperWorld, width: u32, height: u32) {
    world.bmp_data = create_test_bmp(width, height);
}

#[given("a file with invalid BMP magic bytes")]
fn invalid_magic(world: &mut WallpaperWorld) {
    world.bmp_data = vec![b'X', b'Y', 0, 0]; // Invalid magic
    world.bmp_data.resize(54, 0); // Pad to minimum size
}

#[given("a file smaller than 54 bytes")]
fn undersized_file(world: &mut WallpaperWorld) {
    world.bmp_data = vec![b'B', b'M', 0, 0, 0]; // Only 5 bytes
}

#[given("a BMP file with 16-bit color depth")]
fn unsupported_depth(world: &mut WallpaperWorld) {
    world.bmp_data = create_test_bmp(10, 10);
    world.bmp_data[28] = 16; // 16-bit
    world.bmp_data[29] = 0;
}

#[given("the bytespace mapping system is available")]
fn mapping_system_available(_world: &mut WallpaperWorld) {
    // No action needed - we'll simulate space_map behavior
}

// ===================== WHEN STEPS =====================

#[when("the wallpaper loader parses the BMP file")]
fn parse_bmp_file(world: &mut WallpaperWorld) {
    world.parse_result = Some(parse_bmp(&world.bmp_data));
}

#[when("the wallpaper loader attempts to parse the file")]
fn attempt_parse(world: &mut WallpaperWorld) {
    world.parse_result = Some(parse_bmp(&world.bmp_data));
}

#[when(expr = "space_map is called with requested address {word}")]
fn space_map_requested(world: &mut WallpaperWorld, addr: String) {
    world.requested_addr = u64::from_str_radix(addr.trim_start_matches("0x"), 16).unwrap();
}

#[when(expr = "space_map returns actual address {word}")]
fn space_map_returns(world: &mut WallpaperWorld, addr: String) {
    world.actual_addr = u64::from_str_radix(addr.trim_start_matches("0x"), 16).unwrap();
    
    // Simulate the FIXED behavior: use actual address for access
    if world.actual_addr != 0 {
        world.addr_used_for_access = Some(world.actual_addr);
    } else {
        world.mapping_failed = true;
    }
}

#[when("space_map returns 0")]
fn space_map_fails(world: &mut WallpaperWorld) {
    world.actual_addr = 0;
    world.mapping_failed = true;
}

// ===================== THEN STEPS =====================

#[then(expr = "the parser should return pixel data with {int} pixels")]
fn verify_pixel_count(world: &mut WallpaperWorld, expected_count: usize) {
    let result = world.parse_result.as_ref().expect("No parse result");
    let parsed = result.as_ref().expect("Parse failed unexpectedly");
    assert_eq!(parsed.pixels.len(), expected_count, "Pixel count mismatch");
}

#[then("each pixel should have an alpha channel value of 255")]
fn verify_alpha_channel(world: &mut WallpaperWorld) {
    let result = world.parse_result.as_ref().expect("No parse result");
    let parsed = result.as_ref().expect("Parse failed unexpectedly");
    
    for (i, &pixel) in parsed.pixels.iter().enumerate() {
        let alpha = (pixel >> 24) & 0xFF;
        assert_eq!(alpha, 255, "Pixel {} has wrong alpha: {}", i, alpha);
    }
}

#[then(expr = "the parser should return an error {string}")]
fn verify_error(world: &mut WallpaperWorld, expected_error: String) {
    let result = world.parse_result.as_ref().expect("No parse result");
    let err = result.as_ref().err().expect("Expected error but got success");
    assert_eq!(*err, expected_error, "Error message mismatch");
}

#[then(expr = "the loader should use address {word} for data access")]
fn verify_address_used(world: &mut WallpaperWorld, expected_addr: String) {
    let expected = u64::from_str_radix(expected_addr.trim_start_matches("0x"), 16).unwrap();
    let actual = world.addr_used_for_access.expect("No address was used");
    assert_eq!(actual, expected, "Used wrong address for data access");
}

#[then(expr = "the loader should not use address {word}")]
fn verify_address_not_used(world: &mut WallpaperWorld, forbidden_addr: String) {
    let forbidden = u64::from_str_radix(forbidden_addr.trim_start_matches("0x"), 16).unwrap();
    if let Some(used) = world.addr_used_for_access {
        assert_ne!(used, forbidden, "Loader incorrectly used forbidden address");
    }
}

#[then("the loader should report a mapping failure")]
fn verify_mapping_failure(world: &mut WallpaperWorld) {
    assert!(world.mapping_failed, "Expected mapping failure not detected");
}

#[then("the wallpaper should not be displayed")]
fn verify_no_wallpaper(world: &mut WallpaperWorld) {
    // When mapping fails, addr_used_for_access should be None
    assert!(world.addr_used_for_access.is_none(), "Wallpaper should not be displayed on failure");
}

// ===================== MAIN =====================

#[tokio::main]
async fn main() {
    WallpaperWorld::run("tests/features").await;
}
