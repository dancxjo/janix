use abi::syscall_defs::SymbolId;
use abi::wire::common::*;
use abi::wire::graph::*;
use abi::wire::memory::*;
use static_assertions::assert_eq_size;

#[test]
fn test_wire_layout() {
    // UserPtr is 8 bytes
    assert_eq_size!(UserPtr<u8>, [u8; 8]);
    // UserSlice is 16 bytes (ptr + len)
    assert_eq_size!(UserSlice<u8>, [u8; 16]);
    // SymbolId is 4 bytes
    assert_eq_size!(SymbolId, [u8; 4]);

    // WirePropValue: tag(1) + pad(7) + data_0(8) + data_1(8) = 24 bytes
    assert_eq_size!(WirePropValue, [u8; 24]);

    // WireProp: key(4) + pad(4) + value(24) = 32 bytes
    assert_eq_size!(WireProp, [u8; 32]);

    // FrameInfo: id(8) + base(8) + size(8) = 24 bytes
    assert_eq_size!(FrameInfo, [u8; 24]);

    // BatchUpdateEntry has id: ThingId(u64), props_ptr: UserPtr (u64), props_len: u64
    assert_eq_size!(abi::wire::graph::BatchUpdateEntry, [u8; 24]);
}
