use crate::traits::HasLen;
use crate::validation::Validator;

/// Validates the length of the value given.
/// If the validator has `equal` set, it will ignore any `min` and `max` value.
///
/// If you apply it on String, don't forget that the length can be different
/// from the number of visual characters for Unicode
#[must_use]
pub fn validate_length<T: HasLen>(length: Validator, val: T) -> bool {
    match length {
        Validator::Length { min, max, equal } => {
            let val_length = val.length();
            if let Some(eq) = equal {
                return val_length == eq;
            }
            if let Some(m) = min {
                if val_length < m {
                    return false;
                }
            }
            if let Some(m) = max {
                if val_length > m {
                    return false;
                }
            }
        }
        _ => unreachable!(),
    }

    true
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::{validate_length, Validator};

    #[test]
    fn test_validate_length_equal_overrides_min_max() {
        let validator = Validator::Length { min: Some(1), max: Some(2), equal: Some(5) };
        assert_eq!(validate_length(validator, "hello"), true);
    }

    #[test]
    fn test_validate_length_string_min_max() {
        let validator = Validator::Length { min: Some(1), max: Some(10), equal: None };
        assert_eq!(validate_length(validator, "hello"), true);
    }

    #[test]
    fn test_validate_length_string_min_only() {
        let validator = Validator::Length { min: Some(10), max: None, equal: None };
        assert_eq!(validate_length(validator, "hello"), false);
    }

    #[test]
    fn test_validate_length_string_max_only() {
        let validator = Validator::Length { min: None, max: Some(1), equal: None };
        assert_eq!(validate_length(validator, "hello"), false);
    }

    #[test]
    fn test_validate_length_cow() {
        let validator = Validator::Length { min: Some(1), max: Some(2), equal: Some(5) };
        let test: Cow<'static, str> = "hello".into();
        assert_eq!(validate_length(validator, test), true);
        let validator = Validator::Length { min: Some(1), max: Some(2), equal: Some(5) };
        let test: Cow<'static, str> = String::from("hello").into();
        assert_eq!(validate_length(validator, test), true);
    }

    #[test]
    fn test_validate_length_vec() {
        let validator = Validator::Length { min: None, max: None, equal: Some(3) };
        assert_eq!(validate_length(validator, vec![1, 2, 3]), true);
    }

    #[test]
    fn test_validate_length_unicode_chars() {
        let validator = Validator::Length { min: None, max: None, equal: Some(2) };
        assert_eq!(validate_length(validator, "日本"), true);
    }
}
