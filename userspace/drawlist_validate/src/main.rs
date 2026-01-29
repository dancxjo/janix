#![no_std]
#![no_main]

extern crate alloc;

use abi::drawlist::{DrawCmdTag, DrawListError, DrawListReader};
use abi::geometry::RectI32Wire;
use abi::schema::keys;
use alloc::vec::Vec;
use stem::info;
use stem::thing::sys::{bytespace_info, bytespace_read, find, prop_get};
use stem::thing::ThingId;
use abi::ids::HandleId;
use abi::schema::kinds;
use core::time::Duration;

/// Validation result for a single drawlist
#[derive(Debug)]
enum ValidationResult {
    Valid {
        thing_id: ThingId,
        cmd_count: u32,
        warnings: Vec<&'static str>,
    },
    Invalid {
        thing_id: ThingId,
        error: &'static str,
    },
}

/// Validate a drawlist's structure and properties
fn validate_drawlist(thing_id: ThingId) -> ValidationResult {
    let mut warnings = Vec::new();

    // Check if bytespace property exists
    let bs_id_val = match prop_get(thing_id, keys::UI_DRAWLIST_BYTESPACE) {
        Ok(val) if val != 0 => val,
        _ => {
            return ValidationResult::Invalid {
                thing_id,
                error: "No UI_DRAWLIST_BYTESPACE property",
            }
        }
    };

    let bs_id = ThingId::from_u64(bs_id_val);

    // Read the bytespace
    let size = match bytespace_info(bs_id) {
        Ok(s) => s,
        Err(_) => {
            return ValidationResult::Invalid {
                thing_id,
                error: "Failed to read bytespace info",
            }
        }
    };

    if size == 0 {
        return ValidationResult::Invalid {
            thing_id,
            error: "Empty bytespace",
        };
    }

    let mut buf = alloc::vec![0u8; size];
    if bytespace_read(bs_id, 0, &mut buf).is_err() {
        return ValidationResult::Invalid {
            thing_id,
            error: "Failed to read bytespace data",
        };
    }

    // Validate the drawlist format
    match DrawListReader::validate(&buf) {
        Ok(()) => {
            // Format is valid, now check semantic constraints
            let mut reader = DrawListReader::new(&buf).expect("validated");
            let mut cmd_count = 0u32;
            let mut save_depth = 0i32;

            while let Some(cmd) = reader.next() {
                cmd_count += 1;

                match cmd.tag {
                    DrawCmdTag::Save => save_depth += 1,
                    DrawCmdTag::Restore => {
                        save_depth -= 1;
                        if save_depth < 0 {
                            warnings.push("Unbalanced Restore (more restores than saves)");
                        }
                    }
                    DrawCmdTag::SetClipRect => {
                        if let Some((x, y, w, h)) =
                            abi::drawlist::decode_set_clip_rect(cmd.payload)
                        {
                            if w <= 0 || h <= 0 {
                                warnings.push("SetClipRect has non-positive dimensions");
                            }
                        }
                    }
                    DrawCmdTag::FillRect => {
                        if let Some((x, y, w, h, _color)) =
                            abi::drawlist::decode_fill_rect(cmd.payload)
                        {
                            if w <= 0 || h <= 0 {
                                warnings.push("FillRect has non-positive dimensions");
                            }
                        }
                    }
                    _ => {}
                }
            }

            if save_depth != 0 {
                warnings.push("Unbalanced Save/Restore stack at end");
            }

            // Check optional properties
            if let Ok(gen) = prop_get(thing_id, keys::UI_DRAWLIST_GEN) {
                if gen == 0 {
                    warnings.push("Generation counter is 0 (should start at 1)");
                }
            } else {
                warnings.push("No UI_DRAWLIST_GEN property");
            }

            ValidationResult::Valid {
                thing_id,
                cmd_count,
                warnings,
            }
        }
        Err(DrawListError::InvalidHeader) => ValidationResult::Invalid {
            thing_id,
            error: "Invalid drawlist header",
        },
        Err(DrawListError::TruncatedCommand) => ValidationResult::Invalid {
            thing_id,
            error: "Truncated command in drawlist",
        },
        Err(DrawListError::TrailingBytes) => ValidationResult::Invalid {
            thing_id,
            error: "Trailing bytes after drawlist",
        },
    }
}

fn find_and_validate_drawlists() {
    info!("DrawList Validator starting...");
    info!("Searching for drawlists in the graph...");

    let mut validated = 0u32;
    let mut valid = 0u32;
    let mut invalid = 0u32;

    // Find all windows (they typically have drawlists)
    let mut windows = [ThingId::default(); 32];
    if let Ok(count) = find(kinds::UI_WINDOW, &mut windows) {
        info!("Found {} windows to check", count);
        for i in 0..count {
            let window = windows[i];
            
            // Check if this window has a drawlist
            if prop_get(window, keys::UI_DRAWLIST_BYTESPACE).is_ok() {
                validated += 1;
                match validate_drawlist(window) {
                    ValidationResult::Valid {
                        thing_id,
                        cmd_count,
                        warnings,
                    } => {
                        valid += 1;
                        info!(
                            "✓ Window {:X}: VALID ({} commands)",
                            thing_id.to_u64_lossy(),
                            cmd_count
                        );
                        for warning in warnings {
                            info!("  ⚠ {}", warning);
                        }
                    }
                    ValidationResult::Invalid { thing_id, error } => {
                        invalid += 1;
                        info!("✗ Window {:X}: INVALID - {}", thing_id.to_u64_lossy(), error);
                    }
                }
            }
        }
    }

    info!("---");
    info!("Validation complete:");
    info!("  Drawlists validated: {}", validated);
    info!("  Valid: {}", valid);
    info!("  Invalid: {}", invalid);
}

#[stem::main]
fn main() -> ! {
    // Wait a bit for the system to stabilize
    stem::sleep(Duration::from_millis(500));

    find_and_validate_drawlists();

    // Run validation periodically
    let mut iteration = 0u32;
    loop {
        stem::sleep(Duration::from_secs(5));
        iteration += 1;
        info!("--- Iteration {} ---", iteration);
        find_and_validate_drawlists();
    }
}
