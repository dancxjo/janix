use super::ThingId;
use core::fmt;

pub struct DebugThing<'a> {
    id: ThingId,
    _phantom: core::marker::PhantomData<&'a ()>,
}

impl<'a> DebugThing<'a> {
    pub fn new(id: ThingId) -> Self {
        Self {
            id,
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<'a> fmt::Display for DebugThing<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = [0u8; 128]; // Enough for basic description
        match super::sys::describe_thing(self.id, &mut buf) {
            Ok(len) => {
                let s = core::str::from_utf8(&buf[..len]).unwrap_or("<invalid utf8>");
                f.write_str(s)
            }
            Err(_) => {
                // Return error? or print fallback?
                // Requirements say: "Must compile... Must not dump infinite data".
                // If describe fails, fallback to simple ID
                write!(f, "(t{:x}:<error>)", self.id.0)
            }
        }
    }
}
