use std::borrow::Cow;
use std::collections::HashMap;

/// Validates whether the value contains the needle
/// The value needs to implement the Contains trait, which is implement on String, str and Hashmap<String>
/// by default.
#[must_use]
pub fn validate_contains<T: Contains>(val: T, needle: &str) -> bool {
    val.has_element(needle)
}

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

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_validate_contains_string() {
        assert!(validate_contains("hey", "e"));
    }

    #[test]
    fn test_validate_contains_string_can_fail() {
        assert!(!validate_contains("hey", "o"));
    }

    #[test]
    fn test_validate_contains_hashmap_key() {
        let mut map = HashMap::new();
        map.insert("hey".to_string(), 1);
        assert!(validate_contains(map, "hey"));
    }

    #[test]
    fn test_validate_contains_hashmap_key_can_fail() {
        let mut map = HashMap::new();
        map.insert("hey".to_string(), 1);
        assert!(!validate_contains(map, "bob"));
    }

    #[test]
    fn test_validate_contains_cow() {
        let test: Cow<'static, str> = "hey".into();
        assert!(validate_contains(test, "e"));
        let test: Cow<'static, str> = String::from("hey").into();
        assert!(validate_contains(test, "e"));
    }

    #[test]
    fn test_validate_contains_cow_can_fail() {
        let test: Cow<'static, str> = "hey".into();
        assert!(!validate_contains(test, "o"));
        let test: Cow<'static, str> = String::from("hey").into();
        assert!(!validate_contains(test, "o"));
    }
}
