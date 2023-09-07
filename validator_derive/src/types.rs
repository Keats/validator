use darling::util::Override;
use darling::{FromField, FromMeta};

use proc_macro_error::abort;
use syn::spanned::Spanned;
use syn::{Expr, Field, Ident, Path};

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
    pub fn validate(&self, struct_ident: &Ident, all_fields: &Vec<&Field>, current_field: &Field) {
        let field_name = self.ident.clone().expect("Field is not a named field").to_string();
        let field_attrs = &current_field.attrs;

        if let Some(custom) = &self.custom {
            // If function is not a path
            if let Err(e) = &custom.function {
                abort!(
                    e.span(), "Invalid attribute #[validate(custom(...))] on field `{}`:", field_name;
                    note = "Invalid argument for `custom` validator, only paths are allowed";
                    help = "Try formating the argument like `path::to::function` or `\"path::to::function\"`"
                );
            }
        }

        if let Some(length) = &self.length {
            // If length has both `equal` and `min` or `max` argument
            if length.equal.is_some() && (length.min.is_some() || length.max.is_some()) {
                abort! {
                    length.equal.clone().unwrap().span(), "Invalid attribute #[validate(length(...))] on field `{}`:", field_name;
                    note = "Both `equal` and `min` or `max` have been set";
                    help = "Exclusively use either the `equal` or `min` and `max` attributes"
                }
            }

            // Check if validator has no arguments
            if length.equal.is_none() && length.min.is_none() && length.max.is_none() {
                abort!(
                    get_attr(field_attrs, "length").unwrap(), "Invalid attribute #[validate(length(...))] on field `{}`:", field_name;
                    note = "Validator `length` requires at least 1 argument";
                    help = "Add the argument `equal`, `min` or `max`"
                )
            }
        }

        if let Some(must_match) = &self.must_match {
            let other_field = must_match
                .other
                .get_ident()
                .expect("Cannot get ident from `other` field value")
                .to_string();

            // Check if the other field exists
            if !all_fields.iter().any(|f| f.ident.clone().unwrap().to_string() == other_field) {
                abort!(
                    must_match.other.span(), "Invalid attribute for #[validate(must_match(...))] on field `{}`:", field_name;
                    note =  "The `other` field doesn't exist in the struct `{}`", struct_ident;
                    help = "Add the field `{}` to the struct", other_field
                )
            }
        }

        if let Some(range) = &self.range {
            // Check if validator has no arguments
            if range.min.is_none()
                && range.max.is_none()
                && range.exclusive_min.is_none()
                && range.exclusive_max.is_none()
            {
                abort!(
                    get_attr(field_attrs, "range").unwrap(),  "Invalid attribute #[validate(range(...))] on field `{}`:", field_name;
                    note = "Validator `range` requires at least 1 argument";
                    help = "Add the argument `min` or `max`, `exclusive_min` or `exclusive_max`"
                )
            }
        }
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
    pub other: Path,
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
    pub skip_on_field_errors: Option<bool>,
    pub message: Option<String>,
    pub code: Option<String>,
}
