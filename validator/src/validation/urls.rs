use std::borrow::Cow;
use url::Url;

/// Validates whether the string given is a url
pub trait ValidateUrl {
    fn validate_url(&self) -> bool {
        if let Some(u) = self.as_url_string() {
            Url::parse(&u).is_ok()
        } else {
            true
        }
    }

    fn as_url_string(&self) -> Option<Cow<str>>;
}

impl ValidateUrl for String {
    fn as_url_string(&self) -> Option<Cow<str>> {
        Some(Cow::from(self))
    }
}

impl ValidateUrl for Option<String> {
    fn as_url_string(&self) -> Option<Cow<str>> {
        self.as_ref().map(Cow::from)
    }
}

impl ValidateUrl for Option<Option<String>> {
    fn as_url_string(&self) -> Option<Cow<str>> {
        if let Some(u) = self {
            u.as_ref().map(Cow::from)
        } else {
            None
        }
    }
}

impl ValidateUrl for &String {
    fn as_url_string(&self) -> Option<Cow<str>> {
        Some(Cow::from(self.as_str()))
    }
}

impl ValidateUrl for Option<&String> {
    fn as_url_string(&self) -> Option<Cow<str>> {
        self.as_ref().map(|u| Cow::from(*u))
    }
}

impl ValidateUrl for Option<Option<&String>> {
    fn as_url_string(&self) -> Option<Cow<str>> {
        self.flatten().map(Cow::from)
    }
}

impl<'a> ValidateUrl for &'a str {
    fn as_url_string(&self) -> Option<Cow<'_, str>> {
        Some(Cow::from(*self))
    }
}

impl<'a> ValidateUrl for Option<&'a str> {
    fn as_url_string(&self) -> Option<Cow<str>> {
        self.as_ref().map(|u| Cow::from(*u))
    }
}

impl<'a> ValidateUrl for Option<Option<&'a str>> {
    fn as_url_string(&self) -> Option<Cow<str>> {
        self.flatten().map(Cow::from)
    }
}

impl ValidateUrl for Cow<'_, str> {
    fn as_url_string(&self) -> Option<Cow<'_, str>> {
        Some(self.clone())
    }
}

impl ValidateUrl for Option<Cow<'_, str>> {
    fn as_url_string(&self) -> Option<Cow<str>> {
        self.as_ref().cloned()
    }
}

impl ValidateUrl for Option<Option<Cow<'_, str>>> {
    fn as_url_string(&self) -> Option<Cow<str>> {
        self.as_ref().cloned().flatten()
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::ValidateUrl;

    #[test]
    fn test_validate_url() {
        let tests = vec![
            ("http", false),
            ("https://google.com", true),
            ("http://localhost:80", true),
            ("ftp://localhost:80", true),
        ];

        for (url, expected) in tests {
            assert_eq!(url.validate_url(), expected);
        }
    }

    #[test]
    fn test_validate_url_cow() {
        let test: Cow<'static, str> = "http://localhost:80".into();
        assert!(test.validate_url());
        let test: Cow<'static, str> = String::from("http://localhost:80").into();
        assert!(test.validate_url());
        let test: Cow<'static, str> = "http".into();
        assert!(!test.validate_url());
        let test: Cow<'static, str> = String::from("http").into();
        assert!(!test.validate_url());
    }
}
