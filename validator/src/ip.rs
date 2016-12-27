use std::str::FromStr;
use std::net::IpAddr;


/// Validates whether the given string is an IP V4
pub fn validate_ip_v4(val: &str) -> bool {
    match IpAddr::from_str(val) {
        Ok(i) => match i {
            IpAddr::V4(_) => true,
            IpAddr::V6(_) => false,
        },
        Err(_) => false,
    }
}

/// Validates whether the given string is an IP V6
pub fn validate_ip_v6(val: &str) -> bool {
    match IpAddr::from_str(val) {
        Ok(i) => match i {
            IpAddr::V4(_) => false,
            IpAddr::V6(_) => true,
        },
        Err(_) => false,
    }
}

/// Validates whether the given string is an IP
pub fn validate_ip(val: &str) -> bool {
    match IpAddr::from_str(val) {
        Ok(_) => true,
        Err(_) => false,
    }
}


#[cfg(test)]
mod tests {
    use super::{validate_ip_v4, validate_ip_v6, validate_ip};

    #[test]
    fn test_validate_ip() {
        let tests = vec![
            ("1.1.1.1", true),
            ("255.0.0.0", true),
            ("0.0.0.0", true),
            ("256.1.1.1", false),
            ("25.1.1.", false),
            ("25,1,1,1", false),
            ("fe80::223:6cff:fe8a:2e8a", true),
            ("::ffff:254.42.16.14", true),
            ("2a02::223:6cff :fe8a:2e8a", false),
        ];

        for (input, expected) in tests {
            assert_eq!(validate_ip(input), expected);
        }
    }

    #[test]
    fn test_validate_ip_v4() {
        let tests = vec![
            ("1.1.1.1", true),
            ("255.0.0.0", true),
            ("0.0.0.0", true),
            ("256.1.1.1", false),
            ("25.1.1.", false),
            ("25,1,1,1", false),
            ("25.1 .1.1", false),
            ("1.1.1.1\n", false),
            ("٧.2٥.3٣.243", false),
        ];

        for (input, expected) in tests {
            assert_eq!(validate_ip_v4(input), expected);
        }
    }

    #[test]
    fn test_validate_ip_v6() {
        let tests = vec![
            ("fe80::223:6cff:fe8a:2e8a", true),
            ("2a02::223:6cff:fe8a:2e8a", true),
            ("1::2:3:4:5:6:7", true),
            ("::", true),
            ("::a", true),
            ("2::", true),
            ("::ffff:254.42.16.14", true),
            ("::ffff:0a0a:0a0a", true),
            ("::254.42.16.14", true),
            ("::0a0a:0a0a", true),
            ("foo", false),
            ("127.0.0.1", false),
            ("12345::", false),
            ("1::2::3::4", false),
            ("1::zzz", false),
            ("1:2", false),
            ("fe80::223: 6cff:fe8a:2e8a", false),
            ("2a02::223:6cff :fe8a:2e8a", false),
            ("::ffff:999.42.16.14", false),
            ("::ffff:zzzz:0a0a", false),
        ];

        for (input, expected) in tests {
            assert_eq!(validate_ip_v6(input), expected);
        }
    }
}
