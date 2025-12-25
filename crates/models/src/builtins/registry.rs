use alloc::vec::Vec;
use crate::Thing;

pub fn builtin_seed_things() -> Vec<Thing> {
    let mut v = Vec::new();

    // Push meta-kinds first (manual for now)
    v.push(crate::builtins::kinds::builtin_kind_kind());
    v.push(crate::builtins::kinds::builtin_schema_kind());

    // Then macro-generated seeds:
    v.extend(crate::builtins::kinds::seed_link_kind().into_iter());
    v.extend(crate::builtins::kinds::seed_intent_kind().into_iter());
    v.extend(crate::builtins::kinds::seed_observation_kind().into_iter());
    v.extend(crate::builtins::kinds::seed_result_kind().into_iter());

    v
}
