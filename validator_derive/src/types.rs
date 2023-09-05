use darling::error::Accumulator;
use darling::util::Override;
use darling::{FromField, FromMeta};

use syn::spanned::Spanned;
use syn::{Expr, Field, Path};

use crate::utils::get_attr;

// This struct holds all the validation information on a field
// The "ident" and "ty" fields are populated by `darling`
// The others are our attributes for example:
// #[validate(email(message = "asdfg"))]
//            ^^^^^
//
#[derive(Debug, FromField, Clone)]
#[darling(attributes(validate))]
pub struct ValidateField {
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
    pub credit_card: Option<Override<Card>>,
    pub contains: Option<Contains>,
    pub does_not_contain: Option<DoesNotContain>,
    pub email: Option<Override<Email>>,
    pub ip: Option<Override<Ip>>,
    pub length: Option<Length>,
    pub must_match: Option<MustMatch>,
    pub non_control_character: Option<Override<NonControlCharacter>>,
    pub range: Option<Range>,
    pub required: Option<Override<Required>>,
    pub required_nested: Option<Override<Required>>,
    pub url: Option<Override<Url>>,
    pub regex: Option<Regex>,
    pub custom: Option<Custom>,
    pub skip: Option<bool>,
    pub nested: Option<bool>,
}

impl ValidateField {
    pub fn validate(&self, field: &Field) -> Accumulator {
        let mut errors = darling::Error::accumulator();
        let field_name = self.ident.clone().unwrap().to_string();
        let field_attrs = &field.attrs;

        if let Some(custom) = &self.custom {
            // If function is not a path
            if let Err(e) = &custom.function {
                errors.push(
                    darling::Error::custom(format!("Invalid attribute #[validate(custom(...))] on field {}:", field_name)
                ).with_span(&e.span())
                .note("Invalid argument for `custom` validator, only paths are allowed")
                .help("Try formating the argument like `path::to::function` or `\"path::to::function\"`"));
            }
        }

        if let Some(length) = &self.length {
            // If length has both `equal` and `min` or `max` argument
            if length.equal.is_some() && (length.min.is_some() || length.max.is_some()) {
                errors.push(
                    darling::Error::custom(format!(
                        "Invalid attribute #[validate(length(...))] on field {}:",
                        field_name
                    ))
                    .with_span(&length.equal.clone().unwrap().span())
                    .note("Both `equal` and `min` or `max` have been set")
                    .help("Exclusively use either the `equal` or `min` and `max` attributes"),
                )
            }

            if length.equal.is_none() && length.min.is_none() && length.max.is_none() {
                errors.push(
                    darling::Error::custom(format!(
                        "Invalid attribute #[validate(length(...))] on field {}:",
                        field_name
                    ))
                    .with_span(get_attr(field_attrs, "length").unwrap())
                    .note("Validator `length` requires at least 1 argument")
                    .help("Add the argument `equal`, `min` or `max`"),
                )
            }
        }

        errors
    }
}

// Structs to hold the validation information and to provide attributes
// The name of a field here corresponds to an attribute like
// #[validate(card(message = "something's wrong", code = "1234"))]
//                 ^^^^^^^                        ^^^^
//
#[derive(Debug, Clone, FromMeta, Default)]
pub struct Card {
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta)]
pub struct Contains {
    pub pattern: String,
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta)]
pub struct DoesNotContain {
    pub pattern: String,
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta, Default)]
pub struct Email {
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta, Default)]
pub struct Ip {
    pub v4: Option<bool>,
    pub v6: Option<bool>,
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta)]
pub struct Length {
    pub min: Option<Expr>,
    pub max: Option<Expr>,
    pub equal: Option<Expr>,
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta)]
pub struct MustMatch {
    pub other: Expr,
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta, Default)]
pub struct NonControlCharacter {
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta)]
pub struct Range {
    pub min: Option<Expr>,
    pub max: Option<Expr>,
    pub exclusive_min: Option<Expr>,
    pub exclusive_max: Option<Expr>,
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta, Default)]
pub struct Required {
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta, Default)]
pub struct Url {
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta)]
pub struct Regex {
    pub path: Expr,
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta)]
pub struct Custom {
    pub function: darling::Result<Path>,
    pub use_context: Option<bool>,
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, FromMeta)]
pub struct Schema {
    pub function: Path,
    pub use_context: Option<bool>,
    pub message: Option<String>,
    pub code: Option<String>,
}
