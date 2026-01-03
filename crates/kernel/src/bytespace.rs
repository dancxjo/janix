//! ByteSpace storage (stub)
//!
//! Will provide large binary data storage separate from graph.
//! Currently returns ENOSYS for all operations.

use abi::syscall::err::ENOSYS;

/// ByteSpace identifier
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ByteSpaceId(pub u64);

/// Create a new ByteSpace
pub fn create(_size: usize) -> Result<ByteSpaceId, i32> {
    Err(ENOSYS)
}

/// Write to a ByteSpace
pub fn write(_id: ByteSpaceId, _offset: usize, _data: &[u8]) -> Result<usize, i32> {
    Err(ENOSYS)
}

/// Read from a ByteSpace
pub fn read(_id: ByteSpaceId, _offset: usize, _buf: &mut [u8]) -> Result<usize, i32> {
    Err(ENOSYS)
}

/// Delete a ByteSpace
pub fn delete(_id: ByteSpaceId) -> Result<(), i32> {
    Err(ENOSYS)
}
