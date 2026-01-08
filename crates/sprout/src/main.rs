#![no_std]
#![no_main]

extern crate alloc;
use alloc::{string::String, vec, vec::Vec};
use alloc::string::ToString;
use abi::ids::ThingId;
use models::{Service, Thing};
use thing_std::*;
use thing_std::cap::{grant, Cap, CapOp, CapScope};
use thing_std::graph::thing_get_body;

mod plan;
use plan::{build_launch_plan, SystemGraph, LaunchPlanItem};

struct LaunchPlan {
    #[allow(dead_code)]
    service_id: ThingId,
    name: String,
    module: Option<String>,
    deps: Vec<ThingId>,
    caps: Vec<CapOp>,
}

impl From<LaunchPlanItem> for LaunchPlan {
    fn from(item: LaunchPlanItem) -> Self {
        Self {
            service_id: item.service_id,
            name: item.name,
            module: item.module_name,
            deps: item.deps,
            caps: item.caps,
        }
    }
}

#[unsafe(no_mangle)]
pub fn main() {
    log_info("SPROUT: I am alive");

    log_info("SPROUT: building plan");
    let graph = SystemGraph;
    let plan_res = build_launch_plan(&graph);

    let plan: Vec<LaunchPlan> = match plan_res {
        Ok(items) => items.into_iter().map(LaunchPlan::from).collect(),
        Err(e) => {
            match e {
                plan::PlanError::GraphMissing(g) => log_info(&alloc::format!("SPROUT: graph missing {}", g)),
                plan::PlanError::CycleDetected(ids) => log_info(&alloc::format!("SPROUT: cycle detected in {:?}", ids)),
                plan::PlanError::DependencyMissing(dep, dep_on) => log_info(&alloc::format!("SPROUT: dependency missing {} -> {}", dep.low(), dep_on.low())),
            }
            Vec::new()
        }
    };

    log_info(&alloc::format!("SPROUT: launch plan size {}", plan.len()));

    if plan.is_empty() {
        log_info("SPROUT: plan empty; falling back to default module list");
        spawn_fallback();
        log_info("SPROUT: boot sequence complete.");
        return;
    }

    let mut started = vec![false; plan.len()];
    let mut remaining = plan.len();
    let mut stalled_loops = 0;

    while remaining > 0 {
        let mut progressed = false;

        for idx in 0..plan.len() {
            if started[idx] {
                continue;
            }

            if !deps_ready(&plan[idx].deps) {
                continue;
            }

            let module_name = plan[idx]
                .module
                .clone()
                .or_else(|| plan[idx].name.strip_prefix("service.").map(|s| s.to_string()));

            if let Some(name) = module_name {
                if !plan[idx].caps.is_empty() {
                    spawn_with_caps(&name, &plan[idx].caps);
                } else {
                    spawn_and_grant(&name);
                }
            } else {
                log_info(&alloc::format!(
                    "SPROUT: no module mapping for {}, skipping",
                    plan[idx].name
                ));
            }

            started[idx] = true;
            remaining -= 1;
            progressed = true;
        }

        if !progressed {
            stalled_loops += 1;
            if stalled_loops > 100 {
                log_info("SPROUT: launch plan stalled; remaining services not ready");
                break;
            }
            thing_std::time::sleep_ms(50);
        } else {
            stalled_loops = 0;
        }
    }

    log_info("SPROUT: boot sequence complete.");
}

fn deps_ready(deps: &[ThingId]) -> bool {
    deps.iter().all(|id| service_ready(*id))
}

fn service_ready(id: ThingId) -> bool {
    if let Some((body, _)) = thing_get_body(id) {
        if let Ok(svc) = Service::decode_full(&body) {
            return svc.state == 1;
        }
    }
    false
}


fn spawn_fallback() {
    // Spark first - ultra-fast SIMD display test
    spawn_and_grant("spark");
    // Bloom for full compositor
    spawn_and_grant("bloom");

    #[cfg(target_arch = "x86_64")]
    let rtc = Some("rtc_cmos");
    #[cfg(target_arch = "aarch64")]
    let rtc = Some("rtc_pl031");
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    let rtc = None;

    if let Some(r) = rtc {
        spawn_and_grant(r);
    }

    spawn_and_grant("timed");
    spawn_and_grant("textd");
    // spawn_and_grant("clock");
    spawn_and_grant("inputd");
    spawn_and_grant("graphviewer");
}


