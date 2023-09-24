use std::borrow::Cow;

/// Validates whether the given string is an email based on the [HTML5 spec](https://html.spec.whatwg.org/multipage/forms.html#valid-e-mail-address).
/// [RFC 5322](https://tools.ietf.org/html/rfc5322) is not practical in most circumstances and allows email addresses
/// that are unfamiliar to most users.

#[must_use]
pub fn validate_email<'a, T>(val: T) -> bool
where
    T: Into<Cow<'a, str>>,
{
    crate::span::email::validate_email_span(val).is_ok()
}

#[cfg(test)]
mod tests {
    use super::validate_email;
    use std::borrow::Cow;

    #[test]
    fn test_validate_email_cow() {
        let test: Cow<'static, str> = "email@here.com".into();
        assert!(validate_email(test));
        let test: Cow<'static, str> = String::from("email@here.com").into();
        assert!(validate_email(test));
        let test: Cow<'static, str> = "a@[127.0.0.1]\n".into();
        assert!(!validate_email(test));
        let test: Cow<'static, str> = String::from("a@[127.0.0.1]\n").into();
        assert!(!validate_email(test));
    }

    #[test]
    fn test_validate_email_rfc5321() {
        // 65 character local part
        let test = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa@mail.com";
        assert_eq!(validate_email(test), false);
        // 256 character domain part
        let test = "a@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.com";
        assert_eq!(validate_email(test), false);
    }
}
