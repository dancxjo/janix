use abi::{Predicate, ThingId, syscall_defs::SymbolId};
use thing_models::PropValue;

use crate::{graph, graph_kinds, symbols};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Actor {
    Kernel,
    User(UserActor),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserActor {
    pub pid: u64,
    pub package_id: SymbolId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum KindClass {
    Witness,
    General,
}

fn classify_kind(kind: SymbolId) -> Result<KindClass, &'static str> {
    let Some(kind_name) = symbols::resolve(kind) else {
        return Err("Unresolved kind");
    };

    if is_witness_kind(&kind_name) {
        Ok(KindClass::Witness)
    } else {
        Ok(KindClass::General)
    }
}

fn is_witness_kind(kind_name: &str) -> bool {
    kind_name.starts_with("Witness.")
        || kind_name == graph_kinds::KIND_TIME_SOURCE
}

#[derive(Debug)]
pub enum Mutation<'a> {
    CreateThing {
        kind: SymbolId,
        props: &'a [(SymbolId, PropValue)],
    },
    UpdateThing {
        id: ThingId,
        props: &'a [(SymbolId, PropValue)],
    },
    AddLink {
        src: ThingId,
        pred: Predicate,
        dst: ThingId,
    },
    RemoveLink {
        src: ThingId,
        pred: Predicate,
        dst: ThingId,
    },
    DeclareSchema {
        kind: SymbolId,
        fingerprint: u64,
        definition: &'a [u8],
    }
}

pub fn check_policy(actor: &Actor, mutation: &Mutation) -> Result<(), &'static str> {
    match actor {
        Actor::Kernel => Ok(()),
        Actor::User(user) => check_user_policy(user, mutation),
    }
}

fn check_user_policy(_user: &UserActor, mutation: &Mutation) -> Result<(), &'static str> {
    match mutation {
        Mutation::CreateThing { kind, .. } => {
            match classify_kind(*kind)? {
                KindClass::Witness => {
                    Err("User CreateThing denied: Witness kinds are kernel-authored")
                }
                KindClass::General => Ok(()),
            }
        },
        Mutation::UpdateThing { props, .. } => {
            let kind = graph::get_thing_kind(props.get(0).map(|_| ThingId(0)).unwrap_or(ThingId(0))).unwrap_or(SymbolId(0));
            if let Some(kind) = graph::get_thing_kind(match mutation {
                Mutation::UpdateThing { id, .. } => *id,
                _ => ThingId(0),
            }) {
                if classify_kind(kind)? == KindClass::Witness {
                    return Err("User UpdateThing denied: Witness kinds are kernel-authored");
                }
            } else {
                return Err("User UpdateThing denied: unknown ThingId");
            }
            // Check for restricted properties
            for (key, _) in *props {
                let key_name = crate::symbols::resolve(*key).ok_or("Unresolved property key")?;
                if key_name == "active_buffer_index" {
                    return Err("User UpdateThing denied: active_buffer_index is kernel-managed");
                }
            }
            Ok(())
        },
        Mutation::AddLink { src, dst, .. } | Mutation::RemoveLink { src, dst, .. } => {
            if let Some(kind) = graph::get_thing_kind(*src) {
                if classify_kind(kind)? == KindClass::Witness {
                    return Err("User Link mutation denied: cannot target Witness things");
                }
            }
            if let Some(kind) = graph::get_thing_kind(*dst) {
                if classify_kind(kind)? == KindClass::Witness {
                    return Err("User Link mutation denied: cannot target Witness things");
                }
            }
            Ok(())
        },
        Mutation::DeclareSchema { kind, .. } => {
            // Relaxed check for now: just must start with pkg.
            // In a real system we match package_id.
            let kind_name = crate::symbols::resolve(*kind).ok_or("Unresolved schema kind")?;
            if !kind_name.starts_with("pkg.") {
                 return Err("Schema declaration denied: must start with pkg.");
            }
            Ok(())
        },
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MutationResult {
    Created(ThingId),
    Updated(bool),
    Linked(bool),
    Unlinked(bool),
    Declared,
    Nothing,
}
