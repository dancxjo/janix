use abi::ThingId;
use models::{LinkBody, Thing, ThingBody};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestStruct {
    a: u32,
    b: u64,
}

#[test]
fn test_thing_body_roundtrip() {
    let original = TestStruct { a: 42, b: 100 };
    let body = ThingBody::from(&original).expect("encoding failed");
    let decoded: TestStruct = body.decode().expect("decoding failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_thing_roundtrip() {
    let original_body_content = TestStruct { a: 1, b: 2 };
    let body = ThingBody::from(&original_body_content).unwrap();
    
    let thing = Thing {
        id: ThingId(10),
        kind: ThingId(20),
        body,
    };

    let bytes = postcard::to_allocvec(&thing).expect("encode thing");
    let decoded: Thing = postcard::from_bytes(&bytes).expect("decode thing");
    
    assert_eq!(thing, decoded);
    
    // Verify body content persists
    let struct_decoded: TestStruct = decoded.body.decode().expect("decode body");
    assert_eq!(struct_decoded, original_body_content);
}

#[test]
fn test_link_body_roundtrip() {
    let link = LinkBody {
        from: ThingId(1),
        to: ThingId(2),
        predicate: ThingId(3),
    };
    
    let bytes = postcard::to_allocvec(&link).expect("encode link");
    let decoded: LinkBody = postcard::from_bytes(&bytes).expect("decode link");
    assert_eq!(link, decoded);
}
