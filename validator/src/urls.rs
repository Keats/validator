use url::Url;


/// Validates whether the string given is a url
pub fn validate_url(val: &str) -> bool {
    match Url::parse(val) {
        Ok(_) => true,
        Err(_) => false,
    }
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
