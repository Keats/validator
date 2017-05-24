use url::Url;


/// Validates whether the string given is a url
pub fn validate_url(val: &str) -> Result<(), &'static str> {
    Url::parse(val)
        .map(|_| ())
        .map_err(|_| "not a valid URL")
}

#[cfg(test)]
mod tests {
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
}
