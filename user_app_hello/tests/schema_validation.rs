use abi::{ThingId, PropKey, PropValue, PropType};
use user_app_hello::{ManualCounter, AutoCounter};
use userland_std::{Thing, register_schema_for};

#[test]
fn test_manual_counter_schema() {
    let schema = ManualCounter::schema();
    
    // Should have 2 properties
    assert_eq!(schema.len(), 2);
    
    // Verify schema entries
    assert_eq!(*schema[0].0, "count");
    assert_eq!(schema[0].1, PropType::U64);
    
    assert_eq!(*schema[1].0, "active");
    assert_eq!(schema[1].1, PropType::Bool);
}

#[test]
fn test_auto_counter_schema() {
    let schema = AutoCounter::schema();
    
    // Should have 2 properties
    assert_eq!(schema.len(), 2);
    
    // Verify schema entries (note: order matches struct field order)
    assert_eq!(*schema[0].0, "count");
    assert_eq!(schema[0].1, PropType::U64);
    
    assert_eq!(*schema[1].0, "active");
    assert_eq!(schema[1].1, PropType::Bool);
}

#[test]
fn test_schema_registration() {
    // Should be able to register schema
    let result = register_schema_for::<AutoCounter>();
    assert!(result, "Schema registration should succeed");
}

#[test]
fn test_schema_validation_success() {
    // Register schema
    register_schema_for::<AutoCounter>();
    
    // Create a thing with valid properties
    let counter = AutoCounter { count: 100, active: true };
    let id = userland_std::create_thing(&counter);
    
    // Should succeed
    assert!(id.is_some(), "Thing creation should succeed with valid schema");
}

#[test]
fn test_manual_and_derived_schemas_compatible() {
    let manual_schema = ManualCounter::schema();
    let auto_schema = AutoCounter::schema();
    
    // Both should have same structure
    assert_eq!(manual_schema.len(), auto_schema.len());
    
    // Properties should match
    for i in 0..manual_schema.len() {
        assert_eq!(*manual_schema[i].0, *auto_schema[i].0, "Property names should match");
        assert_eq!(manual_schema[i].1, auto_schema[i].1, "Property types should match");
    }
}
