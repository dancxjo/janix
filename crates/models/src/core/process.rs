use abi::SymbolId;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProcessState { New, Running, Exiting, Dead }

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThreadState { New, Runnable, Running, Sleeping, Dead }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProcessBody {
    pub pid: u64,
    pub name: SymbolId,
    pub state: ProcessState,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ThreadBody {
    pub tid: u64,
    pub name: SymbolId,
    pub state: ThreadState,
    pub sleep_until_ns: u64,
}
