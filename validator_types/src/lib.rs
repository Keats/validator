/// Contains all the validators that can be used
///
/// In this crate as it's not allowed to export more than the proc macro
/// in a proc macro crate
#[derive(Debug, Clone, PartialEq)]
pub enum Validator {
    Email,
    Url,
    Custom {
        /// This is the name of the function that should be called
        function: String,
        /// This is the argument type that can be passed in with a macro
        argument_type: Option<String>,
        /// This option stores the way to access the actual value to pass it on
        /// This will be set by impl_validate when the the ValidateArgs type is defined
        argument_access: Option<String>,
    },
    // String is the name of the field to match
    MustMatch(String),
    // value is a &str or a HashMap<String, ..>
    Contains(String),
    // No implementation in this crate, it's all in validator_derive
    Regex(String),
    Range {
        min: Option<ValueOrPath<f64>>,
        max: Option<ValueOrPath<f64>>,
    },
    // Any value that impl HasLen can be validated with Length
    Length {
        min: Option<ValueOrPath<u64>>,
        max: Option<ValueOrPath<u64>>,
        equal: Option<ValueOrPath<u64>>,
    },
    #[cfg(feature = "card")]
    CreditCard,
    #[cfg(feature = "phone")]
    Phone,
    Nested,
    #[cfg(feature = "unic")]
    NonControlCharacter,
    Required,
    RequiredNested,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueOrPath<T: std::fmt::Debug + Clone + PartialEq> {
    Value(T),
    Path(String),
}

impl Validator {
    pub fn code(&self) -> &'static str {
        match *self {
            Validator::MustMatch(_) => "must_match",
            Validator::Email => "email",
            Validator::Url => "url",
            Validator::Custom { .. } => "custom",
            Validator::Contains(_) => "contains",
            Validator::Regex(_) => "regex",
            Validator::Range { .. } => "range",
            Validator::Length { .. } => "length",
            #[cfg(feature = "card")]
            Validator::CreditCard => "credit_card",
            #[cfg(feature = "phone")]
            Validator::Phone => "phone",
            Validator::Nested => "nested",
            #[cfg(feature = "unic")]
            Validator::NonControlCharacter => "non_control_character",
            Validator::Required => "required",
            Validator::RequiredNested => "required_nested",
        }
    }
}
