use phonenumber;


pub fn validate_phone(phone_number: &str) -> bool {
    if let Ok(parsed) = phonenumber::parse(None, phone_number) {
        phonenumber::is_valid(&parsed)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::validate_phone;
    #[test]
    fn test_phone() {
        let tests = vec![
            ("+1 (415) 237-0800", true),
            ("+14152370800", true),
            ("+33642926829", true),
            ("14152370800", false),
            ("0642926829", false),
            ("00642926829", false),
            ("A012", false),
            ("TEXT", false),
        ];

        for (input, expected) in tests {
            println!("{} - {}", input, expected);
            assert_eq!(validate_phone(input), expected);
        }
    }
}
