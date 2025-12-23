use abi::{ThingId, Predicate, syscall_defs::SymbolId};
use thing_models::PropValue;

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
    // 1.2 Centralized Enforcement: Fail Closed.
    // For now, we deny all user mutations until we implement specific allow-lists.
    // In the future, we will check:
    // - Symbol resolution
    // - Package ownership (pkg.*)
    // - Instance ownership (package_id property)
    match mutation {
        Mutation::CreateThing { kind, .. } => {
            let kind_name = crate::symbols::resolve(*kind).ok_or("Unresolved kind")?;
            if kind_name.starts_with("pkg.") || kind_name.starts_with("input.") {
                 Ok(())
            } else {
                 Err("User CreateThing denied: only pkg.* allowed")
            }
        },
        Mutation::UpdateThing { props, .. } => {
            // Check for restricted properties
            for (key, _) in *props {
                let key_name = crate::symbols::resolve(*key).ok_or("Unresolved property key")?;
                if key_name == "active_buffer_index" {
                    return Err("User UpdateThing denied: active_buffer_index is kernel-managed");
                }
            }
            Ok(())
        },
        Mutation::AddLink { .. } => Ok(()),
        Mutation::RemoveLink { .. } => Ok(()),
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
