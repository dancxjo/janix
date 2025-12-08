#![no_std]

/// Process identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProcessId(pub u64);

/// Transaction identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransactionId(pub u64);

/// Node identifier in the graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

/// Thing identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThingId(pub u64);

/// Simple property key
pub type PropKey = &'static str;

/// Simple property value
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PropValue {
    U64(u64),
    I64(i64),
    Bool(bool),
}

/// Property type for schema validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PropType {
    U64,
    I64,
    Bool,
}

/// Schema identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemaId(pub u64);

/// Kernel request from userland
#[derive(Debug, Clone)]
pub enum KernelRequest {
    /// Query the graph
    GraphQuery { node_id: NodeId },
    /// Create a transaction
    CreateTransaction,
    /// Commit a transaction
    CommitTransaction { tx_id: TransactionId },
    /// Log a message
    Log { message: &'static str },
    /// Create a new Thing
    ThingCreate {
        kind: &'static str,
        props: &'static [(PropKey, PropValue)],
    },
    /// Get a Thing
    ThingGet { id: ThingId },
    /// Update a Thing
    ThingUpdate {
        id: ThingId,
        props: &'static [(PropKey, PropValue)],
    },
    /// Register a schema
    SchemaRegister {
        kind: &'static str,
        props: &'static [(&'static PropKey, PropType)],
    },
    /// Get a schema
    SchemaGet {
        kind: &'static str,
    },
}

/// Kernel response to userland
#[derive(Debug, Clone)]
pub enum KernelResponse {
    /// Success with optional data
    Success { data: Option<u64> },
    /// Error with message
    Error { message: &'static str },
    /// Transaction created
    TransactionCreated { tx_id: TransactionId },
    /// Node data
    NodeData { node_id: NodeId, value: u64 },
    /// Thing created
    ThingCreated { id: ThingId },
    /// Thing data
    ThingData {
        id: ThingId,
        kind: &'static str,
        props: &'static [Option<(PropKey, PropValue)>],
    },
    /// Schema registered
    SchemaRegistered { kind: &'static str },
    /// Schema data
    SchemaData {
        kind: &'static str,
        props: &'static [Option<(&'static PropKey, PropType)>],
    },
}
