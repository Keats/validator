/// Validates that the given `value` is inside the defined range.
/// The `max`, `min`, `exclusive_max` and `exclusive_min` parameters are
/// optional and will only be validated if they are not `None`
pub trait ValidateRange<T> {
    fn validate_range(
        &self,
        min: Option<T>,
        max: Option<T>,
        exclusive_min: Option<T>,
        exclusive_max: Option<T>,
    ) -> bool {
        if let Some(max) = max {
            if let Some(gt) = self.greater_than(max) {
                if gt {
                    return false;
                }
            }
        }

        if let Some(min) = min {
            if let Some(lt) = self.less_than(min) {
                if lt {
                    return false;
                }
            }
        }

        if let Some(exclusive_max) = exclusive_max {
            if let Some(lt) = self.less_than(exclusive_max) {
                if !lt {
                    return false;
                }
            }
        }

        if let Some(exclusive_min) = exclusive_min {
            if let Some(gt) = self.greater_than(exclusive_min) {
                if !gt {
                    return false;
                }
            }
        }

        true
    }
    fn greater_than(&self, max: T) -> Option<bool>;
    fn less_than(&self, min: T) -> Option<bool>;
}

pub trait ValidateRangeType {}

impl<T> ValidateRange<T> for T
where
    T: PartialEq + PartialOrd + ValidateRangeType,
{
    fn greater_than(&self, max: T) -> Option<bool> {
        Some(self > &max)
    }

    fn less_than(&self, min: T) -> Option<bool> {
        Some(self < &min)
    }
}

impl<T> ValidateRange<T> for Option<T>
where
    T: PartialEq + PartialOrd + ValidateRangeType,
{
    fn greater_than(&self, max: T) -> Option<bool> {
        self.as_ref().map(|r| r > &max)
    }

    fn less_than(&self, min: T) -> Option<bool> {
        self.as_ref().map(|r| r < &min)
    }
}

impl<T> ValidateRange<T> for Option<Option<T>>
where
    T: PartialEq + PartialOrd + ValidateRangeType,
{
    fn greater_than(&self, max: T) -> Option<bool> {
        if let Some(r) = self {
            r.as_ref().map(|r| r > &max)
        } else {
            None
        }
    }

    fn less_than(&self, min: T) -> Option<bool> {
        if let Some(r) = self {
            r.as_ref().map(|r| r < &min)
        } else {
            None
        }
    }
}

macro_rules! impl_val_range {
    ($t:tt) => {
        impl ValidateRange<$t> for $t {
            fn greater_than(&self, max: $t) -> Option<bool> {
                Some(self > &max)
            }

            fn less_than(&self, min: $t) -> Option<bool> {
                Some(self < &min)
            }
        }

        impl ValidateRange<$t> for Option<$t> {
            fn greater_than(&self, max: $t) -> Option<bool> {
                self.map(|r| r > max)
            }

            fn less_than(&self, min: $t) -> Option<bool> {
                self.map(|r| r < min)
            }
        }

        impl ValidateRange<$t> for Option<Option<$t>> {
            fn greater_than(&self, max: $t) -> Option<bool> {
                self.flatten().map(|r| r > max)
            }

            fn less_than(&self, min: $t) -> Option<bool> {
                self.flatten().map(|r| r < min)
            }
        }
    };
}

impl_val_range!(u8);
impl_val_range!(u16);
impl_val_range!(u32);
impl_val_range!(u64);
impl_val_range!(u128);
impl_val_range!(usize);
impl_val_range!(i8);
impl_val_range!(i16);
impl_val_range!(i32);
impl_val_range!(i64);
impl_val_range!(i128);
impl_val_range!(isize);
impl_val_range!(f32);
impl_val_range!(f64);

#[cfg(test)]
mod tests {
    use crate::validation::range::ValidateRangeType;

    use super::ValidateRange;

    #[test]
    fn test_validate_range_generic_ok() {
        // Unspecified generic type:
        assert!(10.validate_range(Some(-10), Some(10), None, None));
        assert!(0.0.validate_range(Some(0.0), Some(10.0), None, None));

        // Specified type:
        assert!(5u8.validate_range(Some(0), Some(255), None, None));
        assert!(4u16.validate_range(Some(0), Some(16), None, None));
        assert!(6u32.validate_range(Some(0), Some(23), None, None));
    }

    #[test]
    fn test_validate_range_generic_fail() {
        assert!(!5.validate_range(Some(17), Some(19), None, None));
        assert!(!(-1.0).validate_range(Some(0.0), Some(10.0), None, None));
    }

    #[test]
    fn test_validate_range_generic_min_only() {
        assert!(!5.validate_range(Some(10), None, None, None));
        assert!(15.validate_range(Some(10), None, None, None));
    }

    #[test]
    fn test_validate_range_generic_max_only() {
        assert!(5.validate_range(None, Some(10), None, None));
        assert!(!15.validate_range(None, Some(10), None, None));
    }

    #[test]
    fn test_validate_range_generic_exc_ok() {
        assert!(6.validate_range(None, None, Some(5), Some(7)));
        assert!(0.0001.validate_range(None, None, Some(0.0), Some(1.0)));
    }

    #[test]
    fn test_validate_range_generic_exc_fail() {
        assert!(!5.validate_range(None, None, Some(5), None));
    }

    #[test]
    fn test_validate_range_generic_exclusive_max_only() {
        assert!(!10.validate_range(None, None, None, Some(10)));
        assert!(9.validate_range(None, None, None, Some(10)));
    }

    #[test]
    fn test_validate_range_generic_exclusive_min_only() {
        assert!(!10.validate_range(None, None, Some(10), None));
        assert!(9.validate_range(None, None, Some(8), None));
    }

    #[test]
    fn test_validate_range_with_enums() {
        #[derive(PartialEq, PartialOrd)]
        enum Test {
            One,
            Two,
            Three,
            Four,
        }

        impl ValidateRangeType for Test {}

        assert!(Test::Two.validate_range(Some(Test::One), Some(Test::Three), None, None));
        assert!(!Test::Four.validate_range(Some(Test::One), Some(Test::Three), None, None));
    }

    #[test]
    fn test_validate_range_with_option() {
        assert!(Some(5).validate_range(Some(1), Some(10), None, None));
        assert!(!Some(11).validate_range(Some(1), Some(10), None, None));
    }

    #[test]
    fn test_validate_range_with_none_value() {
        let none: Option<usize> = None;
        let none_none: Option<Option<usize>> = None;
        assert!(none.validate_range(Some(1), Some(10), None, None));
        assert!(none.validate_range(Some(1), None, None, Some(10)));
        assert!(none_none.validate_range(Some(1), Some(10), None, None));
    }
}
