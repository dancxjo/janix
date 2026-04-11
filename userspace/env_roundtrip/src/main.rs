//! Smoke test: set / get / list / unset env vars via SYS_ENV_*.
//!
//! Acceptance criteria:
//!   - `std::env::set_var` stores a value
//!   - `std::env::var` retrieves it
//!   - `std::env::vars()` includes the new variable
//!   - `std::env::remove_var` removes it
//!   - subsequent `std::env::var` returns Err
#![feature(restricted_std)]

fn main() {
    let key = "THINGOS_TEST_VAR";
    let value = "roundtrip_ok";

    // set
    // SAFETY: single-threaded, no concurrent env access
    unsafe { std::env::set_var(key, value) };
    println!("[env_roundtrip] set {}={:?}", key, value);

    // get
    match std::env::var(key) {
        Ok(v) if v == value => println!("[env_roundtrip] get OK: {:?}", v),
        Ok(v) => {
            eprintln!("[env_roundtrip] FAIL: expected {:?} got {:?}", value, v);
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("[env_roundtrip] FAIL: var not found: {}", e);
            std::process::exit(1);
        }
    }

    // list
    let vars: Vec<_> = std::env::vars().collect();
    let found = vars.iter().any(|(k, v)| k == key && v == value);
    if found {
        println!("[env_roundtrip] list OK: found key in {} vars", vars.len());
    } else {
        eprintln!("[env_roundtrip] FAIL: key not found in vars() (total: {})", vars.len());
        for (k, v) in &vars {
            eprintln!("  {}={:?}", k, v);
        }
        std::process::exit(1);
    }

    // unset
    // SAFETY: single-threaded
    unsafe { std::env::remove_var(key) };
    match std::env::var(key) {
        Err(_) => println!("[env_roundtrip] unset OK"),
        Ok(v) => {
            eprintln!("[env_roundtrip] FAIL: var still present after remove: {:?}", v);
            std::process::exit(1);
        }
    }

    println!("[env_roundtrip] PASS");
    std::process::exit(0);
}
