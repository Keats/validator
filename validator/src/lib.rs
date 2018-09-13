extern crate regex;
extern crate url;
#[macro_use]
extern crate lazy_static;
extern crate idna;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "card")]
extern crate card_validate;
#[cfg(feature = "phone")]
pub extern crate phonenumber;

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
#[cfg(feature = "phone")]
pub use validation::phone::validate_phone;
pub use validation::range::validate_range;
pub use validation::urls::validate_url;
pub use validation::Validator;

pub use traits::{Contains, HasLen, Validate};
pub use types::{ValidationError, ValidationErrors, ValidationErrorsKind};
