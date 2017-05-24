use types::Validator;

/// Trait to implement if one wants to make the `length` validator
/// work for more types
///
/// A bit sad it's not there by default in Rust
pub trait HasLen {
    fn length(&self) -> u64;
}

impl HasLen for String {
    fn length(&self) -> u64 {
        self.chars().count() as u64
    }
}

impl<'a> HasLen for &'a String {
    fn length(&self) -> u64 {
        self.chars().count() as u64
    }
}

impl<'a> HasLen for &'a str {
    fn length(&self) -> u64 {
        self.chars().count() as u64
    }
}

impl<T> HasLen for Vec<T> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}
impl<'a, T> HasLen for &'a Vec<T> {
    fn length(&self) -> u64 {
        self.len() as u64
    }
}

/// Validates the length of the value given.
/// If the validator has `equal` set, it will ignore any `min` and `max` value.
///
/// If you apply it on String, don't forget that the length can be different
/// from the number of visual characters for Unicode
pub fn validate_length<T: HasLen>(length: Validator, val: T) -> Result<(), String> {
    match length {
        Validator::Length { min, max, equal } => {
            let val_length = val.length();
            if let Some(eq) = equal {
                if val_length != eq {
                    return Err(format!("must be exactly {} characters long", eq));
                }
            }
            if let Some(m) = min {
                if val_length < m {
                    return Err(format!("must be at least {} characters long", m));
                }
            }
            if let Some(m) = max {
                if val_length > m {
                    return Err(format!("must be at most {} characters long", m));
                }
            }
        },
        _ => unreachable!()
    }

    Ok(())
}

#[cfg(test)]
mod tests {
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