fn spawn_and_grant(name: &str) {
    match spawn(name) {
        Ok(id) => {
            // log_info(&alloc::format!("SPROUT: spawned {}, granting caps...", name));
            configure_policy(id, name);
        }
        Err(e) => {
            log_info(&alloc::format!("SPROUT: failed to spawn {}: error {}", name, e));
        }
    }
}

fn spawn_with_caps(name: &str, caps: &[CapOp]) {
    match spawn(name) {
        Ok(id) => {
            for op in caps {
                let _ = grant(
                    id,
                    Cap {
                        op: *op,
                        scope: CapScope::Global,
                    },
                );
            }
        }
        Err(e) => {
            log_info(&alloc::format!("SPROUT: failed to spawn {}: error {}", name, e));
        }
    }
}

fn configure_policy(id: ThingId, name: &str) {
    // Helper to grant global cap
    let global = |op| {
        let _ = grant(id, Cap { op, scope: CapScope::Global });
    };

    // Everyone gets Logging
    global(CapOp::Log);

    match name {
        "bloom" => {
            // Graphics pipeline needs massive permissions for now
            global(CapOp::MemManage); // Framebuffer mapping
            global(CapOp::GraphCreate); // Surfaces
            global(CapOp::GraphLink);
            global(CapOp::GraphUnlink);
            global(CapOp::GraphRead);
            global(CapOp::GraphWrite); // Update display body
            global(CapOp::GraphWatch); // Watch input
        }
        "inputd" => {
            // Driver needs hardware and memory
            global(CapOp::MemManage); // Ring buffer bytespace
            global(CapOp::Hardware);  // Port I/O
            global(CapOp::GraphCreate);
            global(CapOp::GraphLink);
            global(CapOp::GraphRead);
            global(CapOp::GraphWrite);
        }
        "clock" => {
            // Clock needs memory for heap! (Fixes 0x9000... crash)
            global(CapOp::MemManage);
            global(CapOp::GraphRead);
            global(CapOp::GraphCreate);
            global(CapOp::GraphLink);
            global(CapOp::GraphWrite);
        }
        "timed" => {
            // Needs heap
            global(CapOp::MemManage);
            // Needs to find rtc
            global(CapOp::GraphRead);
            // Needs to create system.time
            global(CapOp::GraphCreate);
            global(CapOp::GraphLink);
            global(CapOp::GraphWrite);
        }
        "thingcheck" => {
            // Inspector needs read access
            global(CapOp::GraphRead);
        }
        "rtc_cmos" => {
             global(CapOp::Log);
             global(CapOp::MemManage); // Usually needed for heap/buffers
             global(CapOp::GraphRead); // Find hw thing
             global(CapOp::GraphCreate); // Create device
             global(CapOp::GraphLink);
             global(CapOp::GraphWrite);
             global(CapOp::IoPort);      // The important bit!
        }
        "rtc_pl031" => {
             global(CapOp::Log);
             global(CapOp::GraphRead);
             global(CapOp::GraphCreate);
             global(CapOp::GraphLink);
             global(CapOp::GraphWrite);
             global(CapOp::Hardware); // MMIO needs Hardware cap? Or MemManage?
             // MMIO mapping via sys_space_map needs MemManage
             global(CapOp::MemManage);
        }
        "thread_test" => {
            // Thread test needs memory for heap and bytespace creation
            global(CapOp::MemManage);
            global(CapOp::GraphRead);
            global(CapOp::GraphCreate);
            global(CapOp::GraphLink);
        }
        "graphviewer" => {
            global(CapOp::MemManage); // Required for heap allocation
            global(CapOp::GraphCreate);
            global(CapOp::GraphLink);
            global(CapOp::GraphRead);
            global(CapOp::GraphWrite);
        }
        "textd" => {
            global(CapOp::MemManage);
            global(CapOp::GraphCreate);
            global(CapOp::GraphLink);
            global(CapOp::GraphRead);
            global(CapOp::GraphWrite);
            global(CapOp::GraphWatch);
        }
        "spark" => {
            global(CapOp::MemManage); // Framebuffer mapping
            global(CapOp::GraphRead); // Find display
        }
        _ => {}
    }
}
