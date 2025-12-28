#![cfg_attr(test, allow(unused))]
#![doc(hidden)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![unstable(
    feature = "thread_local_internals",
    reason = "internal details of the thread_local macro",
    issue = "none"
)]

cfg_select! {
    any(
        all(target_family = "wasm", not(target_feature = "atomics")),
        target_os = "uefi",
        target_os = "zkvm",
        target_os = "trusty",
        target_os = "vexos",
        target_os = "thingos",
    ) => {
        mod no_threads;
        pub use no_threads::{EagerStorage, LazyStorage, thread_local_inner};
        pub(crate) use no_threads::{LocalPointer, local_pointer};
    }
    target_thread_local => {
        mod native;
        pub use native::{EagerStorage, LazyStorage, thread_local_inner};
        pub(crate) use native::{LocalPointer, local_pointer};
    }
    _ => {
        mod os;
        pub use os::{Storage, thread_local_inner, value_align};
        pub(crate) use os::{LocalPointer, local_pointer};
    }
}

// destructors, guard, key modules are usually cfg-gated or fall back.
// If we use no_threads, we usually don't need destructors/guard/key?
// Let's check the original file.
// `mod destructors` is guarded by `target_thread_local` AND NOT wasm-no-atomics.
// `mod guard`: guarded by cfg_select.
// `mod key`: guarded by cfg_select.

// We need to provide dummy `guard` and `key` for no_threads?
// `sys/thread_local/no_threads.rs` usually is self-contained.
// But `sys/thread_local/mod.rs` defines the modules.

#[cfg(all(target_thread_local, not(all(target_family = "wasm", not(target_feature = "atomics")))))]
pub(crate) mod destructors {
    // ...
    cfg_select! {
        // ...
        _ => {
            mod list;
            pub(super) use list::register;
            pub(crate) use list::run;
        }
    }
}

pub(crate) mod guard {
    cfg_select! {
        // ...
        any(
            all(target_family = "wasm", not(
                all(target_os = "wasi", target_env = "p1", target_feature = "atomics")
            )),
            target_os = "uefi",
            target_os = "zkvm",
            target_os = "trusty",
            target_os = "vexos",
            target_os = "thingos",
        ) => {
            pub(crate) fn enable() {
            }
        }
        // ...
        _ => {
            mod key;
            pub(crate) use key::enable;
        }
    }
}

pub(crate) mod key {
    cfg_select! {
        // ...
        _ => {}
    }
}

#[inline]
#[allow(dead_code)]
fn abort_on_dtor_unwind(f: impl FnOnce()) {
    let guard = DtorUnwindGuard;
    f();
    core::mem::forget(guard);

    struct DtorUnwindGuard;
    impl Drop for DtorUnwindGuard {
        #[inline]
        fn drop(&mut self) {
            rtabort!("thread local panicked on drop");
        }
    }
}
