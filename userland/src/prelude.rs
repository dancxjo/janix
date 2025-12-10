extern crate alloc;

pub use alloc::boxed::Box;
pub use alloc::format;
pub use alloc::string::{String, ToString};
pub use alloc::vec;
pub use alloc::vec::Vec;

pub use abi::{PropKey, PropType, PropValue, ThingId};

pub use userland_rt::Sys;

pub use userland_std::{
    CpuCoreThing,
    ProcessThing,
    Thing,
    ThreadThing,
    alloc_frame,
    create_process,
    // Add other common exports as needed
    create_thing,
    create_thread,
    free_frame,
    get_type_description,
    list_things_by_kind,
    load_thing,
    memory_summary,
    println,
    register_schema_for,
    scheduler_summary,
    time,
    update_props,
};

/// Convenience logging helper for dynamic Strings
pub fn log_dynamic(sys: &impl Sys, msg: String) {
    // LEAK: To satisfy the ABI's 'static requirements for now.
    let leaked: &'static str = Box::leak(msg.into_boxed_str());
    println(sys, leaked);
}
