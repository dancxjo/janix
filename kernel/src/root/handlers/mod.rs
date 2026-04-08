//! Root service message handlers, organized by domain.

use crate::root::symbols::Interner;

/// Common handler result type: (status, value)
pub type HandlerResult = (i32, u64);

pub fn handle_intern(interner: &mut Interner, name: &str) -> HandlerResult {
    let id = interner.intern(name);
    (0, id as u64)
}
