//! This module contains a collection of validators that can provide simple contextual error messages.
//! For example, in contrast to `validate_email`, which returns a `bool`, `validate_email_span` returns a `Result` with
//! an error type and span. This allows users to display richer error messages, giving contextual hints
//! rather than "this is invalid". For example, consider the invalid email `test@"example.com`:
//! ```
//! # use validator::{EmailError, EmailValidationError, validate_email, validate_email_span};
//! const INVALID_EMAIL: &str = r#"test@"example.com"#;
//!
//! assert_eq!(validate_email(INVALID_EMAIL), false);
//! assert_eq!(
//!     validate_email_span(INVALID_EMAIL),
//!     Err(EmailValidationError { error_type: EmailError::InvalidDomainCharacter, span: 5..5 })
//! )
//! ```

/// Span-based validation of emails according to the [HTML5 spec](https://html.spec.whatwg.org/multipage/forms.html#valid-e-mail-address)
pub mod email;
