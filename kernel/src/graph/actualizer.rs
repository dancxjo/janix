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
            // Heartbeat
            crate::log(&alloc::format!(
                "Actualizer Heartbeat: loops={} pending={} seen={} done={} err={} last={}",
                loops,
                intents_pending,
                intents_seen,
                intents_done,
                intents_error,
                last_processed.0
            ));
            
            // Debug sample
            let intent_kind = symbols::intern("pkg.framebuffer.PresentIntent");
            let mut cursor = ThingId(0);
            let mut count = 0;
            // Just verify first few intents existence to debug graph iteration
            while let Some(id) = graph::next_thing_of_kind(intent_kind, cursor) {
                cursor = id;
                count += 1;
                if count <= 3 {
                     let state = match graph::get_prop(id, "state") {
                        Some(PropValue::U64(v)) => v,
                        Some(PropValue::I64(v)) => v as u64,
                        _ => 999,
                    };
                    crate::log(&alloc::format!("Sample Intent: {} state={}", id.0, state));
                }
                if count > 10 { break; }
            }

            last_log_time = now;
        }

        let mut worked = false;

        // Poll Input
        while let Some(byte) = ps2::pop_keyboard_byte() {
            worked = true;
            let kind = symbols::intern("input.keyboard");
            let props = vec![(symbols::intern("scancode"), PropValue::U64(byte as u64))];
            let mutation = security::Mutation::CreateThing { kind, props: &props };
            let _ = graph::apply_mutation(security::Actor::Kernel, mutation);
        }
        while let Some(byte) = ps2::pop_mouse_byte() {
             worked = true;
            let kind = symbols::intern("input.mouse");
            let props = vec![(symbols::intern("byte"), PropValue::U64(byte as u64))];
            let mutation = security::Mutation::CreateThing { kind, props: &props };
            let _ = graph::apply_mutation(security::Actor::Kernel, mutation);
        }

        // Scan for Pending PresentIntents
        // We collect IDs first to avoid holding any iteration locks longer than needed, 
        // though next_thing_of_kind is safe.
        let intent_kind = symbols::intern("pkg.framebuffer.PresentIntent");
        let mut cursor = ThingId(0);
        let mut pending_batch: Vec<ThingId> = Vec::new(); // Limit batch size
        
        while let Some(intent_id) = graph::next_thing_of_kind(intent_kind, cursor) {
            cursor = intent_id;
            intents_seen = intents_seen.saturating_add(1);
            
            // Check state
            if let Some(state_val) = graph::get_prop(intent_id, "state") {
                 let is_pending = match state_val {
                     PropValue::U64(0) => true,
                     PropValue::I64(0) => true, // Be permissive with number types
                     _ => false,
                 };
                 if is_pending {
                     pending_batch.push(intent_id);
                     if pending_batch.len() >= 10 { break; }
                 }
            }
        }

        intents_pending = pending_batch.len() as u64;

        if !pending_batch.is_empty() {
             worked = true;
             for intent_id in pending_batch {
                 match process_present_intent(intent_id) {
                     Ok(_) => {
                         intents_done += 1;
                         last_processed = intent_id;
                     }
                     Err(e) => {
                         intents_error += 1;
                         crate::log(&alloc::format!("Actualizer Error {}: {}", intent_id.0, e));
                     }
                 }
             }
        }

        if !worked {
            // Sleep briefly to yield CPU
            let sleep_ns = 2_000_000; // 2ms
            let wake_at = crate::time::monotonic_now_ns() + sleep_ns;
            crate::sched::with_scheduler(|sched| {
                sched.sleep_current_thread(wake_at);
            });
            // We must force a context switch now that we marked ourselves sleeping
            // The timer interrupt will eventually switch us, but calling yield_now or waiting for interrupt is better.
            // Since we are in kernel, wait_for_interrupt is okay IF interrupts are enabled.
            // But sleep_current_thread deschedules us. The next interrupt will call helper and pick someone else.
            // So we just need to wait for that interrupt.
            crate::sched::wait_for_interrupt();
        } else {
             // If we did work, just yield to be nice to others
             crate::sched::with_scheduler(|sched| {
                 if let Some(tid) = sched.current_id() {
                     sched.mark_yield(tid);
                 }
             });
             // We need to trigger a switch. in x86 usually via interrupt or explicit call.
             // For now relies on timer tick which is frequent enough.
        }
    }
}

fn process_present_intent(id: ThingId) -> Result<(), &'static str> {
    // 1. Validate (Optimistic, we just checked pending)
    let buf_idx = match graph::get_prop(id, "buffer_index") {
        Some(PropValue::U64(v)) => v,
        Some(PropValue::I64(v)) => v as u64,
        _ => return Err("missing/invalid buffer_index"),
    };

    // 2. Actuate Hardware
    update_display_active_buffer(buf_idx);

    // 3. Mark Done
    let state_sym = symbols::intern("state");
    // 1 = Done
    let props = vec![(state_sym, PropValue::U64(1))];
    let mutation = security::Mutation::UpdateThing { id, props: &props };
    
    match graph::apply_mutation(security::Actor::Kernel, mutation) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
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

        // Manual process
        for id in ids.iter() {
           let res = process_present_intent(*id);
           assert!(res.is_ok());
           let val = graph::get_prop(*id, "state");
           assert!(matches!(val, Some(PropValue::U64(1))));
        }
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
