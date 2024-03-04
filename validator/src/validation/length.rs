use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
};

#[cfg(feature = "indexmap")]
use indexmap::{IndexMap, IndexSet};

/// Validates the length of the value given.
/// If the validator has `equal` set, it will ignore any `min` and `max` value.
///
/// If you apply it on String, don't forget that the length can be different
/// from the number of visual characters for Unicode
#[must_use]
pub fn validate_length<T: ValidateLength>(
    value: T,
    min: Option<u64>,
    max: Option<u64>,
    equal: Option<u64>,
) -> bool {
    value.validate_length(min, max, equal)
}

pub trait ValidateLength {
    fn validate_length(&self, min: Option<u64>, max: Option<u64>, equal: Option<u64>) -> bool {
        let length = self.length();

        if let Some(eq) = equal {
            return length == eq;
        } else {
            if let Some(m) = min {
                if length < m {
                    return false;
                }
            }
            if let Some(m) = max {
                if length > m {
                    return false;
                }
            }
        }

        true
    }

    fn length(&self) -> u64;
}

impl ValidateLength for String {
    fn length(&self) -> u64 {
        self.chars().count() as u64
    }
}

impl<'a> ValidateLength for &'a String {
    fn length(&self) -> u64 {
        self.chars().count() as u64
    }
}

impl<'a> ValidateLength for &'a str {
    fn length(&self) -> u64 {
        self.chars().count() as u64
    }
}

impl<'a> ValidateLength for Cow<'a, str> {
    fn length(&self) -> u64 {
        self.chars().count() as u64
    }
}

impl<T> ValidateLength for Vec<T> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

impl<'a, T> ValidateLength for &'a Vec<T> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

impl<T> ValidateLength for &[T] {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

impl<T, const N: usize> ValidateLength for [T; N] {
    fn length(&self) -> u64 {
        N as u64
    }
}

impl<T, const N: usize> ValidateLength for &[T; N] {
    fn length(&self) -> u64 {
        N as u64
    }
}

impl<'a, K, V, S> ValidateLength for &'a HashMap<K, V, S> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

impl<K, V, S> ValidateLength for HashMap<K, V, S> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

impl<'a, T, S> ValidateLength for &'a HashSet<T, S> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

impl<'a, K, V> ValidateLength for &'a BTreeMap<K, V> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

impl<'a, T> ValidateLength for &'a BTreeSet<T> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

impl<T> ValidateLength for BTreeSet<T> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

#[cfg(feature = "indexmap")]
impl<'a, K, V> ValidateLength for &'a IndexMap<K, V> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

#[cfg(feature = "indexmap")]
impl<'a, T> ValidateLength for &'a IndexSet<T> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

#[cfg(feature = "indexmap")]
impl<T> ValidateLength for IndexSet<T> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::{validate_length, validation::length::ValidateLength};

    #[test]
    fn test_validate_length_equal_overrides_min_max() {
        assert!(validate_length("hello", Some(1), Some(2), Some(5)));
    }

    #[test]
    fn test_validate_length_string_min_max() {
        assert!(validate_length("hello", Some(1), Some(10), None));
    }

    #[test]
    fn test_validate_length_string_min_only() {
        assert!(!validate_length("hello", Some(10), None, None));
    }

    #[test]
    fn test_validate_length_string_max_only() {
        assert!(!validate_length("hello", None, Some(1), None));
    }

    #[test]
    fn test_validate_length_cow() {
        let test: Cow<'static, str> = "hello".into();
        assert!(validate_length(test, None, None, Some(5)));

        let test: Cow<'static, str> = String::from("hello").into();
        assert!(validate_length(test, None, None, Some(5)));
    }

    #[test]
    fn test_validate_length_vec() {
        assert!(validate_length(vec![1, 2, 3], None, None, Some(3)));
    }

    #[test]
    fn test_validate_length_unicode_chars() {
        assert!(validate_length("日本", None, None, Some(2)));
    }

    #[test]
    fn test_validate_length_trait_equal_overrides_min_max() {
        assert!(String::from("hello").validate_length(Some(1), Some(2), Some(5)));
    }

    #[test]
    fn test_validate_length_trait_string_min_max() {
        assert!(String::from("hello").validate_length(Some(1), Some(10), None));
    }

    #[test]
    fn test_validate_length_trait_string_min_only() {
        assert!(!String::from("hello").validate_length(Some(10), None, None));
    }

    #[test]
    fn test_validate_length_trait_string_max_only() {
        assert!(!String::from("hello").validate_length(None, Some(1), None));
    }

    #[test]
    fn test_validate_length_trait_cow() {
        let test: Cow<'static, str> = "hello".into();
        assert!(test.validate_length(None, None, Some(5)));

        let test: Cow<'static, str> = String::from("hello").into();
        assert!(test.validate_length(None, None, Some(5)));
    }

    #[test]
    fn test_validate_length_trait_vec() {
        assert!(vec![1, 2, 3].validate_length(None, None, Some(3)));
    }

    #[test]
    fn test_validate_length_trait_unicode_chars() {
        assert!(String::from("日本").validate_length(None, None, Some(2)));
    }
}
