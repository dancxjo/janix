use abi::ThingId;
use crate::value::ThingBody;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Thing {
    pub id: ThingId,
    pub kind: ThingId,
    pub body: ThingBody,
}
