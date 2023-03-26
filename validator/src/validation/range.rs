/// Validates that the given `value` is inside the defined range. The `max` and `min` parameters are
/// optional and will only be validated if they are not `None`
///
#[must_use]

pub fn validate_range<T>(
    value: T,
    min: Option<T>,
    max: Option<T>,
    exc_min: Option<T>,
    exc_max: Option<T>,
) -> bool
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

    if let Some(exc_max) = exc_max {
        if value >= exc_max {
            return false;
        }
    }

    if let Some(exc_min) = exc_min {
        if value <= exc_min {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::validate_range;

    #[test]
    fn test_validate_range_generic_ok() {
        // Unspecified generic type:
        assert!(validate_range(10, Some(-10), Some(10), None, None));
        assert!(validate_range(0.0, Some(0.0), Some(10.0), None, None));

        // Specified type:
        assert!(validate_range(5u8, Some(0), Some(255), None, None));
        assert!(validate_range(4u16, Some(0), Some(16), None, None));
        assert!(validate_range(6u32, Some(0), Some(23), None, None));
    }

    #[test]
    fn test_validate_range_generic_fail() {
        assert!(!validate_range(5, Some(17), Some(19), None, None));
        assert!(!validate_range(-1.0, Some(0.0), Some(10.0), None, None));
    }

    #[test]
    fn test_validate_range_generic_min_only() {
        assert!(!validate_range(5, Some(10), None, None, None));
        assert!(validate_range(15, Some(10), None, None, None));
    }

    #[test]
    fn test_validate_range_generic_max_only() {
        assert!(validate_range(5, None, Some(10), None, None));
        assert!(!validate_range(15, None, Some(10), None, None));
    }

    #[test]
    fn test_validate_range_generic_exc_ok() {
        assert!(validate_range(6, None, None, Some(5), Some(7)));
        assert!(validate_range(0.0001, None, None, Some(0.0), Some(1.0)));
    }

    #[test]
    fn test_validate_range_generic_exc_fail() {
        assert!(!validate_range(5, None, None, Some(5), None));
    }

    #[test]
    fn test_validate_range_generic_exc_max_only() {
        assert!(!validate_range(10, None, None, None, Some(10)));
        assert!(validate_range(9, None, None, None, Some(10)));
    }

    #[test]
    fn test_validate_range_generic_exc_min_only() {
        assert!(!validate_range(10, None, None, Some(10), None));
        assert!(validate_range(9, None, None, Some(8), None));
    }
}
