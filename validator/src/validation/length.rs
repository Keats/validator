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
pub trait ValidateLength<T>
where
    T: PartialEq + PartialOrd,
{
    fn validate_length(&self, min: Option<T>, max: Option<T>, equal: Option<T>) -> bool {
        if let Some(length) = self.length() {
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
        } else {
            true
        }
    }

    fn length(&self) -> Option<T>;
}

impl ValidateLength<u64> for String {
    fn length(&self) -> Option<u64> {
        Some(self.chars().count() as u64)
    }
}

impl ValidateLength<u64> for Option<String> {
    fn length(&self) -> Option<u64> {
        self.as_ref().map(|s| s.chars().count() as u64)
    }
}

impl ValidateLength<u64> for Option<Option<String>> {
    fn length(&self) -> Option<u64> {
        if let Some(s) = self {
            s.as_ref().map(|s| s.chars().count() as u64)
        } else {
            None
        }
    }
}

impl<'a> ValidateLength<u64> for &'a String {
    fn length(&self) -> Option<u64> {
        Some(self.chars().count() as u64)
    }
}

impl<'a> ValidateLength<u64> for Option<&'a String> {
    fn length(&self) -> Option<u64> {
        self.as_ref().map(|s| s.chars().count() as u64)
    }
}

impl<'a> ValidateLength<u64> for Option<Option<&'a String>> {
    fn length(&self) -> Option<u64> {
        self.flatten().map(|s| s.chars().count() as u64)
    }
}

impl<'a> ValidateLength<u64> for &'a str {
    fn length(&self) -> Option<u64> {
        Some(self.chars().count() as u64)
    }
}

impl<'a> ValidateLength<u64> for Option<&'a str> {
    fn length(&self) -> Option<u64> {
        self.as_ref().map(|s| s.chars().count() as u64)
    }
}

impl<'a> ValidateLength<u64> for Option<Option<&'a str>> {
    fn length(&self) -> Option<u64> {
        self.flatten().map(|s| s.chars().count() as u64)
    }
}

impl<'a> ValidateLength<u64> for Cow<'a, str> {
    fn length(&self) -> Option<u64> {
        Some(self.chars().count() as u64)
    }
}

impl<'a> ValidateLength<u64> for Option<Cow<'a, str>> {
    fn length(&self) -> Option<u64> {
        self.as_ref().map(|s| s.chars().count() as u64)
    }
}

impl<'a> ValidateLength<u64> for Option<Option<Cow<'a, str>>> {
    fn length(&self) -> Option<u64> {
        if let Some(s) = self {
            s.as_ref().map(|s| s.chars().count() as u64)
        } else {
            None
        }
    }
}

impl<T> ValidateLength<u64> for Vec<T> {
    fn length(&self) -> Option<u64> {
        Some(self.len() as u64)
    }
}

impl<T> ValidateLength<u64> for Option<Vec<T>> {
    fn length(&self) -> Option<u64> {
        self.as_ref().map(|v| v.len() as u64)
    }
}

impl<T> ValidateLength<u64> for Option<Option<Vec<T>>> {
    fn length(&self) -> Option<u64> {
        if let Some(v) = self {
            v.as_ref().map(|v| v.len() as u64)
        } else {
            None
        }
    }
}

impl<'a, T> ValidateLength<u64> for &'a Vec<T> {
    fn length(&self) -> Option<u64> {
        Some(self.len() as u64)
    }
}

impl<'a, T> ValidateLength<u64> for Option<&'a Vec<T>> {
    fn length(&self) -> Option<u64> {
        self.as_ref().map(|v| v.len() as u64)
    }
}

impl<'a, T> ValidateLength<u64> for Option<Option<&'a Vec<T>>> {
    fn length(&self) -> Option<u64> {
        if let Some(v) = self {
            v.as_ref().map(|v| v.len() as u64)
        } else {
            None
        }
    }
}

impl<T> ValidateLength<u64> for &[T] {
    fn length(&self) -> Option<u64> {
        Some(self.len() as u64)
    }
}

impl<T> ValidateLength<u64> for Option<&[T]> {
    fn length(&self) -> Option<u64> {
        self.as_ref().map(|v| v.len() as u64)
    }
}

impl<T> ValidateLength<u64> for Option<Option<&[T]>> {
    fn length(&self) -> Option<u64> {
        if let Some(v) = self {
            v.as_ref().map(|v| v.len() as u64)
        } else {
            None
        }
    }
}

impl<T, const N: usize> ValidateLength<u64> for [T; N] {
    fn length(&self) -> Option<u64> {
        Some(N as u64)
    }
}

impl<T, const N: usize> ValidateLength<u64> for Option<[T; N]> {
    fn length(&self) -> Option<u64> {
        Some(N as u64)
    }
}

