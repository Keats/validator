extern crate alloc;

use alloc::borrow::Cow;
use core::convert::TryFrom;
use idna::domain_to_ascii;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{validation::ip::validate_ip, HasLen};

// Valid characters, excluding ASCII a-z, A-Z and 0-9
const VALID_USER_CHARACTERS: &str = r#".!#$%&'*+/=?^_`{|}~-"#;
const VALID_DOMAIN_CHARACTERS: &str = r#".-"#;

lazy_static! {
    // Regex from the specs
    // https://html.spec.whatwg.org/multipage/forms.html#valid-e-mail-address
    // It will mark esoteric email addresses like quoted string as invalid
    static ref EMAIL_USER_RE: Regex = Regex::new(r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+\z").unwrap();
    static ref EMAIL_DOMAIN_RE: Regex = Regex::new(
        r"(?i)^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$"
    ).unwrap();
    // literal form, ipv4 or ipv6 address (SMTP 4.1.3)
    static ref EMAIL_LITERAL_RE: Regex = Regex::new(r"(?i)\[([A-f0-9:\.]+)\]\z").unwrap();
}

/// Represents the error states of an email address
///
/// This enum is always constructed along with an accompanying span that indicates where the
/// error occurred. For more details, see the [`EmailValidationError`] struct.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum EmailError {
    /// The length of the domain is greater than the maximum of 255 characters
    ///
    /// Error spans: domain
    DomainTooLong,
    /// An unknown error in the domain that was caught by the [regex from the whatwg spec](https://html.spec.whatwg.org/multipage/forms.html#valid-e-mail-address)
    ///
    /// Error spans: domain
    DomainRegexFailed,
    /// The provided domain does not contain any characters
    ///
    /// Errors spans: email
    EmptyDomain,
    /// The provided email does not contain any charaacters
    ///
    /// Error spans: email
    EmptyEmail,
    /// An element of the domain does not contain any characters
    ///
    /// Error spans: domain
    EmptySubdomain,
    /// The provided user does not contain any characters
    ///
    /// Error spans: email
    EmptyUser,
    /// The provided domain contains invalid characters
    ///
    /// Error spans: invalid character
    InvalidDomainCharacter,
    /// The provided user contains invalid characters
    ///
    /// Error spans: invalid character
    InvalidUserCharacter,
    /// The provided IP is invalid
    ///
    /// Error spans: IP address
    InvalidIP,
    /// An element of the domain begins with a hyphen (`-`)
    ///
    /// Error spans: leading hyphen character
    LeadingHyphen,
    /// An element of the domain begins with a period (`.`)
    ///
    /// Error spans: leading period character
    LeadingPeriod,
    /// The provided email does not contain the required `@` character
    ///
    /// Error spans: email
    MissingAtCharacter,
    /// The length of an element in the domain is greater than the maximum of 63 characters
    ///
    /// Error spans: invalid subdomain
    SubdomainTooLong,
    /// An element of the domain ends with a hyphen (`-`)
    ///
    /// Error spans: trailing hyphen character
    TrailingHyphen,

    /// An element of the domain ends with a period (`.`)
    ///
    /// Error spans: trailing period character
    TrailingPeriod,
    /// The length of the user was greater than the maximum of 64 characters
    ///
    /// Error spans: invalid user
    UserTooLong,
    /// An unknown error in the user that was caught by the [regex from the whatwg spec](https://html.spec.whatwg.org/multipage/forms.html#valid-e-mail-address)
    ///
    /// Error spans: user
    UserRegexFailed,
    /// The provided domain was unable to be converted into an [IDN](https://en.wikipedia.org/wiki/Internationalized_domain_name)
    ///
    /// Error spans: domain
    Uts46,
}

/// Upon encountering an error, [`validate_email_span`] will return this struct containing the first error reached.
/// The field `error_type` indicates which rule has not been met, and the `span` field provides a simple character range
/// indicating which portion of the email is at fault. This may be a single character, or an entire string. Each enum variant
/// documents the range it spans.
///
/// Example of a single-character span:
/// ```
/// # use validator::{EmailError, EmailValidationError, validate_email_span};
/// // invalid"@example.com
/// //        ^
///
/// assert_eq!(
///     validate_email_span(r#"invalid"@example.com"#),
///     Err(EmailValidationError { error_type: EmailError::InvalidUserCharacter, span: 7..7 })
/// );
/// ```
/// Example of a multi-character span:
/// ```
/// // invalid@example..com
/// //         ^^^^^^^^^^^^
/// # use validator::{EmailError, EmailValidationError, validate_email_span};
/// assert_eq!(
///     validate_email_span("invalid@example..com"),
///     Err(EmailValidationError { error_type: EmailError::EmptySubdomain, span: 15..16 })
/// );
/// ```
///
/// Example of an address-long span:
/// ```
/// # use validator::{EmailError, EmailValidationError, validate_email_span};
/// // no-user.com
/// // ^^^^^^^^^^^
/// assert_eq!(
///     validate_email_span("no-user.com"),
///     Err(EmailValidationError { error_type: EmailError::MissingAtCharacter, span: 0..11 })
/// );
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmailValidationError {
    pub error_type: EmailError,
    pub span: core::ops::Range<u64>,
}

