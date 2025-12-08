use thing_macros::Thing;
use userland_std::{create_thing, load_thing};

#[derive(Thing)]
struct DemoCounter {
    pub count: u64,
    pub active: bool,
}

fn main() {
    userland_std::println("Hello from user_app_hello with Thing!");

    let demo = DemoCounter { count: 42, active: true };
    if let Some(id) = create_thing(&demo) {
        userland_std::println("Created DemoCounter Thing");
        
        if let Some(loaded) = load_thing::<DemoCounter>(id) {
             userland_std::println("Loaded DemoCounter Thing");
             if loaded.count == 42 && loaded.active {
                 userland_std::println("  Data matches!");
             } else {
                 userland_std::println("  Data mismatch!");
             }
        } else {
            userland_std::println("Failed to load Thing");
        }
    } else {
        userland_std::println("Failed to create Thing");
    }
    
    userland_std::println("Goodbye from user_app_hello!");
}
