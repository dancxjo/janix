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
