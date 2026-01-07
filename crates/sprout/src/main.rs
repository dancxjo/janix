#![no_std]
#![no_main]

extern crate alloc;
use alloc::{string::String, vec, vec::Vec};
use alloc::string::ToString;
use abi::ids::ThingId;
use abi::types::{RelationshipRef, AlignedRelBuf};
use models::{Module, Service, Thing};
use thing_std::*;
use thing_std::cap::{grant, Cap, CapOp, CapScope};
use thing_std::graph::{
    relationships_from, symbol_resolve, thing_find, thing_get_body,
};

struct LaunchPlan {
    service_id: ThingId,
    name: String,
    module: Option<String>,
    deps: Vec<ThingId>,
    caps: Vec<CapOp>,
}

#[unsafe(no_mangle)]
pub fn main() {
    log_info("SPROUT: I am alive");

    log_info("SPROUT: building plan");
    let plan = build_plan();
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

fn build_plan() -> Vec<LaunchPlan> {
    // TODO: temporary bypass for broken relationships traversal; fallback boot will spawn core services.
    return Vec::new();

    let pred_contains = thing_std::graph::symbol_intern("predicate.contains");
    let pred_refs = thing_std::graph::symbol_intern("predicate.references");
    let pred_owns = thing_std::graph::symbol_intern("predicate.owns");

    let plan_graphs = [
        "graph.services",
        "graph.services.time",
        "graph.apps.clock",
        "graph.services.core",
    ];

    let mut plan: Vec<LaunchPlan> = Vec::new();

    for graph_name in plan_graphs {
        log_info(&alloc::format!("SPROUT: scan {}", graph_name));
        match thing_find(graph_name) {
            Some(graph_id) => {
                let rels = relationships_of(graph_id);
                log_info(&alloc::format!(
                    "SPROUT: {} has {} relationships",
                    graph_name,
                    rels.len()
                ));
                for rel in relationships_of(graph_id).into_iter().filter(|r| r.kind == pred_contains) {
                    if plan.iter().any(|p| p.service_id == rel.target) {
                        continue;
                    }

                    if let Some((body, digest)) = thing_get_body(rel.target) {
                        log_info(&alloc::format!(
                            "SPROUT: service body id={} len={} digest={}",
                            rel.target.low(),
                            body.len(),
                            digest
                        ));
                        if let Ok(svc) = Service::decode_full(&body) {
                            let mut deps = relationships_of(rel.target)
                                .into_iter()
                                .filter(|r| r.kind == pred_refs)
                                .map(|r| r.target)
                                .collect::<Vec<_>>();
                            log_info(&alloc::format!(
                                "SPROUT: deps found={}",
                                deps.len()
                            ));

                            let module = relationships_of(rel.target)
                                .into_iter()
                                .find(|r| r.kind == pred_owns)
                                .and_then(|r| module_info_from_id(r.target));

                            let name = symbol_resolve(svc.name)
                                .and_then(|bytes| String::from_utf8(bytes).ok())
                                .unwrap_or_else(|| "service.?".to_string());

                            if deps.is_empty() {
                                if let Some((_, module_deps, _)) = module.as_ref() {
                                    deps.extend_from_slice(module_deps);
                                }
                            }

                            plan.push(LaunchPlan {
                                service_id: rel.target,
                                name,
                                module: module.as_ref().map(|m| m.0.clone()),
                                deps,
                                caps: module.map(|(_, _, caps)| caps).unwrap_or_default(),
                            });
                        }
                    }
                }
            }
            None => {
                log_info(&alloc::format!("SPROUT: plan graph missing: {}", graph_name));
            }
        }
    }

    plan
}

fn spawn_fallback() {
    // Bloom first for instant feedback
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
    spawn_and_grant("clock");
    spawn_and_grant("inputd");
    spawn_and_grant("hello_window");
}

fn module_info_from_id(id: ThingId) -> Option<(String, Vec<ThingId>, Vec<CapOp>)> {
    if let Some((body, _)) = thing_get_body(id) {
        if let Ok(module) = Module::decode_full(&body) {
            let name = symbol_resolve(module.name)
                .and_then(|bytes| String::from_utf8(bytes).ok())?;

            let mut deps = Vec::new();
            for i in 0..module.dep_count.min(module.deps.len() as u8) {
                let sym = module.deps[i as usize];
                if let Some(bytes) = symbol_resolve(sym) {
                    if let Ok(s) = String::from_utf8(bytes) {
                        if let Some(id) = thing_find(&s) {
                            deps.push(id);
                        }
                    }
                }
            }

            let mut caps = Vec::new();
            for i in 0..module.cap_count.min(module.caps.len() as u8) {
                caps.push(module.caps[i as usize]);
            }

            return Some((name, deps, caps));
        }
    }
    None
}

fn relationships_of(from: ThingId) -> Vec<RelationshipRef> {
    let mut rels = Vec::new();
    let mut cursor = 0;
    log_info(&alloc::format!("SPROUT: relationships_of start from={}", from.low()));
    loop {
        let mut aligned = AlignedRelBuf::default();
        let buf = &mut aligned.inner;

        match relationships_from(from, cursor, buf) {
            Ok((returned, total)) => {
                log_info(&alloc::format!(
                    "SPROUT: rels chunk cursor={} returned={} total={}",
                    cursor,
                    returned,
                    total
                ));
                let count = returned as usize;
                rels.extend_from_slice(&buf[..count]);

                if returned == 0 || cursor + returned >= total {
                    break;
                }
                cursor += returned;
            }
            Err(e) => {
                log_info(&alloc::format!(
                    "SPROUT: relationships_from failed for {:?}: {}",
                    from,
                    e
                ));
                break;
            }
        }
    }

    rels
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
        "hello_window" => {
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
        _ => {}
    }
}
