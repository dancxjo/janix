#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThingId(pub u64);

use super::debug::DebugThing;
impl ThingId {
    pub fn dbg(&self) -> DebugThing<'_> {
        DebugThing::new(*self)
    }
}
