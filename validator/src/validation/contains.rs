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

impl<T> ValidateContains for Option<T>
where
    T: ValidateContains,
{
    fn validate_contains(&self, needle: &str) -> bool {
        if let Some(v) = self {
            v.validate_contains(needle)
        } else {
            true
        }
    }
}

impl<T> ValidateContains for &T
where
    T: ValidateContains,
{
    fn validate_contains(&self, needle: &str) -> bool {
        T::validate_contains(self, needle)
    }
}

impl<'cow, T> ValidateContains for Cow<'cow, T>
where
    T: ToOwned + ?Sized,
    for<'a> &'a T: ValidateContains,
{
    fn validate_contains(&self, needle: &str) -> bool {
        self.as_ref().validate_contains(needle)
    }
}

impl<'a> ValidateContains for &'a str {
    fn validate_contains(&self, needle: &str) -> bool {
        self.contains(needle)
    }
}

impl<S, H: BuildHasher> ValidateContains for HashMap<String, S, H> {
    fn validate_contains(&self, needle: &str) -> bool {
        self.contains_key(needle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_contains_string() {
        assert!("hey".validate_contains("e"));
    }

    #[test]
    fn test_validate_contains_string_can_fail() {
        assert!(!"hey".validate_contains("o"));
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
