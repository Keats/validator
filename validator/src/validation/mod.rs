pub use validator_types::Validator;

#[cfg(feature = "card")]
pub mod cards;
pub mod contains;
pub mod email;
pub mod ip;
pub mod length;
pub mod must_match;
pub mod must_not_match;
#[cfg(feature = "unic")]
pub mod non_control_character;
#[cfg(feature = "phone")]
pub mod phone;
pub mod range;
pub mod required;
pub mod urls;
