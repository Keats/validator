use once_cell::sync::Lazy;

use darling::util::Override;
use darling::{FromField, FromMeta};

use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{Expr, Field, Ident, Path};

use crate::tokens::cards::credit_card_tokens;
use crate::tokens::contains::contains_tokens;
use crate::tokens::custom::custom_tokens;
use crate::tokens::does_not_contain::does_not_contain_tokens;
use crate::tokens::email::email_tokens;
use crate::tokens::ip::ip_tokens;
use crate::tokens::length::length_tokens;
use crate::tokens::must_match::must_match_tokens;
use crate::tokens::nested::nested_tokens;
use crate::tokens::non_control_character::non_control_char_tokens;
use crate::tokens::range::range_tokens;
use crate::tokens::regex::regex_tokens;
use crate::tokens::required::required_tokens;
use crate::tokens::url::url_tokens;
use crate::utils::get_attr;

static OPTIONS_TYPE: [&str; 3] = ["Option|", "std|option|Option|", "core|option|Option|"];

pub(crate) static NUMBER_TYPES: Lazy<Vec<String>> = Lazy::new(|| {
    let number_types = [
        quote!(usize),
        quote!(u8),
        quote!(u16),
        quote!(u32),
        quote!(u64),
        quote!(u128),
        quote!(isize),
        quote!(i8),
        quote!(i16),
        quote!(i32),
        quote!(i64),
        quote!(i128),
        quote!(f32),
        quote!(f64),
    ];
    let mut tys = Vec::with_capacity(number_types.len() * 3);
    for ty in number_types {
        tys.push(ty.to_string());
        tys.push(quote!(Option<#ty>).to_string());
        tys.push(quote!(Option<Option<#ty> >).to_string());
    }
    tys
});

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
    pub attrs: Vec<syn::Attribute>,
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
    pub url: Option<Override<Url>>,
    pub regex: Option<Regex>,
    #[darling(multiple)]
    pub custom: Vec<Custom>,
    pub skip: Option<bool>,
    pub nested: Option<bool>,
}

impl ValidateField {
    pub fn validate(&self, struct_ident: &Ident, all_fields: &[&Field], current_field: &Field) {
        let field_name = self.ident.clone().expect("Field is not a named field").to_string();
        let field_attrs = &current_field.attrs;
        for attr in field_attrs {
            if matches!(attr.meta, syn::Meta::Path(_)) {
                abort!(
                    current_field.span(), "You need to set at least one validator on field `{}`", field_name;
                    note = "If you want nested validation, use `#[validate(nested)]`"
                )
            }
        }

        for c in &self.custom {
            // If function is not a path
            if let Err(e) = &c.function {
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
            if !all_fields.iter().any(|f| f.ident.clone().unwrap() == other_field) {
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

    /// How many Option<Option< are there before the actual field
    pub fn number_options(&self) -> u8 {
        fn find_option(mut count: u8, ty: &syn::Type) -> u8 {
            if let syn::Type::Path(p) = ty {
                let idents_of_path =
                    p.path.segments.iter().into_iter().fold(String::new(), |mut acc, v| {
                        acc.push_str(&v.ident.to_string());
                        acc.push('|');
                        acc
                    });

                if OPTIONS_TYPE.contains(&idents_of_path.as_str()) {
                    count += 1;
                    if let Some(p) = p.path.segments.first() {
                        if let syn::PathArguments::AngleBracketed(ref params) = p.arguments {
                            if let syn::GenericArgument::Type(ref ty) = params.args.first().unwrap()
                            {
                                count = find_option(count, ty);
                            }
                        }
                    }
                }
            }
            count
        }

        find_option(0, &self.ty)
    }

    pub fn if_let_option_wrapper(
        &self,
        field_name: &Ident,
        is_number_type: bool,
    ) -> (proc_macro2::TokenStream, Box<dyn Fn(proc_macro2::TokenStream) -> proc_macro2::TokenStream>)
    {
        let number_options = self.number_options();
        let field_name = field_name.clone();
        let actual_field =
            if number_options > 0 { quote!(#field_name) } else { quote!(self.#field_name) };
        let binding_pattern =
            if is_number_type { quote!(#field_name) } else { quote!(ref #field_name) };

        match number_options {
            0 => (actual_field.clone(), Box::new(move |tokens| tokens)),
            1 => (
                actual_field.clone(),
                Box::new(move |tokens| {
                    quote!(
                        if let Some(#binding_pattern) = self.#field_name {
                            #tokens
                        }
                    )
                }),
            ),
            2 => (
                actual_field.clone(),
                Box::new(move |tokens| {
                    quote!(
                        if let Some(Some(#binding_pattern)) = self.#field_name {
                            #tokens
                        }
                    )
                }),
            ),
            _ => abort!(
                field_name.span(),
                "Validation on values nested in more than 2 Option are not supported"
            ),
        }
    }

    pub(crate) fn into_tokens(self) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
        let field_name = self.ident.clone().unwrap();
        let field_name_str = self.ident.clone().unwrap().to_string();

        let type_name = self.ty.to_token_stream().to_string();
        let is_number = NUMBER_TYPES.contains(&type_name);

        let (actual_field, wrapper_closure) = self.if_let_option_wrapper(&field_name, is_number);

        let mut validations = Vec::new();
        let mut constraints = Vec::new();

        macro_rules! constraint {
            ($field:expr, $fn:expr) => {
                if let Some(value) = $field {
                    let (validation, constraint) = $fn(value);
                    validations.push(wrapper_closure(validation));
                    constraints.push(constraint);
                }
            };
        }

        // Length validation
        constraint!(self.length, |length| length_tokens(length, &actual_field, &field_name_str));

        // Email validation
        constraint!(self.email, |email| email_tokens(
            match email {
                Override::Inherit => Email::default(),
                Override::Explicit(e) => e,
            },
            &actual_field,
            &field_name_str,
        ));

        // Credit card validation
        constraint!(self.credit_card, |credit_card| credit_card_tokens(
            match credit_card {
                Override::Inherit => Card::default(),
                Override::Explicit(c) => c,
            },
            &actual_field,
            &field_name_str,
        ));

        // Url validation
        constraint!(self.url, |url| url_tokens(
            match url {
                Override::Inherit => Url::default(),
                Override::Explicit(u) => u,
            },
            &actual_field,
            &field_name_str,
        ));

        // Ip address validation
        constraint!(self.ip, |ip| ip_tokens(
            match ip {
                Override::Inherit => Ip::default(),
                Override::Explicit(i) => i,
            },
            &actual_field,
            &field_name_str,
        ));

        // Non control character validation
        constraint!(self.non_control_character, |ncc| non_control_char_tokens(
            match ncc {
                Override::Inherit => NonControlCharacter::default(),
                Override::Explicit(n) => n,
            },
            &actual_field,
            &field_name_str,
        ));

        // Range validation
        constraint!(self.range, |range| range_tokens(range, &actual_field, &field_name_str));

        // Required validation
        if let Some(required) = self.required {
            let (validation, constraint) = required_tokens(
                match required {
                    Override::Inherit => Required::default(),
                    Override::Explicit(r) => r,
                },
                &field_name,
                &field_name_str,
            );
            validations.push(validation);
            constraints.push(constraint);
        }

        // Contains validation
        constraint!(self.contains, |contains| contains_tokens(
            contains,
            &actual_field,
            &field_name_str
        ));

        // Does not contain validation
        constraint!(self.does_not_contain, |does_not_contain| does_not_contain_tokens(
            does_not_contain,
            &actual_field,
            &field_name_str
        ));

        // Must match validation
        // TODO: handle option for other
        constraint!(self.must_match, |must_match| must_match_tokens(
            must_match,
            &actual_field,
            &field_name_str
        ));

        // Regex validation
        constraint!(self.regex, |regex| regex_tokens(regex, &actual_field, &field_name_str));

        // Custom validation
        // We try to be smart when passing arguments
        let is_cow = type_name.contains("Cow <");
        let custom_actual_field = if is_cow {
            quote!(#actual_field.as_ref())
        } else if is_number || type_name.starts_with("&") {
            quote!(#actual_field)
        } else {
            quote!(&#actual_field)
        };
        for c in self.custom {
            let (validation, constraint) = custom_tokens(c, &custom_actual_field, &field_name_str);
            validations.push(wrapper_closure(validation));
            constraints.push(constraint);
        }

        if let Some(true) = self.nested {
            let (validation, constraint) = nested_tokens(&actual_field, &field_name_str, &self.ty);
            validations.push(wrapper_closure(validation));
            constraints.push(constraint);
        }

        (
            quote! {
                #(#validations)*
            },
            quote! {
                #(#constraints)*
            },
        )
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
