use std::borrow::Cow;
use url::Url;

/// Validates whether the string given is a url
#[must_use]
pub fn validate_url<T: ValidateUrl>(val: T) -> bool {
    val.validate_url()
}

pub trait ValidateUrl {
    fn validate_url(&self) -> bool {
        Url::parse(&self.to_url_string()).is_ok()
    }

    fn to_url_string(&self) -> Cow<str>;
}

impl<T: AsRef<str>> ValidateUrl for T {
    fn to_url_string(&self) -> Cow<'_, str> {
        Cow::from(self.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::validate_url;

    #[test]
    fn test_validate_url() {
        let tests = vec![
            ("http", false),
            ("https://google.com", true),
            ("http://localhost:80", true),
            ("ftp://localhost:80", true),
        ];

        for (url, expected) in tests {
            assert_eq!(validate_url(url), expected);
        }
    }

    #[test]
    fn test_validate_url_cow() {
        let test: Cow<'static, str> = "http://localhost:80".into();
        assert!(validate_url(test));
        let test: Cow<'static, str> = String::from("http://localhost:80").into();
        assert!(validate_url(test));
        let test: Cow<'static, str> = "http".into();
        assert!(!validate_url(test));
        let test: Cow<'static, str> = String::from("http").into();
        assert!(!validate_url(test));
    }
}
