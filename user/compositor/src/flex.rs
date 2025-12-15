extern crate alloc;

use alloc::string::{String, ToString};
use core::str::FromStr;

use thing_os::prelude::PropValue;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexDirection {
    Row,
    Column,
}
impl FlexDirection {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Row => "row",
            Self::Column => "column",
        }
    }
    pub fn from_prop(v: &PropValue) -> Option<Self> {
        prop_as_str(v).and_then(|s| Self::from_str(s).ok())
    }
}
impl Default for FlexDirection {
    fn default() -> Self { Self::Column }
}
impl FromStr for FlexDirection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "row" => Ok(Self::Row),
            "column" => Ok(Self::Column),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
}
impl FlexWrap {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoWrap => "nowrap",
            Self::Wrap => "wrap",
        }
    }
    pub fn from_prop(v: &PropValue) -> Option<Self> {
        prop_as_str(v).and_then(|s| Self::from_str(s).ok())
    }
}
impl Default for FlexWrap {
    fn default() -> Self { Self::NoWrap }
}
impl FromStr for FlexWrap {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nowrap" => Ok(Self::NoWrap),
            "wrap" => Ok(Self::Wrap),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JustifyContent {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
}
impl JustifyContent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
            Self::SpaceBetween => "space-between",
            Self::SpaceAround => "space-around",
        }
    }
    pub fn from_prop(v: &PropValue) -> Option<Self> {
        prop_as_str(v).and_then(|s| Self::from_str(s).ok())
    }
}
impl Default for JustifyContent {
    fn default() -> Self { Self::Start }
}
impl FromStr for JustifyContent {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" | "flex-start" => Ok(Self::Start),
            "center" => Ok(Self::Center),
            "end" | "flex-end" => Ok(Self::End),
            "space-between" => Ok(Self::SpaceBetween),
            "space-around" => Ok(Self::SpaceAround),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignItems {
    Start,
    Center,
    End,
    Stretch,
}
impl AlignItems {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
            Self::Stretch => "stretch",
        }
    }
    pub fn from_prop(v: &PropValue) -> Option<Self> {
        prop_as_str(v).and_then(|s| Self::from_str(s).ok())
    }
}
impl Default for AlignItems {
    fn default() -> Self { Self::Stretch }
}
impl FromStr for AlignItems {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" | "flex-start" => Ok(Self::Start),
            "center" => Ok(Self::Center),
            "end" | "flex-end" => Ok(Self::End),
            "stretch" => Ok(Self::Stretch),
            _ => Err(()),
        }
    }
}

fn prop_as_str(v: &PropValue) -> Option<&str> {
    match v {
        PropValue::Str(s) => Some(s.as_str()),
        // If your PropValue has Bytes, add it here. Otherwise ignore.
        _ => None,
    }
}

/// Convenience for optional float stored as text.
pub fn prop_as_f32(v: &PropValue) -> Option<f32> {
    prop_as_str(v).and_then(|s| s.parse::<f32>().ok())
}
