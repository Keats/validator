use crate::validation::Validator;

/// Validates that a number is in the given range
///
#[deprecated(since = "0.12.0", note = "Please use the validate_range_generic function instead")]
#[must_use]
pub fn validate_range(range: Validator, val: f64) -> bool {
    match range {
        Validator::Range { min, max } => validate_range_generic(val, min, max),
        _ => unreachable!(),
    }
}

/// Validates that the given `value` is inside the defined range. The `max` and `min` parameters are
/// optional and will only be validated if they are not `None`
///
pub fn validate_range_generic<T>(value: T, min: Option<T>, max: Option<T>) -> bool
where
    T: PartialOrd + PartialEq,
{
    if let Some(max) = max {
        if value > max {
            return false;
        }
    }

    if let Some(min) = min {
        if value < min {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    // This is needed to suppress the `validate_range` depreciation notice
    // We want to keep the tests to make sure that this function will remain valid
    #![allow(deprecated)]
    use super::{validate_range, validate_range_generic, Validator};

    #[test]
    fn test_validate_range_ok() {
        let validator = Validator::Range { min: Some(0.0), max: Some(10.0) };
        assert_eq!(validate_range(validator, 1_f64), true);
    }

    #[test]
    fn test_validate_range_fail() {
        let validator = Validator::Range { min: Some(0.0), max: Some(10.0) };
        assert_eq!(validate_range(validator, 20_f64), false);
    }

    #[test]
    fn test_validate_range_min_only_valid() {
        let validator = Validator::Range { min: Some(10.0), max: None };
        assert_eq!(validate_range(validator, 10.0), true);
    }

    #[test]
    fn test_validate_range_min_only_invalid() {
        let validator = Validator::Range { min: Some(10.0), max: None };
        assert_eq!(validate_range(validator, 9.0), false);
    }

    #[test]
    fn test_validate_range_max_only_valid() {
        let validator = Validator::Range { min: None, max: Some(10.0) };
        assert_eq!(validate_range(validator, 10.0), true);
    }

    #[test]
    fn test_validate_range_max_only_invalid() {
        let validator = Validator::Range { min: None, max: Some(10.0) };
        assert_eq!(validate_range(validator, 11.0), false);
    }

    #[test]
    fn test_validate_range_generic_ok() {
        // Unspecified generic type:
        assert_eq!(true, validate_range_generic(10, Some(-10), Some(10)));
        assert_eq!(true, validate_range_generic(0.0, Some(0.0), Some(10.0)));

        // Specified type:
        assert_eq!(true, validate_range_generic(5u8, Some(0), Some(255)));
        assert_eq!(true, validate_range_generic(4u16, Some(0), Some(16)));
        assert_eq!(true, validate_range_generic(6u32, Some(0), Some(23)));
    }

    #[test]
    fn test_validate_range_generic_fail() {
        assert_eq!(false, validate_range_generic(5, Some(17), Some(19)));
        assert_eq!(false, validate_range_generic(-1.0, Some(0.0), Some(10.0)));
    }

    #[test]
    fn test_validate_range_generic_min_only() {
        assert_eq!(false, validate_range_generic(5, Some(10), None));
        assert_eq!(true, validate_range_generic(15, Some(10), None));
    }

    #[test]
    fn test_validate_range_generic_max_only() {
        assert_eq!(true, validate_range_generic(5, None, Some(10)));
        assert_eq!(false, validate_range_generic(15, None, Some(10)));
    }
}
