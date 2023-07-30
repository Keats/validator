/// Validates that the given `value` is inside the defined range.
/// The `max`, `min`, `exclusive_max` and `exclusive_min` parameters are
/// optional and will only be validated if they are not `None`
///
#[must_use]
pub fn validate_range<T: ValidateRange<T>>(
    value: T,
    min: Option<T>,
    max: Option<T>,
    exclusive_min: Option<T>,
    exclusive_max: Option<T>,
) -> bool {
    value.validate_range(min, max, exclusive_min, exclusive_max)
}

pub trait ValidateRange<T> {
    fn validate_range(
        &self,
        min: Option<T>,
        max: Option<T>,
        exclusive_min: Option<T>,
        exclusive_max: Option<T>,
    ) -> bool {
        if let Some(max) = max {
            if self.greater_than(max) {
                return false;
            }
        }

        if let Some(min) = min {
            if self.less_than(min) {
                return false;
            }
        }

        if let Some(exclusive_max) = exclusive_max {
            if !self.less_than(exclusive_max) {
                return false;
            }
        }

        if let Some(exclusive_min) = exclusive_min {
            if !self.greater_than(exclusive_min) {
                return false;
            }
        }

        true
    }

    fn greater_than(&self, max: T) -> bool;
    fn less_than(&self, min: T) -> bool;
}

impl<T> ValidateRange<T> for T
where
    T: PartialEq + PartialOrd,
{
    fn greater_than(&self, max: T) -> bool {
        if self > &max {
            return true;
        }

        false
    }

    fn less_than(&self, min: T) -> bool {
        if self < &min {
            return true;
        }

        false
    }
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
    fn test_validate_range_generic_exclusive_max_only() {
        assert!(!validate_range(10, None, None, None, Some(10)));
        assert!(validate_range(9, None, None, None, Some(10)));
    }

    #[test]
    fn test_validate_range_generic_exclusive_min_only() {
        assert!(!validate_range(10, None, None, Some(10), None));
        assert!(validate_range(9, None, None, Some(8), None));
    }

    #[test]
    fn test_validate_range_with_enums() {
        #[derive(PartialEq, PartialOrd)]
        enum Test {
            One,
            Two,
            Three,
            Four,
            Five,
        }

        assert!(validate_range(Test::Three, Some(Test::One), Some(Test::Four), None, None));
        assert!(!validate_range(Test::Five, Some(Test::One), Some(Test::Four), None, None));
    }
}
