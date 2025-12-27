use alloc::vec::Vec;
use crate::Thing;

pub fn builtin_seed_things() -> Vec<Thing> {
    let mut v = Vec::new();

    // 1. Meta-kinds (manual)
    v.push(crate::builtins::kinds::builtin_kind_kind());
    v.push(crate::builtins::kinds::builtin_schema_kind());

    // Schemas for Meta-Kinds
    v.push(crate::builtins::kinds::builtin_kind_schema());
    v.push(crate::builtins::kinds::builtin_schema_schema());

    // 2. Meta-kinds (macro) - Link, Intent, Observation, Result
    v.extend(crate::builtins::kinds::seed_link_kind().into_iter());
    v.extend(crate::builtins::kinds::seed_intent_kind().into_iter());
    v.extend(crate::builtins::kinds::seed_observation_kind().into_iter());
    v.extend(crate::builtins::kinds::seed_result_kind().into_iter());

    // 3. Predicate Kinds (this task)
    v.extend(crate::builtins::predicates::seed_owns_kind().into_iter());
    v.extend(crate::builtins::predicates::seed_has_cap_kind().into_iter());
    v.extend(crate::builtins::predicates::seed_has_schema_kind().into_iter());
    v.extend(crate::builtins::predicates::seed_mounts_kind().into_iter());
    v.extend(crate::builtins::predicates::seed_backed_by_kind().into_iter());
    v.extend(crate::builtins::predicates::seed_launches_kind().into_iter());

    // 4. Core Kinds (this task)
    v.extend(crate::builtins::core_kinds::seed_time_now_kind().into_iter());
    v.extend(crate::builtins::core_kinds::seed_process_kind().into_iter());
    v.extend(crate::builtins::core_kinds::seed_thread_kind().into_iter());
    v.extend(crate::builtins::core_kinds::seed_capability_kind().into_iter());
    v.extend(crate::builtins::core_kinds::seed_graph_kind().into_iter());
    v.extend(crate::builtins::core_kinds::seed_mount_kind().into_iter());
    v.extend(crate::builtins::core_kinds::seed_graph_provider_kind().into_iter());
    v.extend(crate::builtins::core_kinds::seed_buffer_kind().into_iter());
    v.extend(crate::builtins::core_kinds::seed_stream_kind().into_iter());
    v.extend(crate::builtins::core_kinds::seed_keyboard_kind().into_iter());
    v.extend(crate::builtins::core_kinds::seed_key_event_kind().into_iter());
    v.extend(crate::builtins::core_kinds::seed_boot_program_kind().into_iter());

    // Missing Core Kinds
    v.extend(crate::builtins::core_kinds::seed_module_kind().into_iter());
    v.extend(crate::builtins::core_kinds::seed_program_image_kind().into_iter());
    v.extend(crate::builtins::core_kinds::seed_bitmap_kind().into_iter());
    v.extend(crate::builtins::core_kinds::seed_font_kind().into_iter());
    v.extend(crate::builtins::core_kinds::seed_key_event_stream_kind().into_iter());

    // Missing Predicate Kinds
    v.extend(crate::builtins::predicates::seed_has_module_kind().into_iter());
    v.extend(crate::builtins::predicates::seed_binary_image_kind().into_iter());
    v.extend(crate::builtins::predicates::seed_asset_kind().into_iter());
    v.extend(crate::builtins::predicates::seed_provides_font_kind().into_iter());
    v.extend(crate::builtins::predicates::seed_default_font_kind().into_iter());
    v.extend(crate::builtins::predicates::seed_uses_module_kind().into_iter());

    v
}
