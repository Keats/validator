mod traits;
mod types;
mod validation;

#[cfg(feature = "card")]
pub use validation::cards::validate_credit_card;
pub use validation::contains::validate_contains;
pub use validation::email::validate_email;
pub use validation::ip::{validate_ip, validate_ip_v4, validate_ip_v6};
pub use validation::length::validate_length;
pub use validation::must_match::validate_must_match;
#[cfg(feature = "unic")]
pub use validation::non_control_character::validate_non_control_character;
#[cfg(feature = "phone")]
pub use validation::phone::validate_phone;
pub use validation::range::validate_range_generic;

#[deprecated(since = "0.12.0", note = "Please use the validate_range_generic function instead")]
#[allow(deprecated)]
pub use validation::range::validate_range;

pub use validation::required::validate_required;
pub use validation::urls::validate_url;
pub use validation::Validator;

pub use traits::{Contains, HasLen, Validate};
pub use types::{ValidationError, ValidationErrors, ValidationErrorsKind};

#[cfg(feature = "derive")]
pub use validator_derive::Validate;
