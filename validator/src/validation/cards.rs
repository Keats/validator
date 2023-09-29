use std::borrow::Cow;

#[cfg(feature = "card")]
use card_validate::Validate as CardValidate;

pub trait ValidateCreditCard {
    fn validate_credit_card(&self) -> bool {
        let card_string = self.as_credit_card_string();
        CardValidate::from(&card_string).is_ok()
    }

    fn as_credit_card_string(&self) -> Cow<str>;
}

impl<T: AsRef<str>> ValidateCreditCard for T {
    fn as_credit_card_string(&self) -> Cow<str> {
        Cow::from(self.as_ref())
    }
}

#[cfg(test)]
#[cfg(feature = "card")]
mod tests {
    use super::ValidateCreditCard;
    use std::borrow::Cow;

    #[test]
    fn test_credit_card() {
        let tests = vec![
            ("4539571147647251", true),
            ("343380440754432", true),
            ("zduhefljsdfKJKJZHUI", false),
            ("5236313877109141", false),
        ];

        for (input, expected) in tests {
            assert_eq!(input.validate_credit_card(), expected);
        }
    }

    #[test]
    fn test_credit_card_cow() {
        let test: Cow<'static, str> = "4539571147647251".into();
        assert!(test.validate_credit_card());
        let test: Cow<'static, str> = String::from("4539571147647251").into();
        assert!(test.validate_credit_card());
        let test: Cow<'static, str> = "5236313877109141".into();
        assert!(!test.validate_credit_card());
        let test: Cow<'static, str> = String::from("5236313877109141").into();
        assert!(!test.validate_credit_card());
    }
}
