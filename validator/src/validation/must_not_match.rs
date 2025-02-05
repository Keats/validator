/// Validates that the 2 given fields do not match.
/// Both fields are optionals
#[must_use]
pub fn validate_must_not_match<T: PartialEq>(a: T, b: T) -> bool {
    a != b
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::validate_must_not_match;

    #[test]
    fn test_validate_must_not_match_strings_valid() {
        assert!(validate_must_not_match("hey".to_string(), "ho".to_string()))
    }

    #[test]
    fn test_validate_must_not_match_cows_valid() {
        let left: Cow<'static, str> = "hey".into();
        let right: Cow<'static, str> = String::from("ho").into();
        assert!(validate_must_not_match(left, right))
    }

    #[test]
    fn test_validate_must_not_match_numbers() {
        assert!(validate_must_not_match(2, 3))
    }

    #[test]
    fn test_validate_must_not_match_numbers_false() {
        assert_eq!(false, validate_must_not_match(2, 2));
    }

    #[test]
    fn test_validate_must_not_match_numbers_option_false() {
        assert_eq!(false, validate_must_not_match(Some(2), Some(2)));
    }

    #[test]
    fn test_validate_must_not_match_numbers_option_true() {
        assert!(validate_must_not_match(Some(6), Some(7)));
    }

    #[test]
    fn test_validate_must_not_match_none_some() {
        assert!(validate_must_not_match(None, Some(3)));
    }

    #[test]
    fn test_validate_must_not_match_some_none() {
        assert!(validate_must_not_match(Some(3), None));
    }

    #[test]
    fn test_validate_must_not_match_none_none_false() {
        // We need to define one of the values here as rust
        // can not infer the generic type from None and None
        let a: Option<u64> = None;
        assert_eq!(false, validate_must_not_match(a, None));
    }
}