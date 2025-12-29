use crate::thing::Thing;
use crate::value::ThingBody;
use abi::{ThingId, SymbolId};
use core::marker::PhantomData;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone)]
pub struct TypedThing<T> {
    pub thing: Thing,
    _marker: PhantomData<T>,
}

impl<T> TypedThing<T> {
    pub fn new(thing: Thing) -> Self {
        Self {
            thing,
            _marker: PhantomData,
        }
    }
}

impl<T: DeserializeOwned> TypedThing<T> {
    pub fn decode(&self) -> Result<T, postcard::Error> {
        self.thing.body().decode()
    }
}

pub trait ThingTypedExt {
    fn decode<T: DeserializeOwned>(&self) -> Result<T, postcard::Error>;
}

impl ThingTypedExt for Thing {
    fn decode<T: DeserializeOwned>(&self) -> Result<T, postcard::Error> {
        self.body().decode()
    }
}

pub fn make_thing<T: Serialize>(
    id: ThingId,
    kind: SymbolId,
    body: &T,
) -> Result<Thing, postcard::Error> {
    Ok(Thing {
        id,
        kind,
        payload: postcard::to_allocvec(body)?,
    })
}
