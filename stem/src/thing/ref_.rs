use super::{ThingId, Thing};
use core::marker::PhantomData;

#[derive(Debug)]
pub struct ThingRef<T: Thing> {
    pub id: ThingId,
    _p: PhantomData<T>,
}

impl<T: Thing> Clone for ThingRef<T> {
    fn clone(&self) -> Self {
        Self { id: self.id, _p: PhantomData }
    }
}

impl<T: Thing> Copy for ThingRef<T> {}

impl<T: Thing> ThingRef<T> {
    pub unsafe fn new(id: ThingId) -> Self {
        Self { id, _p: PhantomData }
    }
}

use super::debug::DebugThing;
impl<T: super::Thing> ThingRef<T> {
    pub fn dbg(&self) -> DebugThing<'_> {
        DebugThing::new(self.id)
    }
}
