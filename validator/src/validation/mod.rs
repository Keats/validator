#[cfg(feature = "card")]
pub mod cards;
pub mod contains;
pub mod does_not_contain;
#[cfg(feature = "email")]
pub mod email;
pub mod ip;
pub mod length;
pub mod must_match;
// pub mod nested;
pub mod non_control_character;
pub mod range;
#[cfg(feature = "regex")]
pub mod regex;
pub mod required;
#[cfg(feature = "url")]
pub mod urls;
