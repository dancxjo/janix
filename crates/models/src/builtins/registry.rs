use crate::Thing;
use alloc::vec::Vec;

pub fn builtin_seed_things() -> Vec<Thing> {
    let mut v = alloc::vec![
        // 1. Meta-kinds (manual)
        crate::builtins::kinds::builtin_kind_kind(),
        crate::builtins::kinds::builtin_schema_kind(),
        // Schemas for Meta-Kinds
        crate::builtins::kinds::builtin_kind_schema(),
        crate::builtins::kinds::builtin_schema_schema(),
    ];

    // 2. Meta-kinds (macro) - Link, Intent, Observation, Result
    v.extend(crate::builtins::kinds::seed_link_kind());
    v.extend(crate::builtins::kinds::seed_intent_kind());
    v.extend(crate::builtins::kinds::seed_observation_kind());
    v.extend(crate::builtins::kinds::seed_result_kind());

    // 3. Predicate Kinds (this task)
    v.extend(crate::builtins::predicates::seed_owns_kind());
    v.extend(crate::builtins::predicates::seed_has_cap_kind());
    v.extend(crate::builtins::predicates::seed_has_schema_kind());
    v.extend(crate::builtins::predicates::seed_mounts_kind());
    v.extend(crate::builtins::predicates::seed_backed_by_kind());
    v.extend(crate::builtins::predicates::seed_launches_kind());

    // 4. Core Kinds (this task)
    v.extend(crate::builtins::core_kinds::seed_time_now_kind());
    v.extend(crate::builtins::core_kinds::seed_process_kind());
    v.extend(crate::builtins::core_kinds::seed_thread_kind());
    v.extend(crate::builtins::core_kinds::seed_capability_kind());
    v.extend(crate::builtins::core_kinds::seed_graph_kind());
    v.extend(crate::builtins::core_kinds::seed_mount_kind());
    v.extend(crate::builtins::core_kinds::seed_graph_provider_kind());
    v.extend(crate::builtins::core_kinds::seed_buffer_kind());
    v.extend(crate::builtins::core_kinds::seed_stream_kind());
    v.extend(crate::builtins::core_kinds::seed_keyboard_kind());
    v.extend(crate::builtins::core_kinds::seed_key_event_kind());
    v.extend(crate::builtins::core_kinds::seed_boot_program_kind());

    // Missing Core Kinds
    v.extend(crate::builtins::core_kinds::seed_module_kind());
    v.extend(crate::builtins::core_kinds::seed_program_image_kind());
    v.extend(crate::builtins::core_kinds::seed_bitmap_kind());
    v.extend(crate::builtins::core_kinds::seed_font_kind());
    v.extend(crate::builtins::core_kinds::seed_key_event_stream_kind());

    // Missing Predicate Kinds
    v.extend(crate::builtins::predicates::seed_has_module_kind());
    v.extend(crate::builtins::predicates::seed_binary_image_kind());
    v.extend(crate::builtins::predicates::seed_asset_kind());
    v.extend(crate::builtins::predicates::seed_provides_font_kind());
    v.extend(crate::builtins::predicates::seed_default_font_kind());
    v.extend(crate::builtins::predicates::seed_uses_module_kind());
    v.extend(crate::builtins::predicates::seed_has_device_kind());
    v.extend(crate::builtins::predicates::seed_spawned_kind());
    v.extend(crate::builtins::predicates::seed_runs_kind());

    // New Devices & GraphFS
    v.extend(crate::builtins::core_kinds::seed_pci_device_kind());
    v.extend(crate::builtins::core_kinds::seed_serial_port_kind());
    v.extend(crate::builtins::core_kinds::seed_log_stream_kind());
    v.extend(crate::builtins::core_kinds::seed_block_device_kind());
    v.extend(crate::builtins::core_kinds::seed_file_system_kind());
    v.extend(crate::builtins::core_kinds::seed_file_kind());
    v.extend(crate::builtins::core_kinds::seed_dir_kind());
    v.extend(crate::builtins::core_kinds::seed_volume_kind());

    // New Diag
    v.extend(crate::builtins::core_kinds::seed_log_entry_kind());
    v.extend(crate::builtins::core_kinds::seed_error_kind());
    v.extend(crate::builtins::core_kinds::seed_fault_kind());

    // New Predicates
    v.extend(crate::builtins::predicates::seed_has_block_device_kind());
    v.extend(crate::builtins::predicates::seed_has_volume_kind());
    v.extend(crate::builtins::predicates::seed_has_mount_kind());
    v.extend(crate::builtins::predicates::seed_has_entry_kind());
    v.extend(crate::builtins::predicates::seed_backed_by_device_kind());
    v.extend(crate::builtins::predicates::seed_on_volume_kind());
    v.extend(crate::builtins::predicates::seed_is_mounted_on_kind());
    v.extend(crate::builtins::predicates::seed_contains_file_kind());
    v.extend(crate::builtins::predicates::seed_has_keyboard_kind());
    v.extend(crate::builtins::predicates::seed_has_console_kind());
    v.extend(crate::builtins::predicates::seed_has_time_now_kind());

    v
}
