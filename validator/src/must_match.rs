/// Validates that the 2 given fields match.
/// Both fields are optionals
pub fn validate_must_match<T: Eq>(
    a: T,
    a_name: &str,
    b: T,
    b_name: &str,
) -> Result<(), String> {
    if a == b {
        Ok(())
    } else {
        Err(format!("field '{}' must match field '{}'", a_name, b_name))
    }
}

#[cfg(test)]
mod tests {
    use super::{validate_must_match};

    #[test]
    fn test_validate_must_match_strings_valid() {
        assert!(validate_must_match("hey".to_string(), "a", "hey".to_string(), "b"))
    }

    #[test]
    fn test_validate_must_match_numbers() {
        assert!(validate_must_match(2, "a", 2, "b"))
    }

    #[test]
    fn test_validate_must_match_numbers_false() {
        assert_eq!(false, validate_must_match(2, "a", 3, "b"));
    }

}
