use crate::traits::StartsWith;

/// Validates whether the value starts_with the needle
/// The value needs to implement the StartsWith trait, which is implement on String and str
/// by default.
#[must_use]
pub fn validate_starts_with<T: StartsWith>(val: T, needle: &str) -> bool {
    val.has_element(needle)
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;

    #[test]
    fn test_validate_starts_with_string() {
        assert!(validate_starts_with("hey", "h"));
    }

    #[test]
    fn test_validate_starts_with_string_can_fail() {
        assert!(!validate_starts_with("hey", "e"));
        assert!(!validate_starts_with("hey", "y"));
        assert!(!validate_starts_with("hey", "o"));
    }

    #[test]
    fn test_validate_starts_with_cow() {
        let test: Cow<'static, str> = "hey".into();
        assert!(validate_starts_with(test, "h"));
        let test: Cow<'static, str> = String::from("hey").into();
        assert!(validate_starts_with(test, "h"));
    }

    #[test]
    fn test_validate_starts_with_cow_can_fail() {
        let test: Cow<'static, str> = "hey".into();
        assert!(!validate_starts_with(test, "e"));
        let test: Cow<'static, str> = String::from("hey").into();
        assert!(!validate_starts_with(test, "e"));
    }
}
