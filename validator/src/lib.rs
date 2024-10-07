//! # Example:
//!
//! ```ignore, no_run
//! use serde::Deserialize;
//!
//! // A trait that the Validate derive will impl
//! use validator::{Validate, ValidationError};
//!
//! #[derive(Debug, Validate, Deserialize)]
//! struct SignupData {
//!     #[validate(email)]
//!     mail: String,
//!     #[validate(url)]
//!     site: String,
//!     #[validate(length(min = 1), custom(function = "validate_unique_username"))]
//!     #[serde(rename = "firstName")]
//!     first_name: String,
//!     #[validate(range(min = 18, max = 20))]
//!     age: u32,
//! }
//!
//! fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
//!     if username == "xXxShad0wxXx" {
//!         // the value of the username will automatically be added later
//!         return Err(ValidationError::new("terrible_username"));
//!     }
//!
//!     Ok(())
//! }
//!
//! match signup_data.validate() {
//!   Ok(_) => (),
//!   Err(e) => return e;
//! };
//! ```
//!
//! # Available Validations:
//! | Validation              | Notes                                                 |
//! | ----------------------- | ----------------------------------------------------- |
//! | `email`                 |                                                       |
//! | `url`                   |                                                       |
//! | `length`                |                                                       |
//! | `range`                 |                                                       |
//! | `must_match`            |                                                       |
//! | `contains`              |                                                       |
//! | `does_not_contain`      |                                                       |
//! | `custom`                |                                                       |
//! | `regex`                 |                                                       |
//! | `credit_card`           | (Requires the feature `card` to be enabled)           |
//! | `non_control_character` | (Required the feature `unic` to be enabled)           |
//! | `required`              |                                                       |
//!
//! [Checkout the project README of an in-depth usage description with examples.](https://github.com/Keats/validator/blob/master/README.md)
//!
//! # Installation:
//! Add the validator to the dependencies in the Cargo.toml file.
//!
//! ```toml
//! [dependencies]
//! validator = { version = "0.16", features = ["derive"] }
//! ```

mod display_impl;
mod traits;
mod types;
mod validation;

#[cfg(feature = "card")]
pub use validation::cards::ValidateCreditCard;
pub use validation::contains::ValidateContains;
pub use validation::does_not_contain::ValidateDoesNotContain;
pub use validation::email::ValidateEmail;
pub use validation::ip::ValidateIp;
pub use validation::length::ValidateLength;
pub use validation::must_match::validate_must_match;
#[cfg(feature = "unic")]
pub use validation::non_control_character::ValidateNonControlCharacter;
pub use validation::range::ValidateRange;
pub use validation::regex::{AsRegex, ValidateRegex};
pub use validation::required::ValidateRequired;
pub use validation::urls::ValidateUrl;

pub use traits::{Validate, ValidateArgs};
pub use types::{ValidationError, ValidationErrors, ValidationErrorsKind};

#[cfg(feature = "derive")]
pub use validator_derive::Validate;
