use card_validate::{Validate as CardValidate};


pub fn validate_credit_card(card: &str) -> bool {
    CardValidate::from(card).is_ok()
}

#[cfg(test)]
mod tests {
    use super::validate_credit_card;
    #[test]
    fn test_credit_card() {
        let tests = vec![
            ("4539571147647251", true),
            ("343380440754432", true),
            ("zduhefljsdfKJKJZHUI", false),
            ("5236313877109141", false),
        ];

        for (input, expected) in tests {
            assert_eq!(validate_credit_card(input), expected);
        }
    }
}
