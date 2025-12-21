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

```rust
let counter = AutoCounter {
    count: 42,
    active: true,
};

// Falls back to the type description
let description = counter.get_description();
```

#### Get Schema Description from Kernel

```rust
use kernel::graph;

// After schema is registered
let description = graph::get_schema_description("AutoCounter");
```

## Schema Registration

When registering schemas, descriptions must be provided:

```rust
use kernel::graph;
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

- `kernel/tests/description_test.rs` - Tests for schema descriptions
- `user/hello/tests/description_test.rs` - Tests for Thing trait descriptions

Run the tests with:

```bash
cargo test --package kernel --test description_test
cargo test --package hello --test description_test
```