pub fn validate_email_span<'email, T>(value: T) -> Result<(), EmailValidationError>
where
    T: Into<Cow<'email, str>>,
{
    let email = value.into();

    // Email must not be empty
    if email.is_empty() {
        return Err(EmailValidationError { error_type: EmailError::EmptyEmail, span: 0..0 });
    }

    // Email must contain at least one '@' character
    if !email.contains('@') {
        // example.com
        // ^^^^^^^^^^^
        return Err(EmailValidationError {
            error_type: EmailError::MissingAtCharacter,
            span: 0..email.length(),
        });
    }

    // User: everything up until the last `@` character
    let parts: Vec<&str> = email.rsplitn(2, '@').collect();
    let user = parts[1];

    // Domain: everything after the last `@` character
    // Convert the domain to an [IDN](https://en.wikipedia.org/wiki/Internationalized_domain_name)
    let domain = match domain_to_ascii(parts[0]) {
        Ok(domain) => domain,
        Err(_domain_errors) => {
            return Err(EmailValidationError {
                error_type: EmailError::Uts46,
                span: 0..email.length(),
            })
        }
    };

    // Validate the email domain
    validate_domain(&domain, user.length(), email.length())?;

    // Validate the email user
    validate_user(parts[1], email.length())?;

    // No errors found during validation process
    Ok(())
}

fn validate_user(user: &str, email_end: u64) -> Result<(), EmailValidationError> {
    // Ensure validity of all characters in user
    for (index, character) in user.chars().enumerate() {
        if !character.is_alphanumeric() && !VALID_USER_CHARACTERS.contains(character) {
            let column = u64::try_from(index).unwrap();
            // user"@example.com
            //     ^
            return Err(EmailValidationError {
                error_type: EmailError::InvalidUserCharacter,
                span: column..column,
            });
        }
    }

    // validate the length of each part of the email, BEFORE doing the regex
    // according to RFC5321 the max length of the subdomain is 64 characters
    // and the max length of the domain part is 255 characters
    // https://datatracker.ietf.org/doc/html/rfc5321#section-4.5.3.1.1
    if user.is_empty() {
        // @example.com
        // ^^^^^^^^^^^^
        return Err(EmailValidationError { error_type: EmailError::EmptyUser, span: 0..email_end });
    }

    // User has a maximum length of 64
    if user.length() > 64 {
        // pretend_this_is_too_long@example.com
        // ^^^^^^^^^^^^^^^^^^^^^^^^
        return Err(EmailValidationError {
            error_type: EmailError::UserTooLong,
            span: 0..user.length(),
        });
    }

    // Final check, match user against regex from the spec
    if !EMAIL_USER_RE.is_match(user) {
        // exotic_case@example.com
        // ^^^^^^^^^^^
        return Err(EmailValidationError {
            error_type: EmailError::UserRegexFailed,
            span: 0..user.length(),
        });
    }

    Ok(())
}

