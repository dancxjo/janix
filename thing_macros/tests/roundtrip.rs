extern crate alloc;
use abi::ThingId;
use thing_models::{PropValue, Thing};
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
            ("counter".to_string(), PropValue::U64(42)),
            ("variant".to_string(), PropValue::U64(7)),
            ("signed".to_string(), PropValue::I64(-3)),
            ("label".to_string(), PropValue::Str(String::from("roundtrip"))),
            ("ready".to_string(), PropValue::Bool(true)),
            ("letter".to_string(), PropValue::Str(String::from("A"))),
        ]
    );

    let roundtripped = RoundTripThing::from_props(
        thing.id,
        &props
            .iter()
            .map(|(k, v)| Some((k.clone(), v.clone())))
            .collect::<Vec<_>>(),
    );

    assert_eq!(roundtripped, thing);
}
