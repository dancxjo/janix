#![no_std]
#![no_main]

use thing_os::prelude::*;
use thing_os::graph_kinds;
use thing_os::{PropType, PropValue};

#[derive(Clone, Debug)]
struct WitnessUnixTime {
    id: ThingId,
    unix_seconds: u64,
    unix_nanos: u64,
    ticks: u64,
}

impl Thing for WitnessUnixTime {
    const KIND: &'static str = graph_kinds::KIND_WITNESS_UNIX_TIME;
    const DESCRIPTION: &'static str = "Witness Unix Time";

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (graph_kinds::PROP_UNIX_SECONDS, PropType::U64),
            (graph_kinds::PROP_UNIX_NANOS, PropType::U64),
            (graph_kinds::PROP_TICKS_SINCE_BOOT, PropType::U64),
        ]
    }

    fn from_props(id: ThingId, props: &[Option<(String, PropValue)>]) -> Self {
        let mut unix_seconds = 0;
        let mut unix_nanos = 0;
        let mut ticks = 0;

        for p in props {
            if let Some((k, v)) = p {
                match (k.as_str(), v) {
                    (graph_kinds::PROP_UNIX_SECONDS, PropValue::U64(val)) => unix_seconds = *val,
                    (graph_kinds::PROP_UNIX_NANOS, PropValue::U64(val)) => unix_nanos = *val,
                    (graph_kinds::PROP_TICKS_SINCE_BOOT, PropValue::U64(val)) => ticks = *val,
                    (graph_kinds::PROP_UNIX_SECONDS, val) => println!("Mismatch type for seconds: {:?}", val),
                    _ => {}
                }
            }
        }

        WitnessUnixTime {
            id,
            unix_seconds,
            unix_nanos,
            ticks,
        }
    }

    fn to_props(&self, props: &mut Vec<(String, PropValue)>) {
        props.push((String::from(graph_kinds::PROP_UNIX_SECONDS), PropValue::U64(self.unix_seconds)));
        props.push((String::from(graph_kinds::PROP_UNIX_NANOS), PropValue::U64(self.unix_nanos)));
        props.push((String::from(graph_kinds::PROP_TICKS_SINCE_BOOT), PropValue::U64(self.ticks)));
    }
}

#[thing_os::main]
fn main() {
    println!("time_test: starting verification");

    // Negative Test 1: Try to create a Witness thing
    // We cannot use user_create_thing directly with dynamic props easily due to &'static requirement in current lib?
    // Let's check signature: `props: &'static [(PropKey, PropValue)]`
    // Yes, this is restrictive. I might need to construct a static slice or use lower level syscall if I can.
    // However, I can just make a static slice.
    
    static WITNESS_PROPS: &[(String, PropValue)] = &[
        (String::new(), PropValue::U64(0)) // Placeholder, string new is const in 2024? No.
    ];
    // String::new() is not const.
    // I need to skip the static requirement or find a workaround. 
    // Wait, thing_os::user_create_thing signature is `props: &'static [(PropKey, PropValue)]`.
    // `PropKey` is `String`. `String` cannot be static easily unless leaked.
    // I will try to leak it.
    
    let key = String::from(graph_kinds::PROP_UNIX_SECONDS);
    let key_static: &'static String = Box::leak(Box::new(key));
    let props_slice: &'static [(String, PropValue)] = Box::leak(Box::new(vec![
        (key_static.clone(), PropValue::U64(1234567890))
    ]).into_boxed_slice());

    match thing_os::user_create_thing(graph_kinds::KIND_WITNESS_UNIX_TIME, props_slice) {
        Ok(_) => println!("FAILURE: Userland was able to create Witness.UnixTime!"),
        Err(e) => println!("SUCCESS: Userland create rejected: {}", e),
    }

    // Positive Test: Find the Witness thing
    let mut witness_id = None;
    println!("scaning for Witness.UnixTime...");
    for _ in 0..10 {
        if let Some(w) = thing_os::list_things_by_kind::<WitnessUnixTime>().first() {
             witness_id = Some(w.id);
             println!("Found Witness.UnixTime: {:?}", w);
             break;
        }
        thing_os::time::sleep_ms(200);
    }
    
    // Negative Test 2: Try to update the Witness thing
    if let Some(id) = witness_id {
        let update_props_slice: &'static [(String, PropValue)] = Box::leak(Box::new(vec![
            (key_static.clone(), PropValue::U64(9999999999))
        ]).into_boxed_slice());

        match thing_os::user_update_thing(id, update_props_slice) {
            Ok(_) => println!("FAILURE: Userland was able to update Witness.UnixTime!"),
            Err(e) => println!("SUCCESS: Userland update rejected: {}", e),
        }
    } else {
        println!("FAILURE: Could not find Witness.UnixTime to test update");
    }

    // Positive Test Loop
    println!("Monitoring Witness.UnixTime for 5 seconds...");
    for _ in 0..5 {
        if let Some(w) = thing_os::list_things_by_kind::<WitnessUnixTime>().first() {
             println!("Time: {}.{} (ticks={})", w.unix_seconds, w.unix_nanos, w.ticks);
        } else {
             println!("Witness.UnixTime vanished?");
        }
        thing_os::time::sleep_ms(1000);
    }
    
    println!("time_test: verification complete.");
}
