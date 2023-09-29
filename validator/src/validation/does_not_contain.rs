use crate::ValidateContains;

pub trait ValidateDoesNotContain {
    fn validate_does_not_contain(&self, needle: &str) -> bool;
}

impl<T> ValidateDoesNotContain for T
where
    T: ValidateContains,
{
    fn validate_does_not_contain(&self, needle: &str) -> bool {
        !self.validate_contains(needle)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_validate_does_not_contain_string() {
        assert!("hey".validate_does_not_contain("g"));
    }

    #[test]
    fn test_validate_does_not_contain_string_can_fail() {
        assert!(!"hey".validate_does_not_contain("e"));
    }

    #[test]
    fn test_validate_does_not_contain_hashmap_key() {
        let mut map = HashMap::new();
        map.insert("hey".to_string(), 1);
        assert!(map.validate_does_not_contain("bob"));
    }

    #[test]
    fn test_validate_does_not_contain_hashmap_key_can_fail() {
        let mut map = HashMap::new();
        map.insert("hey".to_string(), 1);
        assert!(!map.validate_does_not_contain("hey"));
    }

    #[test]
    fn test_validate_does_not_contain_cow() {
        let test: Cow<'static, str> = "hey".into();
        assert!(test.validate_does_not_contain("b"));
        let test: Cow<'static, str> = String::from("hey").into();
        assert!(test.validate_does_not_contain("b"));
    }

    #[test]
    fn test_validate_does_not_contain_cow_can_fail() {
        let test: Cow<'static, str> = "hey".into();
        assert!(!test.validate_does_not_contain("e"));
        let test: Cow<'static, str> = String::from("hey").into();
        assert!(!test.validate_does_not_contain("e"));
    }
}
