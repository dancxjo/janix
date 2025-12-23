use crate::bridge::ps2;
use crate::graph::{self, security};
use crate::symbols;
use alloc::string::String;
use thing_models::PropValue;
use alloc::vec;
use abi::ThingId;
use alloc::vec::Vec;

pub fn run() -> ! {
    let mut loops: u64 = 0;
    let mut intents_seen: u64 = 0;
    let mut intents_pending: u64 = 0;
    let mut intents_done: u64 = 0;
    let mut intents_error: u64 = 0;
    let mut last_processed: ThingId = ThingId(0);
    let mut last_log_time = crate::time::monotonic_now_ns();

    loop {
        loops += 1;
        let now = crate::time::monotonic_now_ns();
        if now - last_log_time > 1_000_000_000 {
            crate::log(&alloc::format!(
                "Actualizer Heartbeat: loops={} pending={} seen={} done={} err={} last={}",
                loops,
                intents_pending,
                intents_seen,
                intents_done,
                intents_error,
                last_processed.0
            ));
            last_log_time = now;

            // Debug sample of up to 5 intents and their states
            let intent_kind = symbols::intern("pkg.framebuffer.PresentIntent");
            let mut cursor = ThingId(0);
            let mut sample: Vec<(ThingId, u64, u64)> = Vec::new();
            while sample.len() < 5 {
                if let Some(id) = graph::next_thing_of_kind(intent_kind, cursor) {
                    cursor = id;
                    let state = match graph::get_prop(id, "state") {
                        Some(PropValue::U64(v)) => v,
                        _ => 0,
                    };
                    let buf_idx = match graph::get_prop(id, "buffer_index") {
                        Some(PropValue::U64(v)) => v,
                        _ => 0,
                    };
                    sample.push((id, state, buf_idx));
                } else {
                    break;
                }
            }
            if !sample.is_empty() {
                let entries: Vec<String> = sample
                    .into_iter()
                    .map(|(id, state, buf)| alloc::format!("id={} state={} buf={}", id.0, state, buf))
                    .collect();
                crate::log(&alloc::format!(
                    "Actualizer Intent Sample: {}",
                    entries.join(", ")
                ));
            }
        }

        let mut worked = false;

        // Poll keyboard
        while let Some(byte) = ps2::pop_keyboard_byte() {
            worked = true;
            let kind = symbols::intern("input.keyboard");
            let props = vec![(symbols::intern("scancode"), PropValue::U64(byte as u64))];
            let mutation = security::Mutation::CreateThing { kind, props: &props };
            let _ = graph::apply_mutation(security::Actor::Kernel, mutation);
        }

        // Poll mouse
        while let Some(byte) = ps2::pop_mouse_byte() {
            worked = true;
            let kind = symbols::intern("input.mouse");
            let props = vec![(symbols::intern("byte"), PropValue::U64(byte as u64))];
            let mutation = security::Mutation::CreateThing { kind, props: &props };
            let _ = graph::apply_mutation(security::Actor::Kernel, mutation);
        }

        // Scan for PresentIntents in Pending state
        let intent_kind = symbols::intern("pkg.framebuffer.PresentIntent");
        let mut cursor = ThingId(0);
        let mut pending: Vec<ThingId> = Vec::new();
        while let Some(intent_id) = graph::next_thing_of_kind(intent_kind, cursor) {
            cursor = intent_id;
            intents_seen = intents_seen.saturating_add(1);
            if let Some(PropValue::U64(state)) = graph::get_prop(intent_id, "state") {
                if state == 0 {
                    pending.push(intent_id);
                }
            }
        }

        intents_pending = pending.len() as u64;
        if !pending.is_empty() {
            worked = true;
            for intent_id in pending {
                match process_present_intent(intent_id) {
                    Ok(()) => {
                        intents_done = intents_done.saturating_add(1);
                        last_processed = intent_id;
                    }
                    Err(e) => {
                        intents_error = intents_error.saturating_add(1);
                        crate::log(&alloc::format!(
                            "Actualizer: failed to process intent {} ({})",
                            intent_id.0,
                            e
                        ));
                    }
                }
            }
        }

        if !worked {
            // Sleep if both empty, atomically-ish
            crate::sched::without_preemption(|| {
                 let k = ps2::pop_keyboard_byte();
                 let m = ps2::pop_mouse_byte();
                 // We can't check graph efficiently inside this closure without locks which might deadlock if used wrong,
                 // but next_thing_of_kind handles store lock.
                 // For now, simpler sleep is fine. We might wake up a bit late for frame intents but better than spin.
                 
                 if k.is_none() && m.is_none() {
                         let tid = crate::sched::SCHEDULER.lock().current_id();
                         if let Some(tid) = tid {
                             ps2::set_keyboard_waiter(tid);
                             ps2::set_mouse_waiter(tid);
                             // crate::log("Actualizer blocking...");
                             // Sleep for 1ms (or until input wakes us)
                             let now = crate::time::monotonic_now_ns();
                             crate::sched::with_scheduler(|sched| {
                                 sched.sleep_current_thread(now + 1_000_000);
                             });
                             crate::sched::wait_for_interrupt();
                             // crate::log("Actualizer woke up");
                         }
                     } else {
                     // If input arrived, handle it next loop
                 }
            });
        }
    }
}

