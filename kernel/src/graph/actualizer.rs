use crate::bridge::ps2;
use crate::graph::{self, security};
use crate::symbols;
use thing_models::PropValue;
use alloc::vec;

pub fn run() -> ! {
    loop {
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

        if !worked {
            // Sleep if both empty, atomically-ish
            crate::sched::without_preemption(|| {
                 let k = ps2::pop_keyboard_byte();
                 let m = ps2::pop_mouse_byte();

                 if k.is_none() && m.is_none() {
                     if let Some(tid) = crate::sched::SCHEDULER.lock().current_id() {
                         ps2::set_keyboard_waiter(tid);
                         ps2::set_mouse_waiter(tid);
                         crate::sched::block_current_thread();
                     }
                 } else {
                     if let Some(byte) = k {
                        let kind = symbols::intern("input.keyboard");
                         let props = vec![(symbols::intern("scancode"), PropValue::U64(byte as u64))];
                         let mutation = security::Mutation::CreateThing { kind, props: &props };
                         let _ = graph::apply_mutation(security::Actor::Kernel, mutation);
                     }
                     if let Some(byte) = m {
                        let kind = symbols::intern("input.mouse");
                         let props = vec![(symbols::intern("byte"), PropValue::U64(byte as u64))];
                         let mutation = security::Mutation::CreateThing { kind, props: &props };
                         let _ = graph::apply_mutation(security::Actor::Kernel, mutation);
                     }
                 }
            });
            // Schedule handled by block_current_thread() internal HLT/Yield logic
        }
    }
}
