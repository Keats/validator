use std::borrow::Cow;
use url::Url;

/// Validates whether the string given is a url
#[must_use]
pub fn validate_url<T: ValidateUrl>(val: T) -> bool {
    val.validate_url()
}

pub trait ValidateUrl {
    fn validate_url(&self) -> bool {
        if let Some(u) = self.to_url_string() {
            Url::parse(&u).is_ok()
        } else {
            true
        }
    }

    fn to_url_string(&self) -> Option<Cow<str>>;
}

impl ValidateUrl for String {
    fn to_url_string(&self) -> Option<Cow<str>> {
        Some(Cow::from(self))
    }
}

impl ValidateUrl for Option<String> {
    fn to_url_string(&self) -> Option<Cow<str>> {
        if let Some(u) = self {
            Some(Cow::from(u))
        } else {
            None
        }
    }
}

impl ValidateUrl for Option<Option<String>> {
    fn to_url_string(&self) -> Option<Cow<str>> {
        if let Some(u) = self {
            if let Some(u) = u {
                Some(Cow::from(u))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl ValidateUrl for &String {
    fn to_url_string(&self) -> Option<Cow<str>> {
        Some(Cow::from(self.as_str()))
    }
}

impl ValidateUrl for Option<&String> {
    fn to_url_string(&self) -> Option<Cow<str>> {
        if let Some(u) = self {
            Some(Cow::from(*u))
        } else {
            None
        }
    }
}

impl ValidateUrl for Option<Option<&String>> {
    fn to_url_string(&self) -> Option<Cow<str>> {
        if let Some(u) = self {
            if let Some(u) = u {
                Some(Cow::from(*u))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<'a> ValidateUrl for &'a str {
    fn to_url_string(&self) -> Option<Cow<'_, str>> {
        Some(Cow::from(*self))
    }
}

impl<'a> ValidateUrl for Option<&'a str> {
    fn to_url_string(&self) -> Option<Cow<str>> {
        if let Some(u) = self {
            Some(Cow::from(*u))
        } else {
            None
        }
    }
}

impl<'a> ValidateUrl for Option<Option<&'a str>> {
    fn to_url_string(&self) -> Option<Cow<str>> {
        if let Some(u) = self {
            if let Some(u) = u {
                Some(Cow::from(*u))
            } else {
                None
            }
        } else {
            None
        }
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
