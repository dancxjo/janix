#![forbid(unsafe_op_in_unsafe_fn)]

cfg_select! {
    any(target_family = "unix", target_os = "wasi") => {
        mod unix;
        pub use self::unix::*;
    }
    target_os = "windows" => {
        mod windows;
        pub use self::windows::*;
    }
    all(target_vendor = "fortanix", target_env = "sgx") => {
        mod sgx;
        pub use self::sgx::*;
    }
    target_os = "solid_asp3" => {
        mod solid;
        pub use self::solid::*;
    }
    target_os = "hermit" => {
        mod hermit;
        pub use self::hermit::*;
    }
    target_os = "motor" => {
        mod motor;
        pub use self::motor::*;
    }
    target_os = "teeos" => {
        mod teeos;
        pub use self::teeos::*;
    }
    target_os = "trusty" => {
        mod trusty;
        pub use self::trusty::*;
    }
    target_os = "uefi" => {
        mod uefi;
        pub use self::uefi::*;
    }
    target_os = "vexos" => {
        mod vexos;
        pub use self::vexos::*;
    }
    target_os = "xous" => {
        mod xous;
        pub use self::xous::*;
    }
    target_os = "zkvm" => {
        mod zkvm;
        pub use self::zkvm::*;
    }
    target_os = "thingos" => {
        mod thingos;
        pub use self::thingos::*;
    }
    _ => {
        mod unsupported;
        pub use self::unsupported::*;
    }
}
