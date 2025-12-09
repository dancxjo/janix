use abi::Thing;
use user_app_hello::AutoCounter;

#[test]
fn test_auto_counter_has_description() {
    let description = AutoCounter::DESCRIPTION;
    assert!(!description.is_empty(), "AutoCounter should have a description");
    assert!(
        description.contains("counter"),
        "Description should mention 'counter'"
    );
}

#[test]
fn test_get_description_method() {
    let counter = AutoCounter {
        count: 42,
        active: true,
    };
    
    let description = counter.get_description();
    assert_eq!(description, AutoCounter::DESCRIPTION);
    assert!(!description.is_empty());
}

#[test]
fn test_description_is_meaningful() {
    let description = AutoCounter::DESCRIPTION;
    
    // Verify it's not just a placeholder
    assert_ne!(description, "No description provided");
    
    // Verify it contains useful words
    let description_lower = description.to_lowercase();
    assert!(
        description_lower.contains("counter") || description_lower.contains("increment"),
        "Description should describe the counter functionality"
    );
}
