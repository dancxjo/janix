use crate::Thing;
use crate::diag::{LogEntryBody, ErrorBody, FaultBody};
use crate::builtins::ids::{THING_LOG_ENTRY_KIND, THING_ERROR_KIND, THING_FAULT_KIND};
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
