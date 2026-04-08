use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraceEntry {
    pub func_name: String,
    pub args: Vec<u64>,
    pub result: u64,
}

#[derive(Debug)]
pub enum TraceMode {
    None,
    Record(Vec<TraceEntry>),
    Replay(std::vec::IntoIter<TraceEntry>),
}
