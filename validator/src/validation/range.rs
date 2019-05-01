use validation::Validator;

/// Validates that a number is in the given range
///
/// TODO: see if can be generic over the number type
pub fn validate_range(range: Validator, val: f64) -> bool {
    match range {
        Validator::Range { min, max } => {
            if let Some(m) = min {
                if val < m {
                    return false;
                }
            }

            if let Some(m) = max {
                if val > m {
                    return false;
                }
            }

            true
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::{validate_range, Validator};

    #[test]
    fn test_validate_range_ok() {
        let validator = Validator::Range { min: Some(0.0), max: Some(10.0) };
        assert_eq!(validate_range(validator, 1 as f64), true);
    }

    #[test]
    fn test_validate_range_fail() {
        let validator = Validator::Range { min: Some(0.0), max: Some(10.0) };
        assert_eq!(validate_range(validator, 20 as f64), false);
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
}
