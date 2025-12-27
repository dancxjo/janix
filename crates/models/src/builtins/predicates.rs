use crate::predicate_kind;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PredicateBody {}

predicate_kind! {
    kind Owns {
        id: crate::builtins::ids::THING_OWNS_KIND,
        sym: crate::builtins::symbols::SYM_OWNS,
        version: 1,
        schema_id: crate::builtins::ids::THING_OWNS_SCHEMA
    }
}

predicate_kind! {
    kind HasCap {
        id: crate::builtins::ids::THING_HAS_CAP_KIND,
        sym: crate::builtins::symbols::SYM_HAS_CAP,
        version: 1,
        schema_id: crate::builtins::ids::THING_HAS_CAP_SCHEMA
    }
}

predicate_kind! {
    kind HasSchema {
        id: crate::builtins::ids::THING_HAS_SCHEMA_KIND,
        sym: crate::builtins::symbols::SYM_HAS_SCHEMA,
        version: 1,
        schema_id: crate::builtins::ids::THING_HAS_SCHEMA_SCHEMA
    }
}

predicate_kind! {
    kind Mounts {
        id: crate::builtins::ids::THING_MOUNTS_KIND,
        sym: crate::builtins::symbols::SYM_MOUNTS,
        version: 1,
        schema_id: crate::builtins::ids::THING_MOUNTS_SCHEMA
    }
}



predicate_kind! {
    kind BackedBy {
        id: crate::builtins::ids::THING_BACKED_BY_KIND,
        sym: crate::builtins::symbols::SYM_BACKED_BY,
        version: 1,
        schema_id: crate::builtins::ids::THING_BACKED_BY_SCHEMA
    }
}

predicate_kind! {
    kind Launches {
        id: crate::builtins::ids::THING_LAUNCHES_KIND,
        sym: crate::builtins::symbols::SYM_LAUNCHES,
        version: 1,
        schema_id: crate::builtins::ids::THING_LAUNCHES_SCHEMA
    }
}

predicate_kind! {
    kind HasModule {
        id: crate::builtins::ids::THING_HAS_MODULE_KIND,
        sym: crate::builtins::symbols::SYM_HAS_MODULE,
        version: 1,
        schema_id: crate::builtins::ids::THING_HAS_MODULE_SCHEMA
    }
}

predicate_kind! {
    kind BinaryImage {
        id: crate::builtins::ids::THING_BINARY_IMAGE_KIND,
        sym: crate::builtins::symbols::SYM_BINARY_IMAGE,
        version: 1,
        schema_id: crate::builtins::ids::THING_BINARY_IMAGE_SCHEMA
    }
}

predicate_kind! {
    kind Asset {
        id: crate::builtins::ids::THING_ASSET_KIND,
        sym: crate::builtins::symbols::SYM_ASSET,
        version: 1,
        schema_id: crate::builtins::ids::THING_ASSET_SCHEMA
    }
}

predicate_kind! {
    kind ProvidesFont {
        id: crate::builtins::ids::THING_PROVIDES_FONT_KIND,
        sym: crate::builtins::symbols::SYM_PROVIDES_FONT,
        version: 1,
        schema_id: crate::builtins::ids::THING_PROVIDES_FONT_SCHEMA
    }
}

predicate_kind! {
    kind DefaultFont {
        id: crate::builtins::ids::THING_DEFAULT_FONT_KIND,
        sym: crate::builtins::symbols::SYM_DEFAULT_FONT,
        version: 1,
        schema_id: crate::builtins::ids::THING_DEFAULT_FONT_SCHEMA
    }
}

predicate_kind! {
    kind UsesModule {
        id: crate::builtins::ids::THING_USES_MODULE_KIND,
        sym: crate::builtins::symbols::SYM_USES_MODULE,
        version: 1,
        schema_id: crate::builtins::ids::THING_USES_MODULE_SCHEMA
    }
}

predicate_kind! {
    kind HasDevice {
        id: crate::builtins::ids::THING_HAS_DEVICE_KIND,
        sym: crate::builtins::symbols::SYM_HAS_DEVICE,
        version: 1,
        schema_id: crate::builtins::ids::THING_HAS_DEVICE_SCHEMA
    }
}

predicate_kind! {
    kind Spawned {
        id: crate::builtins::ids::THING_SPAWNED_KIND,
        sym: crate::builtins::symbols::SYM_SPAWNED,
        version: 1,
        schema_id: crate::builtins::ids::THING_SPAWNED_SCHEMA
    }
}

predicate_kind! {
    kind Runs {
        id: crate::builtins::ids::THING_RUNS_KIND,
        sym: crate::builtins::symbols::SYM_RUNS,
        version: 1,
        schema_id: crate::builtins::ids::THING_RUNS_SCHEMA
    }
}
