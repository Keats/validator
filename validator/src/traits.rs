use std::borrow::Cow;
use std::collections::HashMap;

use crate::types::ValidationErrors;

/// Trait to implement if one wants to make the `length` validator
/// work for more types
///
/// A bit sad it's not there by default in Rust
pub trait HasLen {
    fn length(&self) -> u64;
}

impl HasLen for String {
    fn length(&self) -> u64 {
        self.chars().count() as u64
    }
}

impl<'a> HasLen for &'a String {
    fn length(&self) -> u64 {
        self.chars().count() as u64
    }
}

impl<'a> HasLen for &'a str {
    fn length(&self) -> u64 {
        self.chars().count() as u64
    }
}

impl<'a> HasLen for Cow<'a, str> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

impl<T> HasLen for Vec<T> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}
impl<'a, T> HasLen for &'a Vec<T> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

/// Trait to implement if one wants to make the `contains` validator
/// work for more types
pub trait Contains {
    #[must_use]
    fn has_element(&self, needle: &str) -> bool;
}

impl Contains for String {
    fn has_element(&self, needle: &str) -> bool {
        self.contains(needle)
    }
}

impl<'a> Contains for &'a String {
    fn has_element(&self, needle: &str) -> bool {
        self.contains(needle)
    }
}

impl<'a> Contains for &'a str {
    fn has_element(&self, needle: &str) -> bool {
        self.contains(needle)
    }
}

impl<'a> Contains for Cow<'a, str> {
    fn has_element(&self, needle: &str) -> bool {
        self.contains(needle)
    }
}

impl<S, H: ::std::hash::BuildHasher> Contains for HashMap<String, S, H> {
    fn has_element(&self, needle: &str) -> bool {
        self.contains_key(needle)
    }
}

impl<'a, S, H: ::std::hash::BuildHasher> Contains for &'a HashMap<String, S, H> {
    fn has_element(&self, needle: &str) -> bool {
        self.contains_key(needle)
    }
}

/// The trait that `validator_derive` implements
pub trait Validate {
    fn validate(&self) -> Result<(), ValidationErrors>;
}

impl<T: Validate> Validate for &T {
    fn validate(&self) -> Result<(), ValidationErrors> {
        T::validate(*self)
    }
}
