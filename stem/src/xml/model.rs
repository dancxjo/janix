//! XML Graph Model constants.

pub mod kinds {
    pub use abi::schema::kinds::{
        XML_DOCUMENT as DOCUMENT,
        XML_ELEMENT as ELEMENT,
        XML_ATTRIBUTE as ATTRIBUTE,
        XML_TEXT as TEXT,
    };
}

pub mod rels {
    pub use abi::schema::rels::{
        HAS_ROOT,
        HAS_CHILD, // Reuse existing UI relation
        HAS_ATTR,
        DERIVED_FROM, // Reuse existing
    };
}

pub mod props {
    pub use abi::schema::keys::{
        SOURCE, // Reuse source
        NAME,   // Reuse name
        TAG,
        TEXT,
        ATTR_NAME,
        ATTR_VALUE,
    };
}
