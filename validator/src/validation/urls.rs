use std::borrow::Cow;
use url::Url;


/// Validates whether the string given is a url
pub fn validate_url<'a, T>(val: T) -> bool
    where T: Into<Cow<'a, str>>
{
    match Url::parse(val.into().as_ref()) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::{validate_url};


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
        assert_eq!(validate_url(test), true);
        let test: Cow<'static, str> = String::from("http://localhost:80").into();
        assert_eq!(validate_url(test), true);
        let test: Cow<'static, str> = "http".into();
        assert_eq!(validate_url(test), false);
        let test: Cow<'static, str> = String::from("http").into();
        assert_eq!(validate_url(test), false);
    }
}
