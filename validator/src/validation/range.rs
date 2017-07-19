use validation::Validator;

/// Validates that a number is in the given range
///
/// TODO: see if can be generic over the number type
pub fn validate_range(range: Validator, val: f64) -> bool {
    match range {
        Validator::Range { min, max } => {
            val >= min && val <= max
        },
        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::{validate_range, Validator};

    #[test]
    fn test_validate_range_ok() {
        let validator = Validator::Range { min: 0.0, max: 10.0 };
        assert_eq!(validate_range(validator, 1 as f64), true);
    }

    #[test]
    fn test_validate_range_fail() {
        let validator = Validator::Range { min: 0.0, max: 10.0 };
        assert_eq!(validate_range(validator, 20 as f64), false);
    }
}