fn process_present_intent(id: ThingId) -> Result<(), &'static str> {
    // Check state is Pending (0)
    let state_val = graph::get_prop(id, "state");
    // "state" 0 = Pending
    if let Some(PropValue::U64(0)) = state_val {
        if let Some(PropValue::U64(buf_idx)) = graph::get_prop(id, "buffer_index") {
            // Update Display
            update_display_active_buffer(buf_idx);

            // Mark Done (1)
            let state_sym = symbols::intern("state");
            let props = vec![(state_sym, PropValue::U64(1))];
            let mutation = security::Mutation::UpdateThing { id, props: &props };
            match graph::apply_mutation(security::Actor::Kernel, mutation) {
                Ok(_) => {
                    crate::log(&alloc::format!("actualizer: present processed id={}", id.0));
                    Ok(())
                }
                Err(e) => Err(e),
            }
        } else {
            Err("missing buffer_index")
        }
    } else {
        Err("intent not pending")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph;
    use crate::graph::security;
    use crate::symbols;
    use thing_models::{PropType, PropValue};

    #[test]
    fn iterates_and_completes_present_intents() {
        let _guard = crate::test_lock();
        graph::store::init();
        symbols::init();
        graph::schema::init();
        graph::events::init();
        graph::index_props::init();

        let kind = symbols::intern("pkg.framebuffer.PresentIntent");
        let desc = symbols::intern("PresentIntent");
        let schema_props = alloc::vec![
            (symbols::intern("buffer_index"), PropType::U64),
            (symbols::intern("state"), PropType::U64),
        ];
        let _ = graph::schema::register_schema(kind, desc, schema_props, alloc::vec![]);

        let mut ids = alloc::vec![];
        for i in 0..3 {
            let props = alloc::vec![
                (symbols::intern("buffer_index"), PropValue::U64(i)),
                (symbols::intern("state"), PropValue::U64(0)),
            ];
            let mutation = security::Mutation::CreateThing { kind, props: &props };
            if let Ok(security::MutationResult::Created(id)) =
                graph::apply_mutation(security::Actor::Kernel, mutation)
            {
                ids.push(id);
            }
        }

        ids.sort_by_key(|id| id.0);

        let mut cursor = ThingId(0);
        let mut seen = alloc::vec![];
        while let Some(id) = graph::next_thing_of_kind(kind, cursor) {
            cursor = id;
            seen.push(id);
        }

        assert_eq!(seen.len(), ids.len());
        assert_eq!(seen, ids);

        let done_props = alloc::vec![(symbols::intern("state"), PropValue::U64(1))];
        let mutation = security::Mutation::UpdateThing { id: ids[0], props: &done_props };
        let res = graph::apply_mutation(security::Actor::Kernel, mutation);
        assert!(matches!(res, Ok(security::MutationResult::Updated(true))));
        assert!(matches!(graph::get_prop(ids[0], "state"), Some(PropValue::U64(1))));
    }
}

fn update_display_active_buffer(new_index: u64) {
    let display_kind = symbols::intern("display");
    let mut cursor = ThingId(0);
    if let Some(d_id) = graph::next_thing_of_kind(display_kind, cursor) {
         let idx_sym = symbols::intern("active_buffer_index");
         let props = vec![(idx_sym, PropValue::U64(new_index))];
         let mutation = security::Mutation::UpdateThing { id: d_id, props: &props };
         let _ = graph::apply_mutation(security::Actor::Kernel, mutation);
    }
}
