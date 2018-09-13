/// Validates that the 2 given fields match.
/// Both fields are optionals
pub fn validate_must_match<T: Eq>(a: T, b: T) -> bool {
    a == b
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::validate_must_match;

    #[test]
    fn test_validate_must_match_strings_valid() {
        assert!(validate_must_match("hey".to_string(), "hey".to_string()))
    }

    #[test]
    fn test_validate_must_match_cows_valid() {
        let left: Cow<'static, str> = "hey".into();
        let right: Cow<'static, str> = String::from("hey").into();
        assert!(validate_must_match(left, right))
    }

    #[test]
    fn test_validate_must_match_numbers() {
        assert!(validate_must_match(2, 2))
    }

    #[test]
    fn test_validate_must_match_numbers_false() {
        assert_eq!(false, validate_must_match(2, 3));
    }
}
