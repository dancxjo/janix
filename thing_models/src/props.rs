use alloc::string::String;
use alloc::vec::Vec;
use abi::syscall_defs::SymbolId;

pub type PropKey = String;

#[derive(Debug, Clone, PartialEq)]
pub enum PropValue {
    U64(u64),
    I64(i64),
    Bool(bool),
    Str(String),
    Blob(Vec<u8>),
    Symbol(SymbolId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PropType {
    U64,
    I64,
    Bool,
    Symbol,
    Str,
    Blob,
}
