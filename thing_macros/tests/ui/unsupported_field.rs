use abi::ThingId;
use thing_macros::Thing;

#[derive(Thing)]
#[thing(description = "Fields with unsupported types should trigger an error")]
struct UnsupportedFieldThing {
    id: ThingId,
    data: Vec<u8>,
}

fn main() {}
