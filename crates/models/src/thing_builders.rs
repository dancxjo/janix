use crate::builtins::ids::{THING_ERROR_KIND, THING_FAULT_KIND, THING_LOG_ENTRY_KIND};
use crate::diag::{ErrorBody, FaultBody, LogEntryBody};
use crate::Thing;
use abi::ThingId;

impl Thing {
    pub fn log(id: ThingId, body: &LogEntryBody) -> Self {
        Thing::with(id, THING_LOG_ENTRY_KIND, body)
    }

    pub fn fault(id: ThingId, body: &FaultBody) -> Self {
        Thing::with(id, THING_FAULT_KIND, body)
    }

    pub fn error(id: ThingId, body: &ErrorBody) -> Self {
        Thing::with(id, THING_ERROR_KIND, body)
    }
}
