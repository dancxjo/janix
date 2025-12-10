use abi::{PropKey, PropValue, ThingId};
use hello::{AutoCounter, ManualCounter};
use userland_std::Thing;

#[test]
fn test_manual_and_derived_thing_compatibility() {
    // Create instances with the same values
    let manual = ManualCounter {
        count: 42,
        active: true,
    };

    let auto = AutoCounter {
        count: 42,
        active: true,
    };

    // Test to_props produces the same output
    let mut manual_props = Vec::new();
    manual.to_props(&mut manual_props);

    let mut auto_props = Vec::new();
    auto.to_props(&mut auto_props);

    // Both should have 2 properties
    assert_eq!(
        manual_props.len(),
        2,
        "ManualCounter should have 2 properties"
    );
    assert_eq!(auto_props.len(), 2, "AutoCounter should have 2 properties");

    // Sort by key to ensure consistent comparison
    manual_props.sort_by_key(|(k, _)| *k);
    auto_props.sort_by_key(|(k, _)| *k);

    // Verify the properties match
    assert_eq!(manual_props, auto_props, "Properties should be identical");

    // Verify specific values
    assert_eq!(manual_props[0].0, "active");
    assert_eq!(manual_props[0].1, PropValue::Bool(true));
    assert_eq!(manual_props[1].0, "count");
    assert_eq!(manual_props[1].1, PropValue::U64(42));
}

#[test]
fn test_from_props_manual_and_derived() {
    // Create test properties
    let props = vec![
        Some(("count", PropValue::U64(99))),
        Some(("active", PropValue::Bool(false))),
    ];

    let fake_id = ThingId(123);

    // Reconstruct both types from the same props
    let manual = ManualCounter::from_props(fake_id, &props);
    let auto = AutoCounter::from_props(fake_id, &props);

    // Verify both reconstructed correctly
    assert_eq!(manual.count, 99);
    assert_eq!(manual.active, false);
    assert_eq!(auto.count, 99);
    assert_eq!(auto.active, false);
}

#[test]
fn test_round_trip_manual() {
    let original = ManualCounter {
        count: 100,
        active: true,
    };

    // Serialize
    let mut props = Vec::new();
    original.to_props(&mut props);

    // Convert to Option slice for from_props
    let props_opts: Vec<Option<(PropKey, PropValue)>> = props.into_iter().map(Some).collect();

    // Deserialize
    let reconstructed = ManualCounter::from_props(ThingId(0), &props_opts);

    // Verify
    assert_eq!(reconstructed.count, original.count);
    assert_eq!(reconstructed.active, original.active);
}

#[test]
fn test_round_trip_derived() {
    let original = AutoCounter {
        count: 200,
        active: false,
    };

    // Serialize
    let mut props = Vec::new();
    original.to_props(&mut props);

    // Convert to Option slice for from_props
    let props_opts: Vec<Option<(PropKey, PropValue)>> = props.into_iter().map(Some).collect();

    // Deserialize
    let reconstructed = AutoCounter::from_props(ThingId(0), &props_opts);

    // Verify
    assert_eq!(reconstructed.count, original.count);
    assert_eq!(reconstructed.active, original.active);
}

#[test]
fn test_kind_constants() {
    // Verify KIND is set correctly
    assert_eq!(ManualCounter::KIND, "demo.ManualCounter");
    assert_eq!(AutoCounter::KIND, "AutoCounter");
}

#[test]
fn test_from_props_with_none_values() {
    // Test that from_props handles None in the props slice
    let props = vec![
        None,
        Some(("count", PropValue::U64(50))),
        None,
        Some(("active", PropValue::Bool(true))),
        None,
    ];

    let manual = ManualCounter::from_props(ThingId(0), &props);
    let auto = AutoCounter::from_props(ThingId(0), &props);

    assert_eq!(manual.count, 50);
    assert_eq!(manual.active, true);
    assert_eq!(auto.count, 50);
    assert_eq!(auto.active, true);
}

#[test]
fn test_from_props_with_unknown_keys() {
    // Test that unknown keys are ignored
    let props = vec![
        Some(("count", PropValue::U64(75))),
        Some(("unknown_key", PropValue::U64(999))),
        Some(("active", PropValue::Bool(false))),
        Some(("another_unknown", PropValue::Bool(true))),
    ];

    let manual = ManualCounter::from_props(ThingId(0), &props);

    // Manual implementation should ignore unknown keys
    assert_eq!(manual.count, 75);
    assert_eq!(manual.active, false);
}

#[test]
fn test_cross_compatible_serialization() {
    // Serialize manual, deserialize as auto
    let manual = ManualCounter {
        count: 333,
        active: true,
    };

    let mut manual_props = Vec::new();
    manual.to_props(&mut manual_props);

    let props_opts: Vec<Option<(PropKey, PropValue)>> =
        manual_props.into_iter().map(Some).collect();
    let auto = AutoCounter::from_props(ThingId(0), &props_opts);

    assert_eq!(auto.count, 333);
    assert_eq!(auto.active, true);

    // Serialize auto, deserialize as manual
    let auto2 = AutoCounter {
        count: 444,
        active: false,
    };

    let mut auto_props = Vec::new();
    auto2.to_props(&mut auto_props);

    let props_opts2: Vec<Option<(PropKey, PropValue)>> = auto_props.into_iter().map(Some).collect();
    let manual2 = ManualCounter::from_props(ThingId(0), &props_opts2);

    assert_eq!(manual2.count, 444);
    assert_eq!(manual2.active, false);
}
