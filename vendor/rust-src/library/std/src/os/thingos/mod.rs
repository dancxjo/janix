
#![stable(feature = "rust1", since = "1.0.0")]

pub mod ffi {
    #![stable(feature = "rust1", since = "1.0.0")]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub trait OsStrExt: sealed::Sealed {
        #[stable(feature = "rust1", since = "1.0.0")]
        fn from_thingos_bytes(b: &[u8]) -> &Self;
        #[stable(feature = "rust1", since = "1.0.0")]
        fn as_thingos_bytes(&self) -> &[u8];
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    impl OsStrExt for crate::ffi::OsStr {
        fn from_thingos_bytes(b: &[u8]) -> &Self {
            use crate::sys::os_str::OsStrExt;
            OsStrExt::from_bytes(b)
        }
        fn as_thingos_bytes(&self) -> &[u8] {
            use crate::sys::os_str::OsStrExt;
            OsStrExt::as_bytes(self)
        }
    }

    mod sealed {
        #![stable(feature = "rust1", since = "1.0.0")]
        #[stable(feature = "rust1", since = "1.0.0")]
        pub trait Sealed {}
    }
    #[stable(feature = "rust1", since = "1.0.0")]
    impl sealed::Sealed for crate::ffi::OsStr {}
}