/// Validate the email domain.
///
/// A domain can either be an
fn validate_domain(
    domain: &str,
    user_end: u64,
    email_end: u64,
) -> Result<(), EmailValidationError> {
    // Email domains can be an IP address surrounded by square brackets (`[]`)
    let is_literal_address = match (domain.chars().next(), domain.chars().last()) {
        (Some('['), Some(']')) => true,
        _ => false,
    };

    // Ensure validity of all characters in domain
    if is_literal_address {
        let potential_ip = domain.get(1..domain.len() - 1).unwrap();
        let ip_range_start = user_end + 2;
        if !validate_ip(potential_ip) {
            // user@[127.0.0.256]
            //        ^^^^^^^^^^^
            return Err(EmailValidationError {
                error_type: EmailError::InvalidIP,
                span: ip_range_start..ip_range_start + potential_ip.length() - 1,
            });
        }
    } else {
        for (index, character) in domain.chars().enumerate() {
            if !character.is_alphanumeric() && !VALID_DOMAIN_CHARACTERS.contains(character) {
                let location = u64::try_from(index).unwrap() + user_end + 1;

                // user@*example.com
                //      ^
                return Err(EmailValidationError {
                    error_type: EmailError::InvalidDomainCharacter,
                    span: location..location,
                });
            }
        }
    }

    // Domain cannot be empty
    if domain.is_empty() {
        // user@
        // ^^^^^
        return Err(EmailValidationError {
            error_type: EmailError::EmptyDomain,
            span: 0..email_end,
        });
    }

    // Domain has a maximum length of 255
    if domain.length() > 255 {
        // user@pretend_this_is_too_long.com
        //      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        return Err(EmailValidationError {
            error_type: EmailError::DomainTooLong,
            span: (user_end + 1)..email_end,
        });
    }

    // Domain cannot begin with a leading period (`.`)
    if domain.starts_with('.') {
        let column = user_end + 1;

        // user@.example.com
        //      ^
        return Err(EmailValidationError {
            error_type: EmailError::LeadingPeriod,
            span: column..column,
        });
    }

    // Domain cannot end with a trailing period (`.`)
    if domain.ends_with('.') {
        let column = email_end - 1;

        // user@example.com.
        //                 ^
        return Err(EmailValidationError {
            error_type: EmailError::TrailingPeriod,
            span: column..column,
        });
    }

    let mut span_start = user_end + 1;
    for subdomain in domain.split('.') {
        // Subdomain has a maximum length of 63
        if subdomain.length() > 63 {
            // user@valid.valid.pretend_this_is_too_long.com
            //                  ^^^^^^^^^^^^^^^^^^^^^^^^
            return Err(EmailValidationError {
                error_type: EmailError::SubdomainTooLong,
                span: span_start..span_start + subdomain.length(),
            });
        }

        // Subdomain cannot be empty
        if subdomain.is_empty() {
            // user@example..com
            //             ^^
            return Err(EmailValidationError {
                error_type: EmailError::EmptySubdomain,
                span: span_start - 1..span_start,
            });
        }

        // Subdomain cannot have a leading hyphen (`-`)
        if subdomain.starts_with('-') {
            // user@-example.com
            //      ^
            return Err(EmailValidationError {
                error_type: EmailError::LeadingHyphen,
                span: span_start..span_start,
            });
        }

        // Subdomain cannot have a trailing hypen (`-`)
        if subdomain.ends_with('-') {
            // user@example.com-
            //                 ^
            return Err(EmailValidationError {
                error_type: EmailError::TrailingHyphen,
                span: span_start + subdomain.length()..span_start + subdomain.length(),
            });
        }

        // Account for extra '.' character between subdomains
        span_start += subdomain.length() + 1;
    }

    // Final check, match domain against regex from the spec
    if !validate_domain_part(domain) {
        // user@exotic_case.com
        //      ^^^^^^^^^^^^^^^
        return Err(EmailValidationError {
            error_type: EmailError::DomainRegexFailed,
            span: user_end + 1..email_end,
        });
    }

    Ok(())
}

