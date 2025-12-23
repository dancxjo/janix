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
        Mutation::CreateThing { .. } => Err("User CreateThing denied by default policy"),
        Mutation::UpdateThing { .. } => Err("User UpdateThing denied by default policy"),
        Mutation::AddLink { .. } => Err("User AddLink denied by default policy"),
        Mutation::RemoveLink { .. } => Err("User RemoveLink denied by default policy"),
        Mutation::DeclareSchema { kind, .. } => {
            // Check ownership: kind must start with "pkg.<package_name>."
            let pkg_id = _user.package_id;
             // We need to resolve symbols to check strings. This is slow but acceptable for declarations.
            let pkg_name = crate::symbols::resolve(pkg_id).ok_or("Unresolved package ID")?;
            let kind_name = crate::symbols::resolve(*kind).ok_or("Unresolved schema kind")?;
            
            let expected_prefix = alloc::format!("pkg.{}.", pkg_name);
            if !kind_name.starts_with(&expected_prefix) {
                 return Err("Schema declaration denied: namespace mismatch");
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
