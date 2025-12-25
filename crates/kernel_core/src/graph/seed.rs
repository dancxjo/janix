use crate::graph::GraphStore;
use thing_models::builtins::kinds::*;

pub fn seed_builtins(store: &mut GraphStore) {
    // 1. Meta-Kinds (Kind, Schema)
    // Order matters: Schema kind + Kind kind first.
    store.insert_seed(builtin_schema_kind());
    store.insert_seed(builtin_kind_kind());

    // 2. Schemas for Meta-Kinds
    store.insert_seed(builtin_schema_schema());
    store.insert_seed(builtin_kind_schema());

    // 3. Core Kinds
    store.insert_seed(builtin_link_kind());
    store.insert_seed(builtin_intent_kind());
    store.insert_seed(builtin_observation_kind());
    store.insert_seed(builtin_result_kind());

    // 4. Schemas for Core Kinds
    store.insert_seed(builtin_link_schema());
    store.insert_seed(builtin_intent_schema());
    store.insert_seed(builtin_observation_schema());
    store.insert_seed(builtin_result_schema());
}
