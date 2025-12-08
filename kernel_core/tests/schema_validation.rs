use abi::{KernelRequest, KernelResponse, PropValue, PropType};
use kernel_core;

#[test]
fn test_schema_registration() {
    kernel_core::init();
    
    let schema = &[
        (&"count", PropType::U64),
        (&"active", PropType::Bool),
    ];
    
    let response = kernel_core::handle_request(KernelRequest::SchemaRegister {
        kind: "TestThing",
        props: schema,
    });
    
    match response {
        KernelResponse::SchemaRegistered { kind } => {
            assert_eq!(kind, "TestThing");
        }
        _ => panic!("Expected SchemaRegistered response"),
    }
}

#[test]
fn test_duplicate_schema_registration() {
    kernel_core::init();
    
    let schema = &[
        (&"field", PropType::U64),
    ];
    
    // Register first time - should succeed
    let response1 = kernel_core::handle_request(KernelRequest::SchemaRegister {
        kind: "DupTest",
        props: schema,
    });
    assert!(matches!(response1, KernelResponse::SchemaRegistered { .. }));
    
    // Register second time - should fail
    let response2 = kernel_core::handle_request(KernelRequest::SchemaRegister {
        kind: "DupTest",
        props: schema,
    });
    assert!(matches!(response2, KernelResponse::Error { .. }));
}

#[test]
fn test_thing_creation_with_valid_schema() {
    kernel_core::init();
    
    // Register schema
    let schema = &[
        (&"count", PropType::U64),
        (&"active", PropType::Bool),
    ];
    kernel_core::handle_request(KernelRequest::SchemaRegister {
        kind: "ValidThing",
        props: schema,
    });
    
    // Create thing with valid properties
    let props = &[
        ("count", PropValue::U64(42)),
        ("active", PropValue::Bool(true)),
    ];
    
    let response = kernel_core::handle_request(KernelRequest::ThingCreate {
        kind: "ValidThing",
        props,
    });
    
    assert!(matches!(response, KernelResponse::ThingCreated { .. }));
}

#[test]
fn test_thing_creation_without_schema() {
    kernel_core::init();
    
    // Try to create thing without registering schema
    let props = &[
        ("count", PropValue::U64(42)),
    ];
    
    let response = kernel_core::handle_request(KernelRequest::ThingCreate {
        kind: "UnregisteredThing",
        props,
    });
    
    // Should fail with error about missing schema
    match response {
        KernelResponse::Error { message } => {
            assert!(message.contains("No schema"));
        }
        _ => panic!("Expected error for unregistered schema"),
    }
}

#[test]
fn test_thing_creation_with_type_mismatch() {
    kernel_core::init();
    
    // Register schema expecting U64
    let schema = &[
        (&"value", PropType::U64),
    ];
    kernel_core::handle_request(KernelRequest::SchemaRegister {
        kind: "TypeMismatchThing",
        props: schema,
    });
    
    // Try to create with Bool instead
    let props = &[
        ("value", PropValue::Bool(true)),
    ];
    
    let response = kernel_core::handle_request(KernelRequest::ThingCreate {
        kind: "TypeMismatchThing",
        props,
    });
    
    // Should fail with type mismatch error
    match response {
        KernelResponse::Error { message } => {
            assert!(message.contains("type mismatch"));
        }
        _ => panic!("Expected error for type mismatch"),
    }
}

#[test]
fn test_thing_creation_with_unknown_property() {
    kernel_core::init();
    
    // Register schema with specific properties
    let schema = &[
        (&"count", PropType::U64),
    ];
    kernel_core::handle_request(KernelRequest::SchemaRegister {
        kind: "StrictThing",
        props: schema,
    });
    
    // Try to create with an extra property not in schema
    let props = &[
        ("count", PropValue::U64(42)),
        ("extra", PropValue::Bool(true)),
    ];
    
    let response = kernel_core::handle_request(KernelRequest::ThingCreate {
        kind: "StrictThing",
        props,
    });
    
    // Should fail because "extra" is not in schema
    match response {
        KernelResponse::Error { message } => {
            assert!(message.contains("Property not"), "Expected 'Property not in schema' error, got: {}", message);
        }
        _ => panic!("Expected error for unknown property"),
    }
}

