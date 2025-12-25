use crate::graph::GraphStore;

pub fn seed_builtins(store: &mut GraphStore) {
    for t in thing_models::builtins::builtin_seed_things() {
        store.insert_seed(t);
    }
}
