use crate::bridge::ps2;
use crate::graph::{self, security};
use crate::symbols;
use thing_models::PropValue;
use alloc::vec;
use abi::ThingId;

pub fn run() -> ! {
    let mut last_intent_id = ThingId(0);
    let mut loops: u64 = 0;
    let mut intents_processed: u64 = 0;
    let mut last_log_time = crate::time::monotonic_now_ns();

    loop {
        loops += 1;
        let now = crate::time::monotonic_now_ns();
        if now - last_log_time > 1_000_000_000 {
            crate::log(&alloc::format!("Actualizer Heartbeat: loops={} intents={}", loops, intents_processed));
            last_log_time = now;
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

        // Poll PresentIntents
        // Only process new ones (ascending ID order)
        let intent_kind = symbols::intern("pkg.framebuffer.PresentIntent");
        while let Some(intent_id) = graph::next_thing_of_kind(intent_kind, last_intent_id) {
             last_intent_id = intent_id;
             worked = true;
             intents_processed += 1;
             process_present_intent(intent_id);
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

fn process_present_intent(id: ThingId) {
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
            let _ = graph::apply_mutation(security::Actor::Kernel, mutation);
            
            crate::log("actualizer: present processed");
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
