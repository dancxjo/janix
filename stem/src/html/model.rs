//! HTML Graph Model constants.

pub mod kinds {
    pub use abi::schema::kinds::{
        HTML_ATTRIBUTE as ATTRIBUTE, HTML_COMMENT as COMMENT, HTML_DOCUMENT as DOCUMENT,
        HTML_ELEMENT as ELEMENT, HTML_TEXT as TEXT,
    };
}

pub mod rels {
    pub use abi::schema::rels::{
        DERIVED_FROM, // Reuse existing
        HAS_ATTR,
        HAS_CHILD, // Reuse existing relation
        HAS_ROOT,
    };
}

pub mod props {
    pub use abi::schema::keys::{
        ATTR_NAME, ATTR_VALUE, HTML_ORDER as ORDER, HTML_TAG as TAG, NAME, SOURCE, TEXT,
    };
}
