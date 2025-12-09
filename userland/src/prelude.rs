extern crate alloc;

pub use alloc::boxed::Box;
pub use alloc::format;
pub use alloc::string::{String, ToString};
pub use alloc::vec;
pub use alloc::vec::Vec;

pub use abi::{PropKey, PropType, PropValue, ThingId};

pub use userland_rt::Sys;

pub use userland_std::{
    Thing,
    println,
    register_schema_for,
    time,
    // Add other common exports as needed
    create_thing,
    load_thing,
    list_things_by_kind,
    create_process,
    create_thread,
    alloc_frame,
    free_frame,
    update_props,
    memory_summary,
    scheduler_summary,
};

/// Convenience logging helper for dynamic Strings
pub fn log_dynamic(sys: &impl Sys, msg: String) {
    // LEAK: To satisfy the ABI's 'static requirements for now.
    let leaked: &'static str = Box::leak(msg.into_boxed_str());
    println(sys, leaked);
}
