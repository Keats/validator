#[cfg(feature = "card")]
pub mod cards;
pub mod contains;
pub mod does_not_contain;
pub mod email;
pub mod ip;
pub mod length;
pub mod must_match;
// pub mod nested;
#[cfg(feature = "unic")]
pub mod non_control_character;
pub mod range;
pub mod regex;
pub mod required;
pub mod urls;
