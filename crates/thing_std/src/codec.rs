use thing_codec::Thing;
use alloc::vec::Vec;

pub struct Codec;

impl Codec {
    /// Encode a model into a full Thing body (envelope + payload)
    pub fn encode<T: Thing>(model: &T) -> Vec<u8> {
        model.encode_full()
    }

    /// Decode and verify a full Thing body
    pub fn decode<T: Thing>(bytes: &[u8], _kernel_digest: u64) -> Result<T, ()> {
        T::decode_full(bytes).map_err(|_| ())
    }
}
