//! CSS Graph Model constants.

pub mod kinds {
    pub use abi::schema::kinds::{
        CSS_AT_RULE as AT_RULE, CSS_DECLARATION as DECLARATION, CSS_RULE as RULE,
        CSS_SELECTOR as SELECTOR, CSS_STYLESHEET as STYLESHEET,
    };
}

pub mod rels {
    pub use abi::schema::rels::{
        DERIVED_FROM,
        HAS_CHILD, // Rule -> Selector, Rule -> Declaration
    };
}

pub mod props {
    pub use abi::schema::keys::{
        CSS_AT_RULE as AT_RULE_NAME, CSS_ORDER as ORDER, CSS_PRELUDE as PRELUDE,
        CSS_PROPERTY as PROPERTY, CSS_SELECTOR_TEXT as SELECTOR_TEXT, CSS_VALUE as VALUE, NAME,
        SOURCE,
    };
}