#[test]
fn test_thing_update_with_schema_validation() {
    kernel_core::init();
    
    // Register schema
    let schema = &[
        (&"count", PropType::U64),
        (&"active", PropType::Bool),
    ];
    kernel_core::handle_request(KernelRequest::SchemaRegister {
        kind: "UpdateTestThing",
        props: schema,
    });
    
    // Create thing
    let create_props = &[
        ("count", PropValue::U64(10)),
        ("active", PropValue::Bool(false)),
    ];
    let create_response = kernel_core::handle_request(KernelRequest::ThingCreate {
        kind: "UpdateTestThing",
        props: create_props,
    });
    
    let thing_id = match create_response {
        KernelResponse::ThingCreated { id } => id,
        _ => panic!("Failed to create thing"),
    };
    
    // Update with valid properties
    let update_props = &[
        ("count", PropValue::U64(20)),
    ];
    let update_response = kernel_core::handle_request(KernelRequest::ThingUpdate {
        id: thing_id,
        props: update_props,
    });
    
    assert!(matches!(update_response, KernelResponse::Success { .. }));
}

#[test]
fn test_thing_update_with_invalid_type() {
    kernel_core::init();
    
    // Register schema
    let schema = &[
        (&"count", PropType::U64),
    ];
    kernel_core::handle_request(KernelRequest::SchemaRegister {
        kind: "UpdateTypeThing",
        props: schema,
    });
    
    // Create thing
    let create_props = &[
        ("count", PropValue::U64(10)),
    ];
    let create_response = kernel_core::handle_request(KernelRequest::ThingCreate {
        kind: "UpdateTypeThing",
        props: create_props,
    });
    
    let thing_id = match create_response {
        KernelResponse::ThingCreated { id } => id,
        _ => panic!("Failed to create thing"),
    };
    
    // Try to update with wrong type
    let update_props = &[
        ("count", PropValue::Bool(true)),
    ];
    let update_response = kernel_core::handle_request(KernelRequest::ThingUpdate {
        id: thing_id,
        props: update_props,
    });
    
    // Should fail with type mismatch
    match update_response {
        KernelResponse::Error { message } => {
            assert!(message.contains("type mismatch"));
        }
        _ => panic!("Expected error for type mismatch on update"),
    }
}

#[test]
fn test_schema_get() {
    kernel_core::init();
    
    // Register a schema
    let schema = &[
        (&"field1", PropType::U64),
        (&"field2", PropType::Bool),
    ];
    kernel_core::handle_request(KernelRequest::SchemaRegister {
        kind: "GetTestThing",
        props: schema,
    });
    
    // Get the schema back
    let response = kernel_core::handle_request(KernelRequest::SchemaGet {
        kind: "GetTestThing",
    });
    
    match response {
        KernelResponse::SchemaData { kind, props } => {
            assert_eq!(kind, "GetTestThing");
            
            // Count non-None properties
            let prop_count = props.iter().filter(|p| p.is_some()).count();
            assert_eq!(prop_count, 2);
            
            // Verify properties
            let prop1 = props[0].unwrap();
            assert_eq!(*prop1.0, "field1");
            assert_eq!(prop1.1, PropType::U64);
            
            let prop2 = props[1].unwrap();
            assert_eq!(*prop2.0, "field2");
            assert_eq!(prop2.1, PropType::Bool);
        }
        _ => panic!("Expected SchemaData response"),
    }
}

#[test]
fn test_schema_get_not_found() {
    kernel_core::init();
    
    let response = kernel_core::handle_request(KernelRequest::SchemaGet {
        kind: "NonExistent",
    });
    
    assert!(matches!(response, KernelResponse::Error { .. }));
}
