use crate::graph::GraphStore;

pub fn seed_builtins(store: &mut GraphStore) {
    crate::klog!(1, "SEED: Generating things...");
    let things = thing_models::builtins::builtin_seed_things();
    crate::klog!(1, "SEED: Generated things.", things.len());

    for (i, t) in things.into_iter().enumerate() {
        crate::klog!(1, "SEED: Inserting item", i);
        store.insert_seed(t);
    }

    // Seed Time Singleton
    // We construct it manually here because it's a runtime singleton instance, not a schema/kind.
    use thing_models::builtins::ids::{THING_TIME_INSTANCE, THING_TIME_NOW_KIND};
    use thing_models::core::time::TimeNow;
    use thing_models::value::ThingBody;
    use thing_models::Thing;
    use abi::wire::typed::{TypedBytes, TypeId, CodecId};

    let time_body = TimeNow {
        system_ns: 0,
        monotonic_ns: 0,
    };

    // We need to wrap it in TypedBytes -> ThingBody because that's what the store expects (roughly)
    // Actually, ThingBody is an enum or struct wrapper. Let's check ThingBody usage.
    // In main.rs: "postcard::to_allocvec(&root_body)... TypedBytes ... ThingBody::from(&typed_root)"
    
    let body_bytes = postcard::to_allocvec(&time_body).expect("Failed to serialize TimeNow");
    let typed_body = TypedBytes {
         type_id: TypeId(THING_TIME_NOW_KIND.0 as u128),
         codec_id: CodecId::POSTCARD,
         bytes: body_bytes,
    };

    if let Ok(tb) = ThingBody::from(&typed_body) {
        let thing = Thing {
            id: THING_TIME_INSTANCE,
            kind: THING_TIME_NOW_KIND,
            body: tb,
        };
        store.insert_seed(thing);
    }
}
