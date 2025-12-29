use crate::builtins::symbols::{SYM_ERROR, SYM_FAULT, SYM_LOG_ENTRY};
use crate::diag::{ErrorBody, FaultBody, LogEntryBody};
use crate::Thing;
use abi::ThingId;

impl Thing {
    pub fn log(id: ThingId, body: &LogEntryBody) -> Self {
        Thing::with(id, SYM_LOG_ENTRY, body)
    }

    pub fn fault(id: ThingId, body: &FaultBody) -> Self {
        Thing::with(id, SYM_FAULT, body)
    }

    pub fn error(id: ThingId, body: &ErrorBody) -> Self {
        Thing::with(id, SYM_ERROR, body)
    }
}