/// Checks if the domain is a valid domain and if not, check whether it's an IP
#[must_use]
fn validate_domain_part(domain: &str) -> bool {
    if EMAIL_DOMAIN_RE.is_match(domain) {
        return true;
    }

    // maybe we have an ip as a domain?
    match EMAIL_LITERAL_RE.captures(domain) {
        Some(caps) => match caps.get(1) {
            Some(c) => return validate_ip(c.as_str()),
            None => false,
        },
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{validate_email_span, EmailError, EmailValidationError};

    #[test]
    fn test_validate_email() {
        // Test cases taken from Django
        // https://github.com/django/django/blob/master/tests/validators/tests.py#L48
        let tests = vec![
            ("email@here.com", Ok(())),
            ("weirder-email@here.and.there.com", Ok(())),
            (r#"!def!xyz%abc@example.com"#, Ok(())),
            ("email@[127.0.0.1]", Ok(())),
            ("email@[2001:dB8::1]", Ok(())),
            ("email@[2001:dB8:0:0:0:0:0:1]", Ok(())),
            ("email@[::fffF:127.0.0.1]", Ok(())),
            ("example@valid-----hyphens.com", Ok(())),
            ("example@valid-with-hyphens.com", Ok(())),
            ("test@domain.with.idn.tld.उदाहरण.परीक्षा", Ok(())),
            (
                r#""test@test"@example.com"#,
                Err(EmailValidationError {
                    error_type: EmailError::InvalidUserCharacter,
                    span: 0..0,
                }),
            ),
            // max length for domain name labels is 63 characters per RFC 1034
            ("a@atm.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", Ok(())),
            ("a@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.atm", Ok(())),
            (
                "a@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbb.atm",
                Ok(()),
            ),
            // 64 * a
            (
                "a@atm.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                Err(EmailValidationError { error_type: EmailError::SubdomainTooLong, span: 6..70 }),
            ),
            ("", Err(EmailValidationError { error_type: EmailError::EmptyEmail, span: 0..0 })),
            (
                "abc",
                Err(EmailValidationError {
                    error_type: EmailError::MissingAtCharacter,
                    span: 0..3,
                }),
            ),
            ("abc@", Err(EmailValidationError { error_type: EmailError::EmptyDomain, span: 0..4 })),
            ("@abc", Err(EmailValidationError { error_type: EmailError::EmptyUser, span: 0..4 })),
            ("abc@bar", Ok(())),
            (
                "a @x.cz",
                Err(EmailValidationError {
                    error_type: EmailError::InvalidUserCharacter,
                    span: 1..1,
                }),
            ),
            (
                "abc@.com",
                Err(EmailValidationError { error_type: EmailError::LeadingPeriod, span: 4..4 }),
            ),
            (
                "something@@somewhere.com",
                Err(EmailValidationError {
                    error_type: EmailError::InvalidUserCharacter,
                    span: 9..9,
                }),
            ),
            // ("email@127.0.0.1", Ok(())),
            (
                "email@[127.0.0.256]",
                Err(EmailValidationError { error_type: EmailError::InvalidIP, span: 7..17 }),
            ),
            (
                "email@[2001:db8::12345]",
                Err(EmailValidationError { error_type: EmailError::InvalidIP, span: 7..21 }),
            ),
            (
                "email@[2001:db8:0:0:0:0:1]",
                Err(EmailValidationError { error_type: EmailError::InvalidIP, span: 7..24 }),
            ),
            (
                "email@[::ffff:127.0.0.256]",
                Err(EmailValidationError { error_type: EmailError::InvalidIP, span: 7..24 }),
            ),
            (
                "example@invalid-.com",
                Err(EmailValidationError { error_type: EmailError::TrailingHyphen, span: 16..16 }),
            ),
            (
                "example@-invalid.com",
                Err(EmailValidationError { error_type: EmailError::LeadingHyphen, span: 8..8 }),
            ),
            (
                "example@invalid.com-",
                Err(EmailValidationError { error_type: EmailError::TrailingHyphen, span: 20..20 }),
            ),
            (
                "example@inv-.alid-.com",
                Err(EmailValidationError { error_type: EmailError::TrailingHyphen, span: 12..12 }),
            ),
            (
                "example@inv-.-alid.com",
                Err(EmailValidationError { error_type: EmailError::TrailingHyphen, span: 12..12 }),
            ),
            (
                r#"test@example.com\n\n<script src="x.js">"#,
                Err(EmailValidationError {
                    error_type: EmailError::InvalidDomainCharacter,
                    span: 16..16,
                }),
            ),
            (
                r#""\\\011"@here.com"#,
                Err(EmailValidationError {
                    error_type: EmailError::InvalidUserCharacter,
                    span: 0..0,
                }),
            ),
            (
                r#""\\\012"@here.com"#,
                Err(EmailValidationError {
                    error_type: EmailError::InvalidUserCharacter,
                    span: 0..0,
                }),
            ),
            (
                "trailingdot@shouldfail.com.",
                Err(EmailValidationError { error_type: EmailError::TrailingPeriod, span: 26..26 }),
            ),
            // Trailing newlines in username or domain not allowed
            (
                "a@b.com\n",
                Err(EmailValidationError {
                    error_type: EmailError::InvalidDomainCharacter,
                    span: 7..7,
                }),
            ),
            (
                "a\n@b.com",
                Err(EmailValidationError {
                    error_type: EmailError::InvalidUserCharacter,
                    span: 1..1,
                }),
            ),
            (
                r#""test@test"\n@example.com"#,
                Err(EmailValidationError {
                    error_type: EmailError::InvalidUserCharacter,
                    span: 0..0,
                }),
            ),
            (
                "a@[127.0.0.1]\n",
                Err(EmailValidationError {
                    error_type: EmailError::InvalidDomainCharacter,
                    span: 2..2,
                }),
            ),
            // underscores are not allowed
            (
                "John.Doe@exam_ple.com",
                Err(EmailValidationError {
                    error_type: EmailError::InvalidDomainCharacter,
                    span: 13..13,
                }),
            ),
            (
                "invalid@example..com",
                Err(EmailValidationError { error_type: EmailError::EmptySubdomain, span: 15..16 }),
            ),
            (
                "invalid@.example.com",
                Err(EmailValidationError { error_type: EmailError::LeadingPeriod, span: 8..8 }),
            ),
        ];

        for (input, expected) in tests {
            // println!("{} - {}", input, expected);
            assert_eq!(
                validate_email_span(input),
                expected,
                "Email `{input}` was not classified correctly",
            );
        }
    }
}
