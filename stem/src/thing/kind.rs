#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThingKind(pub u64);

impl ThingKind {
    // Mirror abi/src/kinds.rs constants
    pub const BYTESPACE_BUFFER: Self = Self(0x1000);
    pub const STREAM_WATCH: Self = Self(0x2000);
    pub const TEST_NODE: Self = Self(0x3000);
}
