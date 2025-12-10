extern crate alloc;
use abi::{PropValue, Thing, ThingId};
use thing_macros::Thing;

#[derive(Clone, Debug, PartialEq, Eq, Thing)]
#[thing(description = "Helper struct for verifying Thing derive conversions")]
struct RoundTripThing {
    id: ThingId,
    counter: u64,
    variant: u32,
    signed: i16,
    label: String,
    ready: bool,
    letter: char,
}

#[test]
fn derived_thing_roundtrips_via_props() {
    let thing = RoundTripThing {
        id: ThingId(0x123),
        counter: 42,
        variant: 7,
        signed: -3,
        label: String::from("roundtrip"),
        ready: true,
        letter: 'A',
    };

    let mut props = Vec::new();
    thing.to_props(&mut props);

    assert_eq!(
        props,
        [
            ("counter", PropValue::U64(42)),
            ("variant", PropValue::U64(7)),
            ("signed", PropValue::I64(-3)),
            ("label", PropValue::Str(String::from("roundtrip"))),
            ("ready", PropValue::Bool(true)),
            ("letter", PropValue::Str(String::from("A"))),
        ]
    );

    let roundtripped = RoundTripThing::from_props(
        thing.id,
        &props
            .iter()
            .map(|(k, v)| Some((*k, v.clone())))
            .collect::<Vec<_>>(),
    );

    assert_eq!(roundtripped, thing);
}
