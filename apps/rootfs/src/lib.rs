#![no_std]

extern crate alloc;

use abi::{ThingId, graph_kinds};
use alloc::{format, vec::Vec};
use thing_models::{BootProfile, BootProgram, ProgramImage};
use userland::prelude::*;
use userland_std::add_edge;

const ROOTFS_IDENTIFIER: &str = "rootfs";

pub fn run<S: Sys>(sys: &mut S) -> ! {
    println(sys, "rootfs: starting");

    let profile_id = ensure_boot_profile(sys);
    let mut existing_programs: Vec<BootProgram> = list_things_by_kind(sys);
    let mut images: Vec<ProgramImage> = list_things_by_kind(sys);
    images.sort_by(|a, b| a.module_index.cmp(&b.module_index));

    let mut next_app_id = existing_programs
        .iter()
        .map(|bp| bp.app_id)
        .max()
        .unwrap_or(0)
        .saturating_add(1);

    let mut added = 0;
    for image in images.iter() {
        if image.identifier == "init" {
            continue;
        }

        if image.identifier == ROOTFS_IDENTIFIER {
            continue;
        }

        if existing_programs
            .iter()
            .any(|bp| bp.binary == image.identifier)
        {
            continue;
        }

        let program = BootProgram {
            id: ThingId(0),
            name: image.identifier.clone(),
            app_id: next_app_id,
            priority: 0,
            binary: image.identifier.clone(),
        };

        match create_thing(sys, &program) {
            Some(program_id) => {
                if add_edge(sys, profile_id, graph_kinds::EDGE_LAUNCHES, program_id) {
                    let mut recorded = program;
                    recorded.id = program_id;
                    existing_programs.push(recorded);
                    next_app_id = next_app_id.saturating_add(1);
                    added += 1;
                } else {
                    log_dynamic(
                        sys,
                        format!(
                            "rootfs: created BootProgram {} but failed to add edge",
                            image.identifier
                        ),
                    );
                }
            }
            None => {
                log_dynamic(
                    sys,
                    format!(
                        "rootfs: failed to create BootProgram for {}",
                        image.identifier
                    ),
                );
            }
        }
    }

    log_dynamic(sys, format!("rootfs: seeded {} BootProgram entries", added));
    sys.exit_thread();
}

fn ensure_boot_profile<S: Sys>(sys: &mut S) -> ThingId {
    let profiles: Vec<BootProfile> = list_things_by_kind(sys);
    match profiles.as_slice() {
        [profile] => {
            log_dynamic(
                sys,
                format!("rootfs: using existing BootProfile id={}", profile.id.0),
            );
            profile.id
        }
        [] => {
            let profile = BootProfile {
                id: ThingId(0),
                version: 1,
            };
            match create_thing(sys, &profile) {
                Some(id) => {
                    log_dynamic(sys, format!("rootfs: created BootProfile id={}", id.0));
                    id
                }
                None => fatal(sys, "rootfs: failed to create BootProfile"),
            }
        }
        many => {
            log_dynamic(
                sys,
                format!(
                    "rootfs: multiple BootProfiles found ({}); picking first",
                    many.len()
                ),
            );
            many[0].id
        }
    }
}

fn fatal<S: Sys>(sys: &mut S, msg: &str) -> ! {
    log_dynamic(sys, msg.into());
    loop {
        sys.sleep_for_ns(100_000_000);
    }
}
