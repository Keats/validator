use std::borrow::Cow;

use card_validate::Validate as CardValidate;

pub trait ValidateCreditCard {
    #[must_use]
    fn validate_credit_card(&self) -> bool {
        let card_string = self.to_credit_card_string();
        CardValidate::from(&card_string).is_ok()
    }

    fn to_credit_card_string(&self) -> Cow<str>;
}

impl<T: AsRef<str>> ValidateCreditCard for T {
    fn to_credit_card_string(&self) -> Cow<str> {
        Cow::from(self.as_ref())
    }
}

#[cfg(test)]
mod tests {
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
        assert!(validate_credit_card(test));
        let test: Cow<'static, str> = String::from("4539571147647251").into();
        assert!(validate_credit_card(test));
        let test: Cow<'static, str> = "5236313877109141".into();
        assert!(!validate_credit_card(test));
        let test: Cow<'static, str> = String::from("5236313877109141").into();
        assert!(!validate_credit_card(test));
    }
}
