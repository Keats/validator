use std::net::IpAddr;
use std::str::FromStr;

pub trait ValidateIp {
    /// Validates whether the given string is an IP V4
    fn validate_ipv4(&self) -> bool;
    /// Validates whether the given string is an IP V6
    fn validate_ipv6(&self) -> bool;
    /// Validates whether the given string is an IP
    fn validate_ip(&self) -> bool;
}

impl<T> ValidateIp for T
where
    T: ToString,
{
    fn validate_ipv4(&self) -> bool {
        IpAddr::from_str(&self.to_string()).map_or(false, |i| i.is_ipv4())
    }

    fn validate_ipv6(&self) -> bool {
        IpAddr::from_str(&self.to_string()).map_or(false, |i| i.is_ipv6())
    }

    fn validate_ip(&self) -> bool {
        IpAddr::from_str(&self.to_string()).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::ValidateIp;
    use std::borrow::Cow;

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
            assert_eq!(input.validate_ip(), expected);
        }
    }

    #[test]
    fn test_validate_ip_cow() {
        let test: Cow<'static, str> = "1.1.1.1".into();
        assert!(test.validate_ip());
        let test: Cow<'static, str> = String::from("1.1.1.1").into();
        assert!(test.validate_ip());
        let test: Cow<'static, str> = "2a02::223:6cff :fe8a:2e8a".into();
        assert!(!test.validate_ip());
        let test: Cow<'static, str> = String::from("2a02::223:6cff :fe8a:2e8a").into();
        assert!(!test.validate_ip());
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
            assert_eq!(input.validate_ipv4(), expected);
        }
    }

    #[test]
    fn test_validate_ip_v4_cow() {
        let test: Cow<'static, str> = "1.1.1.1".into();
        assert!(test.validate_ipv4());
        let test: Cow<'static, str> = String::from("1.1.1.1").into();
        assert!(test.validate_ipv4());
        let test: Cow<'static, str> = "٧.2٥.3٣.243".into();
        assert!(!test.validate_ipv4());
        let test: Cow<'static, str> = String::from("٧.2٥.3٣.243").into();
        assert!(!test.validate_ipv4());
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
            assert_eq!(input.validate_ipv6(), expected);
        }
    }

    #[test]
    fn test_validate_ip_v6_cow() {
        let test: Cow<'static, str> = "fe80::223:6cff:fe8a:2e8a".into();
        assert!(test.validate_ipv6());
        let test: Cow<'static, str> = String::from("fe80::223:6cff:fe8a:2e8a").into();
        assert!(test.validate_ipv6());
        let test: Cow<'static, str> = "::ffff:zzzz:0a0a".into();
        assert!(!test.validate_ipv6());
        let test: Cow<'static, str> = String::from("::ffff:zzzz:0a0a").into();
        assert!(!test.validate_ipv6());
    }
}
