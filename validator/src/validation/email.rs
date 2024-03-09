use idna::domain_to_ascii;
use once_cell::sync::Lazy;
use regex::Regex;
use std::borrow::Cow;

use crate::ValidateIp;

// Regex from the specs
// https://html.spec.whatwg.org/multipage/forms.html#valid-e-mail-address
// It will mark esoteric email addresses like quoted string as invalid
static EMAIL_USER_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+\z").unwrap());
static EMAIL_DOMAIN_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
    ).unwrap()
});
// literal form, ipv4 or ipv6 address (SMTP 4.1.3)
static EMAIL_LITERAL_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\[([a-fA-F0-9:\.]+)\]\z").unwrap());

/// Checks if the domain is a valid domain and if not, check whether it's an IP
#[must_use]
fn validate_domain_part(domain_part: &str) -> bool {
    if EMAIL_DOMAIN_RE.is_match(domain_part) {
        return true;
    }

    // maybe we have an ip as a domain?
    match EMAIL_LITERAL_RE.captures(domain_part) {
        Some(caps) => match caps.get(1) {
            Some(c) => c.as_str().validate_ip(),
            None => false,
        },
        None => false,
    }
}

/// Validates whether the given string is an email based on the [HTML5 spec](https://html.spec.whatwg.org/multipage/forms.html#valid-e-mail-address).
/// [RFC 5322](https://tools.ietf.org/html/rfc5322) is not practical in most circumstances and allows email addresses
/// that are unfamiliar to most users.
pub trait ValidateEmail {
    fn validate_email(&self) -> bool {
        let val = if let Some(v) = self.as_email_string() {
            v
        } else {
            return true;
        };

        if val.is_empty() || !val.contains('@') {
            return false;
        }

        let parts: Vec<&str> = val.rsplitn(2, '@').collect();
        let user_part = parts[1];
        let domain_part = parts[0];

        // validate the length of each part of the email, BEFORE doing the regex
        // according to RFC5321 the max length of the local part is 64 characters
        // and the max length of the domain part is 255 characters
        // https://datatracker.ietf.org/doc/html/rfc5321#section-4.5.3.1.1
        if user_part.chars().count() > 64 || domain_part.chars().count() > 255 {
            return false;
        }

        if !EMAIL_USER_RE.is_match(user_part) {
            return false;
        }

        if !validate_domain_part(domain_part) {
            // Still the possibility of an [IDN](https://en.wikipedia.org/wiki/Internationalized_domain_name)
            return match domain_to_ascii(domain_part) {
                Ok(d) => validate_domain_part(&d),
                Err(_) => false,
            };
        }

        true
    }

    fn as_email_string(&self) -> Option<Cow<str>>;
}

impl<T> ValidateEmail for &T
where
    T: ValidateEmail,
{
    fn as_email_string(&self) -> Option<Cow<str>> {
        T::as_email_string(self)
    }
}

impl ValidateEmail for String {
    fn as_email_string(&self) -> Option<Cow<str>> {
        Some(Cow::from(self))
    }
}

impl<T> ValidateEmail for Option<T>
where
    T: ValidateEmail,
{
    fn as_email_string(&self) -> Option<Cow<str>> {
        let Some(u) = self else {
            return None;
        };

        T::as_email_string(u)
    }
}

impl<'a> ValidateEmail for &'a str {
    fn as_email_string(&self) -> Option<Cow<'_, str>> {
        Some(Cow::from(*self))
    }
}

impl ValidateEmail for Cow<'_, str> {
    fn as_email_string(&self) -> Option<Cow<'_, str>> {
        Some(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::ValidateEmail;

    #[test]
    fn test_validate_email() {
        // Test cases taken from Django
        // https://github.com/django/django/blob/master/tests/validators/tests.py#L48
        let tests = vec![
            ("email@here.com", true),
            ("weirder-email@here.and.there.com", true),
            (r#"!def!xyz%abc@example.com"#, true),
            ("email@[127.0.0.1]", true),
            ("email@[2001:dB8::1]", true),
            ("email@[2001:dB8:0:0:0:0:0:1]", true),
            ("email@[::fffF:127.0.0.1]", true),
            ("example@valid-----hyphens.com", true),
            ("example@valid-with-hyphens.com", true),
            ("test@domain.with.idn.tld.उदाहरण.परीक्षा", true),
            (r#""test@test"@example.com"#, false),
            // max length for domain name labels is 63 characters per RFC 1034
            ("a@atm.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", true),
            ("a@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.atm", true),
            (
                "a@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbb.atm",
                true,
            ),
            // 64 * a
            ("a@atm.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", false),
            ("", false),
            ("abc", false),
            ("abc@", false),
            ("abc@bar", true),
            ("a @x.cz", false),
            ("abc@.com", false),
            ("something@@somewhere.com", false),
            ("email@127.0.0.1", true),
            ("email@[127.0.0.256]", false),
            ("email@[2001:db8::12345]", false),
            ("email@[2001:db8:0:0:0:0:1]", false),
            ("email@[::ffff:127.0.0.256]", false),
            ("example@invalid-.com", false),
            ("example@-invalid.com", false),
            ("example@invalid.com-", false),
            ("example@inv-.alid-.com", false),
            ("example@inv-.-alid.com", false),
            (r#"test@example.com\n\n<script src="x.js">"#, false),
            (r#""\\\011"@here.com"#, false),
            (r#""\\\012"@here.com"#, false),
            ("trailingdot@shouldfail.com.", false),
            // Trailing newlines in username or domain not allowed
            ("a@b.com\n", false),
            ("a\n@b.com", false),
            (r#""test@test"\n@example.com"#, false),
            ("a@[127.0.0.1]\n", false),
            // underscores are not allowed
            ("John.Doe@exam_ple.com", false),
        ];

        for (input, expected) in tests {
            // println!("{} - {}", input, expected);
            assert_eq!(
                input.validate_email(),
                expected,
                "Email `{}` was not classified correctly",
                input
            );
        }
    }

    #[test]
    fn test_validate_email_cow() {
        let test: Cow<'static, str> = "email@here.com".into();
        assert!(test.validate_email());
        let test: Cow<'static, str> = String::from("email@here.com").into();
        assert!(test.validate_email());
        let test: Cow<'static, str> = "a@[127.0.0.1]\n".into();
        assert!(!test.validate_email());
        let test: Cow<'static, str> = String::from("a@[127.0.0.1]\n").into();
        assert!(!test.validate_email());
    }

    #[test]
    fn test_validate_email_rfc5321() {
        // 65 character local part
        let test = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa@mail.com";
        assert_eq!(test.validate_email(), false);
        // 256 character domain part
        let test = "a@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.com";
        assert_eq!(test.validate_email(), false);
    }
}
