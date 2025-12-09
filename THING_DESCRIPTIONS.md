# Thing Descriptions Feature

This document explains how to use the Thing descriptions feature in ThingOS.

## Overview

All Thing types now require a description field that explains in plain natural language what the type represents. Descriptions are declared using macros and stored with schema registrations.

## Usage

### For Types Using the Derive Macro

Add a `#[thing(description = "...")]` attribute above your struct:

```rust
use thing_macros::Thing;

#[derive(Thing)]
#[thing(description = "A thread in the scheduler with execution state and timing information")]
pub struct Thread {
    pub name: String,
    pub state: String,
    pub last_run_ns: i64,
    pub total_run_ns: i64,
    pub process_thing_id: u64,
    pub scheduler_thing_id: u64,
}
```

### For Manual Thing Implementations

Add a `DESCRIPTION` constant to your Thing implementation:

```rust
impl Thing for AutoCounter {
    const KIND: &'static str = "AutoCounter";
    const DESCRIPTION: &'static str = "An automatically incrementing counter with active/inactive state";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        // ... implementation
    }

    fn from_props(_id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        // ... implementation
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        // ... implementation
    }
}
```

### Accessing Descriptions

#### Get Type-Level Description

```rust
// Using the constant directly
let description = AutoCounter::DESCRIPTION;

// Using the helper function
let description = get_type_description::<AutoCounter>();
```

#### Get Instance Description

The `get_description()` method returns an owned `String` to support runtime instance-specific descriptions:

```rust
let counter = AutoCounter {
    count: 42,
    active: true,
};

// Returns the type description as a String
let description = counter.get_description();
// description == "An automatically incrementing counter with active/inactive state"
```

#### Instance-Specific Descriptions

Things can override `get_description()` to provide instance-specific descriptions. Here's an example:

```rust
use abi::{Thing, ThingId, PropKey, PropValue, PropType};

pub struct NamedThing {
    pub name: String,
    pub description: Option<String>,
}

impl Thing for NamedThing {
    const KIND: &'static str = "NamedThing";
    const DESCRIPTION: &'static str = "A thing with an optional custom description";

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

        NamedThing { name, description }
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
```

Usage:

```rust
// Thing with custom description
let custom = NamedThing {
    name: String::from("MyCustomThing"),
    description: Some(String::from("This instance has a special purpose")),
};

assert_eq!(custom.get_description(), "This instance has a special purpose");

// Thing without custom description falls back to type description
let default = NamedThing {
    name: String::from("RegularThing"),
    description: None,
};

assert_eq!(default.get_description(), "A thing with an optional custom description");
```

#### Get Schema Description from Kernel

```rust
use kernel_core::graph;

// After schema is registered
let description = graph::get_schema_description("AutoCounter");
```

## Schema Registration

When registering schemas, descriptions must be provided:

```rust
use kernel_core::graph;
use abi::PropType;

graph::register_schema(
    "MyThing",
    "A custom thing that represents some state",
    &[
        ("field1", PropType::U64),
        ("field2", PropType::Bool),
    ],
)?;
```

## Examples

### Kernel Core Schemas

All built-in kernel schemas have descriptions:

- **PhysFrame**: "A region of physical memory with base address, size, and allocation status"
- **Process**: "A process with process identifier (PID) and execution state"
- **Thread**: "A thread of execution with thread identifier, state, priority, and runtime tracking"

### User Application Things

User applications define their own Thing types with descriptions:

- **AutoCounter**: "An automatically incrementing counter with active/inactive state"
- **HeartbeatThing**: "A heartbeat counter that tracks periodic application activity"
- **DemoState**: "Shared state for demonstration applications tracking hello and heartbeat ticks"

## Testing

Tests are provided to verify the description functionality:

- `kernel_core/tests/description_test.rs` - Tests for schema descriptions
- `user_app_hello/tests/description_test.rs` - Tests for Thing trait descriptions

Run the tests with:

```bash
cargo test --package kernel_core --test description_test
cargo test --package user_app_hello --test description_test
```
