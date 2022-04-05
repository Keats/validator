use idna::domain_to_ascii;
use lazy_static::lazy_static;
use regex::RegexSet;
use std::borrow::Cow;

lazy_static! {
    // Regex for domain name
    static ref LABEL: RegexSet = {
        let exprs = vec![
            r"^[[:alnum:]]+$",
            r"^_?[[:alnum:]]+[[:alnum:]-]*[[:alnum:]]+$",
        ];
        RegexSet::new(exprs).unwrap()
    };
}

/// Validates whether the given string is an domain based on the
/// [RFC 1034](https://tools.ietf.org/html/rfc1034),[RFC 1035](https://tools.ietf.org/html/rfc1035) and [RFC 2181](https://tools.ietf.org/html/rfc2181)
#[must_use]
pub fn validate_domain<'a, T>(val: T) -> bool
    where
        T: Into<Cow<'a, str>>,
{
    let val = val.into();

    let domain = val.as_ref();
    if domain.len() == 0 {
        return false;
    }
    if domain.len() == 1 && domain == "." {
        return true;
    }
    let domain = match domain_to_ascii(val.as_ref()) {
        Ok(domain) => domain,
        Err(_) => {
            return false;
        }
    };
    let mut labels: Vec<&str> = domain.split('.').collect();
    if domain.ends_with(".") {
        labels.pop();
    }

    if labels.len() > 127 {
        return false;
    }
    labels.reverse();

    for (i, label) in labels.iter().enumerate() {
        // max length for domain name labels is 63 characters per RFC 1034
        if label.len() >= 64 {
            return false;
        }
        // the tld must not be a number
        if i == 0 && label.parse::<f64>().is_ok() {
            return false;
        }
        // non-tld label validate
        if !LABEL.is_match(label) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::validation::domain::validate_domain;
    #[test]
    fn test_validate_domain() {
        let tests = vec![
            (r#"."#, true),
            ("com", true),
            ("com.", true),
            ("www.google.com", true),
            ("www.google.com.", true),
            ("谷歌.中国", true),
            ("谷歌.中国.", true),
            ("domain.here.com", true),
            ("weirder-domain.here.and.there.com", true),
            ("www.valid-----hyphens.com", true),
            ("example.valid-with-hyphens.com", true),
            ("_jabber._tcp.gmail.com", true), // domian name only, not suitable for hostname
            ("_sip._udp.gmail.com.", true),
            ("a.atm.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", true),
            ("a.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.atm", true),
            (
                "a.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbb.atm",
                true,
            ),
            // 64 * a
            (r#""test.test".example.com"#, false),
            ("a.atm.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", false),
            ("", false),
            ("a .x.cz", false),
            ("abc..com", false),
            ("something..somewhere.com", false),
            ("email.127.0.0.1", false),
            ("example.invalid-.com", false),
            ("example.-invalid.com", false),
            ("example.invalid.com-", false),
            ("example.inv-.alid-.com", false),
            ("example.inv-.-alid.com", false),
            (r#"test.example.com\n\n<script src="x.js">"#, false),
            (r#""\\\011".here.com"#, false),
            (r#""\\\012".here.com"#, false),
            // Trailing newlines in domain not allowed
            ("a@b.com\n", false),
            ("a\n.b.com", false),
            (r#""test@test"\n.example.com"#, false),
            ("a.127.0.0.1\n", false),
            // underscores are not allowed
            ("John.Doe.exam_ple.com", false),
        ];

        for (input, expected) in tests {
            // println!("{} - {}", input, expected);
            assert_eq!(
                validate_domain(input),
                expected,
                "Domain `{}` was not classified correctly",
                input
            );
        }
    }
}