impl<T, const N: usize> ValidateLength<u64> for Option<Option<[T; N]>> {
    fn length(&self) -> Option<u64> {
        Some(N as u64)
    }
}

impl<T, const N: usize> ValidateLength<u64> for &[T; N] {
    fn length(&self) -> Option<u64> {
        Some(N as u64)
    }
}

impl<T, const N: usize> ValidateLength<u64> for Option<&[T; N]> {
    fn length(&self) -> Option<u64> {
        Some(N as u64)
    }
}

impl<T, const N: usize> ValidateLength<u64> for Option<Option<&[T; N]>> {
    fn length(&self) -> Option<u64> {
        Some(N as u64)
    }
}

impl<K, V, S> ValidateLength<u64> for HashMap<K, V, S> {
    fn length(&self) -> Option<u64> {
        Some(self.len() as u64)
    }
}

impl<K, V, S> ValidateLength<u64> for Option<HashMap<K, V, S>> {
    fn length(&self) -> Option<u64> {
        self.as_ref().map(|v| v.len() as u64)
    }
}

impl<K, V, S> ValidateLength<u64> for Option<Option<HashMap<K, V, S>>> {
    fn length(&self) -> Option<u64> {
        if let Some(v) = self {
            v.as_ref().map(|v| v.len() as u64)
        } else {
            None
        }
    }
}

impl<'a, K, V, S> ValidateLength<u64> for &'a HashMap<K, V, S> {
    fn length(&self) -> Option<u64> {
        Some(self.len() as u64)
    }
}

impl<'a, K, V, S> ValidateLength<u64> for Option<&'a HashMap<K, V, S>> {
    fn length(&self) -> Option<u64> {
        self.as_ref().map(|v| v.len() as u64)
    }
}

impl<'a, K, V, S> ValidateLength<u64> for Option<Option<&'a HashMap<K, V, S>>> {
    fn length(&self) -> Option<u64> {
        if let Some(v) = self {
            v.as_ref().map(|v| v.len() as u64)
        } else {
            None
        }
    }
}

impl<T, S> ValidateLength<u64> for HashSet<T, S> {
    fn length(&self) -> Option<u64> {
        Some(self.len() as u64)
    }
}

impl<T, S> ValidateLength<u64> for Option<HashSet<T, S>> {
    fn length(&self) -> Option<u64> {
        self.as_ref().map(|v| v.len() as u64)
    }
}

impl<T, S> ValidateLength<u64> for Option<Option<HashSet<T, S>>> {
    fn length(&self) -> Option<u64> {
        if let Some(v) = self {
            v.as_ref().map(|v| v.len() as u64)
        } else {
            None
        }
    }
}

impl<'a, T, S> ValidateLength<u64> for &'a HashSet<T, S> {
    fn length(&self) -> Option<u64> {
        Some(self.len() as u64)
    }
}

impl<'a, T, S> ValidateLength<u64> for Option<&'a HashSet<T, S>> {
    fn length(&self) -> Option<u64> {
        self.as_ref().map(|v| v.len() as u64)
    }
}

impl<'a, T, S> ValidateLength<u64> for Option<Option<&'a HashSet<T, S>>> {
    fn length(&self) -> Option<u64> {
        if let Some(v) = self {
            v.as_ref().map(|v| v.len() as u64)
        } else {
            None
        }
    }
}

impl<'a, K, V> ValidateLength<u64> for &'a BTreeMap<K, V> {
    fn length(&self) -> Option<u64> {
        Some(self.len() as u64)
    }
}

impl<'a, K, V> ValidateLength<u64> for Option<&'a BTreeMap<K, V>> {
    fn length(&self) -> Option<u64> {
        self.as_ref().map(|v| v.len() as u64)
    }
}

impl<'a, K, V> ValidateLength<u64> for Option<Option<&'a BTreeMap<K, V>>> {
    fn length(&self) -> Option<u64> {
        if let Some(v) = self {
            v.as_ref().map(|v| v.len() as u64)
        } else {
            None
        }
    }
}

impl<T> ValidateLength<u64> for BTreeSet<T> {
    fn length(&self) -> Option<u64> {
        Some(self.len() as u64)
    }
}

impl<T> ValidateLength<u64> for Option<BTreeSet<T>> {
    fn length(&self) -> Option<u64> {
        self.as_ref().map(|v| v.len() as u64)
    }
}

impl<T> ValidateLength<u64> for Option<Option<BTreeSet<T>>> {
    fn length(&self) -> Option<u64> {
        if let Some(v) = self {
            v.as_ref().map(|v| v.len() as u64)
        } else {
            None
        }
    }
}

