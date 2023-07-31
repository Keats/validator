use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::BuildHasher;

pub trait ValidateContains {
    fn validate_contains(&self, needle: &str) -> bool;
}

impl ValidateContains for String {
    fn validate_contains(&self, needle: &str) -> bool {
        self.contains(needle)
    }
}

impl ValidateContains for &String {
    fn validate_contains(&self, needle: &str) -> bool {
        self.contains(needle)
    }
}

impl<'a> ValidateContains for &'a str {
    fn validate_contains(&self, needle: &str) -> bool {
        self.contains(needle)
    }
}

impl<'a> ValidateContains for Cow<'a, str> {
    fn validate_contains(&self, needle: &str) -> bool {
        self.contains(needle)
    }
}

impl<S, H: BuildHasher> ValidateContains for HashMap<String, S, H> {
    fn validate_contains(&self, needle: &str) -> bool {
        self.contains_key(needle)
    }
}

impl<'a, S, H: BuildHasher> ValidateContains for &'a HashMap<String, S, H> {
    fn validate_contains(&self, needle: &str) -> bool {
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
        assert!("hey".validate_contains("e"));
    }

    #[test]
    fn test_validate_contains_string_can_fail() {
        assert!("hey".validate_contains("o"));
    }

    #[test]
    fn test_validate_contains_hashmap_key() {
        let mut map = HashMap::new();
        map.insert("hey".to_string(), 1);
        assert!(map.validate_contains("hey"));
    }

    #[test]
    fn test_validate_contains_hashmap_key_can_fail() {
        let mut map = HashMap::new();
        map.insert("hey".to_string(), 1);
        assert!(!map.validate_contains("bob"));
    }

    #[test]
    fn test_validate_contains_cow() {
        let test: Cow<'static, str> = "hey".into();
        assert!(test.validate_contains("e"));
        let test: Cow<'static, str> = String::from("hey").into();
        assert!(test.validate_contains("e"));
    }

    #[test]
    fn test_validate_contains_cow_can_fail() {
        let test: Cow<'static, str> = "hey".into();
        assert!(!test.validate_contains("o"));
        let test: Cow<'static, str> = String::from("hey").into();
        assert!(!test.validate_contains("o"));
    }
}
