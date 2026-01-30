//! XML Graph Model constants.

pub mod kinds {
    pub use abi::schema::kinds::{
        XML_ATTRIBUTE as ATTRIBUTE, XML_DOCUMENT as DOCUMENT, XML_ELEMENT as ELEMENT,
        XML_TEXT as TEXT,
    };
}

pub mod rels {
    pub use abi::schema::rels::{
        DERIVED_FROM, // Reuse existing
        HAS_ATTR,
        HAS_CHILD, // Reuse existing UI relation
        HAS_ROOT,
    };
}

pub mod props {
    pub use abi::schema::keys::{
        ATTR_NAME,
        ATTR_VALUE,
        NAME,   // Reuse name
        SOURCE, // Reuse source
        TAG,
        TEXT,
        XML_ORDER,
    };
}
