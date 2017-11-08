pub mod ip;
pub mod email;
pub mod length;
pub mod range;
pub mod urls;
pub mod must_match;
pub mod contains;
pub mod cards;
#[cfg(feature = "phone")]
pub mod phone;

/// Contains all the validators that can be used
///
/// In this crate as it's not allowed to export more than the proc macro
/// in a proc macro crate
#[derive(Debug, Clone, PartialEq)]
pub enum Validator {
    Email,
    Url,
    // String is the path to the function
    Custom(String),
    // String is the name of the field to match
    MustMatch(String),
    // value is a &str or a HashMap<String, ..>
    Contains(String),
    // No implementation in this crate, it's all in validator_derive
    Regex(String),
    Range {
        min: f64,
        max: f64,
    },
    // Any value that impl HasLen can be validated with Length
    Length {
        min: Option<u64>,
        max: Option<u64>,
        equal: Option<u64>,
    },
    CreditCard,
    #[cfg(feature = "phone")]
    Phone,
}

impl Validator {
    pub fn code(&self) -> &'static str {
        match *self {
            Validator::MustMatch(_) => "must_match",
            Validator::Email => "email",
            Validator::Url => "url",
            Validator::Custom(_) => "custom",
            Validator::Contains(_) => "contains",
            Validator::Regex(_) => "regex",
            Validator::Range {..} => "range",
            Validator::Length {..} => "length",
            Validator::CreditCard => "credit_card",
            #[cfg(feature = "phone")]
            Validator::Phone => "phone",
        }
    }
}
