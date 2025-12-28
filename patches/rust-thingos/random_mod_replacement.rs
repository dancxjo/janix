cfg_select! {
    // ... (omitted)
    any(
        all(target_family = "wasm", target_os = "unknown"),
        target_os = "xous",
        target_os = "vexos",
        target_os = "thingos",
    ) => {
        mod unsupported;
        pub use unsupported::{fill_bytes, hashmap_random_keys};
    }
    _ => {}
}

#[cfg(not(any(
    target_os = "linux",
    target_os = "android",
    all(target_family = "wasm", target_os = "unknown"),
    all(target_os = "wasi", not(target_env = "p1")),
    target_os = "xous",
    target_os = "vexos",
    target_os = "thingos",
)))]
pub fn hashmap_random_keys() -> (u64, u64) {
    let mut buf = [0; 16];
    fill_bytes(&mut buf);
    let k1 = u64::from_ne_bytes(buf[..8].try_into().unwrap());
    let k2 = u64::from_ne_bytes(buf[8..].try_into().unwrap());
    (k1, k2)
}