impl<'a, T> ValidateLength<u64> for &'a BTreeSet<T> {
    fn length(&self) -> Option<u64> {
        Some(self.len() as u64)
    }
}

impl<'a, T> ValidateLength<u64> for Option<&'a BTreeSet<T>> {
    fn length(&self) -> Option<u64> {
        self.as_ref().map(|v| v.len() as u64)
    }
}

impl<'a, T> ValidateLength<u64> for Option<Option<&'a BTreeSet<T>>> {
    fn length(&self) -> Option<u64> {
        if let Some(v) = self {
            v.as_ref().map(|v| v.len() as u64)
        } else {
            None
        }
    }
}

#[cfg(feature = "indexmap")]
impl<K, V> ValidateLength<u64> for IndexMap<K, V> {
    fn length(&self) -> Option<u64> {
        Some(self.len() as u64)
    }
}

#[cfg(feature = "indexmap")]
impl<K, V> ValidateLength<u64> for Option<IndexMap<K, V>> {
    fn length(&self) -> Option<u64> {
        self.as_ref().map(|v| v.len() as u64)
    }
}

#[cfg(feature = "indexmap")]
impl<K, V> ValidateLength<u64> for Option<Option<IndexMap<K, V>>> {
    fn length(&self) -> Option<u64> {
        if let Some(v) = self {
            v.as_ref().map(|v| v.len() as u64)
        } else {
            None
        }
    }
}

#[cfg(feature = "indexmap")]
impl<'a, K, V> ValidateLength<u64> for &'a IndexMap<K, V> {
    fn length(&self) -> Option<u64> {
        Some(self.len() as u64)
    }
}

#[cfg(feature = "indexmap")]
impl<'a, K, V> ValidateLength<u64> for Option<&'a IndexMap<K, V>> {
    fn length(&self) -> Option<u64> {
        self.as_ref().map(|v| v.len() as u64)
    }
}

#[cfg(feature = "indexmap")]
impl<'a, K, V> ValidateLength<u64> for Option<Option<&'a IndexMap<K, V>>> {
    fn length(&self) -> Option<u64> {
        if let Some(v) = self {
            v.as_ref().map(|v| v.len() as u64)
        } else {
            None
        }
    }
}

#[cfg(feature = "indexmap")]
impl<T> ValidateLength<u64> for IndexSet<T> {
    fn length(&self) -> Option<u64> {
        Some(self.len() as u64)
    }
}

#[cfg(feature = "indexmap")]
impl<T> ValidateLength<u64> for Option<IndexSet<T>> {
    fn length(&self) -> Option<u64> {
        self.as_ref().map(|v| v.len() as u64)
    }
}

#[cfg(feature = "indexmap")]
impl<T> ValidateLength<u64> for Option<Option<IndexSet<T>>> {
    fn length(&self) -> Option<u64> {
        if let Some(v) = self {
            v.as_ref().map(|v| v.len() as u64)
        } else {
            None
        }
    }
}

#[cfg(feature = "indexmap")]
impl<'a, T> ValidateLength<u64> for &'a IndexSet<T> {
    fn length(&self) -> Option<u64> {
        Some(self.len() as u64)
    }
}

#[cfg(feature = "indexmap")]
impl<'a, T> ValidateLength<u64> for Option<&'a IndexSet<T>> {
    fn length(&self) -> Option<u64> {
        self.as_ref().map(|v| v.len() as u64)
    }
}

#[cfg(feature = "indexmap")]
impl<'a, T> ValidateLength<u64> for Option<Option<&'a IndexSet<T>>> {
    fn length(&self) -> Option<u64> {
        if let Some(v) = self {
            v.as_ref().map(|v| v.len() as u64)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::ValidateLength;

    #[test]
    fn test_validate_length_equal_overrides_min_max() {
        assert!("hello".validate_length(Some(1), Some(2), Some(5)));
    }

    #[test]
    fn test_validate_length_string_min_max() {
        assert!("hello".validate_length(Some(1), Some(10), None));
    }

    #[test]
    fn test_validate_length_string_min_only() {
        assert!(!"hello".validate_length(Some(10), None, None));
    }

    #[test]
    fn test_validate_length_string_max_only() {
        assert!(!"hello".validate_length(None, Some(1), None));
    }

    #[test]
    fn test_validate_length_cow() {
        let test: Cow<'static, str> = "hello".into();
        assert!(test.validate_length(None, None, Some(5)));

        let test: Cow<'static, str> = String::from("hello").into();
        assert!(test.validate_length(None, None, Some(5)));
    }

    #[test]
    fn test_validate_length_vec() {
        assert!(vec![1, 2, 3].validate_length(None, None, Some(3)));
    }

    #[test]
    fn test_validate_length_unicode_chars() {
        assert!("日本".validate_length(None, None, Some(2)));
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
