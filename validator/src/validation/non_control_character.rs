#[cfg(feature = "unic")]
use unic_ucd_common::control;

pub trait ValidateNonControlCharacter {
    fn validate_non_control_character(&self) -> bool {
        self.as_non_control_character_iterator().all(|code| !control::is_control(code))
    }

    fn as_non_control_character_iterator(&self) -> Box<dyn Iterator<Item = char> + '_>;
}

impl<T: AsRef<str>> ValidateNonControlCharacter for T {
    fn as_non_control_character_iterator(&self) -> Box<dyn Iterator<Item = char> + '_> {
        Box::new(self.as_ref().chars())
    }
}

#[cfg(test)]
#[cfg(feature = "unic")]
mod tests {
    use super::ValidateNonControlCharacter;
    use std::borrow::Cow;

    #[test]
    fn test_non_control_character() {
        let tests = vec![
            ("Himmel", true),
            ("आकाश", true),
            ("வானத்தில்", true),
            ("하늘", true),
            ("небо", true),
            ("2H₂ + O₂ ⇌ 2H₂O", true),
            ("\u{000c}", false),
            ("\u{009F}", false),
        ];

        for (input, expected) in tests {
            assert_eq!(input.validate_non_control_character(), expected);
        }
    }

    #[test]
    fn test_non_control_character_cow() {
        let test: Cow<'static, str> = "आकाश".into();
        assert!(test.validate_non_control_character());
        let test: Cow<'static, str> = String::from("வானத்தில்").into();
        assert!(test.validate_non_control_character());
        let test: Cow<'static, str> = "\u{000c}".into();
        assert!(!test.validate_non_control_character());
        let test: Cow<'static, str> = String::from("\u{009F}").into();
        assert!(!test.validate_non_control_character());
    }
}
