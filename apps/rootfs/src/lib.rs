#![no_std]

extern crate alloc;

use abi::{ThingId, graph_kinds};
use alloc::vec::Vec;
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
        if !should_seed_image(&existing_programs, image) {
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
                        format_args!(
                            "rootfs: created BootProgram {} but failed to add link",
                            image.identifier
                        ),
                    );
                }
            }
            None => {
                log_dynamic(
                    sys,
                    format_args!(
                        "rootfs: failed to create BootProgram for {}",
                        image.identifier
                    ),
                );
            }
        }
    }

    log_dynamic(
        sys,
        format_args!("rootfs: seeded {} BootProgram entries", added),
    );
    sys.exit_thread();
}

fn ensure_boot_profile<S: Sys>(sys: &mut S) -> ThingId {
    let profiles: Vec<BootProfile> = list_things_by_kind(sys);
    match profiles.as_slice() {
        [profile] => {
            log_dynamic(
                sys,
                format_args!("rootfs: using existing BootProfile id={}", profile.id.0),
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
                    log_dynamic(sys, format_args!("rootfs: created BootProfile id={}", id.0));
                    id
                }
                None => fatal(sys, "rootfs: failed to create BootProfile"),
            }
        }
        many => {
            log_dynamic(
                sys,
                format_args!(
                    "rootfs: multiple BootProfiles found ({}); picking first",
                    many.len()
                ),
            );
            many[0].id
        }
    }
}

fn fatal<S: Sys>(sys: &mut S, msg: &str) -> ! {
    log_dynamic(sys, format_args!("{}", msg));
    loop {
        sys.sleep_for_ns(100_000_000);
    }
}

fn should_seed_image(existing: &[BootProgram], image: &ProgramImage) -> bool {
    if image.identifier == "init" {
        return false;
    }
    if image.identifier == ROOTFS_IDENTIFIER {
        return false;
    }
    !existing.iter().any(|bp| bp.binary == image.identifier)
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::{KernelRequest, KernelResponse, PropKey, PropValue, Thing, ThingId};
    use alloc::vec::Vec;
    use userland_std::doc_helpers::DocSys;

    fn thing_props<T: Thing>(thing: &T) -> &'static [Option<(PropKey, PropValue)>] {
        let mut props = Vec::new();
        thing.to_props(&mut props);
        DocSys::props_slice(props)
    }

    #[test]
    fn ensure_boot_profile_creates_when_missing() {
        let mut responses = vec![
            KernelResponse::ThingListEntry { id: None },
            KernelResponse::ThingCreated { id: ThingId(88) },
        ];
        responses.push(KernelResponse::Success { data: None });
        let mut sys = DocSys::with_responses(responses);
        assert_eq!(ensure_boot_profile(&mut sys), ThingId(88));
        let requests = sys.requests.borrow();
        assert!(requests.iter().any(|request| matches!(
            request,
            KernelRequest::ThingCreate { kind, .. } if *kind == BootProfile::KIND
        )));
    }

    #[test]
    fn ensure_boot_profile_prefers_existing() {
        let profile = BootProfile {
            id: ThingId(5),
            version: 2,
        };
        let mut responses = Vec::new();
        responses.push(KernelResponse::ThingListEntry {
            id: Some(profile.id),
        });
        responses.push(KernelResponse::ThingData {
            id: profile.id,
            kind: BootProfile::KIND,
            props: thing_props(&profile),
        });
        responses.push(KernelResponse::ThingListEntry { id: None });
        responses.push(KernelResponse::Success { data: None });
        let mut sys = DocSys::with_responses(responses);
        assert_eq!(ensure_boot_profile(&mut sys), profile.id);
    }

    #[test]
    fn should_seed_image_filters_init_and_duplicates() {
        let existing = vec![BootProgram {
            id: ThingId(1),
            name: "demo".to_string(),
            app_id: 0,
            priority: 0,
            binary: "demo".to_string(),
        }];
        let image = ProgramImage {
            id: ThingId(2),
            identifier: "demo".to_string(),
            module_index: 0,
            base_phys: 0,
            size: 0,
        };
        assert!(!should_seed_image(&existing, &image));

        let init_image = ProgramImage {
            id: ThingId(3),
            identifier: "init".to_string(),
            module_index: 1,
            base_phys: 0,
            size: 0,
        };
        assert!(!should_seed_image(&[], &init_image));

        let rootfs_image = ProgramImage {
            id: ThingId(4),
            identifier: ROOTFS_IDENTIFIER.to_string(),
            module_index: 2,
            base_phys: 0,
            size: 0,
        };
        assert!(!should_seed_image(&[], &rootfs_image));

        let new_image = ProgramImage {
            id: ThingId(5),
            identifier: "new".to_string(),
            module_index: 3,
            base_phys: 0,
            size: 0,
        };
        assert!(should_seed_image(&[], &new_image));
    }
}
