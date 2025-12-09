use kernel_core::graph;

#[test]
fn test_schema_description_storage() {
    graph::init();
    
    // Register a schema with a description
    let result = graph::register_schema(
        "TestThing",
        "A test thing for description validation",
        &[("prop1", abi::PropType::U64)],
    );
    
    assert!(result.is_ok());
    
    // Verify we can retrieve the description
    let description = graph::get_schema_description("TestThing");
    assert!(description.is_some());
    assert_eq!(description.unwrap(), "A test thing for description validation");
}

#[test]
fn test_schema_description_not_found() {
    graph::init();
    
    // Try to get description for non-existent schema
    let description = graph::get_schema_description("NonExistent");
    assert!(description.is_none());
}

#[test]
fn test_multiple_schemas_with_descriptions() {
    graph::init();
    
    // Register multiple schemas
    let result1 = graph::register_schema(
        "Thing1",
        "First test thing",
        &[("field1", abi::PropType::Bool)],
    );
    assert!(result1.is_ok(), "Failed to register Thing1: {:?}", result1);
    
    let result2 = graph::register_schema(
        "Thing2",
        "Second test thing",
        &[("field2", abi::PropType::Str)],
    );
    assert!(result2.is_ok(), "Failed to register Thing2: {:?}", result2);
    
    // Verify each has its own description
    assert_eq!(
        graph::get_schema_description("Thing1"),
        Some("First test thing")
    );
    assert_eq!(
        graph::get_schema_description("Thing2"),
        Some("Second test thing")
    );
}

#[test]
fn test_model_schemas_have_descriptions() {
    graph::init();
    kernel_core::model::init_schemas();
    
    // Verify all model schemas have descriptions
    let schemas_to_check = [
        "PhysFrame",
        "FramePool",
        "AddressSpace",
        "VirtRegion",
        "Process",
        "Thread",
        "CpuCore",
    ];
    
    for schema_name in &schemas_to_check {
        let description = graph::get_schema_description(schema_name);
        assert!(
            description.is_some(),
            "Schema {} should have a description",
            schema_name
        );
        
        let desc = description.unwrap();
        assert!(
            !desc.is_empty(),
            "Schema {} description should not be empty",
            schema_name
        );
        assert_ne!(
            desc, "No description provided",
            "Schema {} should have a meaningful description",
            schema_name
        );
    }
}
