use abi::{PropKey, PropType, PropValue, Thing, ThingId};
use user_app_hello::AutoCounter;

// Test Thing type with instance-specific description support
pub struct DescribableThing {
    pub name: String,
    pub description: Option<String>,
}

impl Thing for DescribableThing {
    const KIND: &'static str = "DescribableThing";
    const DESCRIPTION: &'static str = "A thing that supports instance-specific descriptions";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("name", PropValue::Str(self.name.clone())));
        if let Some(ref desc) = self.description {
            out.push(("description", PropValue::Str(desc.clone())));
        }
    }

    fn from_props(_id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut name = String::new();
        let mut description = None;

        for prop in props {
            if let Some((k, v)) = prop {
                match *k {
                    "name" => {
                        if let PropValue::Str(val) = v {
                            name = val.clone();
                        }
                    }
                    "description" => {
                        if let PropValue::Str(val) = v {
                            description = Some(val.clone());
                        }
                    }
                    _ => {}
                }
            }
        }

        DescribableThing { name, description }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("name", PropType::Str),
            ("description", PropType::Str),
        ]
    }

    // Override to return instance-specific description if available
    fn get_description(&self) -> String {
        self.description.clone().unwrap_or_else(|| String::from(Self::DESCRIPTION))
    }
}


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
    assert_eq!(description, String::from(AutoCounter::DESCRIPTION));
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

#[test]
fn test_instance_specific_description() {
    let thing = DescribableThing {
        name: String::from("MyThing"),
        description: Some(String::from("This is my custom instance description")),
    };
    
    let description = thing.get_description();
    assert_eq!(description, "This is my custom instance description");
    assert_ne!(description, DescribableThing::DESCRIPTION);
}

#[test]
fn test_fallback_to_type_description() {
    let thing = DescribableThing {
        name: String::from("AnotherThing"),
        description: None,
    };
    
    let description = thing.get_description();
    assert_eq!(description, String::from(DescribableThing::DESCRIPTION));
}

#[test]
fn test_instance_vs_type_description() {
    // Thing with instance description
    let instance_desc = DescribableThing {
        name: String::from("Thing1"),
        description: Some(String::from("Custom description for instance")),
    };
    
    // Thing without instance description
    let type_desc = DescribableThing {
        name: String::from("Thing2"),
        description: None,
    };
    
    // They should have different descriptions
    assert_ne!(instance_desc.get_description(), type_desc.get_description());
    
    // But the same type description
    assert_eq!(DescribableThing::DESCRIPTION, DescribableThing::DESCRIPTION);
}
